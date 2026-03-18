import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import fs from "node:fs";
import { bindContracts, findPolicyViolation, loadYaml, matchPattern, summarizeRunnerOutput } from "./lib.mjs";

const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");

test("wildcard subscriptions bind to emitted image events", () => {
  const tagger = loadYaml(path.join(rootDir, "contracts", "image.tagger.contract.yaml"));
  const logger = loadYaml(path.join(rootDir, "contracts", "telemetry.logger.contract.yaml"));

  const bindings = bindContracts(tagger, logger);
  assert.deepEqual(bindings, [
    {
      event: "image.analyzed.v1",
      schema: "contracts/schemas/image.analyzed.v1.json",
    },
  ]);
});

test("policy file denies evaluator placement in browser", () => {
  const policy = JSON.parse(
    fs.readFileSync(path.join(rootDir, "contracts", "policies", "org.telemetry.standard.json"), "utf-8"),
  );
  const evaluator = loadYaml(path.join(rootDir, "contracts", "ai.model.evaluator.contract.yaml"));

  assert.equal(findPolicyViolation(policy, evaluator), "policy.deny forbid_evaluator_in_browser");
});

test("exact and wildcard event matches behave predictably", () => {
  assert.equal(matchPattern("image.*", "image.analyzed.v1"), true);
  assert.equal(matchPattern("image.analyzed.v1", "image.analyzed.v1"), true);
  assert.equal(matchPattern("image.analyzed.v1", "image.analyzed.v2"), false);
});

test("runner output summary captures the parity-critical fields", () => {
  const summary = summarizeRunnerOutput(`
    [warn] policy.violation policy.deny forbid_evaluator_in_browser continuing due to fail-open
    [info] binding.created image.analyzed.v1 → telemetry.logger
    [info] binding.created image.analyzed.v1 → edge.cache
    [info] validation.passed event_schema=image.analyzed.v1
    [info] telemetry.ok {"source":"telemetry.logger","event":"image.analyzed.v1","status":"passed"}
    [info] cache.ok {"source":"edge.cache","event":"image.analyzed.v1","status":"passed"}
    [info] evaluator.ok {"id":"img-001","score":0.7}
  `);

  assert.deepEqual(summary.bindings, [
    "[info] binding.created image.analyzed.v1 → edge.cache",
    "[info] binding.created image.analyzed.v1 → telemetry.logger",
  ]);
  assert.match(summary.policyLine, /policy\.deny/);
  assert.match(summary.validationLine, /validation\.passed/);
  assert.equal(summary.telemetry.status, "passed");
  assert.equal(summary.cache.status, "passed");
  assert.equal(summary.evaluator.score, 0.7);
});
