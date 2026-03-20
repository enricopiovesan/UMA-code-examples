#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pushd "$ROOT_DIR" >/dev/null

./scripts/validate_decisions.sh
cargo test --locked --manifest-path rust/Cargo.toml
npm test --prefix ts

for lab in $(./scripts/list_labs.sh); do
  ./scripts/run_decision_demo.sh "$lab" >/dev/null
  ./scripts/run_decision_demo_ts.sh "$lab" >/dev/null
  ./scripts/compare_impls.sh "$lab"
done

./scripts/diff_decisions.sh lab1-capability-projection lab6-queryable-trace >/dev/null

echo "Chapter 12 discoverability labs passed."
popd >/dev/null
