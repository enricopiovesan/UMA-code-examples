#!/usr/bin/env bash
set -euo pipefail

FROM_SCENARIO="${1:-}"
TO_SCENARIO="${2:-}"

if [[ -z "${FROM_SCENARIO}" || -z "${TO_SCENARIO}" ]]; then
  echo "Usage: ./scripts/policy_diff.sh <from-scenario> <to-scenario>" >&2
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
