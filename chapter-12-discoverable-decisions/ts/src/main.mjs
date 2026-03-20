import {
  diffReports,
  listLabs,
  loadReport,
  projectRoot,
  validateAll
} from "./lib.mjs";

function usage() {
  console.error("Usage:");
  console.error("  node ts/src/main.mjs list");
  console.error("  node ts/src/main.mjs render <lab> [text|json]");
  console.error("  node ts/src/main.mjs validate [lab]");
  console.error("  node ts/src/main.mjs diff <from-lab> <to-lab>");
  process.exit(1);
}

function formatReport(report) {
  const lines = [
    `${report.title}`,
    "",
    `Scenario: ${report.scenario}`,
    `Summary: ${report.summary}`,
    `Decision Question: ${report.decisionQuestion}`,
    "",
    `Verdict: ${report.assessment.verdict}`
  ];

  if (report.assessment.warnings.length === 0) {
    lines.push("Warnings: none", "");
  } else {
    lines.push("Warnings:");
    for (const warning of report.assessment.warnings) {
      lines.push(`- ${warning.code}: ${warning.message}`);
    }
    lines.push("");
  }

  lines.push(
    "Decision Axes:",
    `- projection scope: ${report.axes.projectionScope}`,
    `- proposal visibility: ${report.axes.proposalVisibility}`,
    `- authority model: ${report.axes.authorityModel}`,
    `- revision model: ${report.axes.revisionModel}`,
    `- execution model: ${report.axes.executionModel}`,
    `- traceability: ${report.axes.traceability}`,
    "",
    "Discoverable Surfaces:"
  );

  for (const surface of report.surfaces) {
    lines.push(
      `- ${surface.name} (${surface.runtime}, scope=${surface.scope}, authority=${surface.authority})`,
      `  visible: ${surface.visibleCapabilities.join(", ")}`,
      `  hidden: ${surface.hiddenConstraints.length === 0 ? "none" : surface.hiddenConstraints.join(", ")}`,
      `  queries: ${surface.queries.join(" | ")}`
    );
  }

  lines.push(
    "",
    "Proposal:",
    `- status: ${report.proposal.status}`,
    `- summary: ${report.proposal.summary}`,
    `- assumptions: ${report.proposal.assumptions.join(" | ")}`,
    `- unresolved: ${report.proposal.unresolved.length === 0 ? "none" : report.proposal.unresolved.join(" | ")}`,
    "",
    "Validation:",
    `- status: ${report.validation.status}`,
    `- summary: ${report.validation.summary}`,
    `- violations: ${report.validation.violations.length === 0 ? "none" : report.validation.violations.join(" | ")}`,
    `- guidance: ${report.validation.guidance.length === 0 ? "none" : report.validation.guidance.join(" | ")}`,
    `- authority notes: ${report.validation.authorityNotes.length === 0 ? "none" : report.validation.authorityNotes.join(" | ")}`,
    "",
    "Execution:",
    `- status: ${report.execution.status}`,
    `- summary: ${report.execution.summary}`,
    `- selected capabilities: ${report.execution.selectedCapabilities.length === 0 ? "none" : report.execution.selectedCapabilities.join(", ")}`,
    `- placement: ${report.execution.placement.length === 0 ? "none" : report.execution.placement.join(" | ")}`,
    "",
    "Trace:",
    `- status: ${report.trace.status}`,
    `- summary: ${report.trace.summary}`,
    `- artifacts: ${report.trace.artifacts.join(" | ")}`,
    `- queries: ${report.trace.queries.join(" | ")}`,
    "",
    "Functions, Not Fixed Roles:",
    `- planning: ${report.functions.planning}`,
    `- validation: ${report.functions.validation}`,
    `- execution: ${report.functions.execution}`,
    "",
    "Runtime Decisions:"
  );
  for (const item of report.runtimeDecisions) {
    lines.push(`- ${item}`);
  }
  lines.push("", "Reader Value:");
  for (const item of report.expectations) {
    lines.push(`- ${item}`);
  }

  return lines.join("\n");
}

function formatDiff(diff) {
  const lines = [
    `From: ${diff.from}`,
    `To: ${diff.to}`,
    `Verdict: ${diff.verdictChange}`
  ];

  if (diff.changedAxes.length === 0) {
    lines.push("Changed Axes: none");
  } else {
    lines.push("Changed Axes:");
    diff.changedAxes.forEach((item) => lines.push(`- ${item}`));
  }

  if (diff.changedStages.length === 0) {
    lines.push("Changed Stages: none");
  } else {
    lines.push("Changed Stages:");
    diff.changedStages.forEach((item) => lines.push(`- ${item}`));
  }

  lines.push(
    `Added Warnings: ${diff.addedWarnings.length === 0 ? "none" : diff.addedWarnings.join(", ")}`,
    `Removed Warnings: ${diff.removedWarnings.length === 0 ? "none" : diff.removedWarnings.join(", ")}`
  );

  return lines.join("\n");
}

const root = projectRoot();
const [command, ...rest] = process.argv.slice(2);

switch (command) {
  case "list":
    listLabs(root).forEach((lab) => console.log(lab));
    break;
  case "render": {
    const [lab, format = "text"] = rest;
    if (!lab) usage();
    const report = loadReport(root, lab);
    console.log(format === "json" ? JSON.stringify(report, null, 2) : formatReport(report));
    break;
  }
  case "validate": {
    const [lab] = rest;
    if (lab) {
      const report = loadReport(root, lab);
      console.log(`Validated ${report.scenario}: ${report.surfaces.length} surfaces, verdict=${report.assessment.verdict}`);
    } else {
      validateAll(root).forEach((line) => console.log(line));
    }
    break;
  }
  case "diff": {
    const [fromLab, toLab] = rest;
    if (!fromLab || !toLab) usage();
    console.log(formatDiff(diffReports(loadReport(root, fromLab), loadReport(root, toLab))));
    break;
  }
  default:
    usage();
}
