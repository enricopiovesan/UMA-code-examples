#!/usr/bin/env bash
set -euo pipefail

if ! command -v node >/dev/null 2>&1; then
  echo "Node.js is required"
  exit 1
fi

if ! command -v wasmtime >/dev/null 2>&1; then
  os="$(uname -s | tr '[:upper:]' '[:lower:]')"
  arch="$(uname -m)"
  case "$arch" in
    arm64) arch="aarch64" ;;
    x86_64) arch="x86_64" ;;
  esac
  case "$os" in
    darwin) os="macos" ;;
    linux) os="linux" ;;
  esac
  for base in ".bin" "../.bin"; do
    found=$(find "$base" -maxdepth 2 -type f -path "*${arch}-${os}*/wasmtime" 2>/dev/null | head -n 1 || true)
    if [ -n "${found:-}" ]; then
      PATH="$(dirname "$found"):$PATH"
      export PATH
      break
    fi
  done
fi

if ! command -v wasmtime >/dev/null 2>&1; then
  echo "Wasmtime is required to execute the TypeScript reference runner"
  exit 1
fi

if [ ! -f services/image.tagger/target/wasm32-wasip1/release/image_tagger.wasm ] || \
   [ ! -f services/edge.cache/target/wasm32-wasip1/release/edge_cache.wasm ]; then
  echo "Built WASI artifacts are missing."
  echo "Run BUILD_OPTIONAL_JS=1 ./scripts/build_all.sh first."
  exit 1
fi

if [ ! -f services/telemetry.logger/dist/index.js ] || \
   [ ! -f services/ai.model.evaluator/dist/index.js ]; then
  echo "Built TypeScript service artifacts are missing."
  echo "Run BUILD_OPTIONAL_JS=1 ./scripts/build_all.sh first."
  exit 1
fi

POLICY_FAIL_MODE="${POLICY_FAIL_MODE:-open}" node runtime/runner.js
