# UMA-code-examples Agent Guide

## What this repo is
Book companion for Universal Microservices Architecture. Each `chapter-XX` folder
is a validated lab or reference example. The repo proves UMA with runnable code.

## Final acceptance gate
Before finishing any task, run:
```bash
./scripts/smoke_reader_paths.sh
```
If it fails, the task is not done.

## Repo map
- `chapter-04-*` through `chapter-13-*` — validated reader labs, Rust-first
- `book-site/` — public website source for universalmicroservices.com
- `scripts/` — repo-level smoke, coverage, and contributor checks
- `benchmarks/` — generated benchmark proof artifacts

## Language rules
- Rust is the authoritative implementation path
- TypeScript is a parity path where present; keep it aligned, never primary
- Shell scripts in `scripts/` and chapter `scripts/` are reader-facing

## What is validated vs illustrative
- Chapter smoke scripts and chapter READMEs define the validated path
- `adapters/` directories are usually illustrative host examples unless a chapter README says otherwise
- `book-site/` is public-facing; changes there affect the live site

## Useful verification commands
Full repo:
```bash
./scripts/check_reader_docs.sh
./scripts/check_rust_coverage.sh
./scripts/check_business_logic_coverage.sh
./scripts/smoke_reader_paths.sh
```
Chapter-local parity, where available:
```bash
cd <chapter-dir>
./scripts/compare_impls.sh
```

## Coverage floor — do not go below these
- Line: 56% repo-wide
- Function: 63% repo-wide
- Region: 56% repo-wide
- Business logic crates: 100% line, function, region

## Never do
- Modify `contracts/` or schemas without updating the matching tests
- Change `scripts/smoke_reader_paths.sh` behavior without updating CI
- Treat illustrative adapters as validated production behavior
- Break Rust warning hygiene on validated crates
- Advertise “write once, run everywhere”; the repo position is “write once, run where it makes sense”
