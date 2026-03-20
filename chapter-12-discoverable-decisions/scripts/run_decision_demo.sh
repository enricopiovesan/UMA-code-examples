#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_decision_demo.sh <lab-name>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 12 labs." >&2
  exit 0
fi

LAB="${1:-}"

if [[ -z "$LAB" ]]; then
  echo "Usage: ./scripts/run_decision_demo.sh <lab-name>" >&2
  exit 1
fi

cargo run --locked --quiet --manifest-path "$ROOT_DIR/rust/Cargo.toml" -- render "$LAB" text
