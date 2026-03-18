import path from "node:path";
import process from "node:process";
import { renderEvent } from "./lib.mjs";

function usage() {
  console.error("Usage:");
  console.error("  node ts/src/main.mjs analyze <absolute-or-relative-image-path>");
}

async function main() {
  const [command, imagePath] = process.argv.slice(2);
  if (command === "--help" || command === "-h") {
    usage();
    process.exit(0);
  }

  if (command !== "analyze" || !imagePath) {
    usage();
    process.exit(1);
  }

  const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");
  const event = await renderEvent(rootDir, path.resolve(imagePath));
  console.log(JSON.stringify(event));
}

await main();
