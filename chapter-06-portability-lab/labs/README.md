# Chapter 6 Labs

These labs accompany the Chapter 6 portability example.

## Lab 1: Native and WASM parity

Command:

```bash
./scripts/run_lab.sh lab1-native-wasm-parity
```

Goal:
- prove that the shared `image.analyzed` event payload is identical across the native and WASI runners

What to pay attention to:
- the diff step should pass cleanly
- the native runner may emit an extra `gpu.telemetry.reported` event, but that event is outside the shared parity check on purpose

## Lab 2: Shared payload digest

Command:

```bash
./scripts/run_lab.sh lab2-shared-payload-digest
```

Goal:
- reduce the parity proof to a digest of the emitted payloads so you can reason about the shared contract result directly

What to pay attention to:
- the native and WASM SHA-256 digests should match

## Lab 3: Failure paths and capability gates

Command:

```bash
./scripts/run_lab.sh lab3-failure-paths-and-capability-gates
```

Goal:
- see which failures are portable and which behaviors are intentionally target-specific

What to pay attention to:
- malformed input should fail fast
- the WASI runner should fail without the expected preopen
- the native runner should still emit `gpu.telemetry.reported` to make the capability boundary explicit

## Lab 4: Rust and TypeScript reference parity

Command:

```bash
./scripts/run_lab.sh lab4-rust-ts-reference-parity
```

Goal:
- show that the portable analysis logic can be mirrored in a TypeScript reference implementation while Rust remains the real native/WASI portability target

What to pay attention to:
- the Rust `image.analyzed` payload should match the TypeScript reference payload for both `sample.pgm` and `bright.pgm`
- the parity check is about shared analysis behavior, not about replacing the native/WASI portability story

## Reflection

You got value from these labs if you can explain:

- why the shared payload is the portability proof
- why GPU telemetry is not part of the portable path
- how the contract keeps both targets aligned even when one target exposes extra capabilities
