#!/usr/bin/env bash
set -euo pipefail

# Illustrative runner for the edge host scaffold.  This repository does not
# currently ship the generated JS/Wasm binding package that `hosts/edge/run.ts`
# expects, so fail fast with a concrete message instead of an opaque module
# resolution error.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

usage() {
  cat <<'EOF'
Explain why the Chapter 5 edge host is still illustrative and fail with guidance.

Usage:
  bash tests/integration/run_edge.sh
EOF
}

if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

pushd "$ROOT_DIR" > /dev/null

if [[ ! -f hosts/edge/pkg/uma_runtime.js ]]; then
  echo "Edge host scaffold is not fully wired in this sample." >&2
  echo "Generate a compatible JS/Wasm package and update hosts/edge/run.ts before using this script." >&2
  exit 1
fi

echo "Running edge host via Node..."
node hosts/edge/run.ts

popd > /dev/null
