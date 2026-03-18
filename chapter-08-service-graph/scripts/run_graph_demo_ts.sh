#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_graph_demo_ts.sh [scenario-name]" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 8 scenarios." >&2
  exit 0
fi

SCENARIO="${1:-${CH8_SCENARIO:-lab1-upload-only}}"

node ts/src/main.mjs render "${SCENARIO}"
