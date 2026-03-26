name: validate-chapter
description: Validate a specific chapter lab. Use when asked to validate, check, fix, or test any chapter-XX directory. Runs the chapter smoke path, chapter tests, and parity checks where applicable.
---

## Steps

1. Identify the chapter directory from the user's request.
2. Find the chapter smoke script:
```bash
find <chapter-dir>/scripts -maxdepth 1 -name 'smoke_*.sh'
```
3. Run the chapter smoke script from the chapter root.
4. If the chapter has `scripts/list_labs.sh` and `scripts/run_lab.sh`, list labs and run the first lab directly.
5. If the chapter has a TypeScript parity path, run:
```bash
npm test --prefix ts
./scripts/compare_impls.sh
```
   These parity checks assume `wasmtime` is available on `PATH`. In this repo the expected local pinned binary path is usually:
```bash
/Users/piovese/Documents/UMA-code-examples/.bin/wasmtime-v39.0.0-aarch64-macos/wasmtime
```
   Verify it with:
```bash
wasmtime --version
```
6. Run Rust tests for the chapter's validated manifest. Common patterns are:
```bash
cargo test --locked --manifest-path <chapter>/Cargo.toml
cargo test --locked --manifest-path <chapter>/rust/Cargo.toml
cargo test --locked --manifest-path <chapter>/runtime/Cargo.toml
```
7. Report pass/fail per step, exact failing command, and the most likely fix.

## What counts as success

All chapter-local checks exit `0`. If the task changes repo behavior, the repo is not fixed until `./scripts/smoke_reader_paths.sh` also passes at the root.

## Never do

- Do not invent a generic Cargo path without checking the chapter layout.
- Do not mark illustrative adapter output as validated behavior.
- Do not skip parity checks when a `ts/` path and `scripts/compare_impls.sh` both exist.
- Do not mark anything fixed until `./scripts/smoke_reader_paths.sh` passes at the repo root.
