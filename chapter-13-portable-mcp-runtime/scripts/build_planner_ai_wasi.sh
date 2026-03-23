#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo build \
  --manifest-path "$ROOT_DIR/planner-ai-wasi/Cargo.toml" \
  --target wasm32-wasip1

echo "Built Chapter 13 PlannerAI WASI module."
