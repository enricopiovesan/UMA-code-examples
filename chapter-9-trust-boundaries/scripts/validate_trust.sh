#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/validate_trust.sh [scenario-name ...]" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 9 scenarios." >&2
  exit 0
fi

node runtime/validate.mjs "$@"
