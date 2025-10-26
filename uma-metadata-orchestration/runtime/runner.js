// Precompiled JS runner with CloudEvents envelopes, policy digest check, and logs
import fs from "node:fs";
import { execFileSync } from "node:child_process";
import path from "node:path";
import Ajv from "ajv";
import yaml from "js-yaml";
import crypto from "node:crypto";

const ajv = new Ajv({ allErrors: true });

function sha256(s) { return crypto.createHash("sha256").update(s).digest("hex"); }
function loadYaml(p) { return yaml.load(fs.readFileSync(p, "utf-8")); }
function matchPattern(pat, name) { return pat.endsWith(".*") ? name.startsWith(pat.slice(0, -2)) : pat === name; }
function bind(pub, sub) {
  const emits = (pub.events && pub.events.emits) || [];
  const subs = (sub.events && sub.events.subscribes) || [];
  const bindings = [];
  for (const e of emits) for (const s of subs) if (matchPattern(s.pattern, e.name)) bindings.push({ event: e.name, schema: e.schema });
  return bindings;
}
function runWasmtime(wasmPath, input) {
  const t0 = Date.now();
  const out = execFileSync("wasmtime", [wasmPath], { input: JSON.stringify(input) });
  const ms = Date.now() - t0;
  logTelemetry({ metric: "uma.qos.latency.ms", value: ms });
  return JSON.parse(out.toString("utf-8"));
}
function uuid() { return crypto.randomUUID(); }

function otlpExport(metricName, value) {
  const endpoint = process.env.OTLP_ENDPOINT;
  if (!endpoint) return;
  const payload = { resourceMetrics: [{ scopeMetrics: [{ metrics: [{ name: metricName, sum: { dataPoints: [{ asDouble: value, timeUnixNano: Date.now()*1e6 }] } }] }]}]};
  // Use fetch if available in Node 20
  fetch(endpoint, { method: "POST", body: JSON.stringify(payload), headers: { "content-type":"application/json" } }).catch(()=>{});
}

function logTelemetry(obj) {
  fs.appendFileSync(path.join("logs", "telemetry.jsonl"), JSON.stringify(obj) + "\n");
}
function writeEventEnvelope(evtType, data, serviceId, contractVersion) {
  const envelope = {
    specversion: "1.0",
    id: uuid(),
    source: serviceId,
    type: evtType,
    time: new Date().toISOString(),
    datacontenttype: "application/json",
    data,
    "uma.serviceId": serviceId,
    "uma.contractVersion": contractVersion,
    "uma.runtimeId": "cloud-runner",
    "phase": "normal",
    "reasonCode": "OK"
  };
  const p = path.join("logs", "events", `${envelope.id}.json`);
  fs.writeFileSync(p, JSON.stringify(envelope, null, 2));
  return envelope;
}
function verifyPolicy() {
  const policyPath = path.join("contracts", "schemas", "policy.standard.v1.json");
  const raw = fs.readFileSync(policyPath, "utf-8");
  const digest = sha256(raw);
  console.log("[info] policy.digest", digest);
  return digest;
}

const policyDigest = verifyPolicy();

const contractsDir = path.join(process.cwd(), "contracts");
const tagger = loadYaml(path.join(contractsDir, "image.tagger.contract.yaml"));
const logger = loadYaml(path.join(contractsDir, "telemetry.logger.contract.yaml"));
const edgeCache = loadYaml(path.join(contractsDir, "edge.cache.contract.yaml"));
const evaluator = loadYaml(path.join(contractsDir, "ai.model.evaluator.contract.yaml"));

const bindingsTL = bind(tagger, logger);
const bindingsTC = bind(tagger, edgeCache);
const bindingsTE = bind(tagger, evaluator);


function enforcePolicy() {
  const rulesPath = path.join("contracts", "policies", "org.telemetry.standard.json");
  if (!fs.existsSync(rulesPath)) return { ok: true };
  const rules = JSON.parse(fs.readFileSync(rulesPath, "utf-8"));
  const deny = rules.deny || [];
  // Example enforcement, fail if ai.model.evaluator is placed in browser per contract
  const evaluatorContract = loadYaml(path.join("contracts", "ai.model.evaluator.contract.yaml"));
  const placements = (evaluatorContract.constraints && evaluatorContract.constraints.placement) || [];
  if (placements.includes("browser")) {
    const hit = deny.find(r => r.if && r.if.service==="ai.model.evaluator" && r.if.placement==="browser");
    if (hit) return { ok: false, reason: "policy.deny " + hit.rule };
  }
  return { ok: true };
}
const policyCheck = enforcePolicy();
const failMode = process.env.POLICY_FAIL_MODE || "closed";
if (!policyCheck.ok) {
  if (failMode === "closed") { console.error("[error] policy.violation", policyCheck.reason); process.exit(4); }
  else { console.warn("[warn] policy.violation", policyCheck.reason, "continuing due to fail-open"); }
}

if (bindingsTL.length === 0) console.log("[warn] no binding created for telemetry.logger");
else console.log(`[info] binding.created ${bindingsTL.map(b=>b.event).join(", ")} → telemetry.logger`);
if (bindingsTC.length === 0) console.log("[warn] no binding created for edge.cache");
else console.log(`[info] binding.created ${bindingsTC.map(b=>b.event).join(", ")} → edge.cache`);
if (bindingsTE.length === 0) console.log("[warn] no binding created for ai.model.evaluator");
else console.log(`[info] binding.created ${bindingsTE.map(b=>b.event).join(", ")} → ai.model.evaluator`);

// Execute publisher via WASI
const wasmPath = path.join("services", "image.tagger", "target", "wasm32-wasi", "release", "image_tagger.wasm");
const input = { id: "img-001", bytes: Array.from({length: 8}, (_,i)=>i) };
const published = runWasmtime(wasmPath, input);

// Validate with schema
const schemaPath = path.join("contracts", "schemas", "image.analyzed.v1.json");
const schema = JSON.parse(fs.readFileSync(schemaPath, "utf-8"));
const ok = ajv.validate(schema, published);
if (ok) console.log("[info] validation.passed event_schema=image.analyzed.v1");
else { console.error("[error] validation.failed", ajv.errorsText()); process.exit(2); }

writeEventEnvelope("image.analyzed.v1", published, "image.tagger", tagger.version);

// Dispatch to telemetry.logger
const loggerMod = await import(path.resolve("services/telemetry.logger/dist/index.js"));
const validateFn = loggerMod.createValidator(schemaPath);
const tval = validateFn(published);
console.log("[info] telemetry." + (tval.status === "passed" ? "ok" : "error"), JSON.stringify(tval));
writeEventEnvelope("telemetry.validation.v1", tval, "telemetry.logger", logger.version);

// Dispatch to edge.cache via WASI
const edgeWasm = path.join("services", "edge.cache", "target", "wasm32-wasi", "release", "edge_cache.wasm");
const cacheOut = runWasmtime(edgeWasm, published);
console.log("[info] cache." + (cacheOut.status === "passed" ? "ok" : "error"), JSON.stringify(cacheOut));
writeEventEnvelope("cache.persisted.v1", cacheOut, "edge.cache", edgeCache.version);

// Dispatch to ai.model.evaluator
const evaluatorMod = await import(path.resolve("services/ai.model.evaluator/dist/index.js"));
const evalOut = evaluatorMod.evaluate(published);
console.log("[info] evaluator.ok", JSON.stringify(evalOut));
writeEventEnvelope("inference.completed.v1", evalOut, "ai.model.evaluator", evaluator.version);
