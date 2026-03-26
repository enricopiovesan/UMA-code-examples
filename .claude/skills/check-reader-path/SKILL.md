---
name: check-reader-path
description: Simulate a first-time reader path and verify the full repo remains coherent end to end. Use when asked to validate the repo, check all chapters, or confirm CI acceptance locally.
---

## Steps

1. Run the full reader smoke from the repo root:
```bash
./scripts/smoke_reader_paths.sh
```
2. Run the fresh checkout simulation:
```bash
./scripts/simulate_fresh_reader_checkout.sh
```
3. Run the docs contract check:
```bash
./scripts/check_reader_docs.sh
```
4. If the task touched tests or coverage, also run:
```bash
./scripts/check_rust_coverage.sh
./scripts/check_business_logic_coverage.sh
```
5. Report exactly which script failed, the chapter involved, and the error output.

## What counts as success

All repo-level validation scripts exit `0` with no chapter failures reported.

## Context

`./scripts/smoke_reader_paths.sh` is the primary acceptance gate for the repo. A PR is not done if that script fails.
