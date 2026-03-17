import path from "node:path";
import process from "node:process";
import { evaluateTrust, loadScenario } from "./trust_lib.mjs";

function diffList(left, right, toKey) {
  const leftKeys = new Set(left.map(toKey));
  const rightKeys = new Set(right.map(toKey));

  return {
    added: [...rightKeys].filter((key) => !leftKeys.has(key)).sort(),
    removed: [...leftKeys].filter((key) => !rightKeys.has(key)).sort()
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

async function main() {
  const fromScenario = process.argv[2];
  const toScenario = process.argv[3];

  if (!fromScenario || !toScenario) {
    throw new Error("Usage: node runtime/trust_diff.mjs <from-scenario> <to-scenario>");
  }

  const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
  const fromReport = evaluateTrust(await loadScenario(rootDir, fromScenario));
  const toReport = evaluateTrust(await loadScenario(rootDir, toScenario));

  const fromEntries = fromReport.auditLog.map((entry) => `${entry.decision}:${entry.kind}:${entry.subject}:${entry.reason}`);
  const toEntries = toReport.auditLog.map((entry) => `${entry.decision}:${entry.kind}:${entry.subject}:${entry.reason}`);
  const auditDiff = diffList(
    fromEntries.map((value) => ({ value })),
    toEntries.map((value) => ({ value })),
    (entry) => entry.value,
  );

  process.stdout.write(`Trust diff: ${fromScenario} -> ${toScenario}\n`);
  process.stdout.write(`Outcome: ${fromReport.outcome} -> ${toReport.outcome}\n\n`);
  printSection("Added trust decisions", auditDiff.added);
  process.stdout.write("\n");
  printSection("Removed trust decisions", auditDiff.removed);
}

main().catch((error) => {
  process.stderr.write(`${error.message}\n`);
  process.exitCode = 1;
});
