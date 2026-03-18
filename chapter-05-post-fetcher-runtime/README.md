# Chapter 5: Post Fetcher Runtime Lab

This example demonstrates Chapter 5 of the UMA book: the runtime layer around a pure service.
It shows how contracts, adapter binding, deterministic event ordering, and lifecycle metadata work together around a small HTTP-fetching service.
The validated reader path is the native Rust cloud host smoke flow, with a TypeScript reference runtime kept in parity for the core lab scenarios.

## Key concepts

- pure service logic stays separate from host capabilities
- the runtime selects and records the `network.fetch` adapter binding
- validation can stop execution before side effects happen
- event ordering and lifecycle metadata make runtime behavior auditable

## Prerequisites

- Rust 1.77 or newer
- `cargo`
- `python3` for the local fixture server used by the cloud host path
- `jq` for the guided lab checks and golden-fixture comparisons
- Optional: `npm` if you want to inspect the illustrative browser host scaffolding

## Validation status

- Validated path: `./scripts/smoke_runtime_labs.sh`
- Main implementation: Rust workspace rooted at `Cargo.toml`
- Secondary implementation: TypeScript reference runtime in `ts/`
- Guided reader labs: `./scripts/list_labs.sh` and `./scripts/run_lab.sh`
- Implementation parity check: `./scripts/compare_impls.sh`
- Optional paths: browser and edge host sketches remain illustrative, not validated quick-starts

## Quick start

```bash
./scripts/list_labs.sh
./scripts/run_lab.sh lab1-cloud-golden-path
./scripts/run_lab.sh lab2-header-validation-fail-fast
./scripts/run_lab.sh lab3-adapter-binding-and-wrappers
./scripts/run_lab.sh lab4-rust-ts-parity
./scripts/smoke_runtime_labs.sh
```

Expected log signals:

```text
Integration test passed: output matches golden fixture.
Building native runtime CLI...
Running the service in cloud host via native CLI...
Chapter 5 smoke run completed successfully.
```

## Reader path

Use this order if you just finished Chapter 5 and want the best learning path:

1. `./scripts/list_labs.sh`
2. `./scripts/run_lab.sh lab1-cloud-golden-path`
3. `./scripts/run_lab.sh lab2-header-validation-fail-fast`
4. `./scripts/run_lab.sh lab3-adapter-binding-and-wrappers`
5. `./scripts/run_lab.sh lab4-rust-ts-parity`

Expected satisfaction point:
- by the end of lab 3, you should be able to explain how the runtime validates input, chooses an adapter implementation, and records that decision without changing the pure service logic

## Questions a reader might ask

### "What am I supposed to learn from this?"

You should leave this lab able to explain:

- what belongs in the service crate versus what belongs in the runtime layer
- why runtime validation should fail fast before side effects happen
- how lifecycle metadata proves which adapter path actually ran
- how a TypeScript reference runtime can model the same Chapter 5 behavior while Rust remains the validated default path

### "What should I pay attention to in the output?"

The most important signals are:

- the deterministic event sequence in `output.events`
- the `network.fetch` binding recorded in `lifecycle.bindings`
- the final `lifecycle.state`

### "How do I know if the lab gave me value?"

You got value from the Chapter 5 lab if you can explain all three of these points after running it:

- the service logic normalized a post, but the runtime owned validation, fetch orchestration, and lifecycle recording
- invalid headers stopped the run before any `fetch_request` event happened
- enabling retry/cache wrappers changed the binding record without changing the normalized service output

## Layout

- `contracts/`, JSON contracts for the service, runtime policy, adapter capability, and metadata schema
- `service/`, pure normalization logic and service-facing API types
- `runtime/`, runtime orchestration, adapter binding, event bus, lifecycle record, and native CLI entrypoint
- `ts/`, TypeScript reference runtime kept in parity with the core Chapter 5 scenarios
- `adapters/`, capability adapter definitions plus illustrative TS/browser scaffolding
- `hosts/`, cloud and edge host shims
- `tests/`, fixtures and integration scripts
- `scripts/`, guided Chapter 5 lab helpers

## Service contract

The service accepts input like this:

```json
{
  "request": {
    "url": "http://127.0.0.1:18080/posts/1",
    "headers": {
      "accept": "application/json"
    }
  },
  "runId": "demo-001"
}
```

On success it emits a normalized post plus a deterministic event log:

