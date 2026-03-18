import { access, readdir, readFile } from "node:fs/promises";
import path from "node:path";

function isNonEmptyString(value) {
  return typeof value === "string" && value.trim().length > 0;
}

function assertNonEmptyString(value, fieldName, servicePath) {
  if (!isNonEmptyString(value)) {
    throw new Error(`${servicePath}: "${fieldName}" must be a non-empty string`);
  }
}

function validateStringArray(value, fieldName, servicePath) {
  if (!Array.isArray(value)) {
    throw new Error(`${servicePath}: "${fieldName}" must be an array`);
  }

  for (const entry of value) {
    if (!isNonEmptyString(entry)) {
      throw new Error(`${servicePath}: "${fieldName}" entries must be non-empty strings`);
    }
  }
}

function validateCapabilityList(value, servicePath) {
  if (!Array.isArray(value)) {
    throw new Error(`${servicePath}: "capabilities" must be an array`);
  }

  const seenCapabilities = new Set();
  for (const capability of value) {
    if (typeof capability !== "object" || capability === null || Array.isArray(capability)) {
      throw new Error(`${servicePath}: each capability must be an object`);
    }

    assertNonEmptyString(capability.id, "capabilities[].id", servicePath);
    assertNonEmptyString(capability.version, "capabilities[].version", servicePath);

    const capabilityKey = `${capability.id}@${capability.version}`;
    if (seenCapabilities.has(capabilityKey)) {
      throw new Error(`${servicePath}: duplicate capability "${capabilityKey}"`);
    }
    seenCapabilities.add(capabilityKey);
  }
}

function validateEventList(value, fieldName, servicePath) {
  if (!Array.isArray(value)) {
    throw new Error(`${servicePath}: "${fieldName}" must be an array`);
  }

  const seenEvents = new Set();
  for (const eventContract of value) {
    if (typeof eventContract !== "object" || eventContract === null || Array.isArray(eventContract)) {
      throw new Error(`${servicePath}: each "${fieldName}" entry must be an object`);
    }

    assertNonEmptyString(eventContract.name, `${fieldName}[].name`, servicePath);
    assertNonEmptyString(eventContract.schema, `${fieldName}[].schema`, servicePath);

    if (seenEvents.has(eventContract.name)) {
      throw new Error(`${servicePath}: duplicate event "${eventContract.name}" in "${fieldName}"`);
    }
    seenEvents.add(eventContract.name);
  }
}

async function assertSchemaExists(rootDir, schemaPath, servicePath) {
  const absoluteSchemaPath = path.join(rootDir, schemaPath);
  try {
    await access(absoluteSchemaPath);
  } catch {
    throw new Error(`${servicePath}: missing schema reference "${schemaPath}"`);
  }
}

export async function listScenarios(rootDir) {
  const scenariosDir = path.join(rootDir, "scenarios");
  const entries = await readdir(scenariosDir, { withFileTypes: true });
  return entries.filter((entry) => entry.isDirectory()).map((entry) => entry.name).sort();
}

function normalizeContract(contract) {
  return {
    kind: contract.kind,
    specVersion: contract.specVersion,
    id: contract.service.id,
    serviceVersion: contract.service.version,
    summary: contract.service.summary,
    placements: contract.service.placements,
    capabilities: contract.capabilities.map((capability) => capability.id),
    capabilityVersions: contract.capabilities.map((capability) => ({
      id: capability.id,
      version: capability.version,
    })),
    consumes: contract.events.consumes.map((eventContract) => eventContract.name),
    emits: contract.events.emits.map((eventContract) => eventContract.name),
    eventSchemas: {
      consumes: contract.events.consumes.map((eventContract) => ({
        name: eventContract.name,
        schema: eventContract.schema,
      })),
      emits: contract.events.emits.map((eventContract) => ({
        name: eventContract.name,
        schema: eventContract.schema,
      })),
    },
    io: contract.io,
  };
}

