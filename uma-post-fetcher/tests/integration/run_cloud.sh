#!/usr/bin/env bash
set -euo pipefail

# Integration test script for the cloud host.  Builds the WebAssembly
# component and compares the output against the golden fixture.  Requires
# `jq` for JSON comparison.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

GOLDEN="$ROOT_DIR/tests/fixtures/expected_golden.json"

pushd "$ROOT_DIR" > /dev/null

echo "Running cloud host..."
# Invoke the cloud host script and capture its output.  The service
# prints two JSON strings: the service output and the lifecycle
# metadata.  They are separated by whitespace or newlines, so we
# capture the first two non‑empty lines.
RAW_OUTPUT=$(bash hosts/cloud/run.sh)
echo "Raw output:" "$RAW_OUTPUT"

# Extract the first two JSON strings.  We assume they are printed on
# separate lines.  If your WASI runtime prints them on a single
# line separated by space, adjust the parsing accordingly.
OUT_JSON=$(echo "$RAW_OUTPUT" | grep -v '^$' | head -n1)
META_JSON=$(echo "$RAW_OUTPUT" | grep -v '^$' | tail -n +2 | head -n1)

if [[ -z "$OUT_JSON" || -z "$META_JSON" ]]; then
  echo "Failed to parse output JSON strings" >&2
  exit 1
fi

# Combine the two JSON strings into a single object using jq and
# compare it against the golden fixture.  jq -S sorts object keys for
# deterministic comparison.
ACTUAL=$(jq -n --arg out "$OUT_JSON" --arg meta "$META_JSON" '{output: ($out | fromjson), lifecycle: ($meta | fromjson)}' | jq -S .)
EXPECTED=$(jq -S . "$GOLDEN")

echo "Comparing actual output to golden fixture..."
if diff <(echo "$ACTUAL") <(echo "$EXPECTED"); then
  echo "Integration test passed: output matches golden fixture." >&2
else
  echo "Integration test failed: output differs from golden fixture." >&2
  exit 1
fi

popd > /dev/null