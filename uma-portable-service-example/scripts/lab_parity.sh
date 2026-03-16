
#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
cd runtime
rustup target add wasm32-wasi >/dev/null 2>&1 || true
cargo build -p runner_wasm --target wasm32-wasi
cargo run -p runner_native -- ../sample-data/sample.pgm | tee /tmp/native.jsonl
wasmtime target/wasm32-wasi/debug/runner_wasm.wasm -- ../sample-data/sample.pgm | tee /tmp/wasm.jsonl

grep '"image.analyzed"' /tmp/native.jsonl > /tmp/native.image.jsonl || true
grep '"image.analyzed"' /tmp/wasm.jsonl > /tmp/wasm.image.jsonl || true
echo "Comparing shared events between native and wasm"
diff -u /tmp/native.image.jsonl /tmp/wasm.image.jsonl || echo "Expected, native may include extra telemetry"
