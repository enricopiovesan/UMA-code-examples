#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

add_local_wasmtime_to_path() {
  local candidate

  for candidate in "$ROOT_DIR"/.bin/wasmtime-*; do
    if [[ -x "$candidate/wasmtime" ]]; then
      export PATH="$candidate:$PATH"
      return
    fi

    if [[ -x "$candidate/bin/wasmtime" ]]; then
      export PATH="$candidate/bin:$PATH"
      return
    fi
  done
}

add_local_wasmtime_to_path

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

echo "== Chapter 9: chapter-9-trust-boundaries =="
pushd "$ROOT_DIR/chapter-9-trust-boundaries" >/dev/null
./scripts/smoke_trust_labs.sh
popd >/dev/null

echo "== Chapter 10: chapter-10-architectural-tradeoffs =="
pushd "$ROOT_DIR/chapter-10-architectural-tradeoffs" >/dev/null
./scripts/smoke_arch_labs.sh
popd >/dev/null

echo "Smoke run completed successfully."
