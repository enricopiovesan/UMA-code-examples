import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { mkdtemp, mkdir, writeFile } from "node:fs/promises";
import os from "node:os";
import {
  buildGraph,
  formatGraphDiff,
  listScenarios,
  loadContracts,
  loadScenario,
  validateContract,
} from "./lib.mjs";

const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");

test("lab3 has the expected graph edges", async () => {
  const { graph } = await loadScenario(rootDir, "lab3-indexer");

  assert.equal(graph.services.length, 3);
  assert.deepEqual(graph.edges, [
    { from: "image-tagger", event: "image.tagged", to: "metadata-indexer" },
    { from: "upload-service", event: "image.uploaded", to: "image-tagger" },
  ]);
});

test("broken compatibility creates a waiting consumer", async () => {
  const { graph } = await loadScenario(rootDir, "lab4-broken-compat");

  assert.deepEqual(graph.waiting, [
    { service: "metadata-indexer", event: "image.tagged" },
  ]);
});

test("missing schema is rejected during validation", async () => {
  await assert.rejects(
    validateContract(
      {
        kind: "uma.service-contract",
        specVersion: "1.0",
        service: {
          id: "bad-service",
          version: "1.0.0",
          summary: "Broken test service",
          placements: ["cloud"],
        },
        capabilities: [{ id: "media.ingest", version: "1.0" }],
        events: {
          consumes: [],
          emits: [{ name: "image.uploaded", schema: "contracts/schemas/missing.event.v1.json" }],
        },
        io: {
          inputSchema: "contracts/schemas/image-upload.request.v1.json",
          outputSchema: "contracts/schemas/image-upload.response.v1.json",
        },
      },
      "services/bad-service.json",
      rootDir,
    ),
    /missing schema reference/,
  );
});

test("duplicate service ids are rejected", async () => {
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), "chapter8-ts-"));
  const scenarioDir = path.join(tempRoot, "scenarios", "duplicate-service-ids");
  const servicesDir = path.join(scenarioDir, "services");
  const schemasDir = path.join(tempRoot, "contracts", "schemas");

  await mkdir(servicesDir, { recursive: true });
  await mkdir(schemasDir, { recursive: true });

  for (const name of [
    "image-upload.request.v1.json",
    "image-upload.response.v1.json",
    "image-uploaded.event.v1.json",
  ]) {
    await writeFile(path.join(schemasDir, name), "{\n  \"type\": \"object\"\n}\n", "utf8");
  }

  const duplicateContract = {
    kind: "uma.service-contract",
    specVersion: "1.0",
    service: {
      id: "duplicate-service",
      version: "1.0.0",
      summary: "Duplicate id service",
      placements: ["cloud"],
    },
    capabilities: [{ id: "media.ingest", version: "1.0" }],
    events: {
      consumes: [],
      emits: [{ name: "image.uploaded", schema: "contracts/schemas/image-uploaded.event.v1.json" }],
    },
    io: {
      inputSchema: "contracts/schemas/image-upload.request.v1.json",
      outputSchema: "contracts/schemas/image-upload.response.v1.json",
    },
  };

  await writeFile(path.join(servicesDir, "service-a.json"), `${JSON.stringify(duplicateContract, null, 2)}\n`, "utf8");
  await writeFile(path.join(servicesDir, "service-b.json"), `${JSON.stringify(duplicateContract, null, 2)}\n`, "utf8");

  await assert.rejects(loadContracts(tempRoot, scenarioDir), /duplicate service id/);
});

test("graph diff reports a broken compatibility edge removal", async () => {
  const diff = await formatGraphDiff(rootDir, "lab3-indexer", "lab4-broken-compat");
  assert.match(diff, /Removed edges/);
  assert.match(diff, /metadata-indexer waiting for image.tagged/);
});

test("scenario list is stable", async () => {
  assert.deepEqual(await listScenarios(rootDir), [
    "lab1-upload-only",
    "lab2-image-tagger",
    "lab3-indexer",
    "lab4-broken-compat",
    "lab5-fixed-compat",
  ]);
});

test("graph builder marks missing emitters as waiting consumers", () => {
  const graph = buildGraph([
    {
      id: "uploader",
      serviceVersion: "1.0.0",
      summary: "Uploads images",
      placements: ["cloud"],
      capabilities: ["media.ingest"],
      capabilityVersions: [{ id: "media.ingest", version: "1.0" }],
      consumes: [],
      emits: ["image.uploaded"],
      eventSchemas: {
        consumes: [],
        emits: [{ name: "image.uploaded", schema: "contracts/schemas/image-uploaded.event.v1.json" }],
      },
      io: {
        inputSchema: "contracts/schemas/image-upload.request.v1.json",
        outputSchema: "contracts/schemas/image-upload.response.v1.json",
      },
    },
    {
      id: "downstream",
      serviceVersion: "1.0.0",
      summary: "Audits tagged images",
      placements: ["cloud"],
      capabilities: ["media.audit"],
      capabilityVersions: [{ id: "media.audit", version: "1.0" }],
      consumes: ["image.tagged"],
      emits: [],
      eventSchemas: {
        consumes: [{ name: "image.tagged", schema: "contracts/schemas/image-tagged.event.v1.json" }],
        emits: [],
      },
      io: {
        inputSchema: "contracts/schemas/metadata-index.request.v1.json",
        outputSchema: "contracts/schemas/metadata-index.response.v1.json",
      },
    },
  ]);

  assert.deepEqual(graph.waiting, [{ service: "downstream", event: "image.tagged" }]);
});
