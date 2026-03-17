import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { evaluateTrust, listScenarios, loadScenario, validateServiceContract } from "./trust_lib.mjs";

const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");

test("lab1 trusted service is allowed", async () => {
  const report = evaluateTrust(await loadScenario(rootDir, "lab1-trusted-service"));
  assert.equal(report.outcome, "allow");
  assert.equal(report.auditLog[0].reason, "execution.trusted");
});

test("lab2 undeclared permission is denied", async () => {
  const report = evaluateTrust(await loadScenario(rootDir, "lab2-undeclared-permission"));
  assert.equal(report.outcome, "deny");
  assert.equal(report.auditLog[0].reason, "permission.undeclared");
});

test("lab3 untrusted dependency is denied", async () => {
  const report = evaluateTrust(await loadScenario(rootDir, "lab3-untrusted-dependency"));
  assert.equal(report.outcome, "deny");
  assert.equal(report.auditLog[0].reason, "dependency.provenance.untrusted");
});

test("lab4 forbidden communication is denied", async () => {
  const report = evaluateTrust(await loadScenario(rootDir, "lab4-forbidden-communication"));
  assert.equal(report.outcome, "deny");
  assert.equal(report.auditLog.at(-1).reason, "communication.forbidden");
});

test("lab5 restored communication is allowed", async () => {
  const report = evaluateTrust(await loadScenario(rootDir, "lab5-restored-compliance"));
  assert.equal(report.outcome, "allow");
  assert.equal(report.auditLog.at(-1).reason, "communication.trusted");
});

test("scenario listing returns the reader lab order", async () => {
  assert.deepEqual(await listScenarios(rootDir), [
    "lab1-trusted-service",
    "lab2-undeclared-permission",
    "lab3-untrusted-dependency",
    "lab4-forbidden-communication",
    "lab5-restored-compliance"
  ]);
});

test("unknown scenario fails with a helpful error", async () => {
  await assert.rejects(loadScenario(rootDir, "does-not-exist"), /unknown scenario/);
});

test("service validation rejects an untrusted contract kind", async () => {
  await assert.rejects(
    validateServiceContract(
      {
        kind: "uma.service-contract",
        specVersion: "1.0",
        service: {
          id: "bad-service",
          version: "1.0.0",
          summary: "Broken service",
          publisher: "uma.book.team",
          trustTier: "internal",
          placements: ["cloud"]
        },
        capabilities: [{ id: "x", version: "1.0" }],
        permissions: ["storage.read"],
        dependencies: [{ name: "dep", version: "1.0", provenance: "verified", checksum: "sha256-dep" }],
        events: { consumes: [], emits: [] },
        io: {
          inputSchema: "contracts/schemas/redaction.request.v1.json",
          outputSchema: "contracts/schemas/redaction.response.v1.json"
        }
      },
      "services/bad-service.json",
      rootDir,
    ),
    /uma.trusted-service-contract/,
  );
});