```json
{
  "normalizedPost": {
    "id": 1,
    "userId": 1,
    "title": "<string>",
    "body": "<string>"
  },
  "events": [
    { "t": "0", "type": "start", "data": { "runId": "demo-001" } },
    { "t": "1", "type": "fetch_request", "data": { "url": "<string>" } },
    { "t": "2", "type": "fetch_response", "data": { "status": 200 } },
    { "t": "3", "type": "normalized", "data": { "id": 1 } },
    { "t": "4", "type": "end", "data": {} }
  ]
}
```

## Reader labs

See [labs/README.md](labs/README.md) for the guided Chapter 5 lab notes.

### `lab1-cloud-golden-path`

Runs the validated cloud host path and compares the output against the checked-in golden fixture.

### `lab2-header-validation-fail-fast`

Feeds an invalid header into the native CLI path and proves that validation stops the run before any fetch happens.

### `lab3-adapter-binding-and-wrappers`

Enables the retry and cache wrappers and verifies that the runtime binding record changes to `cache-retry-host-fetch`.

### `lab4-rust-ts-parity`

Runs the Rust and TypeScript implementations against the validated Chapter 5 scenarios and compares their summarized runtime behavior.

## Manual commands

If you want the lower-level commands instead of the guided labs:

```bash
cargo test --locked --workspace
bash hosts/cloud/run.sh
bash tests/integration/run_cloud.sh
```

## Contracts and runtime behavior

### Adapter capability contract

The runtime depends on one capability, `network.fetch`, described in [adapter.network.contract.json](contracts/adapter.network.contract.json).
The lifecycle record persists which implementation satisfied that capability.

### Runtime policy

[policy.runtime.json](contracts/policy.runtime.json) documents the intended adapter-selection and observability behavior for the sample.
The current runtime does not fully parse this policy file yet; in this example it acts as the declared runtime contract rather than a fully interpreted policy engine.

### Lifecycle metadata schema

[metadata.schema.json](contracts/metadata.schema.json) defines the persisted lifecycle record shape, including:

- service identity
- policy reference
- capability bindings
- event log
- final state
- logical clock

## Environment variables

The runtime supports a few environment variables for the lab:

| Variable | Description |
| --- | --- |
| `UMA_ENABLE_RETRY` | Wraps the selected adapter with `RetryAdapter` |
| `UMA_ENABLE_CACHE` | Wraps the selected adapter with `CacheAdapter` |
| `UMA_POLICY_PATH` | Not implemented yet; would point the runtime to a custom policy file |
| `UMA_DEMO_PORT` | Overrides the localhost fixture port used by `hosts/cloud/run.sh` |

## Browser and edge

The browser and edge files remain illustrative sketches.
They are useful as reference material for where a JS/Wasm binding layer would go, but they are not part of the validated quick-start path.

- [tests/integration/run_browser.md](tests/integration/run_browser.md) explains the browser scaffold
- [tests/integration/run_edge.sh](tests/integration/run_edge.sh) fails fast with guidance instead of pretending the edge path is turnkey

## Reports and tests

- `cargo test --locked --workspace` verifies the service and runtime crates
- `npm test --prefix ts` verifies the TypeScript reference runtime
- `./scripts/smoke_runtime_labs.sh` is the validated Chapter 5 smoke path
- `./scripts/compare_impls.sh` checks Rust/TypeScript parity for the core labs
- `bash tests/integration/run_cloud.sh` compares the cloud output against the golden fixture
- `runtime/src/tests.rs` covers runtime determinism, fail-fast validation, adapter wrapping, and parse-error handling

## Troubleshooting

- If `./scripts/run_lab.sh` or `hosts/cloud/run.sh` says `python3 is required`, install Python 3 for the local fixture server.
- If `jq` is missing, the guided labs and golden comparison scripts will fail early with an explicit message.
- If you want to explore the browser scaffold, install dependencies under `adapters/network/ts-host/`, but treat that path as illustrative rather than validated.
- If you see network differences on your machine, use the built-in localhost fixture path instead of external internet endpoints; the validated scripts already do this.

## Notes

- The validated Chapter 5 path is still Rust-first. The TypeScript runtime is a parity/reference implementation for the chapter concepts, while the browser and edge host files remain illustrative sketches around the same runtime model.
- The runtime is deliberately deterministic: no timers or random values influence event ordering.
- The logical clock increments once per emitted event so host behavior is easy to compare.

## Reflection checklist

- Did the labs make the runtime layer responsibilities more obvious than the raw code alone?
- Did the failure-path lab clearly show why validation belongs before adapter execution?
- Did the binding record make the capability indirection feel concrete instead of abstract?

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can point to the exact event sequence that proves the runtime behaved deterministically
- you can explain why invalid headers stop before fetch rather than after it
- you can show where the lifecycle record captures the actual adapter decision the runtime made
