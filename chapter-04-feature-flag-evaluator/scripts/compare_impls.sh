#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if ! command -v node >/dev/null 2>&1; then
  echo "Missing required command: node" >&2
  exit 1
fi

if ! command -v wasmtime >/dev/null 2>&1; then
  echo "Missing required command: wasmtime" >&2
  exit 1
fi

cargo build --release --target wasm32-wasip1 -p ff_eval_wasi_app --manifest-path "$ROOT_DIR/Cargo.toml" >/dev/null
WASM_PATH="$ROOT_DIR/target/wasm32-wasip1/release/ff_eval_wasi_app.wasm"

for lab in $("${ROOT_DIR}/scripts/list_labs.sh"); do
  input_path="$ROOT_DIR/labs/inputs/${lab}.json"
  rust_output="$(wasmtime "$WASM_PATH" < "$input_path")"
  ts_output="$(node "$ROOT_DIR/ts/src/main.mjs" < "$input_path")"

  node - "$lab" "$rust_output" "$ts_output" <<'EOF'
const assert = require('assert');

const [, , lab, rustOutput, tsOutput] = process.argv;
assert.deepStrictEqual(JSON.parse(tsOutput), JSON.parse(rustOutput));
console.log(`Implementation parity verified for ${lab}`);
EOF
done
