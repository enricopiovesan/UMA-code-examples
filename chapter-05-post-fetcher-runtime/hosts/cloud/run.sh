#!/usr/bin/env bash
set -euo pipefail

# Build and run the native CLI entrypoint for the runtime package.  The
# sample still supports a WASI build, but the reader quick-start uses the
# native path so outbound HTTP works without additional host bindings.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

usage() {
  cat <<'EOF'
Build and run the Chapter 5 native cloud host path against the hermetic fixture adapter.

Usage:
  bash hosts/cloud/run.sh

Environment:
  UMA_ENABLE_RETRY     Enable the retry wrapper
  UMA_ENABLE_CACHE     Enable the cache wrapper
EOF
}

if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

pushd "$ROOT_DIR" > /dev/null

echo "Building native runtime CLI..."
cargo build -p uma_runtime --release

BIN_PATH="target/release/uma_runtime"
if [[ ! -x "$BIN_PATH" ]]; then
  echo "Failed to build native runtime CLI at $BIN_PATH" >&2
  exit 1
fi

echo "Running the service in cloud host via native CLI..."
INPUT='{"request":{"url":"uma-fixture://sample-post","headers":{"accept":"application/json"}},"runId":"demo-001"}'
printf '%s' "$INPUT" | "$BIN_PATH"

popd > /dev/null
