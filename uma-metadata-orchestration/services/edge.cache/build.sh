#!/usr/bin/env bash
set -euo pipefail
rustup target add wasm32-wasip1 >/dev/null 2>&1 || true
cargo build --release --target wasm32-wasip1
echo "Built: target/wasm32-wasip1/release/edge_cache.wasm"
