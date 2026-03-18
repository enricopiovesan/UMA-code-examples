#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/compare_impls.sh" >&2
  echo "Compares the Rust and TypeScript Chapter 7 runners on the baseline orchestration flow." >&2
  exit 0
fi

rust_output="$(mktemp)"
ts_output="$(mktemp)"
trap 'rm -f "$rust_output" "$ts_output"' EXIT

./scripts/run_cloud.sh >"$rust_output" 2>&1
./scripts/run_cloud_ts.sh >"$ts_output" 2>&1

node - "$rust_output" "$ts_output" <<'EOF'
import fs from "node:fs";
import { summarizeRunnerOutput } from "./runtime/lib.mjs";

const [, , rustPath, tsPath] = process.argv;
const rust = summarizeRunnerOutput(fs.readFileSync(rustPath, "utf-8"));
const ts = summarizeRunnerOutput(fs.readFileSync(tsPath, "utf-8"));

if (JSON.stringify(rust) !== JSON.stringify(ts)) {
  console.error("Implementation mismatch between Rust and TypeScript runners");
  console.error(JSON.stringify({ rust, ts }, null, 2));
  process.exit(1);
}

console.log("Implementation parity verified for Chapter 7 baseline flow");
EOF
