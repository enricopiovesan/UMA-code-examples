#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/contract_diff.sh <from-scenario> <to-scenario>" >&2
  echo "Show the raw Git diff for the Chapter 8 service contracts." >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 8 scenarios." >&2
  exit 0
fi

FROM_SCENARIO="${1:-}"
TO_SCENARIO="${2:-}"

if [[ -z "${FROM_SCENARIO}" || -z "${TO_SCENARIO}" ]]; then
  echo "Usage: ./scripts/contract_diff.sh <from-scenario> <to-scenario>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 8 scenarios." >&2
  exit 1
fi

if [[ ! -d "scenarios/${FROM_SCENARIO}" || ! -d "scenarios/${TO_SCENARIO}" ]]; then
  echo "Unknown scenario in contract diff request." >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 8 scenarios." >&2
  exit 1
fi

set +e
git --no-pager diff --no-index -- "scenarios/${FROM_SCENARIO}/services" "scenarios/${TO_SCENARIO}/services"
status=$?
set -e

if [[ $status -gt 1 ]]; then
  exit "$status"
fi
