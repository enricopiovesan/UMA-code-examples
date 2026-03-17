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
    ["choices.granularity", raw.choices?.granularity],
    ["choices.event_semantics", raw.choices?.event_semantics],
    ["choices.metadata_quality", raw.choices?.metadata_quality],
    ["choices.orchestration", raw.choices?.orchestration],
    ["choices.runtime_placement", raw.choices?.runtime_placement],
    ["choices.state_model", raw.choices?.state_model]
  ];
  for (const [field, value] of required) {
    if (!value || `${value}`.trim() === "") {
      throw new Error(`${label}: "${field}" must be a non-empty string`);
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
      summary: service.summary,
      placements: [...service.placements],
      consumes: [...(service.events?.consumes || [])],
      emits: [...(service.events?.emits || [])],
      metadata: {
        fields: [...(service.metadata?.fields || [])],
        selection_rank: service.metadata?.selection_rank ?? null
      }
    };
  }).sort((a, b) => a.id.localeCompare(b.id));

  const interactions = raw.interactions
    .map((interaction) => ({ ...interaction }))
    .sort((a, b) =>
      a.from.localeCompare(b.from) ||
      a.event.localeCompare(b.event) ||
      a.to.localeCompare(b.to)
    );

  const choices = {
    granularity: raw.choices.granularity,
    eventSemantics: raw.choices.event_semantics,
    metadataQuality: raw.choices.metadata_quality,
    orchestration: raw.choices.orchestration,
    runtimePlacement: raw.choices.runtime_placement,
    stateModel: raw.choices.state_model
  };

  return {
    scenario: label,
    title: raw.title,
    summary: raw.summary,
    services,
    interactions,
    choices,
    expectations: [...raw.expectations],
    assessment: assess(services, interactions, choices)
  };
}

function inferOverGranular(services, interactions, choices) {
  if (choices.granularity === "over-granular") {
    return true;
  }

  return interactions.length >= 4 &&
    services.length >= 5 &&
    choices.orchestration !== "rigid-centralized";
}

export function assess(services, interactions, choices) {
  const warnings = [];
  if (inferOverGranular(services, interactions, choices)) {
    warnings.push({
      code: "over_granular",
      message: "The workflow is split across too many narrowly scoped capabilities."
    });
  }
  if (choices.eventSemantics === "ambiguous") {
    warnings.push({
      code: "hidden_event_coupling",
      message: "Events are too vague and create hidden coupling between consumers."
    });
  }
  if (choices.metadataQuality === "bloated") {
    warnings.push({
      code: "metadata_bloat",
      message: "Metadata contains too much detail and loses clarity as a control plane."
    });
  }
  if (choices.orchestration === "rigid-centralized") {
    warnings.push({
      code: "over_orchestrated",
      message: "A central coordinator is dictating too much workflow behavior."
    });
  }
  if (
    choices.runtimePlacement === "ambiguous-selection" ||
    services.filter((service) => service.metadata.selection_rank == null).length >= 2
  ) {
    warnings.push({
      code: "runtime_ambiguity",
      message: "Multiple capabilities can satisfy the same role without deterministic selection."
    });
  }
  if (choices.stateModel === "duplicated-projections") {
    warnings.push({
      code: "state_drift",
      message: "State is being duplicated without a clear capability boundary or projection purpose."
    });
  }

  let verdict = "coherent";
  if (warnings.length > 0) {
    verdict = warnings.some((warning) => warning.code === "over_orchestrated")
      ? "fragile"
      : warnings.some((warning) => warning.code === "runtime_ambiguity")
        ? "ambiguous"
        : "needs-attention";
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
    ["granularity", from.choices.granularity, to.choices.granularity],
    ["event semantics", from.choices.eventSemantics, to.choices.eventSemantics],
    ["metadata quality", from.choices.metadataQuality, to.choices.metadataQuality],
    ["orchestration", from.choices.orchestration, to.choices.orchestration],
    ["runtime placement", from.choices.runtimePlacement, to.choices.runtimePlacement],
    ["state model", from.choices.stateModel, to.choices.stateModel]
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
