#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_lab.sh <scenario-id>" >&2
  exit 0
fi

SCENARIO="${1:-}"
if [[ -z "$SCENARIO" ]]; then
  echo "Usage: ./scripts/run_lab.sh <scenario-id>" >&2
  exit 1
fi

cargo run --locked --quiet --manifest-path "$ROOT_DIR/rust/Cargo.toml" -- render "$SCENARIO" text
