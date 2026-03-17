#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/validate_graph_contracts.sh [scenario-name ...]" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 8 scenarios." >&2
  exit 0
fi

  cargo run --locked --quiet -- validate "$@"
