#!/bin/bash
#
# Run all test vectors through the compiled evaluator using wasmtime.
#
# Usage:
#   ./scripts/run_vectors.sh
#
# Ensure that you have built the evaluator with:
#   cargo build --release --target wasm32-wasi -p ff_eval_wasi_app
# and that `wasmtime` is installed and available on your PATH.

set -euo pipefail
WASM="$(dirname "$0")/../target/wasm32-wasi/release/ff_eval_wasi_app.wasm"
if [ ! -f "$WASM" ]; then
  echo "error: wasm module not found at $WASM"
  echo "build the module with: cargo build --release --target wasm32-wasi -p ff_eval_wasi_app"
  exit 1
fi

VECTORS_DIR="$(dirname "$0")/../tests/vectors"
for vector in "$VECTORS_DIR"/*.json; do
  echo "=== Running $(basename "$vector") ==="
  if ! wasmtime "$WASM" < "$vector"; then
    echo "error: evaluator failed for $vector"
    exit 1
  fi
done
echo "All vectors executed successfully."