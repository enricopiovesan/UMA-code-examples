# Benchmark And Footprint Notes

These measurements are a reproducible local proof point, not a universal performance claim.

- generated: `2026-03-26T02:41:48Z`
- environment: `macOS-26.3-arm64-arm-64bit-Mach-O`
- rust: `rustc 1.94.0 (4a4ef493e 2026-03-02)`
- node: `v23.11.0`
- wasmtime: `wasmtime 39.0.0 (56b81c98a 2025-11-20)`

## Chapter 4: Feature Flag Evaluator

- WASI module size: `198.00 KiB`
- benchmark input: `lab2-rollout-match.json`
- first measured run: `51.63 ms` (Rust/WASI), `96.45 ms` (TypeScript/Node)
- peak RSS: `17.09 MiB` (Rust/WASI), `37.95 MiB` (TypeScript/Node)

| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| rust_wasi_via_wasmtime | 33.42 | 31.93 | 22.7 | 57.89 | 20 |
| typescript_node | 271.17 | 125.61 | 82.44 | 1662.06 | 20 |

## Chapter 6: Portability Lab

- native runner size: `5.31 MiB`
- WASI runner size: `352.33 KiB`
- benchmark input: `sample-data/sample.pgm`
- first measured run: `336.64 ms` (native), `212.73 ms` (WASI via Wasmtime)
- peak RSS: `7.80 MiB` (native), `45.88 MiB` (WASI via Wasmtime)

| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| native_runner | 56.73 | 45.79 | 25.79 | 157.13 | 20 |
| wasi_runner_via_wasmtime | 379.63 | 290.29 | 152.34 | 1616.95 | 20 |

## Chapter 13: Reference Runtime CLI

- CLI binary size: `947.95 KiB`
- benchmark input: `use-case-1-basic-report`
- first measured run: `964.64 ms`
- peak RSS: `2.19 MiB`

| Path | Mean (ms) | Median (ms) | Min (ms) | Max (ms) | Runs |
| --- | ---: | ---: | ---: | ---: | ---: |
| render_json_cli | 512.36 | 312.45 | 48.06 | 2228.3 | 20 |

## Interpretation

- Chapter 4 shows a very small portable evaluator module with comparable Rust/WASI and TypeScript invocation timings on the same contract-driven input.
- Chapter 6 shows the expected tradeoff: the native runner stays faster, while the WASI runner remains compact and behaviorally aligned.
- Chapter 13 shows the reference runtime can expose a deterministic report path from one release CLI binary (`947.95 KiB`) with a mean local render time of `512.36 ms` for `use-case-1-basic-report`.
- The important proof is not “fastest everywhere.” It is that portable behavior remains measurable, comparable, and explicit across runtime choices.
