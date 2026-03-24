#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pushd "$ROOT_DIR" >/dev/null

./scripts/setup_models.sh
./scripts/build_planner_ai_wasi.sh
./scripts/build_summarizer_ai_wasi.sh
./scripts/build_translator_ai_wasi.sh

./scripts/list_labs.sh --ids-only >/dev/null
./scripts/run_lab.sh use-case-1-basic-report >/dev/null
cargo run --locked --quiet --manifest-path rust/Cargo.toml -- render use-case-1-basic-report json >/dev/null

./scripts/smoke_mcp_server.sh

echo "Chapter 13 portable MCP labs passed."
popd >/dev/null
