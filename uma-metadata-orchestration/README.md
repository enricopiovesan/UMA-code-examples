# UMA Metadata Orchestration Example

This example demonstrates Chapter 7 of the UMA Book, Contracts, Events, and Orchestration.
It is production oriented, contract first, and runnable on a typical developer machine.
The validated quick-start path is the cloud runner. The repository also includes lightweight browser and edge harnesses that illustrate how the same orchestration model can be surfaced in those environments.

## Key concepts

- Contract driven orchestration, emits and subscribes create bindings without workflow code.
- CloudEvents with UMA extensions for observability and proof of behavior.
- Policy aware execution using signed manifests and strict failure mode.
- Deterministic behavior, the same input yields the same output, the runtime validates it.

## Prerequisites

- Rust 1.76 or newer
- Node.js 20 or newer
- Wasmtime 20 or newer, https://github.com/bytecodealliance/wasmtime
- jq and yq are optional

## Quick start

```bash
./scripts/build_all.sh
./scripts/run_cloud.sh
```

`build_all.sh` compiles the WASI services and installs the root and service-level Node.js dependencies required by the runner.

Expected log excerpt:

```
[info] policy.digest <sha256>
[warn] policy.violation policy.deny forbid_evaluator_in_browser continuing due to fail-open
[info] binding.created image.analyzed.v1 → telemetry.logger
[info] validation.passed event_schema=image.analyzed.v1
[info] telemetry.ok {"source":"telemetry.logger","event":"image.analyzed.v1","status":"passed"}
```

## Layout

- contracts, all service contracts and JSON Schemas
- services, source code for services
- runtime, orchestration and validation runner
- scripts, build, run, and validation helpers
- docs, extra references

## Notes

- The runner executes the Rust WASI module with wasmtime over stdin and stdout, then validates the produced event using the declared schema, then dispatches it to the telemetry subscriber.
- Policy adapter performs signature and digest checks as hooks, you can wire it to your own organization policy registry.


## Progressive learning path

- step1-interface-only, naive direct calls without contracts
- step2-contracts-added, contracts exist and are validated, wiring is manual
- step3-orchestrated-runtime, runtime discovers contracts and binds automatically

## Tools

- tools/validator.js, layered validation and drift audit
- tools/graph.js, generates docs/diagrams/orchestration_graph.mmd

## Logs

- logs/telemetry.jsonl, metrics
- logs/events/*.json, CloudEvents with UMA extensions

## Reports and tests

- scripts/report.sh, aggregates latency and counts
- tests/test_orchestration.sh, verifies deterministic event data

## Third consumer

- services/ai.model.evaluator with contract ai.model.evaluator.contract.yaml emits inference.completed.v1


## Browser and Edge

- Open `browser/index.html` in a local server to see the browser simulation harness output. It mirrors the orchestration log shape, but it does not execute the Rust WASI services directly in the browser.
- Run the Deno edge harness if you have Deno installed:
```bash
deno run --allow-run edge/edge_runner.ts
```

The edge harness shells out to `node runtime/runner.js` for parity; it is a thin demonstration wrapper rather than a separate validated execution path.

## Policy rule, fail closed

There is a real deny rule in contracts/policies/org.telemetry.standard.json. The runner enforces it.
Set POLICY_FAIL_MODE=closed to stop on violation, or any other value to continue.

## OpenTelemetry export

Set OTLP_ENDPOINT to an HTTP collector to receive metrics. The runner exports latency as uma.qos.latency.ms.

## Failure path labs

See labs/README.md for three guided exercises: schema mismatch, policy violation, and latency drift.

## Reflection checklist

- Did we deliver what the reader wants, a working, understandable, and verifiable example?
- Could we do better within this context, we keep infrastructure minimal, add only the features that reinforce Chapter 7.
