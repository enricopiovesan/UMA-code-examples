#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

require_heading() {
  local file="$1"
  local heading="$2"
  if ! rg -q "^## ${heading}$" "$file"; then
    echo "Missing heading '## ${heading}' in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local text="$2"
  if ! rg -q "$text" "$file"; then
    echo "Missing expected text '${text}' in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_heading "$ROOT_DIR/README.md" "Reader Setup"
require_heading "$ROOT_DIR/README.md" "Chapter Status"
require_text "$ROOT_DIR/README.md" "Validated path"

for chapter in \
  "$ROOT_DIR/uma-metadata-orchestration/README.md" \
  "$ROOT_DIR/chapter-8-service-graph/README.md" \
  "$ROOT_DIR/chapter-9-trust-boundaries/README.md" \
  "$ROOT_DIR/chapter-10-architectural-tradeoffs/README.md" \
  "$ROOT_DIR/chapter-11-evolution-without-fragmentation/README.md"
do
  require_heading "$chapter" "Prerequisites"
  require_heading "$chapter" "Quick start"
  require_heading "$chapter" "Validation status"
  require_heading "$chapter" "Troubleshooting"
done

echo "Reader documentation checks passed."
