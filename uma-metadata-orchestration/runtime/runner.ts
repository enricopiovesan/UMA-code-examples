import fs from "node:fs";
import { execFileSync } from "node:child_process";
import path from "node:path";
import Ajv from "ajv";

const ajv = new Ajv({ allErrors: true });

type Contract = {
  name: string;
  version: string;
  events?: {
    emits?: { name: string; schema: string }[];
    subscribes?: { pattern: string; policy?: string; schema?: string }[];
  };
  constraints?: { placement?: string[] };
  policies?: { requires?: string[] };
  orchestration?: any;
};

function loadYaml(p: string): Contract {
  const yaml = require("js-yaml");
  return yaml.load(fs.readFileSync(p, "utf-8"));
}

function matchPattern(pat: string, name: string) {
  if (pat.endsWith(".*")) return name.startsWith(pat.slice(0, -2));
  return pat === name;
}

function bind(pub: Contract, sub: Contract) {
  const emits = pub.events?.emits || [];
  const subs = sub.events?.subscribes || [];
  const bindings: { event: string; schema: string }[] = [];
  for (const e of emits) for (const s of subs)
    if (matchPattern(s.pattern, e.name)) bindings.push({ event: e.name, schema: e.schema });
  return bindings;
}

function runWasmtime(wasmPath: string, input: any): any {
  const out = execFileSync("wasmtime", [wasmPath], { input: JSON.stringify(input) });
  return JSON.parse(out.toString("utf-8"));
}

async function main() {
  const contractsDir = path.join(process.cwd(), "contracts");
  const tagger = loadYaml(path.join(contractsDir, "image.tagger.contract.yaml"));
  const logger = loadYaml(path.join(contractsDir, "telemetry.logger.contract.yaml"));
  const orchestrator = loadYaml(path.join(contractsDir, "ai.workflow.orchestrator.contract.yaml"));
  const edgeCache = loadYaml(path.join(contractsDir, "edge.cache.contract.yaml"));

  const bindingsTL = bind(tagger, logger);
  const bindingsTC = bind(tagger, edgeCache);
  if (bindingsTL.length === 0) {
    console.log("[warn] no binding created for telemetry.logger");
  } else {
    console.log(`[info] binding.created ${bindingsTL.map(b=>b.event).join(", ")} → telemetry.logger`);
  }
  if (bindingsTC.length === 0) {
    console.log("[warn] no binding created for edge.cache");
  } else {
    console.log(`[info] binding.created ${bindingsTC.map(b=>b.event).join(", ")} → edge.cache`);
  }

  // Execute publisher via WASI
  const wasmPath = path.join("services", "image.tagger", "target", "wasm32-wasi", "release", "image_tagger.wasm");
  const input = { id: "img-001", bytes: Array.from({length: 8}, (_,i)=>i) };
  const published = runWasmtime(wasmPath, input);

  // Validate with schema
  const schemaPath = path.join("contracts", "schemas", "image.analyzed.v1.json");
  const schema = JSON.parse(fs.readFileSync(schemaPath, "utf-8"));
  const ok = ajv.validate(schema, published);
  if (ok) console.log("[info] validation.passed event_schema=image.analyzed.v1");
  else {
    console.error("[error] validation.failed", ajv.errorsText());
    process.exit(2);
  }

  // Dispatch to telemetry.logger (TypeScript build assumed to dist/)
  const loggerMod = await import(path.resolve("services/telemetry.logger/dist/index.js"));
  const validateFn = loggerMod.createValidator(schemaPath);
  const tval = validateFn(published);
  console.log("[info] telemetry." + (tval.status === "passed" ? "ok" : "error"), JSON.stringify(tval));

  // Dispatch to edge.cache via WASI
  const edgeWasm = path.join("services", "edge.cache", "target", "wasm32-wasi", "release", "edge_cache.wasm");
  const cacheOut = runWasmtime(edgeWasm, published);
  console.log("[info] cache." + (cacheOut.status === "passed" ? "ok" : "error"), JSON.stringify(cacheOut));
}

main().catch(e => { console.error(e); process.exit(1); });
