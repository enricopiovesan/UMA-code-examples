
#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACT_DIR="${PORTABILITY_LAB_ARTIFACT_DIR:-$ROOT_DIR/.artifacts/chapter6}"
NATIVE_JSONL="$ARTIFACT_DIR/native.jsonl"
WASM_JSONL="$ARTIFACT_DIR/wasm.jsonl"

usage() {
  cat <<'EOF'
Show SHA-256 digests for the shared Chapter 6 image analysis payloads.

Usage:
  ./scripts/digest_shared.sh

Run ./scripts/lab_parity.sh first so the native and WASM JSONL artifacts exist.
EOF
}

if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "Missing required command: jq" >&2
  exit 1
fi

if [[ ! -f "$NATIVE_JSONL" || ! -f "$WASM_JSONL" ]]; then
  echo "Missing parity artifacts. Run ./scripts/lab_parity.sh first." >&2
  exit 1
fi

echo "Native digest"
jq -c 'select(.event=="image.analyzed")|.payload' "$NATIVE_JSONL" | shasum -a 256
echo "WASM digest"
jq -c 'select(.event=="image.analyzed")|.payload' "$WASM_JSONL" | shasum -a 256
