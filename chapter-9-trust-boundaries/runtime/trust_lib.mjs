import { access, readdir, readFile } from "node:fs/promises";
import path from "node:path";

function isNonEmptyString(value) {
  return typeof value === "string" && value.trim().length > 0;
}

function assertNonEmptyString(value, fieldName, fileLabel) {
  if (!isNonEmptyString(value)) {
    throw new Error(`${fileLabel}: "${fieldName}" must be a non-empty string`);
  }
}

function validateStringArray(value, fieldName, fileLabel) {
  if (!Array.isArray(value)) {
    throw new Error(`${fileLabel}: "${fieldName}" must be an array`);
  }

  for (const entry of value) {
    if (!isNonEmptyString(entry)) {
      throw new Error(`${fileLabel}: "${fieldName}" entries must be non-empty strings`);
    }
  }
}

async function assertPathExists(rootDir, relativePath, fileLabel, fieldName) {
  try {
    await access(path.join(rootDir, relativePath));
  } catch {
    throw new Error(`${fileLabel}: "${fieldName}" points to missing file "${relativePath}"`);
  }
}

function validateCapabilities(value, fileLabel) {
  if (!Array.isArray(value)) {
    throw new Error(`${fileLabel}: "capabilities" must be an array`);
  }

  const seen = new Set();
  for (const capability of value) {
    if (typeof capability !== "object" || capability === null || Array.isArray(capability)) {
      throw new Error(`${fileLabel}: each capability must be an object`);
    }
    assertNonEmptyString(capability.id, "capabilities[].id", fileLabel);
    assertNonEmptyString(capability.version, "capabilities[].version", fileLabel);

    const key = `${capability.id}@${capability.version}`;
    if (seen.has(key)) {
      throw new Error(`${fileLabel}: duplicate capability "${key}"`);
    }
    seen.add(key);
  }
}

function validateDependencies(value, fileLabel) {
  if (!Array.isArray(value)) {
    throw new Error(`${fileLabel}: "dependencies" must be an array`);
  }

  for (const dependency of value) {
    if (typeof dependency !== "object" || dependency === null || Array.isArray(dependency)) {
      throw new Error(`${fileLabel}: each dependency must be an object`);
    }
    assertNonEmptyString(dependency.name, "dependencies[].name", fileLabel);
    assertNonEmptyString(dependency.version, "dependencies[].version", fileLabel);
    assertNonEmptyString(dependency.provenance, "dependencies[].provenance", fileLabel);
    assertNonEmptyString(dependency.checksum, "dependencies[].checksum", fileLabel);
  }
}

function validateEvents(value, fieldName, fileLabel) {
  if (!Array.isArray(value)) {
    throw new Error(`${fileLabel}: "${fieldName}" must be an array`);
  }

  const seen = new Set();
  for (const eventContract of value) {
    if (typeof eventContract !== "object" || eventContract === null || Array.isArray(eventContract)) {
      throw new Error(`${fileLabel}: each "${fieldName}" entry must be an object`);
    }
    assertNonEmptyString(eventContract.name, `${fieldName}[].name`, fileLabel);
    assertNonEmptyString(eventContract.schema, `${fieldName}[].schema`, fileLabel);
    if (seen.has(eventContract.name)) {
      throw new Error(`${fileLabel}: duplicate event "${eventContract.name}" in "${fieldName}"`);
    }
    seen.add(eventContract.name);
  }
}

export async function listScenarios(rootDir) {
  const scenariosDir = path.join(rootDir, "scenarios");
  const entries = await readdir(scenariosDir, { withFileTypes: true });
  return entries.filter((entry) => entry.isDirectory()).map((entry) => entry.name).sort();
}

function normalizeService(contract) {
  return {
    kind: contract.kind,
    specVersion: contract.specVersion,
    id: contract.service.id,
    version: contract.service.version,
    summary: contract.service.summary,
    publisher: contract.service.publisher,
    trustTier: contract.service.trustTier,
    placements: contract.service.placements,
    permissions: contract.permissions,
    capabilities: contract.capabilities,
    dependencies: contract.dependencies,
    consumes: contract.events.consumes.map((eventContract) => eventContract.name),
    emits: contract.events.emits.map((eventContract) => eventContract.name),
    eventSchemas: contract.events,
    io: contract.io
  };
}

