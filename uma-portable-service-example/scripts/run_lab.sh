#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'EOF'
Run one of the guided Chapter 6 reader labs.

Usage:
  ./scripts/run_lab.sh <lab-name>

Available labs:
  lab1-native-wasm-parity
  lab2-shared-payload-digest
  lab3-failure-paths-and-capability-gates
EOF
}

run_lab() {
  case "$1" in
    lab1-native-wasm-parity)
      "$ROOT_DIR/scripts/lab_parity.sh"
      ;;
    lab2-shared-payload-digest)
      "$ROOT_DIR/scripts/lab_parity.sh"
      "$ROOT_DIR/scripts/digest_shared.sh"
      ;;
    lab3-failure-paths-and-capability-gates)
      "$ROOT_DIR/scripts/break_trust.sh"
      ;;
    *)
      echo "Unknown lab: $1" >&2
      echo >&2
      usage >&2
      exit 1
      ;;
  esac
}

if [[ "${1:-}" == "--help" || $# -eq 0 ]]; then
  usage
  [[ $# -eq 0 ]] && exit 1
  exit 0
fi

run_lab "$1"
