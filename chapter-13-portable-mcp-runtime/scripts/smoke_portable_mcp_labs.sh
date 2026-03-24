#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pushd "$ROOT_DIR" >/dev/null

# Reader smoke should validate the stable entry path, not force full AI setup.
# The heavier model downloads and WASI AI builds are already covered elsewhere.
./scripts/list_labs.sh --ids-only >/dev/null
./scripts/run_lab.sh use-case-1-basic-report >/dev/null
cargo run --locked --quiet --manifest-path rust/Cargo.toml -- render use-case-1-basic-report json >/dev/null

./scripts/smoke_mcp_server.sh

echo "Chapter 13 portable MCP labs passed."
popd >/dev/null
