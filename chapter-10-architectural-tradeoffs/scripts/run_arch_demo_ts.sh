#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_arch_demo_ts.sh [lab-name]" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 10 labs." >&2
  exit 0
fi

LAB="${1:-lab1-baseline}"
node ts/src/main.mjs render "${LAB}"
