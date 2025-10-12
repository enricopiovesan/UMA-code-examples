
#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null; then
  echo "jq is required for this script"
  exit 1
fi

cd "$(dirname "$0")/.."
native="/tmp/native.jsonl"
wasm="/tmp/wasm.jsonl"
if [ ! -f "$native" ] || [ ! -f "$wasm" ]; then
  echo "Run lab_parity.sh first"
  exit 1
fi

echo "Native digest"
jq -c 'select(.event=="image.analyzed")|.payload' "$native" | sha256sum
echo "WASM digest"
jq -c 'select(.event=="image.analyzed")|.payload' "$wasm" | sha256sum