export async function validateContract(contract, servicePath, rootDir) {
  if (typeof contract !== "object" || contract === null || Array.isArray(contract)) {
    throw new Error(`${servicePath}: contract must be an object`);
  }

  assertNonEmptyString(contract.kind, "kind", servicePath);
  assertNonEmptyString(contract.specVersion, "specVersion", servicePath);
  if (contract.kind !== "uma.service-contract") {
    throw new Error(`${servicePath}: "kind" must be "uma.service-contract"`);
  }

  if (typeof contract.service !== "object" || contract.service === null || Array.isArray(contract.service)) {
    throw new Error(`${servicePath}: "service" must be an object`);
  }

  assertNonEmptyString(contract.service.id, "service.id", servicePath);
  assertNonEmptyString(contract.service.version, "service.version", servicePath);
  assertNonEmptyString(contract.service.summary, "service.summary", servicePath);
  validateStringArray(contract.service.placements, "service.placements", servicePath);
  validateCapabilityList(contract.capabilities, servicePath);

  if (typeof contract.events !== "object" || contract.events === null || Array.isArray(contract.events)) {
    throw new Error(`${servicePath}: "events" must be an object`);
  }

  validateEventList(contract.events.consumes, "events.consumes", servicePath);
  validateEventList(contract.events.emits, "events.emits", servicePath);

  if (typeof contract.io !== "object" || contract.io === null || Array.isArray(contract.io)) {
    throw new Error(`${servicePath}: "io" must be an object`);
  }

  assertNonEmptyString(contract.io.inputSchema, "io.inputSchema", servicePath);
  assertNonEmptyString(contract.io.outputSchema, "io.outputSchema", servicePath);

  const schemaRefs = [
    contract.io.inputSchema,
    contract.io.outputSchema,
    ...contract.events.consumes.map((eventContract) => eventContract.schema),
    ...contract.events.emits.map((eventContract) => eventContract.schema),
  ];

  for (const schemaRef of schemaRefs) {
    await assertSchemaExists(rootDir, schemaRef, servicePath);
  }

  return normalizeContract(contract);
}

export async function loadContracts(rootDir, scenarioDir) {
  const servicesDir = path.join(scenarioDir, "services");
  const entries = await readdir(servicesDir, { withFileTypes: true });
  const services = [];
  const seenServiceIds = new Set();

  for (const entry of entries) {
    if (!entry.isFile() || !entry.name.endsWith(".json")) {
      continue;
    }

    const servicePath = path.join(servicesDir, entry.name);
    const source = await readFile(servicePath, "utf8");
    const parsed = JSON.parse(source);
    const contract = await validateContract(parsed, path.relative(scenarioDir, servicePath), rootDir);

    if (seenServiceIds.has(contract.id)) {
      throw new Error(`duplicate service id "${contract.id}" in ${path.relative(rootDir, scenarioDir)}`);
    }
    seenServiceIds.add(contract.id);
    services.push(contract);
  }

  services.sort((left, right) => left.id.localeCompare(right.id));
  return services;
}

export function buildGraph(services) {
  const emitters = new Map();

  for (const service of services) {
    for (const eventName of service.emits) {
      if (!emitters.has(eventName)) {
        emitters.set(eventName, []);
      }
      emitters.get(eventName).push(service.id);
    }
  }

  const edges = [];
  const waiting = [];

  for (const service of services) {
    for (const eventName of service.consumes) {
      const producers = emitters.get(eventName) ?? [];
      if (producers.length === 0) {
        waiting.push({ service: service.id, event: eventName });
        continue;
      }

      for (const producer of producers) {
        edges.push({ from: producer, event: eventName, to: service.id });
      }
    }
  }

  edges.sort((left, right) => {
    return (
      left.from.localeCompare(right.from) ||
      left.event.localeCompare(right.event) ||
      left.to.localeCompare(right.to)
    );
  });
  waiting.sort((left, right) => {
    return left.service.localeCompare(right.service) || left.event.localeCompare(right.event);
  });

  return { services, edges, waiting };
}

export function formatTextGraph(graph, scenarioName) {
  const lines = [`Scenario: ${scenarioName}`, "", "Services"];

  for (const service of graph.services) {
    lines.push(`- ${service.id} v${service.serviceVersion}`);
    lines.push(`  summary: ${service.summary}`);
    lines.push(`  placements: ${service.placements.join(", ")}`);

    for (const capability of service.capabilityVersions) {
      lines.push(`  capability: ${capability.id}@${capability.version}`);
    }
    for (const eventContract of service.eventSchemas.consumes) {
      lines.push(`  consumes: ${eventContract.name} (${eventContract.schema})`);
    }
    for (const eventContract of service.eventSchemas.emits) {
      lines.push(`  emits: ${eventContract.name} (${eventContract.schema})`);
    }
  }

  lines.push("", "Edges");
  if (graph.edges.length === 0) {
    lines.push("- none");
  } else {
    for (const edge of graph.edges) {
      lines.push(`- ${edge.from} -> ${edge.event} -> ${edge.to}`);
    }
  }

  lines.push("", "Waiting Consumers");
  if (graph.waiting.length === 0) {
    lines.push("- none");
  } else {
    for (const entry of graph.waiting) {
      lines.push(`- ${entry.service} waiting for ${entry.event}`);
    }
  }

  return `${lines.join("\n")}\n`;
}

export async function loadScenario(rootDir, scenarioName) {
  const availableScenarios = await listScenarios(rootDir);
  if (!availableScenarios.includes(scenarioName)) {
    throw new Error(
      `unknown scenario "${scenarioName}". Available scenarios: ${availableScenarios.join(", ")}`,
    );
  }

  const scenarioDir = path.join(rootDir, "scenarios", scenarioName);
  const services = await loadContracts(rootDir, scenarioDir);
  return { scenarioDir, services, graph: buildGraph(services) };
}
