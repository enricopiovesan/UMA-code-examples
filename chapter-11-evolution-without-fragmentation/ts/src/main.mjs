import { diffReports, listLabs, loadReport, projectRoot } from "./lib.mjs";

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
    `Scenario: ${report.scenario}`,
    `Title: ${report.title}`,
    `Summary: ${report.summary}`,
    `Verdict: ${report.assessment.verdict}`,
    "",
    "Evolution Axes",
    `- contract anchor: ${report.choices.contractAnchor}`,
    `- versioning: ${report.choices.versioning}`,
    `- runtime governance: ${report.choices.runtimeGovernance}`,
    `- duplication: ${report.choices.duplication}`,
    `- event semantics: ${report.choices.eventSemantics}`,
    `- adoption mode: ${report.choices.adoptionMode}`,
    "",
    "Capabilities"
  ];
  for (const service of report.services) {
    lines.push(`- ${service.id} ${service.version} (${service.capability})`);
    lines.push(`  summary: ${service.summary}`);
    lines.push(`  placements: ${service.placements.join(", ")}`);
    if (service.consumers.length > 0) {
      lines.push(`  consumed by: ${service.consumers.join(", ")}`);
    }
  }

  lines.push("", "Interaction Surface");
  if (report.interactions.length === 0) {
    lines.push("- none");
  } else {
    for (const interaction of report.interactions) {
      lines.push(`- ${interaction.from} -> ${interaction.relation} -> ${interaction.to}`);
    }
  }

  lines.push("", "Runtime Decisions");
  if (report.runtimeDecisions.length === 0) {
    lines.push("- none");
  } else {
    for (const item of report.runtimeDecisions) {
      lines.push(`- ${item}`);
    }
  }

  lines.push("", "Warnings");
  if (report.assessment.warnings.length === 0) {
    lines.push("- none");
  } else {
    for (const warning of report.assessment.warnings) {
      lines.push(`- ${warning.code}: ${warning.message}`);
    }
  }

  lines.push("", "Reader Value");
  for (const expectation of report.expectations) {
    lines.push(`- ${expectation}`);
  }

  return lines.join("\n");
}

function formatDiff(diff) {
  const lines = [
    `Evolution diff: ${diff.from} -> ${diff.to}`,
    `Verdict: ${diff.verdictChange}`,
    "",
    "Changed axes"
  ];
  if (diff.changedAxes.length === 0) {
    lines.push("- none");
  } else {
    for (const axis of diff.changedAxes) {
      lines.push(`- ${axis}`);
    }
  }
  lines.push("", "Added warnings");
  if (diff.addedWarnings.length === 0) {
    lines.push("- none");
  } else {
    for (const warning of diff.addedWarnings) {
      lines.push(`- ${warning}`);
    }
  }
  lines.push("", "Removed warnings");
  if (diff.removedWarnings.length === 0) {
    lines.push("- none");
  } else {
    for (const warning of diff.removedWarnings) {
      lines.push(`- ${warning}`);
    }
  }
  return lines.join("\n");
}

const args = process.argv.slice(2);
const command = args.shift();
if (!command) usage();

const root = projectRoot();

switch (command) {
  case "list":
    for (const lab of listLabs(root)) {
      console.log(lab);
    }
    break;
  case "render": {
    const lab = args.shift();
    if (!lab) usage();
    const format = args.shift() || "text";
    const report = loadReport(root, lab);
    console.log(format === "json" ? JSON.stringify(report, null, 2) : formatReport(report));
    break;
  }
  case "validate": {
    const lab = args.shift();
    if (lab) {
      const report = loadReport(root, lab);
      console.log(
        `Validated ${report.scenario}: ${report.services.length} capabilities, ${report.interactions.length} interactions, verdict=${report.assessment.verdict}`
      );
    } else {
      for (const item of listLabs(root)) {
        const report = loadReport(root, item);
        console.log(
          `Validated ${report.scenario}: ${report.services.length} capabilities, ${report.interactions.length} interactions, verdict=${report.assessment.verdict}`
        );
      }
    }
    break;
  }
  case "diff": {
    const from = args.shift();
    const to = args.shift();
    if (!from || !to) usage();
    console.log(formatDiff(diffReports(loadReport(root, from), loadReport(root, to))));
    break;
  }
  default:
    usage();
}
