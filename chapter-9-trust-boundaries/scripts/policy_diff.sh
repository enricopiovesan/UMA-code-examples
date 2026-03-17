#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/policy_diff.sh <from-scenario> <to-scenario>" >&2
  echo "Show the raw Git diff for the Chapter 9 scenario files." >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 9 scenarios." >&2
  exit 0
fi

FROM_SCENARIO="${1:-}"
TO_SCENARIO="${2:-}"

if [[ -z "${FROM_SCENARIO}" || -z "${TO_SCENARIO}" ]]; then
  echo "Usage: ./scripts/policy_diff.sh <from-scenario> <to-scenario>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 9 scenarios." >&2
  exit 1
fi

if [[ ! -d "scenarios/${FROM_SCENARIO}" || ! -d "scenarios/${TO_SCENARIO}" ]]; then
  echo "Unknown scenario in policy diff request." >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 9 scenarios." >&2
  exit 1
fi

set +e
git --no-pager diff --no-index -- "scenarios/${FROM_SCENARIO}" "scenarios/${TO_SCENARIO}"
status=$?
set -e

if [[ $status -gt 1 ]]; then
  exit "$status"
fi
