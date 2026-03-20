#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_decision_demo_ts.sh <lab-name>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 12 labs." >&2
  exit 0
fi

LAB="${1:-}"

if [[ -z "$LAB" ]]; then
  echo "Usage: ./scripts/run_decision_demo_ts.sh <lab-name>" >&2
  exit 1
fi

node ts/src/main.mjs render "$LAB" text
