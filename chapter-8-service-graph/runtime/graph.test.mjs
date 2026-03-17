import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { mkdtemp, mkdir, writeFile } from "node:fs/promises";
import os from "node:os";
import { buildGraph, loadContracts, loadScenario, validateContract } from "./graph_lib.mjs";

const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");

test("lab3 graph connects the upload, tagger, and indexer services", async () => {
  const { graph } = await loadScenario(rootDir, "lab3-indexer");

  assert.equal(graph.services.length, 3);
  assert.equal(graph.services[0].serviceVersion, "1.0.0");
  assert.deepEqual(graph.edges, [
    { from: "image-tagger", event: "image.tagged", to: "metadata-indexer" },
    { from: "upload-service", event: "image.uploaded", to: "image-tagger" },
  ]);
  assert.deepEqual(graph.waiting, []);
});

test("broken compatibility moves the downstream service into waiting", async () => {
  const { graph } = await loadScenario(rootDir, "lab4-broken-compat");

  assert.deepEqual(graph.edges, [
    { from: "upload-service", event: "image.uploaded", to: "image-tagger" },
  ]);
  assert.deepEqual(graph.waiting, [
    { service: "metadata-indexer", event: "image.tagged" },
  ]);
});

test("contract validation rejects malformed contracts", async () => {
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
        capabilities: "media.ingest",
        events: {
          consumes: [],
          emits: [],
        },
        io: {
          inputSchema: "contracts/schemas/image-upload.request.v1.json",
          outputSchema: "contracts/schemas/image-upload.response.v1.json",
        },
      },
      "services/bad-service.json",
      rootDir,
    ),
    /"capabilities" must be an array/,
  );
});

test("contract validation rejects missing schema references", async () => {
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

test("contract validation rejects duplicate capabilities in the same service", async () => {
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
        capabilities: [
          { id: "media.ingest", version: "1.0" },
          { id: "media.ingest", version: "1.0" },
        ],
        events: {
          consumes: [],
          emits: [{ name: "image.uploaded", schema: "contracts/schemas/image-uploaded.event.v1.json" }],
        },
        io: {
          inputSchema: "contracts/schemas/image-upload.request.v1.json",
          outputSchema: "contracts/schemas/image-upload.response.v1.json",
        },
      },
      "services/bad-service.json",
      rootDir,
    ),
    /duplicate capability/,
  );
});

test("contract validation rejects duplicate events in the same list", async () => {
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
          emits: [
            { name: "image.uploaded", schema: "contracts/schemas/image-uploaded.event.v1.json" },
            { name: "image.uploaded", schema: "contracts/schemas/image-uploaded.event.v1.json" },
          ],
        },
        io: {
          inputSchema: "contracts/schemas/image-upload.request.v1.json",
          outputSchema: "contracts/schemas/image-upload.response.v1.json",
        },
      },
      "services/bad-service.json",
      rootDir,
    ),
    /duplicate event/,
  );
});

test("loading contracts rejects duplicate service ids in one scenario", async () => {
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), "chapter8-"));
  const scenarioDir = path.join(tempRoot, "scenarios", "duplicate-service-ids");
  const servicesDir = path.join(scenarioDir, "services");
  const schemasDir = path.join(tempRoot, "contracts", "schemas");

  await mkdir(servicesDir, { recursive: true });
  await mkdir(schemasDir, { recursive: true });

  const schemaFiles = {
    "image-upload.request.v1.json": { type: "object" },
    "image-upload.response.v1.json": { type: "object" },
    "image-uploaded.event.v1.json": { type: "object" },
  };

  for (const [name, body] of Object.entries(schemaFiles)) {
    await writeFile(path.join(schemasDir, name), `${JSON.stringify(body, null, 2)}\n`, "utf8");
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
