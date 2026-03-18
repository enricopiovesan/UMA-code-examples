#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/compare_impls.sh <scenario-name>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 9 scenarios." >&2
  exit 0
fi

SCENARIO="${1:-}"

if [[ -z "$SCENARIO" ]]; then
  echo "Usage: ./scripts/compare_impls.sh <scenario-name>" >&2
  exit 1
fi

RUST_JSON="$(cargo run --locked --quiet -- render "$SCENARIO" json)"
TS_JSON="$(node ts/src/main.mjs render "$SCENARIO" json)"

node - "$SCENARIO" "$RUST_JSON" "$TS_JSON" <<'EOF'
const [, , lab, rustRaw, tsRaw] = process.argv;
const rust = JSON.parse(rustRaw);
const ts = JSON.parse(tsRaw);

const summarize = (report) => ({
  outcome: report.outcome,
  serviceIds: report.services.map((service) => service.id).sort(),
  auditLog: report.auditLog
    .map((entry) => `${entry.decision}:${entry.kind}:${entry.subject}:${entry.reason}`)
    .sort()
});

const left = summarize(rust);
const right = summarize(ts);

if (JSON.stringify(left) !== JSON.stringify(right)) {
  console.error(`Implementation mismatch for ${lab}`);
  console.error(JSON.stringify({ rust: left, ts: right }, null, 2));
  process.exit(1);
}

console.log(`Implementation parity verified for ${lab}`);
EOF
