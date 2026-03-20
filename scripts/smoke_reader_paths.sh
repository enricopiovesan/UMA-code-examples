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

echo "== Chapter 4: chapter-04-feature-flag-evaluator =="
pushd "$ROOT_DIR/chapter-04-feature-flag-evaluator" >/dev/null
./scripts/smoke_flag_labs.sh
popd >/dev/null

echo "== Chapter 5: chapter-05-post-fetcher-runtime =="
pushd "$ROOT_DIR/chapter-05-post-fetcher-runtime" >/dev/null
./scripts/smoke_runtime_labs.sh
popd >/dev/null

echo "== Chapter 6: chapter-06-portability-lab =="
pushd "$ROOT_DIR/chapter-06-portability-lab" >/dev/null
./scripts/smoke_portability_labs.sh
popd >/dev/null

echo "== Chapter 7: chapter-07-metadata-orchestration =="
pushd "$ROOT_DIR/chapter-07-metadata-orchestration" >/dev/null
./scripts/smoke_orchestration_labs.sh
popd >/dev/null

echo "== Chapter 8: chapter-08-service-graph =="
pushd "$ROOT_DIR/chapter-08-service-graph" >/dev/null
./scripts/smoke_graph_labs.sh
popd >/dev/null

echo "== Chapter 9: chapter-09-trust-boundaries =="
pushd "$ROOT_DIR/chapter-09-trust-boundaries" >/dev/null
./scripts/smoke_trust_labs.sh
popd >/dev/null

echo "== Chapter 10: chapter-10-architectural-tradeoffs =="
pushd "$ROOT_DIR/chapter-10-architectural-tradeoffs" >/dev/null
./scripts/smoke_arch_labs.sh
popd >/dev/null

echo "== Chapter 11: chapter-11-evolution-without-fragmentation =="
pushd "$ROOT_DIR/chapter-11-evolution-without-fragmentation" >/dev/null
./scripts/smoke_evolution_labs.sh
popd >/dev/null

echo "== Chapter 12: chapter-12-discoverable-decisions =="
pushd "$ROOT_DIR/chapter-12-discoverable-decisions" >/dev/null
./scripts/smoke_discoverability_labs.sh
popd >/dev/null

echo "Smoke run completed successfully."
