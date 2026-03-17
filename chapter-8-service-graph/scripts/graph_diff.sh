#!/usr/bin/env bash
set -euo pipefail

FROM_SCENARIO="${1:-}"
TO_SCENARIO="${2:-}"

if [[ -z "${FROM_SCENARIO}" || -z "${TO_SCENARIO}" ]]; then
  echo "Usage: ./scripts/graph_diff.sh <from-scenario> <to-scenario>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 8 scenarios." >&2
  exit 1
fi

cargo run --locked --quiet -- graph-diff "${FROM_SCENARIO}" "${TO_SCENARIO}"
