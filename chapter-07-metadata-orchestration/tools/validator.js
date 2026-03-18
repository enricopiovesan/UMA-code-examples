#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";
import yaml from "js-yaml";
import Ajv from "ajv";

const ajv = new Ajv({ allErrors: true });
const contractsDir = path.join(process.cwd(), "contracts");
const schemasDir = path.join(contractsDir, "schemas");
const logsDir = path.join(process.cwd(), "logs");

function sha256(s) { return crypto.createHash("sha256").update(s).digest("hex"); }
function loadYaml(p) { return yaml.load(fs.readFileSync(p, "utf-8")); }
function requireFile(p) { if (!fs.existsSync(p)) { console.error("[error] file.missing", p); process.exit(2);} }

function validatePolicyDigest() {
  const policyPath = path.join(schemasDir, "policy.standard.v1.json");
  requireFile(policyPath);
  const raw = fs.readFileSync(policyPath, "utf-8");
  const digest = sha256(raw);
  console.log("[info] policy.digest", digest);
  return digest;
}

function readContracts() {
  const files = fs.readdirSync(contractsDir).filter(f => f.endsWith(".contract.yaml"));
  return files.map(f => ({ name: f, data: loadYaml(path.join(contractsDir, f)) }));
}

function validateContractsSchema(contracts) {
  const schema = JSON.parse(fs.readFileSync(path.join(schemasDir, "uma.contract.v1.json"), "utf-8"));
  const validate = ajv.compile(schema);
  let okAll = true;
  for (const c of contracts) {
    const ok = validate(c.data);
    if (!ok) {
      okAll = false;
      console.error("[error] contract.schema.failed", c.name, ajv.errorsText(validate.errors || []));
    } else {
      console.log("[info] contract.schema.passed", c.name);
    }
  }
  if (!okAll) process.exit(3);
}

function versionCompatible(a, b) {
  const pa = a.split(".").map(Number);
  const pb = b.split(".").map(Number);
  return pa[0] === pb[0];
}

function checkVersions(contracts) {
  const tagger = contracts.find(c => c.data.name === "image.tagger");
  const logger = contracts.find(c => c.data.name === "telemetry.logger");
  if (tagger && logger) {
    const ok = versionCompatible(tagger.data.version, logger.data.version);
    console.log(ok ? "[info] version.compatible image.tagger ↔ telemetry.logger"
                   : "[warn] version.mismatch image.tagger ↔ telemetry.logger");
  }
}

function auditDrift() {
  const telemetryLog = path.join(logsDir, "telemetry.jsonl");
  if (!fs.existsSync(telemetryLog)) {
    console.log("[warn] audit.skip no telemetry.jsonl found");
    return;
  }
  const lines = fs.readFileSync(telemetryLog, "utf-8").trim().split("\n").filter(Boolean);
  const latencies = lines.map(l => { try { return JSON.parse(l); } catch { return null; } })
                         .filter(x => x && x.metric === "uma.qos.latency.ms")
                         .map(x => x.value).sort((a,b)=>a-b);
  if (latencies.length === 0) { console.log("[warn] audit.skip no latency metrics"); return; }
  const idx = Math.max(0, Math.floor(0.99*latencies.length)-1);
  const p99 = latencies[idx];
  const target = 50;
  if (p99 > target) console.log(`[warn] drift.latency observed ${p99}ms > ${target}ms target`);
  else console.log(`[info] drift.latency within ${p99}ms ≤ ${target}ms target`);
}

function main() {
  validatePolicyDigest();
  const contracts = readContracts();
  validateContractsSchema(contracts);
  checkVersions(contracts);
  if (process.argv[2] === "audit") auditDrift();
}
main();
