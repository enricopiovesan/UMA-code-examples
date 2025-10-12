
# UMA Chapter 6.7 Example

This example demonstrates a UMA service that runs the same logic on both a WASM binary and a native binary, using one contract and one runtime shape.

## Structure

- `CONTRACT.json` the UMA contract for the service.
- `runtime/` Rust workspace with crates for contract types, a tiny JSONL bus, shared core logic, and two runners.
- `sample-data/sample.pgm` a small test image.
- `.github/workflows/ci.yml` basic CI that builds and tests.

## Requirements

- Rust 1.77+
- For WASM build, `rustup target add wasm32-wasi`
- Optional, a WASI runner like `wasmtime` to execute the `.wasm`

## Build and Run

```bash
cd runtime
rustup target add wasm32-wasi
cargo build -p runner_wasm --target wasm32-wasi
cargo run -p runner_native -- ../sample-data/sample.pgm
cargo run -p runner_native --features gpu -- ../sample-data/sample.pgm
```

To run the WASM binary with `wasmtime`:
```bash
wasmtime target/wasm32-wasi/debug/runner_wasm.wasm -- ../sample-data/sample.pgm
```

Events are emitted as JSON lines to stdout, for example:
```json
{"event":"image.analyzed","payload":{"service":"uma.image-analyzer:1.0.0","path":"../sample-data/sample.pgm","tags":["high_contrast"],"metrics":{"width":8,"height":8,"avg":0.5,"contrast":1.0}}}
```

## Tests

Run all tests:
```bash
cd runtime
cargo test --all
```

What tests cover:
- Contract loader reads and validates the top-level `CONTRACT.json`.
- Bus layer formats events deterministically.
- Core image parser and analyzer are deterministic and robust.
- Workspace smoke test ensures the contract is reachable by runners.

## CI

A basic GitHub Actions workflow is included under `.github/workflows/ci.yml`. It builds all crates, runs tests, and compiles the WASM target to ensure portability.
```


## Contract enforcement

The bus validates every emitted payload against the event schema stored in `CONTRACT.json` using `jsonschema`. If a mismatch is found, the process fails fast. This keeps behavior aligned with the contract.

## Makefile shortcuts

- `make build` builds native and WASM.
- `make test` runs all tests.
- `make run-native` runs the native runner.
- `make run-wasm` executes the WASM with wasmtime if present.

## Troubleshooting

- If the WASM runner cannot read the sample image, pass `--dir=.` or run from repo root depending on your wasmtime settings.
- GPU telemetry requires the `gpu` feature and a compatible adapter. If not present, the service still emits a `gpu.telemetry.reported` with a reason field.


## Reader labs

- `scripts/lab_parity.sh` runs both targets on the same input and compares shared events.
- `scripts/digest_shared.sh` computes content hashes of the shared event payloads to prove parity.
- `scripts/break_trust.sh` demonstrates controlled failure modes that remain deterministic.

## Portability matrix

| Concern | WASM runner | Native runner | Where to look |
| --- | --- | --- | --- |
| Contract loading | Yes | Yes | `contract::Contract::load_from` |
| File access | Via WASI preopens | OS file API | README, wasmtime flags |
| GPU access | No | Optional `wgpu` feature | `runner_native`, capability gate |
| Event schema check | Yes | Yes | `bus::publish_validated` |
| Deterministic analysis | Yes | Yes | `core_service::analyze_image` |
| Time source | None in portable path | Used for telemetry only | `chrono::Utc` in native |

## Contract parameters

The contract includes `parameters.tagging.avg_dark_threshold` and `avg_bright_threshold`. Edit these values in `CONTRACT.json` and rerun `scripts/lab_parity.sh` to see how tags change without any code edits.
