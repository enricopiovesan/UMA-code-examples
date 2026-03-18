import path from "node:path";
import process from "node:process";
import {
  formatGraphDiff,
  formatTextGraph,
  listScenarios,
  loadScenario,
  validateScenarios,
} from "./lib.mjs";

function printUsage() {
  process.stdout.write(
    "Usage:\n" +
      "  node ts/src/main.mjs list\n" +
      "  node ts/src/main.mjs render <lab-name> [text|json]\n" +
      "  node ts/src/main.mjs validate [lab-name ...]\n" +
      "  node ts/src/main.mjs diff <from-lab> <to-lab>\n",
  );
}

async function main() {
  const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");
  const command = process.argv[2] ?? "render";

  if (command === "--help" || command === "-h") {
    printUsage();
    return;
  }

  if (command === "list") {
    for (const scenarioName of await listScenarios(rootDir)) {
      process.stdout.write(`${scenarioName}\n`);
    }
    return;
  }

  if (command === "render") {
    const scenarioName = process.argv[3] ?? "lab1-upload-only";
    const format = process.argv[4] ?? "text";
    const { graph } = await loadScenario(rootDir, scenarioName);

    if (format === "json") {
      process.stdout.write(`${JSON.stringify({ scenario: scenarioName, ...graph }, null, 2)}\n`);
      return;
    }

    process.stdout.write(formatTextGraph(graph, scenarioName));
    return;
  }

  if (command === "validate") {
    const scenarioNames = process.argv.slice(3);
    for (const line of await validateScenarios(rootDir, scenarioNames)) {
      process.stdout.write(`${line}\n`);
    }
    return;
  }

  if (command === "diff") {
    const fromScenario = process.argv[3];
    const toScenario = process.argv[4];
    if (!fromScenario || !toScenario) {
      throw new Error("Usage: node ts/src/main.mjs diff <from-lab> <to-lab>");
    }
    process.stdout.write(await formatGraphDiff(rootDir, fromScenario, toScenario));
    return;
  }

  throw new Error(`Unknown command "${command}"`);
}

main().catch((error) => {
  process.stderr.write(`${error.message}\n`);
  process.exitCode = 1;
});
