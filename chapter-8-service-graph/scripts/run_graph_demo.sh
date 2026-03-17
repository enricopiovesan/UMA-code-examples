#!/usr/bin/env bash
set -euo pipefail

SCENARIO="${1:-${CH8_SCENARIO:-lab1-upload-only}}"

echo "Running Chapter 8 graph demo for ${SCENARIO}"
node runtime/graph.mjs "${SCENARIO}"
