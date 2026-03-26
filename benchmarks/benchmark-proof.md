# Benchmark And Footprint Notes

These measurements are a reproducible local proof point, not a universal performance claim.

- generated: `2026-03-26T02:12:16Z`
- environment: `macOS-26.3-arm64-arm-64bit-Mach-O`
- rust: `rustc 1.94.0 (4a4ef493e 2026-03-02)`
- node: `v23.11.0`
- wasmtime: `wasmtime 39.0.0 (56b81c98a 2025-11-20)`

## Chapter 4: Feature Flag Evaluator

- WASI module size: `198.00 KiB`
- benchmark input: `lab2-rollout-match.json`

| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| rust_wasi_via_wasmtime | 28.01 | 28.58 | 21.23 | 39.42 | 20 |
| typescript_node | 91.89 | 84.26 | 70.0 | 174.95 | 20 |

## Chapter 6: Portability Lab

- native runner size: `5.31 MiB`
- WASI runner size: `352.33 KiB`
- benchmark input: `sample-data/sample.pgm`

| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| native_runner | 21.87 | 21.4 | 13.15 | 38.64 | 20 |
| wasi_runner_via_wasmtime | 99.74 | 101.35 | 74.9 | 126.02 | 20 |

## Chapter 13: Reference Runtime CLI

- CLI binary size: `947.95 KiB`
- benchmark input: `use-case-1-basic-report`

| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| render_json_cli | 21.01 | 20.93 | 10.93 | 31.1 | 20 |

## Interpretation

- Chapter 4 shows a very small portable evaluator module with comparable Rust/WASI and TypeScript invocation timings on the same contract-driven input.
- Chapter 6 shows the expected tradeoff: the native runner stays faster, while the WASI runner remains compact and behaviorally aligned.
- Chapter 13 shows the reference runtime can expose a deterministic report path from one release CLI binary (`947.95 KiB`) with a mean local render time of `21.01 ms` for `use-case-1-basic-report`.
- The important proof is not “fastest everywhere.” It is that portable behavior remains measurable, comparable, and explicit across runtime choices.
