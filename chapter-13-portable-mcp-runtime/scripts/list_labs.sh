#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST_PATH="$ROOT_DIR/rust/Cargo.toml"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/list_labs.sh [--ids-only]" >&2
  exit 0
fi

if [[ "${1:-}" == "--ids-only" ]]; then
  cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- list
  exit 0
fi

while IFS= read -r lab; do
  report="$(cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- render "$lab" text)"
  title="$(printf '%s\n' "$report" | sed -n '1p')"
  summary="$(printf '%s\n' "$report" | sed -n '4s/^Summary: //p')"
  printf '%s - %s\n' "$lab" "$title"
  printf '  %s\n' "$summary"
done < <(cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- list)
