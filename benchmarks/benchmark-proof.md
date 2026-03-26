# Benchmark And Footprint Notes

These measurements are a reproducible local proof point, not a universal performance claim.

- generated: `2026-03-26T01:52:46Z`
- environment: `macOS-26.3-arm64-arm-64bit-Mach-O`
- rust: `rustc 1.94.0 (4a4ef493e 2026-03-02)`
- node: `v23.11.0`
- wasmtime: `wasmtime 39.0.0 (56b81c98a 2025-11-20)`

## Chapter 4: Feature Flag Evaluator

- WASI module size: `198.00 KiB`
- benchmark input: `lab2-rollout-match.json`

| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| rust_wasi_via_wasmtime | 32.48 | 30.54 | 21.18 | 62.39 | 20 |
| typescript_node | 110.37 | 103.75 | 67.12 | 193.22 | 20 |

## Chapter 6: Portability Lab

- native runner size: `5.31 MiB`
- WASI runner size: `352.33 KiB`
- benchmark input: `sample-data/sample.pgm`

| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| native_runner | 23.98 | 24.96 | 14.29 | 32.93 | 20 |
| wasi_runner_via_wasmtime | 102.26 | 103.4 | 89.13 | 118.4 | 20 |

## Interpretation

- Chapter 4 shows a very small portable evaluator module with comparable Rust/WASI and TypeScript invocation timings on the same contract-driven input.
- Chapter 6 shows the expected tradeoff: the native runner stays faster, while the WASI runner remains compact and behaviorally aligned.
- The important proof is not “fastest everywhere.” It is that portable behavior remains measurable, comparable, and explicit across runtime choices.
