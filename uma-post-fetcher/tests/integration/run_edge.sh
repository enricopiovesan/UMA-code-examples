#!/usr/bin/env bash
set -euo pipefail

# Integration test for the edge host.  Assumes the WASM module has been
# compiled with wasm-bindgen targeting Node or Deno.  Uses Node to run the
# generated JS wrapper and prints the output.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

pushd "$ROOT_DIR" > /dev/null

echo "Running edge host via Node..."
node hosts/edge/run.ts

popd > /dev/null