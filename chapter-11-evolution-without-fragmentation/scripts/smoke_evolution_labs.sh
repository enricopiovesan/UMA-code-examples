#!/usr/bin/env bash
set -euo pipefail

cargo test --locked --manifest-path rust/Cargo.toml
(
  cd ts
  npm test
)

./scripts/validate_evolution.sh
./scripts/list_labs.sh
./scripts/compare_impls.sh lab1-contract-anchor
./scripts/run_evolution_demo.sh lab1-contract-anchor
./scripts/diff_evolution.sh lab1-contract-anchor lab2-behavioral-drift
./scripts/compare_impls.sh lab2-behavioral-drift
./scripts/run_evolution_demo.sh lab2-behavioral-drift
./scripts/diff_evolution.sh lab2-behavioral-drift lab3-duplicate-implementations
./scripts/compare_impls.sh lab3-duplicate-implementations
./scripts/run_evolution_demo.sh lab3-duplicate-implementations
./scripts/diff_evolution.sh lab3-duplicate-implementations lab4-version-sprawl
./scripts/compare_impls.sh lab4-version-sprawl
./scripts/run_evolution_demo.sh lab4-version-sprawl
./scripts/diff_evolution.sh lab4-version-sprawl lab5-runtime-governed-coexistence
./scripts/compare_impls.sh lab5-runtime-governed-coexistence
./scripts/run_evolution_demo.sh lab5-runtime-governed-coexistence
./scripts/diff_evolution.sh lab5-runtime-governed-coexistence lab6-hybrid-adoption
./scripts/compare_impls.sh lab6-hybrid-adoption
./scripts/run_evolution_demo.sh lab6-hybrid-adoption

echo "Chapter 11 smoke run completed successfully."
