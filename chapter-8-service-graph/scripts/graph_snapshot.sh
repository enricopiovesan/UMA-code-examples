#!/usr/bin/env bash
set -euo pipefail

SCENARIO="${1:-${CH8_SCENARIO:-lab1-upload-only}}"

node runtime/graph.mjs "${SCENARIO}" json
