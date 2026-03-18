# Contributing

This repository is organized around reader-facing chapter labs. Changes should preserve the validated reader path, not just make the code compile.

## Acceptance Bar

Before opening or merging a change, run the checks that apply to your work:

- `./scripts/check_reader_docs.sh`
- `./scripts/smoke_reader_paths.sh`
- `./scripts/simulate_fresh_reader_checkout.sh`
- `./scripts/check_rust_coverage.sh`
- `RUSTFLAGS='-D warnings' cargo test --locked --manifest-path <chapter-manifest>`
- `npm test --prefix <chapter-ts-dir>` for chapters that keep a TypeScript parity path

If a chapter has both Rust and TypeScript implementations, keep Rust as the validated default path and maintain parity in the reader-visible scenarios.

## Reader Contract

When changing a chapter example:

- keep the README aligned with the actual quick start and validated path
- keep optional or illustrative paths clearly labeled as such
- update smoke scripts when the validated reader path changes
- update tests when behavior, errors, or outputs change
- prefer deterministic fixtures over live external dependencies

## Documentation Standard

Validated chapter READMEs should keep these sections:

- `## Prerequisites`
- `## Quick start`
- `## Validation status`
- `## Troubleshooting`

At the repo root, keep:

- `## Reader Setup`
- `## Reader Journey`
- `## Chapter Status`
- `## Chapter Index`

At the chapter level, keep:

- `## Key concepts`
- `## Prerequisites`
- `## Validation status`
- `## Quick start`
- `## Reader path`
- `## Layout`
- `## Troubleshooting`
- `## Value check`

## Coverage Gate

Rust coverage is reported with:

```bash
./scripts/report_rust_coverage.sh
```

The current CI gate enforces a minimum floor for the validated reader chapters:

- line coverage: `53%`
- function coverage: `50%`
- region coverage: `52%`

Those numbers are a floor, not a target. Raise them when the measured baseline improves.

## Clean Checkout Validation

To simulate a first-time reader using only tracked files from the current branch:

```bash
./scripts/simulate_fresh_reader_checkout.sh
```

This exports the current `HEAD` to a temporary directory and reruns the reader docs and smoke checks there.
