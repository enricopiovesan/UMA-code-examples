# Contributing

This repository is organized around reader-facing chapter labs. Changes should preserve the validated reader path, not just make the code compile.

The bar for a good contribution here is simple:

- keep the repo useful to a first-time reader
- keep the validated chapter paths honest
- make runtime, docs, and demos tell the same story

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
- keep the live reference app aligned with what the runtime actually does
- do not present illustrative scaffolding as validated behavior

## Documentation Standard

Validated chapter READMEs should keep these sections:

- `## Prerequisites`
- `## Quick start`
- `## Validation status`
- `## Troubleshooting`

At the repo root, keep:

- `## Start Here`
- `## Reader Setup`
- `## Reader Journey`
- `## Why This Repo Exists`
- `## What Makes UMA Different`
- `## What You Can Try Today`
- `## Repo Structure`
- `## If You Want To Evaluate UMA Honestly`
- `## Learn More`

At the chapter level, keep:

- `## Learning path position`
- `## Key concepts`
- `## Prerequisites`
- `## Validation status`
- `## Quick start`
- `## Reader path`
- `## Layout`
- `## Troubleshooting`
- `## Value check`

When the root README changes materially, update any contributor guidance that references its structure.

## Public Repo Hygiene

This repo is also a public landing page for UMA.

When relevant, keep these elements current:

- the root `README.md`
- the Chapter 13 `README.md`
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- [SECURITY.md](SECURITY.md)
- [.github/CODEOWNERS](.github/CODEOWNERS)
- [CITATION.cff](CITATION.cff)

If a change affects the public story of the repo, also review:

- the live reference app link
- the social preview image used in the READMEs
- GitHub Actions badges and links

## Coverage Gate

Rust coverage is reported with:

```bash
./scripts/report_rust_coverage.sh
```

Business-logic-focused Rust coverage is reported with:

```bash
./scripts/report_business_logic_coverage.sh
```

The current CI gate enforces a minimum floor for the validated reader chapters:

- line coverage: `53%`
- function coverage: `50%`
- region coverage: `52%`

Those numbers are a floor, not a target. Raise them when the measured baseline improves.

For pure business-logic crates, the desired target is stricter:

- line coverage: `100%`
- function coverage: `100%`
- region coverage: `100%`

That target is tracked separately through:

```bash
./scripts/check_business_logic_coverage.sh
```

Do not force that standard onto CLI entrypoints, runtime shells, or host adapters. The target applies to deterministic domain logic where full branch coverage is realistic and valuable.

## Clean Checkout Validation

To simulate a first-time reader using only tracked files from the current branch:

```bash
./scripts/simulate_fresh_reader_checkout.sh
```

This exports the current `HEAD` to a temporary directory and reruns the reader docs and smoke checks there.
