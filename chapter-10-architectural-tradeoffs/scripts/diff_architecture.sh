#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/diff_architecture.sh <from-lab> <to-lab>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 10 labs." >&2
  exit 0
fi

FROM_LAB="${1:-}"
TO_LAB="${2:-}"

if [[ -z "$FROM_LAB" || -z "$TO_LAB" ]]; then
  echo "Usage: ./scripts/diff_architecture.sh <from-lab> <to-lab>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 10 labs." >&2
  exit 1
fi

cargo run --locked --quiet --manifest-path rust/Cargo.toml -- diff "$FROM_LAB" "$TO_LAB"
