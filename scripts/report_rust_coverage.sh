#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${ROOT_DIR}/coverage"
SUMMARY_PATH="${OUTPUT_DIR}/reader-rust-coverage.md"

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
  "Chapter 5|uma-post-fetcher/Cargo.toml|--workspace"
  "Chapter 6|uma-portable-service-example/runtime/Cargo.toml|--workspace"
  "Chapter 7|uma-metadata-orchestration/runtime-rust/Cargo.toml|"
  "Chapter 8|chapter-8-service-graph/Cargo.toml|"
  "Chapter 9|chapter-9-trust-boundaries/Cargo.toml|"
  "Chapter 10|chapter-10-architectural-tradeoffs/rust/Cargo.toml|"
  "Chapter 11|chapter-11-evolution-without-fragmentation/rust/Cargo.toml|"
)

printf '# Reader Rust Coverage\n\n' > "$SUMMARY_PATH"
printf '| Chapter | Lines | Functions | Regions |\n' >> "$SUMMARY_PATH"
printf '| --- | ---: | ---: | ---: |\n' >> "$SUMMARY_PATH"

for entry in "${entries[@]}"; do
  IFS='|' read -r label manifest extra <<<"$entry"
  json_path="${OUTPUT_DIR}/$(echo "$label" | tr '[:upper:] ' '[:lower:]-').json"

  if [[ -n "$extra" ]]; then
    cargo llvm-cov --manifest-path "${ROOT_DIR}/${manifest}" --json --summary-only --output-path "$json_path" $extra >/dev/null
  else
    cargo llvm-cov --manifest-path "${ROOT_DIR}/${manifest}" --json --summary-only --output-path "$json_path" >/dev/null
  fi

  lines="$(jq -r '.data[0].totals.lines.percent' "$json_path")"
  functions="$(jq -r '.data[0].totals.functions.percent' "$json_path")"
  regions="$(jq -r '.data[0].totals.regions.percent' "$json_path")"

  printf '| %s | %.2f%% | %.2f%% | %.2f%% |\n' "$label" "$lines" "$functions" "$regions" >> "$SUMMARY_PATH"
done

echo "Coverage summary written to ${SUMMARY_PATH#$ROOT_DIR/}"
