import path from "node:path";
import process from "node:process";
import { readdir } from "node:fs/promises";
import { loadScenario } from "./graph_lib.mjs";

async function listScenarios(rootDir) {
  const scenariosDir = path.join(rootDir, "scenarios");
  const entries = await readdir(scenariosDir, { withFileTypes: true });
  return entries.filter((entry) => entry.isDirectory()).map((entry) => entry.name).sort();
}

async function main() {
  const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
  const scenarioNames = process.argv.slice(2);
  const targets = scenarioNames.length > 0 ? scenarioNames : await listScenarios(rootDir);

  for (const scenarioName of targets) {
    const { graph } = await loadScenario(rootDir, scenarioName);
    process.stdout.write(
      `Validated ${scenarioName}: ${graph.services.length} services, ${graph.edges.length} edges, ${graph.waiting.length} waiting consumers\n`,
    );
  }
}

main().catch((error) => {
  process.stderr.write(`${error.message}\n`);
  process.exitCode = 1;
});
