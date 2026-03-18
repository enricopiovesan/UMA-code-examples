import path from "node:path";
import process from "node:process";
import { listScenarios, loadScenario } from "./trust_lib.mjs";

async function main() {
  const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
  const args = process.argv.slice(2);

  if (args[0] === "--help" || args[0] === "-h") {
    process.stdout.write(
      "Usage: node runtime/validate.mjs [scenario-name ...]\n" +
        "       node runtime/validate.mjs --list\n",
    );
    return;
  }

  if (args[0] === "--list") {
    for (const scenarioName of await listScenarios(rootDir)) {
      process.stdout.write(`${scenarioName}\n`);
    }
    return;
  }

  const targets = args.length > 0 ? args : await listScenarios(rootDir);
  for (const scenarioName of targets) {
    const scenario = await loadScenario(rootDir, scenarioName);
    process.stdout.write(
      `Validated ${scenarioName}: ${scenario.services.length} services, ${scenario.runtimePlan.executions.length} executions, ${scenario.runtimePlan.communications.length} communications\n`,
    );
  }
}

main().catch((error) => {
  process.stderr.write(`${error.message}\n`);
  process.exitCode = 1;
});
