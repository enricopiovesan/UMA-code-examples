#!/usr/bin/env bash
set -euo pipefail

# Integration test script for the cloud host.  Builds the WebAssembly
# component and compares the output against the golden fixture.  Requires
# `jq` for JSON comparison.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

GOLDEN="$ROOT_DIR/tests/fixtures/expected_golden.json"

usage() {
  cat <<'EOF'
Run the validated Chapter 5 cloud integration test and compare it to the golden fixture.

Usage:
  bash tests/integration/run_cloud.sh
EOF
}

if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

pushd "$ROOT_DIR" > /dev/null

echo "Running cloud host..."
# Invoke the cloud host script and capture its output.  The script now
# prints one pretty‑formatted JSON object containing both the output and
# lifecycle payloads.
RAW_OUTPUT=$(bash hosts/cloud/run.sh)
echo "Raw output:" "$RAW_OUTPUT"

if [[ -z "$RAW_OUTPUT" ]]; then
  echo "Cloud host script produced no output" >&2
  exit 1
fi

JSON_OUTPUT=$(printf '%s\n' "$RAW_OUTPUT" | sed -n '/^{/,$p')
if [[ -z "$JSON_OUTPUT" ]]; then
  echo "Failed to isolate JSON payload from cloud host output" >&2
  exit 1
fi

ACTUAL=$(printf '%s\n' "$JSON_OUTPUT" | jq -S .)
EXPECTED=$(jq -S . "$GOLDEN")

echo "Comparing actual output to golden fixture..."
if diff <(echo "$ACTUAL") <(echo "$EXPECTED"); then
  echo "Integration test passed: output matches golden fixture." >&2
else
  echo "Integration test failed: output differs from golden fixture." >&2
  exit 1
fi

popd > /dev/null
