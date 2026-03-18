#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'EOF'
Compare the Chapter 6 Rust portability implementation with the TypeScript reference implementation.

Usage:
  ./scripts/compare_impls.sh [sample|bright]

If no argument is provided, both built-in sample images are checked.
EOF
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

run_single() {
  local label="$1"
  local image_path
  local rust_json
  local ts_json

  case "$label" in
    sample)
      image_path="$ROOT_DIR/sample-data/sample.pgm"
      ;;
    bright)
      image_path="$ROOT_DIR/sample-data/bright.pgm"
      ;;
    *)
      echo "Unknown sample label: $label" >&2
      usage >&2
      exit 1
      ;;
  esac

  rust_json="$(
    cd "$ROOT_DIR/runtime"
    cargo run --locked --quiet -p runner_native -- "$image_path" | grep '"event":"image.analyzed"'
  )"
  ts_json="$(cd "$ROOT_DIR" && node ts/src/main.mjs analyze "$image_path")"

  node - "$label" "$rust_json" "$ts_json" <<'EOF'
const assert = require("node:assert/strict");
const [, , label, rustRaw, tsRaw] = process.argv;
const rust = JSON.parse(rustRaw);
const ts = JSON.parse(tsRaw);

try {
  assert.deepStrictEqual(rust, ts);
} catch {
  console.error(`Implementation mismatch for ${label}`);
  console.error(JSON.stringify({ rust, ts }, null, 2));
  process.exit(1);
}

console.log(`Implementation parity verified for ${label}`);
EOF
}

require_cmd cargo
require_cmd node

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

if [[ $# -eq 0 ]]; then
  run_single sample
  run_single bright
else
  run_single "$1"
fi
