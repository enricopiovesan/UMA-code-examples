#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

add_local_wasmtime_to_path() {
  local candidate

  for candidate in "$ROOT_DIR"/../.bin/wasmtime-* "$ROOT_DIR"/.bin/wasmtime-*; do
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
require_cmd rustup
require_cmd wasmtime
require_cmd jq

pushd "$ROOT_DIR/runtime" >/dev/null
cargo test --locked --all
popd >/dev/null

"$ROOT_DIR/scripts/list_labs.sh"
"$ROOT_DIR/scripts/run_lab.sh" lab1-native-wasm-parity >/dev/null
"$ROOT_DIR/scripts/run_lab.sh" lab2-shared-payload-digest
"$ROOT_DIR/scripts/run_lab.sh" lab3-failure-paths-and-capability-gates

echo "Chapter 6 smoke run completed successfully."
