#!/usr/bin/env bash
set -euo pipefail
rm -rf logs/events logs/telemetry.jsonl
./scripts/build_all.sh
./scripts/run_cloud.sh >/dev/null
# Find the most recent image.analyzed event in logs/events and compare data
file=$(grep -l '"type": "image.analyzed.v1"' logs/events/*.json | tail -n 1)
if [ -z "$file" ]; then
  echo "No image.analyzed.v1 event found"
  exit 2
fi
jq '.data' "$file" > /tmp/evt.json
diff -u expected/image.analyzed.v1.json /tmp/evt.json && echo "Test passed"
