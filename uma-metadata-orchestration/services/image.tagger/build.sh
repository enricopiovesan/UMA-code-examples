#!/usr/bin/env bash
set -euo pipefail
rustup target add wasm32-wasi >/dev/null 2>&1 || true
cargo build --release --target wasm32-wasi
echo "Built: target/wasm32-wasi/release/image_tagger.wasm"
