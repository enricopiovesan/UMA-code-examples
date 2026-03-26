# Scripts

This directory contains the top-level repo scripts used for reader validation, coverage, and contributor quality checks.

## Reader validation

- `smoke_reader_paths.sh`
  - runs the validated reader path across the repo
  - use this when you want confidence that the book-aligned examples still work end to end

- `simulate_fresh_reader_checkout.sh`
  - simulates a fresh reader setup flow from a clean checkout
  - use this when changing setup instructions, prerequisites, or repo entry points

## Rust coverage

- `report_rust_coverage.sh`
  - generates the general Rust coverage report for the validated chapter crates

- `check_rust_coverage.sh`
  - enforces the general Rust coverage floor used in CI for reader validation

- `report_business_logic_coverage.sh`
  - generates the focused coverage report for pure business-logic crates

- `check_business_logic_coverage.sh`
  - enforces the `100%` business-logic coverage target used in CI

## Benchmark proof

- `report_benchmark_proof.py`
  - builds the benchmark targets, measures a small reproducible set of local timings, and writes the published benchmark artifacts in `benchmarks/`

## Reader documentation

- `check_reader_docs.sh`
  - verifies that the root README and chapter READMEs still match the expected reader-facing structure

## Typical contributor workflow

If you changed core code or documentation, the most useful sequence is:

```bash
./scripts/check_reader_docs.sh
./scripts/check_business_logic_coverage.sh
./scripts/check_rust_coverage.sh
./scripts/smoke_reader_paths.sh
```

You do not need to run every script for every change. Use the narrower checks when working on one chapter or one surface.
