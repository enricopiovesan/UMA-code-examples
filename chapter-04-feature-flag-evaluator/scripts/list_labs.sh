#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

find "$ROOT_DIR/labs/inputs" -maxdepth 1 -type f -name '*.json' -print \
  | sed "s#^$ROOT_DIR/labs/inputs/##" \
  | sed 's/\.json$//' \
  | sort
