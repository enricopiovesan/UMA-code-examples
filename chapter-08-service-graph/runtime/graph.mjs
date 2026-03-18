import path from "node:path";
import process from "node:process";
import { formatTextGraph, listScenarios, loadScenario } from "./graph_lib.mjs";

function printUsage() {
  process.stdout.write(
    "Usage: node runtime/graph.mjs <scenario-name> [text|json]\n" +
      "       node runtime/graph.mjs --list\n",
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

  const scenarioName = command ?? process.env.CH8_SCENARIO ?? "lab1-upload-only";
  const format = process.argv[3] ?? process.env.CH8_FORMAT ?? "text";
  const { graph } = await loadScenario(rootDir, scenarioName);

  if (format === "json") {
    process.stdout.write(`${JSON.stringify({ scenario: scenarioName, ...graph }, null, 2)}\n`);
    return;
  }

  process.stdout.write(formatTextGraph(graph, scenarioName));
}

main().catch((error) => {
  process.stderr.write(`${error.message}\n`);
  process.exitCode = 1;
});
