import { readFile } from "node:fs/promises";
import process from "node:process";
import { runJson } from "./lib.mjs";

async function readStdin() {
  const chunks = [];
  for await (const chunk of process.stdin) {
    chunks.push(chunk);
  }
  return Buffer.concat(chunks).toString("utf8");
}

function usage() {
  console.error("Usage:");
  console.error("  node ts/src/main.mjs");
  console.error("  node ts/src/main.mjs run <input-json-file>");
}

async function main() {
  const [command, filePath] = process.argv.slice(2);

  if (command === "--help" || command === "-h") {
    usage();
    process.exit(0);
  }

  let inputJson;
  if (!command) {
    inputJson = await readStdin();
  } else if (command === "run" && filePath) {
    inputJson = await readFile(filePath, "utf8");
  } else {
    usage();
    process.exit(1);
  }

  const report = await runJson(inputJson);
  console.log(JSON.stringify(report, null, 2));
}

await main();
