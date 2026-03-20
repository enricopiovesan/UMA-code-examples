#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST_PATH="$ROOT_DIR/rust/Cargo.toml"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/validate_lab.sh [scenario-id]" >&2
  exit 0
fi

if [[ -n "${1:-}" ]]; then
  cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- validate "$1"
else
  cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- validate
fi
