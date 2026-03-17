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
    "Architectural Choices",
    `- granularity: ${report.choices.granularity}`,
    `- event semantics: ${report.choices.eventSemantics}`,
    `- metadata quality: ${report.choices.metadataQuality}`,
    `- orchestration: ${report.choices.orchestration}`,
    `- runtime placement: ${report.choices.runtimePlacement}`,
    `- state model: ${report.choices.stateModel}`,
    "",
    "Services"
  ];
  for (const service of report.services) {
    lines.push(`- ${service.id} (${service.capability})`);
    lines.push(`  summary: ${service.summary}`);
    lines.push(`  placements: ${service.placements.join(", ")}`);
    if (service.consumes.length > 0) {
      lines.push(`  consumes: ${service.consumes.join(", ")}`);
    }
    if (service.emits.length > 0) {
      lines.push(`  emits: ${service.emits.join(", ")}`);
    }
  }

  lines.push(
    "",
    "Interaction Flow"
  );
  if (report.interactions.length === 0) {
    lines.push("- none");
  } else {
    for (const interaction of report.interactions) {
      lines.push(`- ${interaction.from} -> ${interaction.event} -> ${interaction.to}`);
    }
  }

  lines.push(
    "",
    "Warnings"
  );
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
    `Architecture diff: ${diff.from} -> ${diff.to}`,
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
        `Validated ${report.scenario}: ${report.services.length} services, ${report.interactions.length} interactions, verdict=${report.assessment.verdict}`
      );
    } else {
      for (const item of listLabs(root)) {
        const report = loadReport(root, item);
        console.log(
          `Validated ${report.scenario}: ${report.services.length} services, ${report.interactions.length} interactions, verdict=${report.assessment.verdict}`
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
