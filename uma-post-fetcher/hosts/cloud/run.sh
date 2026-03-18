#!/usr/bin/env bash
set -euo pipefail

# Build and run the native CLI entrypoint for the runtime package.  The
# sample still supports a WASI build, but the reader quick-start uses the
# native path so outbound HTTP works without additional host bindings.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

usage() {
  cat <<'EOF'
Build and run the Chapter 5 native cloud host path against a local fixture server.

Usage:
  bash hosts/cloud/run.sh

Environment:
  UMA_DEMO_PORT        Override the local fixture port (default: 18080)
  UMA_ENABLE_RETRY     Enable the retry wrapper
  UMA_ENABLE_CACHE     Enable the cache wrapper
EOF
}

if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

pushd "$ROOT_DIR" > /dev/null

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required for the local fixture server used by this quick-start." >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
PORT="${UMA_DEMO_PORT:-18080}"
SERVER_PID=""

cleanup() {
  if [[ -n "$SERVER_PID" ]]; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
  rm -rf "$TMP_DIR"
}

trap cleanup EXIT

mkdir -p "$TMP_DIR/posts"
cp tests/fixtures/sample_post.json "$TMP_DIR/posts/1"
python3 -m http.server "$PORT" --bind 127.0.0.1 --directory "$TMP_DIR" >/dev/null 2>&1 &
SERVER_PID=$!
sleep 1

echo "Building native runtime CLI..."
cargo build -p uma_runtime --release

BIN_PATH="target/release/uma_runtime"
if [[ ! -x "$BIN_PATH" ]]; then
  echo "Failed to build native runtime CLI at $BIN_PATH" >&2
  exit 1
fi

echo "Running the service in cloud host via native CLI..."
INPUT='{"request":{"url":"http://127.0.0.1:'"$PORT"'/posts/1","headers":{"accept":"application/json"}},"runId":"demo-001"}'
printf '%s' "$INPUT" | "$BIN_PATH"

popd > /dev/null
