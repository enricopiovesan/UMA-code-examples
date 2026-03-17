#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_evolution_demo_ts.sh [lab-name]" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 11 labs." >&2
  exit 0
fi

LAB="${1:-lab1-contract-anchor}"
node ts/src/main.mjs render "${LAB}"
