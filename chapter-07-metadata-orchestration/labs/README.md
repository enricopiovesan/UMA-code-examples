# Chapter 7 Labs

These labs turn the Chapter 7 example into a guided reader path instead of a set of manual edits.

## 1. Baseline cloud orchestration

```bash
./scripts/run_lab.sh lab1-baseline-cloud-flow
```

Expected value:
- contracts bind `image.tagger` to its subscribers automatically
- the Rust runner validates the event schema and writes CloudEvents envelopes
- the reader can see the policy warning, event validation, telemetry, cache, and evaluator flow in one run

## 2. Rust and TypeScript parity

```bash
./scripts/run_lab.sh lab2-rust-ts-parity
```

Expected value:
- the Rust runner remains the validated default
- the TypeScript reference runner produces the same orchestration summary
- the reader sees that contracts and event bindings, not language choice, define the orchestration behavior

## 3. Fail-closed policy enforcement

```bash
./scripts/run_lab.sh lab3-policy-fail-closed
```

Expected value:
- the same deny rule that only warns in the default quick-start now stops execution
- the reader sees the difference between fail-open observability and fail-closed enforcement

## 4. Telemetry audit

```bash
./scripts/run_lab.sh lab4-telemetry-audit
```

Expected value:
- the validator audits latency after a real orchestration run
- the reader sees how Chapter 7 connects contract validation, orchestration, and operational drift checks
