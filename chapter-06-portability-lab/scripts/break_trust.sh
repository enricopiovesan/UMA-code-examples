
#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

usage() {
  cat <<'EOF'
Exercise Chapter 6 failure paths and capability gates.

Usage:
  ./scripts/break_trust.sh

This lab covers:
  - malformed input rejection
  - WASI preopen enforcement
  - native GPU telemetry fallback behavior
EOF
}

if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

add_local_wasmtime_to_path() {
  local candidate

  for candidate in "$ROOT_DIR"/../.bin/wasmtime-* "$ROOT_DIR"/.bin/wasmtime-*; do
    if [[ -x "$candidate/wasmtime" ]]; then
      export PATH="$candidate:$PATH"
      return
    fi

    if [[ -x "$candidate/bin/wasmtime" ]]; then
      export PATH="$candidate/bin:$PATH"
      return
    fi
  done
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

add_local_wasmtime_to_path
require_cmd cargo
require_cmd rustup
require_cmd wasmtime

printf "not a pgm" > "$TMP_DIR/bad.pgm"

pushd "$ROOT_DIR/runtime" >/dev/null
echo "Corrupted image run: expect native validation failure"
if cargo run --locked -p runner_native -- "$TMP_DIR/bad.pgm"; then
  echo "Expected native runner to reject malformed PGM input." >&2
  exit 1
fi

echo "WASM runner without repo preopen: expect access denial or file-open failure"
rustup target add wasm32-wasip1 >/dev/null 2>&1 || true
cargo build --locked -p runner_wasm --target wasm32-wasip1 >/dev/null
if wasmtime run target/wasm32-wasip1/debug/runner_wasm.wasm "$TMP_DIR/bad.pgm"; then
  echo "Expected the WASM runner to fail without the required preopen directory." >&2
  exit 1
fi

echo "Native GPU telemetry fallback still emits gpu.telemetry.reported"
if ! cargo run --locked -p runner_native -- ../sample-data/sample.pgm | grep -q '"event":"gpu.telemetry.reported"'; then
  echo "Expected native runner to emit gpu.telemetry.reported." >&2
  exit 1
fi
popd >/dev/null

echo "Failure-path lab completed successfully."
