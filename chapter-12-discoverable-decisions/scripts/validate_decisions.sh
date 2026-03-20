#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/validate_decisions.sh [lab-name]" >&2
  echo "Without a lab name, validate all Chapter 12 scenarios." >&2
  exit 0
fi

if [[ -n "${1:-}" ]]; then
  cargo run --locked --quiet --manifest-path "$ROOT_DIR/rust/Cargo.toml" -- validate "$1"
else
  cargo run --locked --quiet --manifest-path "$ROOT_DIR/rust/Cargo.toml" -- validate
fi
