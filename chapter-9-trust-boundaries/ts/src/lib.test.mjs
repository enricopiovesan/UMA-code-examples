import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { diffReports, evaluateTrust, listScenarios, loadScenario, validateServiceContract } from "./lib.mjs";

const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");

test("lab1 is allowed", async () => {
  const report = evaluateTrust(await loadScenario(rootDir, "lab1-trusted-service"));
  assert.equal(report.outcome, "allow");
  assert.equal(report.auditLog[0].reason, "execution.trusted");
});

test("undeclared permission is denied", async () => {
  const report = evaluateTrust(await loadScenario(rootDir, "lab2-undeclared-permission"));
  assert.equal(report.outcome, "deny");
  assert.equal(report.auditLog[0].reason, "permission.undeclared");
});

test("forbidden partner communication is denied", async () => {
  const report = evaluateTrust(await loadScenario(rootDir, "lab4-forbidden-communication"));
  assert.equal(report.outcome, "deny");
  assert.equal(report.auditLog.at(-1).reason, "communication.forbidden");
});

test("lab4 to lab5 diff restores communication", async () => {
  const diff = await diffReports(rootDir, "lab4-forbidden-communication", "lab5-restored-compliance");
  assert.match(diff, /Outcome: deny -> allow/);
  assert.match(diff, /allow:communication:upload-bridge->internal-audit-sink:communication.trusted/);
});

test("missing schema is rejected while loading service", async () => {
  await assert.rejects(
    validateServiceContract(
      {
        kind: "uma.trusted-service-contract",
        specVersion: "1.0",
        service: {
          id: "bad-service",
          version: "1.0.0",
          summary: "Broken service",
          publisher: "uma.book.team",
          trustTier: "internal",
          placements: ["cloud"],
        },
        capabilities: [{ id: "x", version: "1.0" }],
        permissions: ["storage.read"],
        dependencies: [{ name: "dep", version: "1.0", provenance: "verified", checksum: "sha256-dep" }],
        events: {
          consumes: [],
          emits: [{ name: "case.redacted", schema: "contracts/schemas/missing.event.v1.json" }],
        },
        io: {
          inputSchema: "contracts/schemas/redaction.request.v1.json",
          outputSchema: "contracts/schemas/redaction.response.v1.json",
        },
      },
      "services/bad-service.json",
      rootDir,
    ),
    /missing file/,
  );
});

test("scenario list is stable", async () => {
  assert.deepEqual(await listScenarios(rootDir), [
    "lab1-trusted-service",
    "lab2-undeclared-permission",
    "lab3-untrusted-dependency",
    "lab4-forbidden-communication",
    "lab5-restored-compliance",
  ]);
});

test("duplicate capability is rejected", async () => {
  await assert.rejects(
    validateServiceContract(
      {
        kind: "uma.trusted-service-contract",
        specVersion: "1.0",
        service: {
          id: "bad-service",
          version: "1.0.0",
          summary: "Broken service",
          publisher: "uma.book.team",
          trustTier: "internal",
          placements: ["cloud"],
        },
        capabilities: [
          { id: "x", version: "1.0" },
          { id: "x", version: "1.0" },
        ],
        permissions: ["storage.read"],
        dependencies: [{ name: "dep", version: "1.0", provenance: "verified", checksum: "sha256-dep" }],
        events: { consumes: [], emits: [] },
        io: {
          inputSchema: "contracts/schemas/redaction.request.v1.json",
          outputSchema: "contracts/schemas/redaction.response.v1.json",
        },
      },
      "services/bad-service.json",
      rootDir,
    ),
    /duplicate capability/,
  );
});
