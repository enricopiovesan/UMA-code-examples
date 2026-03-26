# UMA-code-examples Agent Guide

## What this repo is
Book companion for Universal Microservices Architecture. Each chapter-XX folder
is a self-contained validated lab. The repo proves the UMA model with runnable
code, not just descriptions.

## Must not break
Before finishing any task, run:
```bash
./scripts/smoke_reader_paths.sh
```
This is the single acceptance gate. If it fails, the task is not done.

## Repo map
- `chapter-04-*` through `chapter-13-*` — validated reader labs (Rust-first)
- `book-site/` — public site source (universalmicroservices.com)
- `scripts/` — repo-level validation and smoke helpers

## Language rules
- Rust is the validated default implementation path
- TypeScript is a parity path — keep it aligned, never promote it above Rust
- Shell scripts in `scripts/` are reader-facing — keep them readable

## What is validated vs illustrative
- `core/` and `wasi-app/` directories inside any chapter = validated, do not break
- `adapters/` directories inside any chapter = ILLUSTRATIVE, not production paths
- `book-site/` = public-facing, handle with care

## Verification commands by chapter type
Rust chapter:
```bash
RUSTFLAGS='-D warnings' cargo test --locked --manifest-path <chapter>/rust/Cargo.toml
```
TypeScript parity check:
```bash
./scripts/compare_impls.sh
```
Coverage check:
```bash
./scripts/check_rust_coverage.sh
./scripts/check_business_logic_coverage.sh
```

## Coverage floor — do not go below these
- Line: 53% (repo-wide)
- Function: 50% (repo-wide)
- Business logic crates: 100% line, function, region

## Never do
- Modify `contracts/` schemas without updating corresponding tests
- Change `scripts/smoke_reader_paths.sh` behavior without updating CI
- Treat adapter files as validated behavior
- Add dependencies to core/ crates (they must stay stdlib-only)
