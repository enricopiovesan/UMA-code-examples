#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

has_text() {
  local pattern="$1"
  local file="$2"

  if command -v rg >/dev/null 2>&1; then
    rg -q "$pattern" "$file"
  else
    grep -Eq "$pattern" "$file"
  fi
}

require_heading() {
  local file="$1"
  local heading="$2"
  if ! has_text "^## ${heading}$" "$file"; then
    echo "Missing heading '## ${heading}' in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local text="$2"
  if ! has_text "$text" "$file"; then
    echo "Missing expected text '${text}' in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_heading "$ROOT_DIR/README.md" "Why This Repo Exists"
require_heading "$ROOT_DIR/README.md" "What Makes UMA Different"
require_heading "$ROOT_DIR/README.md" "What You Can Try Today"
require_heading "$ROOT_DIR/README.md" "Start Here"
require_heading "$ROOT_DIR/README.md" "Reader Journey"
require_heading "$ROOT_DIR/README.md" "Repo Structure"
require_heading "$ROOT_DIR/README.md" "Reader Setup"
require_heading "$ROOT_DIR/README.md" "If You Want To Evaluate UMA Honestly"
require_heading "$ROOT_DIR/README.md" "Learn More"
require_heading "$ROOT_DIR/README.md" "License"
require_text "$ROOT_DIR/README.md" "chapter-04-feature-flag-evaluator"
require_text "$ROOT_DIR/README.md" "What problem does UMA solve"
require_text "$ROOT_DIR/README.md" "Buy the book and learn more about UMA"
require_text "$ROOT_DIR/README.md" "Try the live UMA Reference APP"
require_text "$ROOT_DIR/README.md" "smoke_reader_paths.sh"
require_text "$ROOT_DIR/README.md" "reference-application"

require_heading "$ROOT_DIR/CONTRIBUTING.md" "Acceptance Bar"
require_heading "$ROOT_DIR/CONTRIBUTING.md" "Reader Contract"
require_heading "$ROOT_DIR/CONTRIBUTING.md" "Coverage Gate"

for chapter in \
  "$ROOT_DIR/chapter-04-feature-flag-evaluator/README.md" \
  "$ROOT_DIR/chapter-05-post-fetcher-runtime/README.md" \
  "$ROOT_DIR/chapter-06-portability-lab/README.md" \
  "$ROOT_DIR/chapter-07-metadata-orchestration/README.md" \
  "$ROOT_DIR/chapter-08-service-graph/README.md" \
  "$ROOT_DIR/chapter-09-trust-boundaries/README.md" \
  "$ROOT_DIR/chapter-10-architectural-tradeoffs/README.md" \
  "$ROOT_DIR/chapter-11-evolution-without-fragmentation/README.md" \
  "$ROOT_DIR/chapter-12-discoverable-decisions/README.md" \
  "$ROOT_DIR/chapter-13-portable-mcp-runtime/README.md"
do
  require_heading "$chapter" "Learning path position"
  require_heading "$chapter" "Key concepts"
  require_heading "$chapter" "Prerequisites"
  require_heading "$chapter" "Validation status"
  require_heading "$chapter" "Quick start"
  require_heading "$chapter" "Reader path"
  require_heading "$chapter" "Layout"
  require_heading "$chapter" "Troubleshooting"
  require_heading "$chapter" "Value check"
done

echo "Reader documentation checks passed."
