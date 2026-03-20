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

function ensureNonEmpty(value, field, label) {
  if (!value || `${value}`.trim() === "") {
    throw new Error(`${label}: "${field}" must be a non-empty string`);
  }
}

function ensureList(values, field, label) {
  for (const value of values) {
    ensureNonEmpty(value, field, label);
  }
}

function validateScenario(raw, label) {
  [
    ["title", raw.title],
    ["summary", raw.summary],
    ["decision_question", raw.decision_question],
    ["proposal.status", raw.proposal?.status],
    ["proposal.summary", raw.proposal?.summary],
    ["validation.status", raw.validation?.status],
    ["validation.summary", raw.validation?.summary],
    ["execution.status", raw.execution?.status],
    ["execution.summary", raw.execution?.summary],
    ["trace.status", raw.trace?.status],
    ["trace.summary", raw.trace?.summary],
    ["functions.planning", raw.functions?.planning],
    ["functions.validation", raw.functions?.validation],
    ["functions.execution", raw.functions?.execution],
    ["axes.projection_scope", raw.axes?.projection_scope],
    ["axes.proposal_visibility", raw.axes?.proposal_visibility],
    ["axes.authority_model", raw.axes?.authority_model],
    ["axes.revision_model", raw.axes?.revision_model],
    ["axes.execution_model", raw.axes?.execution_model],
    ["axes.traceability", raw.axes?.traceability]
  ].forEach(([field, value]) => ensureNonEmpty(value, field, label));

  ensureList(raw.expectations || [], "expectations", label);
  ensureList(raw.runtime_decisions || [], "runtime_decisions", label);
  ensureList(raw.proposal.assumptions || [], "proposal.assumptions", label);
  ensureList(raw.proposal.unresolved || [], "proposal.unresolved", label);
  ensureList(raw.validation.violations || [], "validation.violations", label);
  ensureList(raw.validation.guidance || [], "validation.guidance", label);
  ensureList(raw.validation.authority_notes || [], "validation.authority_notes", label);
  ensureList(raw.execution.selected_capabilities || [], "execution.selected_capabilities", label);
  ensureList(raw.execution.placement || [], "execution.placement", label);
  ensureList(raw.trace.artifacts || [], "trace.artifacts", label);
  ensureList(raw.trace.queries || [], "trace.queries", label);

  const seen = new Set();
  const surfaces = raw.surfaces.map((surface) => {
    ensureNonEmpty(surface.name, "surfaces[].name", label);
    ensureNonEmpty(surface.runtime, "surfaces[].runtime", label);
    ensureNonEmpty(surface.scope, "surfaces[].scope", label);
    ensureNonEmpty(surface.authority, "surfaces[].authority", label);
    ensureList(surface.visible_capabilities || [], "surfaces[].visible_capabilities", label);
    ensureList(surface.hidden_constraints || [], "surfaces[].hidden_constraints", label);
    ensureList(surface.queries || [], "surfaces[].queries", label);
    if (seen.has(surface.name)) {
      throw new Error(`${label}: duplicate surface name "${surface.name}"`);
    }
    seen.add(surface.name);
    return {
      name: surface.name,
      runtime: surface.runtime,
      scope: surface.scope,
      authority: surface.authority,
      visibleCapabilities: [...surface.visible_capabilities],
      hiddenConstraints: [...surface.hidden_constraints],
      queries: [...surface.queries]
    };
  }).sort((a, b) => a.name.localeCompare(b.name));

  const proposal = {
    status: raw.proposal.status,
    summary: raw.proposal.summary,
    assumptions: [...raw.proposal.assumptions],
    unresolved: [...raw.proposal.unresolved]
  };

  const validation = {
    status: raw.validation.status,
    summary: raw.validation.summary,
    violations: [...raw.validation.violations],
    guidance: [...raw.validation.guidance],
    authorityNotes: [...raw.validation.authority_notes]
  };

  const execution = {
    status: raw.execution.status,
    summary: raw.execution.summary,
    selectedCapabilities: [...raw.execution.selected_capabilities],
    placement: [...raw.execution.placement]
  };

  const trace = {
    status: raw.trace.status,
    summary: raw.trace.summary,
    artifacts: [...raw.trace.artifacts],
    queries: [...raw.trace.queries]
  };

  const functions = { ...raw.functions };
  const axes = {
    projectionScope: raw.axes.projection_scope,
    proposalVisibility: raw.axes.proposal_visibility,
    authorityModel: raw.axes.authority_model,
    revisionModel: raw.axes.revision_model,
    executionModel: raw.axes.execution_model,
    traceability: raw.axes.traceability
  };

  return {
    scenario: label,
    title: raw.title,
    summary: raw.summary,
    decisionQuestion: raw.decision_question,
    surfaces,
    proposal,
    validation,
    execution,
    trace,
    functions,
    axes,
    expectations: [...raw.expectations],
    runtimeDecisions: [...raw.runtime_decisions],
    assessment: assess(axes, validation, execution)
  };
}

