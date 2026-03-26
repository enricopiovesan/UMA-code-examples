#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SUMMARY_PATH="${ROOT_DIR}/coverage/reader-rust-coverage.md"

MIN_LINES="${MIN_LINES:-56}"
MIN_FUNCTIONS="${MIN_FUNCTIONS:-63}"
MIN_REGIONS="${MIN_REGIONS:-56}"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd jq

"${ROOT_DIR}/scripts/report_rust_coverage.sh"

status=0

for json_path in "${ROOT_DIR}"/coverage/chapter-*.json; do
  chapter="$(basename "$json_path" .json)"
  lines="$(jq -r '.data[0].totals.lines.percent' "$json_path")"
  functions="$(jq -r '.data[0].totals.functions.percent' "$json_path")"
  regions="$(jq -r '.data[0].totals.regions.percent' "$json_path")"

  awk -v value="$lines" -v min="$MIN_LINES" 'BEGIN { exit !(value + 0 >= min + 0) }' || {
    echo "${chapter}: line coverage ${lines}% is below ${MIN_LINES}%." >&2
    status=1
  }

  awk -v value="$functions" -v min="$MIN_FUNCTIONS" 'BEGIN { exit !(value + 0 >= min + 0) }' || {
    echo "${chapter}: function coverage ${functions}% is below ${MIN_FUNCTIONS}%." >&2
    status=1
  }

  awk -v value="$regions" -v min="$MIN_REGIONS" 'BEGIN { exit !(value + 0 >= min + 0) }' || {
    echo "${chapter}: region coverage ${regions}% is below ${MIN_REGIONS}%." >&2
    status=1
  }
done

if [[ "$status" -ne 0 ]]; then
  exit "$status"
fi

echo "Rust coverage thresholds passed. Summary: ${SUMMARY_PATH#$ROOT_DIR/}"
