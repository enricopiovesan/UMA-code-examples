# UMA Metadata Orchestration Example

This example demonstrates Chapter 7 of the UMA Book, Contracts, Events, and Orchestration.
It is contract first, runnable on a typical developer machine, and structured as a reader-facing hands-on lab.
The validated default path is the Rust cloud runner. The repository also includes a TypeScript reference runner plus lightweight browser and edge harnesses so the same orchestration model can be surfaced in other environments.

## Key concepts

- Contract driven orchestration where emits and subscribes create bindings without workflow code
- CloudEvents with UMA extensions for observability and proof of behavior
- Policy aware execution with fail-open and fail-closed runtime modes
- Deterministic orchestration where the same input yields the same output and event evidence

## Prerequisites

- Rust 1.76 or newer
- Wasmtime 20 or newer, https://github.com/bytecodealliance/wasmtime
- Node.js 20 or newer
- jq and yq are optional

## Validation status

- Validated path: `./scripts/smoke_orchestration_labs.sh`
- Main implementation: Rust cloud runner in `runtime-rust/`
- Secondary implementation: TypeScript reference runner in `runtime/`
- Implementation parity check: `./scripts/compare_impls.sh`

## Quick start

```bash
./scripts/list_labs.sh
./scripts/run_lab.sh lab1-baseline-cloud-flow
./scripts/compare_impls.sh
./scripts/smoke_orchestration_labs.sh
```

`run_lab.sh lab1-baseline-cloud-flow` compiles the WASI services and the Rust cloud runner, then executes the full Chapter 7 orchestration flow.
`compare_impls.sh` verifies that the Rust and TypeScript runners report the same orchestration summary for the baseline flow.

Expected log excerpt:

```text
[info] policy.digest <sha256>
[warn] policy.violation policy.deny forbid_evaluator_in_browser continuing due to fail-open
[info] binding.created image.analyzed.v1 → telemetry.logger
[info] validation.passed event_schema=image.analyzed.v1
[info] telemetry.ok {"source":"telemetry.logger","event":"image.analyzed.v1","status":"passed"}
```

## Reader path

Use this order if you are following Chapter 7 as a first-time reader:

1. `./scripts/list_labs.sh`
2. `./scripts/run_lab.sh lab1-baseline-cloud-flow`
3. `./scripts/run_lab.sh lab2-rust-ts-parity`
4. `./scripts/run_lab.sh lab3-policy-fail-closed`
5. `./scripts/run_lab.sh lab4-telemetry-audit`

Expected satisfaction point:
- by the end of lab 4, you should be able to explain how contracts create bindings, how policy changes execution mode, and how telemetry proves orchestration behavior

## Questions a reader might ask

### "What am I supposed to learn from this?"

You should leave this lab able to explain:

- how event contracts create orchestration bindings without embedding workflow code
- why policy evaluation belongs in the orchestration path rather than outside it
- how telemetry and event envelopes provide evidence that the runtime behaved as expected

### "Why both Rust and TypeScript?"

Rust is the validated default path because it is the primary implementation standard for these examples.
TypeScript is included in parity so the same orchestration model can be inspected through a second stack without changing the event and contract semantics.

### "What should I pay attention to in the output?"

The most important lines are:

- `binding.created`
- `policy.violation`
- `validation.passed`
- `telemetry.ok`
- `cache.ok`
- `evaluator.ok`

Those lines together show discovery, policy handling, schema validation, subscriber dispatch, and the final evaluation result.

### "How do I know if the lab gave me value?"

You got value from the Chapter 7 lab if you can explain all three of these points after running it:

- the runtime created bindings from contracts rather than from hardcoded workflow steps
- changing policy fail mode changed runtime behavior without changing the services
- telemetry and event logs provided proof of what actually happened during orchestration

## Layout

- `contracts/`, all service contracts and JSON Schemas
- `services/`, source code for the orchestrated services
- `runtime-rust/`, validated Rust orchestration runner
- `runtime/`, secondary TypeScript reference runner and testable helper library
- `scripts/`, build, run, parity, and validation helpers
- `labs/`, guided reader exercises
- `docs/`, extra references

## Notes

- The validated runner executes the Rust WASI module with wasmtime over stdin and stdout, then validates the produced event using the declared schema, then dispatches it to the telemetry and cache subscribers.
- The TypeScript runner is kept in parity as a secondary implementation, not the primary quick-start.
- Policy evaluation remains part of the orchestration path, not a separate manual review step.

## Progressive learning path

- `step1-interface-only`, naive direct calls without contracts
- `step2-contracts-added`, contracts exist and are validated, wiring is manual
- `step3-orchestrated-runtime`, runtime discovers contracts and binds automatically

## Tools

- `tools/validator.js`, layered validation and drift audit
- `tools/graph.js`, generates `docs/diagrams/orchestration_graph.mmd`

## Logs

- `logs/telemetry.jsonl`, metrics
- `logs/events/*.json`, CloudEvents with UMA extensions

## Reports and tests

- `scripts/report.sh`, aggregates latency and counts
- `tests/test_orchestration.sh`, verifies deterministic event data
- `npm test`, validates the TypeScript orchestration helper behavior

## Third consumer

- `services/ai.model.evaluator` with contract `ai.model.evaluator.contract.yaml` emits `inference.completed.v1`

## Browser and Edge

- Open `browser/index.html` in a local server to see the browser simulation harness output. It mirrors the orchestration log shape, but it does not execute the Rust WASI services directly in the browser.
- Run the Deno edge harness if you have Deno installed:

```bash
deno run --allow-run edge/edge_runner.ts
```

The browser and edge harnesses remain optional demonstrations.
The validated Chapter 7 path is `./scripts/smoke_orchestration_labs.sh`, which exercises the Rust default runner, the TypeScript parity runner, fail-closed policy handling, and the telemetry audit path.

## Troubleshooting

- If `./scripts/run_cloud.sh` says `Wasmtime is required`, install Wasmtime or place a matching release under the repo `.bin/` directory.
- If `./scripts/run_cloud_ts.sh` says built TypeScript service artifacts are missing, rerun `BUILD_OPTIONAL_JS=1 ./scripts/build_all.sh`.
- If `./scripts/run_cloud.sh` says built WASI artifacts are missing, rerun `./scripts/build_all.sh`.
- If `npm test` fails, ensure you are using Node.js 20 or newer.
- If you want the TypeScript parity runner or the browser and edge helper artifacts, run `BUILD_OPTIONAL_JS=1 ./scripts/build_all.sh`.

## Policy rule, fail closed

There is a real deny rule in `contracts/policies/org.telemetry.standard.json`.
Set `POLICY_FAIL_MODE=closed` to stop on violation, or any other value to continue.

## OpenTelemetry export

Set `OTLP_ENDPOINT` to an HTTP collector to receive metrics. The runner exports latency as `uma.qos.latency.ms`.

## Labs

See [labs/README.md](/Users/piovese/Documents/UMA-code-examples/uma-metadata-orchestration/labs/README.md) for the guided Chapter 7 reader labs.

## Reflection checklist

- Did we deliver what the reader wants, a working, understandable, and verifiable example?
- Could we do better within this context, we keep infrastructure minimal and add only the features that reinforce Chapter 7?

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can point to the exact contract-driven bindings that created the orchestration flow
- you can explain why fail-open and fail-closed policy modes produce different operational outcomes
- you can show how telemetry and event envelopes make the orchestration behavior auditable
