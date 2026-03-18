#!/usr/bin/env bash
set -euo pipefail

if [ ! -d node_modules ]; then
  npm install
fi

./scripts/validate_contracts.sh
cargo test --locked --manifest-path runtime-rust/Cargo.toml
npm test
./scripts/list_labs.sh
./scripts/run_lab.sh lab1-baseline-cloud-flow >/dev/null
BUILD_OPTIONAL_JS=1 ./scripts/build_all.sh >/dev/null
./scripts/compare_impls.sh
if POLICY_FAIL_MODE=closed ./scripts/run_cloud.sh >/tmp/chapter7-policy.log 2>&1; then
  echo "Expected fail-closed policy lab to stop the runner" >&2
  exit 1
fi
grep -q 'policy.violation' /tmp/chapter7-policy.log
./scripts/run_lab.sh lab4-telemetry-audit
./tests/test_orchestration.sh

echo "Chapter 7 smoke run completed successfully."
