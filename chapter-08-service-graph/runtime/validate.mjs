import path from "node:path";
import process from "node:process";
import { listScenarios, loadScenario } from "./graph_lib.mjs";

async function main() {
  const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
  const scenarioNames = process.argv.slice(2);

  if (scenarioNames[0] === "--help" || scenarioNames[0] === "-h") {
    process.stdout.write(
      "Usage: node runtime/validate.mjs [scenario-name ...]\n" +
        "       node runtime/validate.mjs --list\n",
    );
    return;
  }

  if (scenarioNames[0] === "--list") {
    for (const scenarioName of await listScenarios(rootDir)) {
      process.stdout.write(`${scenarioName}\n`);
    }
    return;
  }

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
