#!/usr/bin/env bash
set -euo pipefail

FROM_SCENARIO="${1:-}"
TO_SCENARIO="${2:-}"

if [[ -z "${FROM_SCENARIO}" || -z "${TO_SCENARIO}" ]]; then
  echo "Usage: ./scripts/trust_diff.sh <from-scenario> <to-scenario>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 9 scenarios." >&2
  exit 1
fi

node runtime/trust_diff.mjs "${FROM_SCENARIO}" "${TO_SCENARIO}"
