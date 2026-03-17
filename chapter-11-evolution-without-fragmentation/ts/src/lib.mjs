import fs from "node:fs";
import path from "node:path";

export function projectRoot() {
  return path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");
}

export function listLabs(rootDir) {
  return fs
    .readdirSync(path.join(rootDir, "scenarios"), { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort();
}

function validateScenario(raw, label) {
  const required = [
    ["title", raw.title],
    ["summary", raw.summary],
    ["choices.contract_anchor", raw.choices?.contract_anchor],
    ["choices.versioning", raw.choices?.versioning],
    ["choices.runtime_governance", raw.choices?.runtime_governance],
    ["choices.duplication", raw.choices?.duplication],
    ["choices.event_semantics", raw.choices?.event_semantics],
    ["choices.adoption_mode", raw.choices?.adoption_mode]
  ];
  for (const [field, value] of required) {
    if (!value || `${value}`.trim() === "") {
      throw new Error(`${label}: "${field}" must be a non-empty string`);
    }
  }

  for (const field of raw.expectations || []) {
    if (!field || `${field}`.trim() === "") {
      throw new Error(`${label}: "expectations" must contain non-empty strings`);
    }
  }

  for (const field of raw.runtime_decisions || []) {
    if (!field || `${field}`.trim() === "") {
      throw new Error(`${label}: "runtime_decisions" must contain non-empty strings`);
    }
  }

  const ids = new Set();
  const services = raw.services.map((service) => {
    if (ids.has(service.id)) {
      throw new Error(`${label}: duplicate service id "${service.id}"`);
    }
    ids.add(service.id);
    return {
      id: service.id,
      capability: service.capability,
      version: service.version,
      summary: service.summary,
      placements: [...service.placements],
      consumers: [...(service.consumers || [])]
    };
  }).sort((a, b) => a.id.localeCompare(b.id));

  const interactions = raw.interactions.map((interaction) => ({ ...interaction }))
    .sort((a, b) =>
      a.from.localeCompare(b.from) ||
      a.relation.localeCompare(b.relation) ||
      a.to.localeCompare(b.to)
    );

  const choices = {
    contractAnchor: raw.choices.contract_anchor,
    versioning: raw.choices.versioning,
    runtimeGovernance: raw.choices.runtime_governance,
    duplication: raw.choices.duplication,
    eventSemantics: raw.choices.event_semantics,
    adoptionMode: raw.choices.adoption_mode
  };

  return {
    scenario: label,
    title: raw.title,
    summary: raw.summary,
    services,
    interactions,
    choices,
    expectations: [...raw.expectations],
    runtimeDecisions: [...(raw.runtime_decisions || [])],
    assessment: assess(choices)
  };
}

export function assess(choices) {
  const warnings = [];

  if (choices.contractAnchor === "drifting") {
    warnings.push({
      code: "behavioral_drift",
      message: "The same contract is being stretched across incompatible behavioral expectations."
    });
  }

  if (choices.duplication === "cross-environment") {
    warnings.push({
      code: "duplicate_behavior",
      message: "The same conceptual capability now exists in multiple implementations that can drift apart."
    });
  }

  if (choices.versioning === "uncontrolled-proliferation") {
    warnings.push({
      code: "version_fragmentation",
      message: "Multiple versions coexist without clear compatibility or lifecycle rules."
    });
  }

  if (choices.eventSemantics === "unstable") {
    warnings.push({
      code: "semantic_instability",
      message: "Events remain structurally valid but their meaning is no longer stable for all consumers."
    });
  }

  if (choices.runtimeGovernance === "manual-only") {
    warnings.push({
      code: "manual_governance_limit",
      message: "Human coordination is carrying behavior alignment instead of runtime enforcement."
    });
  }

  if (choices.adoptionMode === "hybrid" && choices.runtimeGovernance !== "runtime-enforced") {
    warnings.push({
      code: "hybrid_boundary_risk",
      message: "Hybrid adoption without strong boundary enforcement leaves legacy assumptions exposed."
    });
  }

  const governedShape = choices.runtimeGovernance === "runtime-enforced" &&
    (choices.versioning === "controlled-coexistence" || choices.adoptionMode === "hybrid");

  let verdict = "coherent";
  if (warnings.some((warning) => warning.code === "version_fragmentation")) {
    verdict = "fragmented";
  } else if (governedShape) {
    verdict = "governed";
  } else if (warnings.length === 0) {
    verdict = "coherent";
  } else {
    verdict = "at-risk";
  }

  return { verdict, warnings };
}

export function loadReport(rootDir, lab) {
  const labs = listLabs(rootDir);
  if (!labs.includes(lab)) {
    throw new Error(`unknown lab "${lab}". Available labs: ${labs.join(", ")}`);
  }
  const raw = JSON.parse(fs.readFileSync(path.join(rootDir, "scenarios", lab, "scenario.json"), "utf8"));
  return validateScenario(raw, lab);
}

export function diffReports(from, to) {
  const left = new Set(from.assessment.warnings.map((warning) => warning.code));
  const right = new Set(to.assessment.warnings.map((warning) => warning.code));
  const changedAxes = [];
  const axes = [
    ["contract anchor", from.choices.contractAnchor, to.choices.contractAnchor],
    ["versioning", from.choices.versioning, to.choices.versioning],
    ["runtime governance", from.choices.runtimeGovernance, to.choices.runtimeGovernance],
    ["duplication", from.choices.duplication, to.choices.duplication],
    ["event semantics", from.choices.eventSemantics, to.choices.eventSemantics],
    ["adoption mode", from.choices.adoptionMode, to.choices.adoptionMode]
  ];
  for (const [label, leftValue, rightValue] of axes) {
    if (leftValue !== rightValue) {
      changedAxes.push(`${label}: ${leftValue} -> ${rightValue}`);
    }
  }

  return {
    from: from.scenario,
    to: to.scenario,
    verdictChange: `${from.assessment.verdict} -> ${to.assessment.verdict}`,
    addedWarnings: [...right].filter((item) => !left.has(item)).sort(),
    removedWarnings: [...left].filter((item) => !right.has(item)).sort(),
    changedAxes
  };
}
