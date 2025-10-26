#!/usr/bin/env bash
set -euo pipefail
if [ ! -f logs/telemetry.jsonl ]; then
  echo "No telemetry found"
  exit 0
fi
count=$(wc -l < logs/telemetry.jsonl | tr -d ' ')
p99=$(cat logs/telemetry.jsonl | jq -r 'select(.metric=="uma.qos.latency.ms") | .value' | sort -n | awk 'BEGIN{c=0} {a[c++]=$1} END{if(c==0){print "n/a"} else {idx=int(0.99*c); if(idx<1){idx=1}; print a[idx-1]}}')
echo "Telemetry entries: $count"
echo "Latency p99: $p99 ms"
