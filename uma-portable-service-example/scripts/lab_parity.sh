
#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACT_DIR="${PORTABILITY_LAB_ARTIFACT_DIR:-$ROOT_DIR/.artifacts/chapter6}"
NATIVE_JSONL="$ARTIFACT_DIR/native.jsonl"
WASM_JSONL="$ARTIFACT_DIR/wasm.jsonl"

usage() {
  cat <<'EOF'
Run the Chapter 6 native-versus-WASM parity lab.

Usage:
  ./scripts/lab_parity.sh

Artifacts:
  Writes JSONL outputs under .artifacts/chapter6/ unless PORTABILITY_LAB_ARTIFACT_DIR is set.
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

mkdir -p "$ARTIFACT_DIR"

pushd "$ROOT_DIR/runtime" >/dev/null
rustup target add wasm32-wasip1 >/dev/null 2>&1 || true
cargo build --locked -p runner_wasm --target wasm32-wasip1 >/dev/null
cargo run --locked -p runner_native -- ../sample-data/sample.pgm | tee "$NATIVE_JSONL"
wasmtime run --dir=.. target/wasm32-wasip1/debug/runner_wasm.wasm ../sample-data/sample.pgm | tee "$WASM_JSONL"
popd >/dev/null

grep '"image.analyzed"' "$NATIVE_JSONL" > "$ARTIFACT_DIR/native.image.jsonl" || true
grep '"image.analyzed"' "$WASM_JSONL" > "$ARTIFACT_DIR/wasm.image.jsonl" || true

echo "Comparing shared image.analyzed events between native and WASM"
if diff -u "$ARTIFACT_DIR/native.image.jsonl" "$ARTIFACT_DIR/wasm.image.jsonl"; then
  echo "Parity check passed: shared events are identical."
else
  echo "Shared image analysis diverged between native and WASM." >&2
  exit 1
fi
