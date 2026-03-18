import process from "node:process";
import { spawnSync } from "node:child_process";

function readScenario(scenarioName) {
  const result = spawnSync(
    process.execPath,
    ["runtime/graph.mjs", scenarioName, "json"],
    { cwd: process.cwd(), encoding: "utf8" },
  );

  if (result.status !== 0) {
    throw new Error(result.stderr.trim() || `Failed to load scenario ${scenarioName}`);
  }

  return JSON.parse(result.stdout);
}

function diffLists(left, right, toKey) {
  const leftKeys = new Set(left.map(toKey));
  const rightKeys = new Set(right.map(toKey));

  return {
    added: [...rightKeys].filter((key) => !leftKeys.has(key)).sort(),
    removed: [...leftKeys].filter((key) => !rightKeys.has(key)).sort(),
  };
}

function printSection(title, values) {
  process.stdout.write(`${title}\n`);
  if (values.length === 0) {
    process.stdout.write("- none\n");
    return;
  }

  for (const value of values) {
    process.stdout.write(`- ${value}\n`);
  }
}

function main() {
  const fromScenario = process.argv[2];
  const toScenario = process.argv[3];

  if (!fromScenario || !toScenario) {
    throw new Error("Usage: node runtime/graph_diff.mjs <from-scenario> <to-scenario>");
  }

  const left = readScenario(fromScenario);
  const right = readScenario(toScenario);

  const serviceDiff = diffLists(left.services, right.services, (entry) => entry.id);
  const edgeDiff = diffLists(left.edges, right.edges, (entry) => `${entry.from} -> ${entry.event} -> ${entry.to}`);
  const waitDiff = diffLists(left.waiting, right.waiting, (entry) => `${entry.service} waiting for ${entry.event}`);

  process.stdout.write(`Graph diff: ${fromScenario} -> ${toScenario}\n\n`);
  printSection("Added services", serviceDiff.added);
  process.stdout.write("\n");
  printSection("Removed services", serviceDiff.removed);
  process.stdout.write("\n");
  printSection("Added edges", edgeDiff.added);
  process.stdout.write("\n");
  printSection("Removed edges", edgeDiff.removed);
  process.stdout.write("\n");
  printSection("Added waiting consumers", waitDiff.added);
  process.stdout.write("\n");
  printSection("Removed waiting consumers", waitDiff.removed);
}

try {
  main();
} catch (error) {
  process.stderr.write(`${error.message}\n`);
  process.exitCode = 1;
}
