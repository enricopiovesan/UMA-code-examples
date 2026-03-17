#!/usr/bin/env bash
set -euo pipefail
# Compile runner TypeScript on the fly using node --loader if available or ts-node.
# For simplicity, transpile with a local tsc pass if present, otherwise run via ts-node/register.
if ! command -v node >/dev/null 2>&1; then
  echo "Node.js is required"
  exit 1
fi
if ! command -v wasmtime >/dev/null 2>&1; then
  echo "Wasmtime is required to execute WASI modules"
  exit 1
fi
if [ ! -d node_modules/ajv ]; then
  echo "Runtime dependencies are missing."
  echo "Run ./scripts/build_all.sh first so the root runner dependencies are installed."
  exit 1
fi
if [ ! -f services/image.tagger/target/wasm32-wasip1/release/image_tagger.wasm ] || \
   [ ! -f services/edge.cache/target/wasm32-wasip1/release/edge_cache.wasm ]; then
  echo "Built WASI artifacts are missing."
  echo "Run ./scripts/build_all.sh first to compile the services."
  exit 1
fi
# The quick-start path is fail-open so readers can see the full orchestration flow.
POLICY_FAIL_MODE="${POLICY_FAIL_MODE:-open}" node runtime/runner.js
