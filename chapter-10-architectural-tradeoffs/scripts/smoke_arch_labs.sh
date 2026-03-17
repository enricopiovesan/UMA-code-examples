#!/usr/bin/env bash
set -euo pipefail

cargo test --locked --manifest-path rust/Cargo.toml
(
  cd ts
  npm test
)

./scripts/validate_architecture.sh
./scripts/list_labs.sh
./scripts/compare_impls.sh lab1-baseline
./scripts/run_arch_demo.sh lab1-baseline
./scripts/diff_architecture.sh lab1-baseline lab2-over-granular
./scripts/compare_impls.sh lab2-over-granular
./scripts/run_arch_demo.sh lab2-over-granular
./scripts/diff_architecture.sh lab2-over-granular lab3-hidden-event-coupling
./scripts/compare_impls.sh lab3-hidden-event-coupling
./scripts/run_arch_demo.sh lab3-hidden-event-coupling
./scripts/diff_architecture.sh lab3-hidden-event-coupling lab4-runtime-ambiguity
./scripts/compare_impls.sh lab4-runtime-ambiguity
./scripts/run_arch_demo.sh lab4-runtime-ambiguity
./scripts/diff_architecture.sh lab4-runtime-ambiguity lab5-over-orchestrated
./scripts/compare_impls.sh lab5-over-orchestrated
./scripts/run_arch_demo.sh lab5-over-orchestrated
./scripts/diff_architecture.sh lab5-over-orchestrated lab6-recovered-architecture
./scripts/compare_impls.sh lab6-recovered-architecture
./scripts/run_arch_demo.sh lab6-recovered-architecture

echo "Chapter 10 smoke run completed successfully."
