#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo run --locked --quiet --manifest-path "$ROOT_DIR/rust/Cargo.toml" -- mcp-serve
