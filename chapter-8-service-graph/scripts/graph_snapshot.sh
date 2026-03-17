#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/graph_snapshot.sh [scenario-name]" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 8 scenarios." >&2
  exit 0
fi

SCENARIO="${1:-${CH8_SCENARIO:-lab1-upload-only}}"

cargo run --offline --quiet -- render "${SCENARIO}" json
