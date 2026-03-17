import path from "node:path";
import process from "node:process";
import { evaluateTrust, formatAuditReport, listScenarios, loadScenario } from "./trust_lib.mjs";

function printUsage() {
  process.stdout.write(
    "Usage: node runtime/runner.mjs <scenario-name> [text|json]\n" +
      "       node runtime/runner.mjs --list\n",
  );
}

async function main() {
  const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
  const command = process.argv[2];

  if (command === "--help" || command === "-h") {
    printUsage();
    return;
  }

  if (command === "--list") {
    for (const scenarioName of await listScenarios(rootDir)) {
      process.stdout.write(`${scenarioName}\n`);
    }
    return;
  }

  const scenarioName = command ?? "lab1-trusted-service";
  const format = process.argv[3] ?? "text";
  const report = evaluateTrust(await loadScenario(rootDir, scenarioName));

  if (format === "json") {
    process.stdout.write(`${JSON.stringify({ scenario: scenarioName, ...report }, null, 2)}\n`);
    return;
  }

  process.stdout.write(formatAuditReport(scenarioName, report));
}

main().catch((error) => {
  process.stderr.write(`${error.message}\n`);
  process.exitCode = 1;
});
