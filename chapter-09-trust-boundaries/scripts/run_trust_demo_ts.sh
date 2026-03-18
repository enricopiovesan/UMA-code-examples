#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_trust_demo_ts.sh [scenario-name]" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 9 scenarios." >&2
  exit 0
fi

SCENARIO="${1:-lab1-trusted-service}"

node ts/src/main.mjs render "${SCENARIO}"
