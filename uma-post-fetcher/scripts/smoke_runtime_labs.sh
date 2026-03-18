#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd cargo
require_cmd bash
require_cmd jq
require_cmd python3

pushd "$ROOT_DIR" >/dev/null
cargo test --locked --workspace
./scripts/list_labs.sh
./scripts/run_lab.sh lab1-cloud-golden-path
./scripts/run_lab.sh lab2-header-validation-fail-fast >/dev/null
./scripts/run_lab.sh lab3-adapter-binding-and-wrappers >/dev/null
popd >/dev/null

echo "Chapter 5 smoke run completed successfully."
