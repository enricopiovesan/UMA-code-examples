#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOCAL_WASMTIME="$ROOT_DIR/.bin/wasmtime-v39.0.0-aarch64-macos"

if [[ -d "$LOCAL_WASMTIME" ]]; then
  export PATH="$LOCAL_WASMTIME:$PATH"
fi

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd cargo
require_cmd node
require_cmd npm
require_cmd wasmtime

echo "== Chapter 4: feature-flag-evaluator =="
pushd "$ROOT_DIR/feature-flag-evaluator" >/dev/null
cargo test -p ff_eval_core
cargo build --release --target wasm32-wasip1 -p ff_eval_wasi_app
./scripts/run_vectors.sh
popd >/dev/null

echo "== Chapter 5: uma-post-fetcher =="
pushd "$ROOT_DIR/uma-post-fetcher" >/dev/null
cargo test --workspace
bash hosts/cloud/run.sh
bash tests/integration/run_cloud.sh
popd >/dev/null

echo "== Chapter 6: uma-portable-service-example =="
pushd "$ROOT_DIR/uma-portable-service-example/runtime" >/dev/null
cargo test --all
popd >/dev/null
pushd "$ROOT_DIR/uma-portable-service-example" >/dev/null
./scripts/lab_parity.sh
popd >/dev/null

echo "== Chapter 7: uma-metadata-orchestration =="
pushd "$ROOT_DIR/uma-metadata-orchestration" >/dev/null
./scripts/build_all.sh
./scripts/run_cloud.sh
bash -x ./tests/test_orchestration.sh
popd >/dev/null

echo "== Chapter 8: chapter-8-service-graph =="
pushd "$ROOT_DIR/chapter-8-service-graph" >/dev/null
./scripts/smoke_graph_labs.sh
popd >/dev/null

echo "Smoke run completed successfully."
