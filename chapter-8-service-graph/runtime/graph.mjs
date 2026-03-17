import path from "node:path";
import process from "node:process";
import { formatTextGraph, loadScenario } from "./graph_lib.mjs";

async function main() {
  const scenarioName = process.argv[2] ?? process.env.CH8_SCENARIO ?? "lab1-upload-only";
  const format = process.argv[3] ?? process.env.CH8_FORMAT ?? "text";
  const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
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
