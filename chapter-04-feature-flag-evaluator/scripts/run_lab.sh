#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IMPL="rust"
LAB=""

usage() {
  cat <<'EOF'
Usage:
  ./scripts/run_lab.sh <lab-name>
  ./scripts/run_lab.sh --impl ts <lab-name>

Options:
  --impl rust|ts   Choose which implementation to run. Rust is the validated default.
  --help           Show this help message.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --impl)
      IMPL="${2:-}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      LAB="$1"
      shift
      ;;
  esac
done

if [[ -z "$LAB" ]]; then
  usage >&2
  exit 1
fi

INPUT_PATH="$ROOT_DIR/labs/inputs/${LAB}.json"
if [[ ! -f "$INPUT_PATH" ]]; then
  echo "Unknown lab: $LAB" >&2
  echo "Available labs:" >&2
  "$ROOT_DIR/scripts/list_labs.sh" >&2
  exit 1
fi

case "$IMPL" in
  rust)
    WASM_PATH="$ROOT_DIR/target/wasm32-wasip1/release/ff_eval_wasi_app.wasm"
    if [[ ! -f "$WASM_PATH" ]]; then
      cargo build --release --target wasm32-wasip1 -p ff_eval_wasi_app --manifest-path "$ROOT_DIR/Cargo.toml" >/dev/null
    fi
    if ! command -v wasmtime >/dev/null 2>&1; then
      echo "Missing required command: wasmtime" >&2
      exit 1
    fi
    wasmtime "$WASM_PATH" < "$INPUT_PATH"
    ;;
  ts)
    node "$ROOT_DIR/ts/src/main.mjs" < "$INPUT_PATH"
    ;;
  *)
    echo "Unsupported implementation: $IMPL" >&2
    exit 1
    ;;
esac
