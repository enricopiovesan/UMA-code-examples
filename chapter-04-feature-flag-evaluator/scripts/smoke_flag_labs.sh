#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

add_local_wasmtime_to_path() {
  local candidate

  for candidate in "$ROOT_DIR"/../.bin/wasmtime-*; do
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

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

add_local_wasmtime_to_path

require_cmd cargo
require_cmd node
require_cmd npm
require_cmd wasmtime

cargo test --locked -p ff_eval_core --manifest-path "$ROOT_DIR/Cargo.toml"
cargo build --release --target wasm32-wasip1 -p ff_eval_wasi_app --manifest-path "$ROOT_DIR/Cargo.toml" >/dev/null
npm test --prefix "$ROOT_DIR/ts"
"$ROOT_DIR/scripts/list_labs.sh"
"$ROOT_DIR/scripts/run_vectors.sh"
"$ROOT_DIR/scripts/run_lab.sh" lab4-rule-language >/dev/null
"$ROOT_DIR/scripts/compare_impls.sh"

echo "Chapter 4 smoke run completed successfully."
