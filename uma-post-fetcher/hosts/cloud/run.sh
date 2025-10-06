#!/usr/bin/env bash
set -euo pipefail

# Build the runtime for the wasm32-wasi target.  Requires Rust with the
# wasm32-wasi target installed and `wasmtime` available in PATH.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
pushd "$ROOT_DIR" > /dev/null

echo "Building WebAssembly module..."
cargo build -p uma_runtime --release --target wasm32-wasi

WASM_PATH="target/wasm32-wasi/release/uma_runtime.wasm"
if [[ ! -f "$WASM_PATH" ]]; then
  echo "Failed to build WebAssembly module at $WASM_PATH" >&2
  exit 1
fi

echo "Running the service in cloud host via wasmtime..."
INPUT='{"request":{"url":"https://jsonplaceholder.typicode.com/posts/1","headers":{"accept":"application/json"}},"runId":"demo-001"}'
wasmtime run "$WASM_PATH" --invoke run_json "$INPUT"

popd > /dev/null