export async function validateServiceContract(contract, fileLabel, rootDir) {
  if (typeof contract !== "object" || contract === null || Array.isArray(contract)) {
    throw new Error(`${fileLabel}: contract must be an object`);
  }

  assertNonEmptyString(contract.kind, "kind", fileLabel);
  assertNonEmptyString(contract.specVersion, "specVersion", fileLabel);
  if (contract.kind !== "uma.trusted-service-contract") {
    throw new Error(`${fileLabel}: "kind" must be "uma.trusted-service-contract"`);
  }

  if (typeof contract.service !== "object" || contract.service === null || Array.isArray(contract.service)) {
    throw new Error(`${fileLabel}: "service" must be an object`);
  }

  assertNonEmptyString(contract.service.id, "service.id", fileLabel);
  assertNonEmptyString(contract.service.version, "service.version", fileLabel);
  assertNonEmptyString(contract.service.summary, "service.summary", fileLabel);
  assertNonEmptyString(contract.service.publisher, "service.publisher", fileLabel);
  assertNonEmptyString(contract.service.trustTier, "service.trustTier", fileLabel);
  validateStringArray(contract.service.placements, "service.placements", fileLabel);
  validateCapabilities(contract.capabilities, fileLabel);
  validateStringArray(contract.permissions, "permissions", fileLabel);
  validateDependencies(contract.dependencies, fileLabel);

  if (typeof contract.events !== "object" || contract.events === null || Array.isArray(contract.events)) {
    throw new Error(`${fileLabel}: "events" must be an object`);
  }
  validateEvents(contract.events.consumes, "events.consumes", fileLabel);
  validateEvents(contract.events.emits, "events.emits", fileLabel);

  if (typeof contract.io !== "object" || contract.io === null || Array.isArray(contract.io)) {
    throw new Error(`${fileLabel}: "io" must be an object`);
  }
  assertNonEmptyString(contract.io.inputSchema, "io.inputSchema", fileLabel);
  assertNonEmptyString(contract.io.outputSchema, "io.outputSchema", fileLabel);

  const refs = [
    contract.io.inputSchema,
    contract.io.outputSchema,
    ...contract.events.consumes.map((entry) => entry.schema),
    ...contract.events.emits.map((entry) => entry.schema)
  ];

  for (const ref of refs) {
    await assertPathExists(rootDir, ref, fileLabel, "schema");
  }

  return normalizeService(contract);
}

export async function loadPolicy(rootDir) {
  const policyPath = path.join(rootDir, "contracts", "policies", "runtime-policy.json");
  const raw = JSON.parse(await readFile(policyPath, "utf8"));

  if (raw.kind !== "uma.runtime-trust-policy") {
    throw new Error('contracts/policies/runtime-policy.json: invalid policy kind');
  }
  validateStringArray(raw.trustedPublishers, "trustedPublishers", "runtime-policy.json");
  validateStringArray(raw.allowedTrustTiers, "allowedTrustTiers", "runtime-policy.json");

  return raw;
}

export async function loadScenario(rootDir, scenarioName) {
  const availableScenarios = await listScenarios(rootDir);
  if (!availableScenarios.includes(scenarioName)) {
    throw new Error(`unknown scenario "${scenarioName}". Available scenarios: ${availableScenarios.join(", ")}`);
  }

  const scenarioDir = path.join(rootDir, "scenarios", scenarioName);
  const runtimePath = path.join(scenarioDir, "runtime.json");
  const runtimePlan = JSON.parse(await readFile(runtimePath, "utf8"));

  assertNonEmptyString(runtimePlan.placement, "placement", `scenarios/${scenarioName}/runtime.json`);
  if (!Array.isArray(runtimePlan.executions) || !Array.isArray(runtimePlan.communications)) {
    throw new Error(`scenarios/${scenarioName}/runtime.json: "executions" and "communications" must be arrays`);
  }

  const servicesDir = path.join(scenarioDir, "services");
  const entries = await readdir(servicesDir, { withFileTypes: true });
  const services = [];
  const seenIds = new Set();

  for (const entry of entries) {
    if (!entry.isFile() || !entry.name.endsWith(".json")) {
      continue;
    }

    const fileLabel = path.relative(scenarioDir, path.join(servicesDir, entry.name));
    const raw = JSON.parse(await readFile(path.join(servicesDir, entry.name), "utf8"));
    const service = await validateServiceContract(raw, fileLabel, rootDir);
    if (seenIds.has(service.id)) {
      throw new Error(`duplicate service id "${service.id}" in scenarios/${scenarioName}`);
    }
    seenIds.add(service.id);
    services.push(service);
  }

  services.sort((left, right) => left.id.localeCompare(right.id));
  return { scenarioName, services, runtimePlan, policy: await loadPolicy(rootDir) };
}

function buildAuditEntry(kind, subject, decision, reason, details) {
  return { kind, subject, decision, reason, details };
}

