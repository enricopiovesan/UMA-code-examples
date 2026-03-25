#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${ROOT_DIR}/coverage"
SUMMARY_PATH="${OUTPUT_DIR}/business-logic-rust-coverage.md"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd cargo
require_cmd cargo-llvm-cov
require_cmd jq

mkdir -p "$OUTPUT_DIR"

entries=(
  "Chapter 4 core|chapter-04-feature-flag-evaluator/core/Cargo.toml|--lib"
  "Chapter 5 service|chapter-05-post-fetcher-runtime/Cargo.toml|--package service --lib"
  "Chapter 6 core_service|chapter-06-portability-lab/runtime/Cargo.toml|--package core_service --lib"
  "Chapter 8 service graph|chapter-08-service-graph/Cargo.toml|--lib"
  "Chapter 9 trust boundaries|chapter-09-trust-boundaries/Cargo.toml|--lib"
  "Chapter 10 tradeoffs|chapter-10-architectural-tradeoffs/rust/Cargo.toml|--lib"
  "Chapter 11 evolution|chapter-11-evolution-without-fragmentation/rust/Cargo.toml|--lib"
  "Chapter 12 discoverable decisions|chapter-12-discoverable-decisions/rust/Cargo.toml|--lib"
  "Chapter 13 portable mcp runtime|chapter-13-portable-mcp-runtime/rust/Cargo.toml|--lib --ignore-filename-regex chapter-13-portable-mcp-runtime/rust/src/(mcp|hosted|rendering|storage|lib_tests)\\.rs"
)

printf '# Business Logic Rust Coverage\n\n' > "$SUMMARY_PATH"
printf 'This report tracks the crates that are closest to pure domain or business logic. Runtime shells, CLIs, and host adapters remain in the general reader coverage report.\n\n' >> "$SUMMARY_PATH"
printf '| Target | Lines | Functions | Regions |\n' >> "$SUMMARY_PATH"
printf '| --- | ---: | ---: | ---: |\n' >> "$SUMMARY_PATH"

for entry in "${entries[@]}"; do
  IFS='|' read -r label manifest extra <<<"$entry"
  slug="$(echo "$label" | tr '[:upper:] ' '[:lower:]-' | tr -cd 'a-z0-9-')"
  json_path="${OUTPUT_DIR}/business-${slug}.json"

  read -r -a extra_args <<<"$extra"
  cargo llvm-cov --manifest-path "${ROOT_DIR}/${manifest}" --json --summary-only --output-path "$json_path" "${extra_args[@]}" >/dev/null

  lines="$(jq -r '.data[0].totals.lines.percent' "$json_path")"
  functions="$(jq -r '.data[0].totals.functions.percent' "$json_path")"
  regions="$(jq -r '.data[0].totals.regions.percent' "$json_path")"

  printf '| %s | %.2f%% | %.2f%% | %.2f%% |\n' "$label" "$lines" "$functions" "$regions" >> "$SUMMARY_PATH"
done

echo "Business logic coverage summary written to ${SUMMARY_PATH#$ROOT_DIR/}"
