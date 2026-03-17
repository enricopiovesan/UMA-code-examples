#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pushd "$ROOT_DIR" >/dev/null
cargo test --offline
./scripts/validate_trust.sh
./scripts/list_labs.sh
./scripts/run_trust_demo.sh lab1-trusted-service
./scripts/trust_diff.sh lab1-trusted-service lab2-undeclared-permission
./scripts/run_trust_demo.sh lab2-undeclared-permission
./scripts/trust_diff.sh lab2-undeclared-permission lab3-untrusted-dependency
./scripts/run_trust_demo.sh lab3-untrusted-dependency
./scripts/trust_diff.sh lab3-untrusted-dependency lab4-forbidden-communication
./scripts/run_trust_demo.sh lab4-forbidden-communication
./scripts/trust_diff.sh lab4-forbidden-communication lab5-restored-compliance
./scripts/run_trust_demo.sh lab5-restored-compliance
popd >/dev/null

echo "Chapter 9 smoke run completed successfully."