export function assess(axes, validation, execution) {
  const warnings = [];

  if (axes.proposalVisibility === "hidden") {
    warnings.push({
      code: "proposal_hidden",
      message: "The system is changing behavior, but the proposed decision is not visible as an inspectable artifact."
    });
  }

  if (axes.authorityModel === "local-only") {
    warnings.push({
      code: "authority_gap",
      message: "Local reasoning is acting without authoritative validation of trust, compliance, or global constraints."
    });
  }

  if (validation.status === "violations" && validation.guidance.length === 0) {
    warnings.push({
      code: "validation_without_guidance",
      message: "Constraint failures are detected, but the planner is not given actionable remediation guidance."
    });
  }

  if (axes.revisionModel === "unbounded") {
    warnings.push({
      code: "unbounded_revision",
      message: "The planner and validator can keep negotiating indefinitely, which makes the decision lifecycle difficult to reason about."
    });
  }

  if (axes.executionModel === "implicit-replan") {
    warnings.push({
      code: "implicit_replanning",
      message: "Execution is reinterpreting the approved intent instead of resolving a validated proposal."
    });
  }

  if (axes.traceability === "absent") {
    warnings.push({
      code: "missing_trace",
      message: "The system produces an outcome without first-class trace artifacts that explain how the decision was formed."
    });
  }

  if (axes.traceability === "proposal-only" && execution.status === "resolved") {
    warnings.push({
      code: "partial_trace",
      message: "The system records planning artifacts, but not the full path from approved proposal to concrete execution."
    });
  }

  let verdict;
  if (warnings.some((warning) => ["implicit_replanning", "unbounded_revision"].includes(warning.code))) {
    verdict = "unstable";
  } else if (warnings.some((warning) => ["proposal_hidden", "authority_gap"].includes(warning.code))) {
    verdict = "opaque";
  } else if (
    axes.authorityModel === "authoritative-validation" &&
    axes.revisionModel === "single-revision" &&
    axes.executionModel === "approved-resolution" &&
    axes.traceability === "full-trace" &&
    axes.proposalVisibility === "queryable"
  ) {
    verdict = "governed";
  } else {
    verdict = "discoverable";
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

export function validateAll(rootDir) {
  return listLabs(rootDir).map((lab) => {
    const report = loadReport(rootDir, lab);
    return `Validated ${report.scenario}: ${report.surfaces.length} surfaces, verdict=${report.assessment.verdict}`;
  });
}

export function diffReports(from, to) {
  const left = new Set(from.assessment.warnings.map((warning) => warning.code));
  const right = new Set(to.assessment.warnings.map((warning) => warning.code));
  const changedAxes = [];
  const axes = [
    ["projection scope", from.axes.projectionScope, to.axes.projectionScope],
    ["proposal visibility", from.axes.proposalVisibility, to.axes.proposalVisibility],
    ["authority model", from.axes.authorityModel, to.axes.authorityModel],
    ["revision model", from.axes.revisionModel, to.axes.revisionModel],
    ["execution model", from.axes.executionModel, to.axes.executionModel],
    ["traceability", from.axes.traceability, to.axes.traceability]
  ];
  for (const [label, leftValue, rightValue] of axes) {
    if (leftValue !== rightValue) {
      changedAxes.push(`${label}: ${leftValue} -> ${rightValue}`);
    }
  }

  const changedStages = [];
  const stages = [
    ["proposal status", from.proposal.status, to.proposal.status],
    ["validation status", from.validation.status, to.validation.status],
    ["execution status", from.execution.status, to.execution.status],
    ["trace status", from.trace.status, to.trace.status]
  ];
  for (const [label, leftValue, rightValue] of stages) {
    if (leftValue !== rightValue) {
      changedStages.push(`${label}: ${leftValue} -> ${rightValue}`);
    }
  }

  return {
    from: from.scenario,
    to: to.scenario,
    verdictChange: `${from.assessment.verdict} -> ${to.assessment.verdict}`,
    addedWarnings: [...right].filter((item) => !left.has(item)).sort(),
    removedWarnings: [...left].filter((item) => !right.has(item)).sort(),
    changedAxes,
    changedStages
  };
}
