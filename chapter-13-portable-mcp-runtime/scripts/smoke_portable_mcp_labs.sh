#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pushd "$ROOT_DIR" >/dev/null

./scripts/setup_models.sh
./scripts/build_planner_ai_wasi.sh
./scripts/build_summarizer_ai_wasi.sh
./scripts/build_translator_ai_wasi.sh

cargo test --locked --manifest-path rust/Cargo.toml

for lab in $(./scripts/list_labs.sh --ids-only); do
  ./scripts/run_lab.sh "$lab" >/dev/null
done

./scripts/smoke_mcp_server.sh

echo "Chapter 13 portable MCP labs passed."
popd >/dev/null
