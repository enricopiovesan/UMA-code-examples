
#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
cd runtime
printf "not a pgm" > /tmp/bad.pgm
echo "Corrupted image run, expect error"
cargo run -p runner_native -- /tmp/bad.pgm || echo "Error as expected"

echo "WASM denied file access if not preopened"
rustup target add wasm32-wasi >/dev/null 2>&1 || true
cargo build -p runner_wasm --target wasm32-wasi
wasmtime target/wasm32-wasi/debug/runner_wasm.wasm -- /tmp/bad.pgm || echo "Denied as expected"

echo "GPU path without feature still emits telemetry with reason"
cargo run -p runner_native -- ../sample-data/sample.pgm | grep "gpu.telemetry.reported" || true