function evaluateExecution(service, execution, placement, policy) {
  const placementRule = policy.placementRules[placement];
  if (!placementRule) {
    return buildAuditEntry("execution", service.id, "deny", "placement.unknown", { placement });
  }
  if (!service.placements.includes(placement)) {
    return buildAuditEntry("execution", service.id, "deny", "placement.forbidden", { placement });
  }
  if (!policy.trustedPublishers.includes(service.publisher)) {
    return buildAuditEntry("execution", service.id, "deny", "publisher.untrusted", { publisher: service.publisher });
  }
  if (!policy.allowedTrustTiers.includes(service.trustTier)) {
    return buildAuditEntry("execution", service.id, "deny", "trust_tier.blocked", { trustTier: service.trustTier });
  }

  for (const dependency of service.dependencies) {
    if (dependency.provenance !== "verified") {
      return buildAuditEntry("execution", service.id, "deny", "dependency.provenance.untrusted", {
        dependency: dependency.name,
        provenance: dependency.provenance
      });
    }
    if (!isNonEmptyString(dependency.checksum)) {
      return buildAuditEntry("execution", service.id, "deny", "dependency.checksum.missing", {
        dependency: dependency.name
      });
    }
  }

  for (const permission of execution.requestedPermissions) {
    if (!service.permissions.includes(permission)) {
      return buildAuditEntry("execution", service.id, "deny", "permission.undeclared", { permission });
    }
    if (!placementRule.allowedPermissions.includes(permission)) {
      return buildAuditEntry("execution", service.id, "deny", "permission.forbidden", {
        permission,
        placement
      });
    }
  }

  return buildAuditEntry("execution", service.id, "allow", "execution.trusted", {
    placement,
    requestedPermissions: execution.requestedPermissions
  });
}

function evaluateCommunication(source, target, communication, policy, executionDecisions) {
  const sourceExecution = executionDecisions.get(source.id);
  const targetExecution = executionDecisions.get(target.id);

  if (!source.emits.includes(communication.event)) {
    return buildAuditEntry("communication", `${source.id}->${target.id}`, "deny", "event.not_emitted", {
      event: communication.event
    });
  }
  if (!target.consumes.includes(communication.event)) {
    return buildAuditEntry("communication", `${source.id}->${target.id}`, "deny", "event.not_consumed", {
      event: communication.event
    });
  }
  if (sourceExecution?.decision !== "allow" || targetExecution?.decision !== "allow") {
    return buildAuditEntry("communication", `${source.id}->${target.id}`, "deny", "execution.not_trusted", {
      event: communication.event
    });
  }

  const eventRule = policy.eventRules[communication.event];
  if (!eventRule) {
    return buildAuditEntry("communication", `${source.id}->${target.id}`, "deny", "event.policy.missing", {
      event: communication.event
    });
  }
  if (!eventRule.allowedConsumerTiers.includes(target.trustTier)) {
    return buildAuditEntry("communication", `${source.id}->${target.id}`, "deny", "communication.forbidden", {
      event: communication.event,
      consumerTrustTier: target.trustTier
    });
  }

  return buildAuditEntry("communication", `${source.id}->${target.id}`, "allow", "communication.trusted", {
    event: communication.event
  });
}

export function evaluateTrust(scenario) {
  const serviceMap = new Map(scenario.services.map((service) => [service.id, service]));
  const auditLog = [];
  const executionDecisions = new Map();

  for (const execution of scenario.runtimePlan.executions) {
    const service = serviceMap.get(execution.service);
    if (!service) {
      auditLog.push(buildAuditEntry("execution", execution.service, "deny", "service.not_found", {}));
      continue;
    }
    const decision = evaluateExecution(service, execution, scenario.runtimePlan.placement, scenario.policy);
    executionDecisions.set(service.id, decision);
    auditLog.push(decision);
  }

  for (const communication of scenario.runtimePlan.communications) {
    const source = serviceMap.get(communication.from);
    const target = serviceMap.get(communication.to);
    if (!source || !target) {
      auditLog.push(
        buildAuditEntry("communication", `${communication.from}->${communication.to}`, "deny", "service.not_found", {
          event: communication.event
        }),
      );
      continue;
    }
    auditLog.push(evaluateCommunication(source, target, communication, scenario.policy, executionDecisions));
  }

  const denied = auditLog.filter((entry) => entry.decision === "deny");
  return {
    placement: scenario.runtimePlan.placement,
    services: scenario.services,
    auditLog,
    outcome: denied.length === 0 ? "allow" : "deny"
  };
}

export function formatAuditReport(scenarioName, report) {
  const lines = [
    `Scenario: ${scenarioName}`,
    `Placement: ${report.placement}`,
    `Outcome: ${report.outcome}`,
    "",
    "Services"
  ];

  for (const service of report.services) {
    lines.push(`- ${service.id} v${service.version}`);
    lines.push(`  publisher: ${service.publisher}`);
    lines.push(`  trust tier: ${service.trustTier}`);
    lines.push(`  placements: ${service.placements.join(", ")}`);
    lines.push(`  permissions: ${service.permissions.join(", ")}`);
  }

  lines.push("", "Audit Log");
  for (const entry of report.auditLog) {
    lines.push(`- [${entry.decision}] ${entry.kind} ${entry.subject}: ${entry.reason}`);
  }

  return `${lines.join("\n")}\n`;
}
