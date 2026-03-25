#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SUMMARY_PATH="${ROOT_DIR}/coverage/business-logic-rust-coverage.md"

TARGET_LINES="${TARGET_LINES:-100}"
TARGET_FUNCTIONS="${TARGET_FUNCTIONS:-100}"
TARGET_REGIONS="${TARGET_REGIONS:-100}"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd jq

bash "${ROOT_DIR}/scripts/report_business_logic_coverage.sh"

status=0

for json_path in "${ROOT_DIR}"/coverage/business-*.json; do
  target="$(basename "$json_path" .json)"
  lines="$(jq -r '.data[0].totals.lines.percent' "$json_path")"
  functions="$(jq -r '.data[0].totals.functions.percent' "$json_path")"
  regions="$(jq -r '.data[0].totals.regions.percent' "$json_path")"

  awk -v value="$lines" -v target="$TARGET_LINES" 'BEGIN { exit !(value + 0 >= target + 0) }' || {
    echo "${target}: line coverage ${lines}% is below ${TARGET_LINES}%." >&2
    status=1
  }

  awk -v value="$functions" -v target="$TARGET_FUNCTIONS" 'BEGIN { exit !(value + 0 >= target + 0) }' || {
    echo "${target}: function coverage ${functions}% is below ${TARGET_FUNCTIONS}%." >&2
    status=1
  }

  awk -v value="$regions" -v target="$TARGET_REGIONS" 'BEGIN { exit !(value + 0 >= target + 0) }' || {
    echo "${target}: region coverage ${regions}% is below ${TARGET_REGIONS}%." >&2
    status=1
  }
done

if [[ "$status" -ne 0 ]]; then
  exit "$status"
fi

echo "Business logic Rust coverage target passed. Summary: ${SUMMARY_PATH#$ROOT_DIR/}"
