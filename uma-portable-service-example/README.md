# UMA Portability Lab

This example demonstrates Chapter 6 of the UMA book: one contract, one core service, and two runtime targets that stay behaviorally aligned.
It is a reader-facing hands-on lab, not just a code dump.
The validated path proves that the same `image.analyzed` payload is emitted by both the native runner and the WASI runner while capability-gated native telemetry remains explicit.

## Key concepts

- one UMA contract drives both native and WASI execution
- shared logic lives in a single Rust crate and is reused without forks
- contract parameters change behavior without code edits
- parity is checked through emitted events, not by trusting the implementation

## Prerequisites

- Rust 1.77 or newer
- `rustup target add wasm32-wasip1`
- Wasmtime 20 or newer on your `PATH`
- `jq` for the shared-payload digest lab

## Validation status

- Validated path: `./scripts/smoke_portability_labs.sh`
- Main implementation: Rust workspace in `runtime/`
- Reader labs: `./scripts/list_labs.sh` and `./scripts/run_lab.sh`
- Shared payload parity check: `./scripts/lab_parity.sh`

## Quick start

```bash
./scripts/list_labs.sh
./scripts/run_lab.sh lab1-native-wasm-parity
./scripts/run_lab.sh lab2-shared-payload-digest
./scripts/run_lab.sh lab3-failure-paths-and-capability-gates
./scripts/smoke_portability_labs.sh
```

Expected output signals:

```text
Comparing shared image.analyzed events between native and WASM
Parity check passed: shared events are identical.
Native digest
WASM digest
Failure-path lab completed successfully.
```

## Reader path

Use this order if you just finished Chapter 6 and want the best learning path:

1. `./scripts/list_labs.sh`
2. `./scripts/run_lab.sh lab1-native-wasm-parity`
3. `./scripts/run_lab.sh lab2-shared-payload-digest`
4. `./scripts/run_lab.sh lab3-failure-paths-and-capability-gates`

Expected satisfaction point:
- by the end of lab 3, you should be able to explain what stayed portable, what remained target-specific, and why the contract still governed both targets

## Questions a reader might ask

### "What am I supposed to learn from this?"

You should leave this lab able to explain:

- how one contract can drive both a native binary and a WASI binary
- why parity should be asserted from emitted events rather than assumed from shared code
- how target-specific capabilities such as GPU telemetry can stay explicit without contaminating the portable path

### "What should I pay attention to in the output?"

The most important signals are:

- the identical `image.analyzed` event payload emitted by both targets
- the shared payload digests, which prove parity at the payload level
- the `gpu.telemetry.reported` event, which shows the native-only capability boundary clearly

### "How do I know if the lab gave me value?"

You got value from the Chapter 6 lab if you can explain all three of these points after running it:

- the portable behavior is proven by the shared `image.analyzed` payload, not by trusting the code structure
- target-specific behavior is isolated in the native runner instead of leaking into the portable WASI path
- contract parameters in `CONTRACT.json` can change tagging outcomes without changing the shared Rust implementation

## Layout

- `CONTRACT.json`, Chapter 6 contract for the image analyzer
- `runtime/`, Rust workspace with the contract, bus, shared core logic, and both runners
- `sample-data/`, example input images
- `scripts/`, parity, digest, failure-path, and smoke helpers
- `docs/`, supporting notes such as the runtime sequence diagram

## Contract parameters

The contract includes:

- `parameters.tagging.avg_dark_threshold`
- `parameters.tagging.avg_bright_threshold`

Edit those values in [CONTRACT.json](/Users/piovese/Documents/UMA-code-examples/uma-portable-service-example/CONTRACT.json) and rerun `./scripts/run_lab.sh lab1-native-wasm-parity`.
This is the cleanest way to see Chapter 6’s point that behavior can evolve through the contract without forking the service logic.

## Build and run manually

If you want the lower-level commands instead of the guided labs:

```bash
cd runtime
cargo test --locked --all
cargo build --locked -p runner_wasm --target wasm32-wasip1
cargo run --locked -p runner_native -- ../sample-data/sample.pgm
wasmtime run --dir=.. target/wasm32-wasip1/debug/runner_wasm.wasm ../sample-data/sample.pgm
```

Example event:

```json
{"event":"image.analyzed","payload":{"service":"uma.image-analyzer:1.0.0","path":"../sample-data/sample.pgm","tags":["high_contrast"],"metrics":{"width":8,"height":8,"avg":0.5,"contrast":1.0}}}
```

## Reader labs

See [labs/README.md](/Users/piovese/Documents/UMA-code-examples/uma-portable-service-example/labs/README.md) for the guided Chapter 6 lab notes.

### `lab1-native-wasm-parity`

Runs the native and WASI targets on the same input and compares their shared `image.analyzed` event line by line.

### `lab2-shared-payload-digest`

Builds on lab 1 and computes SHA-256 digests of the shared payloads so parity stays obvious even if you only care about the JSON payload itself.

### `lab3-failure-paths-and-capability-gates`

Exercises malformed input rejection, WASI preopen enforcement, and the native GPU telemetry fallback event.

## Portability matrix

| Concern | WASM runner | Native runner | Where to look |
| --- | --- | --- | --- |
| Contract loading | Yes | Yes | `contract::Contract::load_from` |
| File access | Via WASI preopens | OS file API | `scripts/break_trust.sh` |
| GPU access | No | Optional `gpu` feature | `runtime/crates/runner_native` |
| Event schema check | Yes | Yes | `bus::publish_validated` |
| Deterministic analysis | Yes | Yes | `core_service::analyze_image_data` |
| Time source | None in portable path | Used for telemetry only | `runner_native` |

## Reports and tests

- `cargo test --locked --all` verifies the workspace
- `./scripts/smoke_portability_labs.sh` is the validated Chapter 6 smoke path
- `runtime/tests/events.rs` checks the native runner output shape
- `runtime/tests/smoke.rs` checks that the shared contract is reachable

## Troubleshooting

- If `./scripts/lab_parity.sh` says `Missing required command: wasmtime`, install Wasmtime or place a matching release under the repo `.bin/` directory.
- If the WASM runner cannot read the sample image, make sure the `--dir=..` preopen is present; the validated scripts already pass it.
- If `./scripts/digest_shared.sh` says artifacts are missing, rerun `./scripts/run_lab.sh lab1-native-wasm-parity`.
- If you want to see the native GPU feature path with a real adapter, run `cargo run --locked -p runner_native --features gpu -- ../sample-data/sample.pgm` from `runtime/`.

## Notes

- The bus validates event payloads against the JSON schema in `CONTRACT.json`.
- The WASI runner intentionally omits the native GPU telemetry path.
- The validated reader path is Rust only for this chapter; there is no TypeScript parity implementation here because the point of Chapter 6 is portability across native and WASI targets of the same Rust service.

## Reflection checklist

- Did the parity lab prove portability through observable output rather than assumption?
- Did the failure-path lab make the capability boundary between portable and native behavior obvious?
- Did the contract parameter experiment make behavior changes feel safer than code forks?

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can point to the exact emitted event that proves native and WASI stayed aligned
- you can explain why GPU telemetry belongs in the target-specific native path and not in the portable runner
- you can show how contract parameters let the behavior evolve without changing the shared service logic
