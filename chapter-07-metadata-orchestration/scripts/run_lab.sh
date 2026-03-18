#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_lab.sh <lab-name>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 7 labs." >&2
  exit 0
fi

LAB="${1:-}"

if [[ -z "$LAB" ]]; then
  echo "Usage: ./scripts/run_lab.sh <lab-name>" >&2
  exit 1
fi

case "$LAB" in
  lab1-baseline-cloud-flow)
    ./scripts/build_all.sh
    ./scripts/run_cloud.sh
    ;;
  lab2-rust-ts-parity)
    BUILD_OPTIONAL_JS=1 ./scripts/build_all.sh
    ./scripts/compare_impls.sh
    ;;
  lab3-policy-fail-closed)
    ./scripts/build_all.sh
    POLICY_FAIL_MODE=closed ./scripts/run_cloud.sh
    ;;
  lab4-telemetry-audit)
    ./scripts/build_all.sh
    ./scripts/run_cloud.sh >/dev/null
    node tools/validator.js audit
    ;;
  *)
    echo "Unknown lab: $LAB" >&2
    echo "Run ./scripts/list_labs.sh to see the available Chapter 7 labs." >&2
    exit 1
    ;;
esac
