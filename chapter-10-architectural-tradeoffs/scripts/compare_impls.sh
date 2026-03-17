#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/compare_impls.sh <lab-name>" >&2
  echo "Run ./scripts/list_labs.sh to see the available Chapter 10 labs." >&2
  exit 0
fi

LAB="${1:-}"

if [[ -z "$LAB" ]]; then
  echo "Usage: ./scripts/compare_impls.sh <lab-name>" >&2
  exit 1
fi

RUST_JSON="$(cargo run --offline --quiet --manifest-path rust/Cargo.toml -- render "$LAB" json)"
TS_JSON="$(node ts/src/main.mjs render "$LAB" json)"

node - "$LAB" "$RUST_JSON" "$TS_JSON" <<'EOF'
const [, , lab, rustRaw, tsRaw] = process.argv;
const rust = JSON.parse(rustRaw);
const ts = JSON.parse(tsRaw);

const summarize = (report) => ({
  verdict: report.assessment.verdict,
  warnings: report.assessment.warnings.map((item) => item.code).sort(),
  choices: report.choices,
  serviceIds: report.services.map((service) => service.id).sort()
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
