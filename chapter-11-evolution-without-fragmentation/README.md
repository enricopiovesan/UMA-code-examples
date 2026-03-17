# Chapter 11: Evolution Without Fragmentation

This example turns Chapter 11 of the UMA book into a reader-facing hands-on lab.
The focus is not on inventing more architecture. The focus is on showing how systems drift, duplicate, fragment, and then recover when evolution is governed explicitly.

Chapter 11 is about:

- architecture continuing after deployment instead of ending at release
- behavioral drift accumulating through locally valid changes
- duplication and parallel versions emerging when drift is not controlled
- contracts anchoring behavior as systems evolve
- runtime governance resolving coexistence instead of leaving it to coordination
- hybrid adoption in brownfield systems without requiring a rewrite

## Folder layout

```text
chapter-11-evolution-without-fragmentation/
  README.md
  scenarios/
    lab1-contract-anchor/
    lab2-behavioral-drift/
    lab3-duplicate-implementations/
    lab4-version-sprawl/
    lab5-runtime-governed-coexistence/
    lab6-hybrid-adoption/
  rust/
    Cargo.toml
    src/
  ts/
    package.json
    src/
  scripts/
    list_labs.sh
    run_evolution_demo.sh
    validate_evolution.sh
    diff_evolution.sh
    compare_impls.sh
    run_evolution_demo_ts.sh
    smoke_evolution_labs.sh
```

## Prerequisites

- Rust 1.76 or newer
- Node.js 20 or newer
- a checkout of this repository

No external services are required for the validated path.

## Validation status

- Validated path: `./scripts/smoke_evolution_labs.sh`
- Main implementation: Rust CLI under `rust/`
- Secondary implementation: TypeScript reference implementation under `ts/`
- Implementation parity check: `./scripts/compare_impls.sh <lab>`

## Quick start

```bash
cd chapter-11-evolution-without-fragmentation
./scripts/list_labs.sh
./scripts/run_evolution_demo.sh lab1-contract-anchor
./scripts/diff_evolution.sh lab1-contract-anchor lab2-behavioral-drift
./scripts/compare_impls.sh lab1-contract-anchor
./scripts/smoke_evolution_labs.sh
```

If you want to inspect the TypeScript implementation directly:

```bash
./scripts/run_evolution_demo_ts.sh lab1-contract-anchor
```

The TypeScript renderer prints the same reader-facing sections as Rust so you can compare implementations without losing explanatory detail.

## Reader path

Use this order if you are following Chapter 11 as a first-time reader:

1. `./scripts/list_labs.sh`
2. `./scripts/validate_evolution.sh lab1-contract-anchor`
3. `./scripts/run_evolution_demo.sh lab1-contract-anchor`
4. `./scripts/diff_evolution.sh lab1-contract-anchor lab2-behavioral-drift`
5. `./scripts/run_evolution_demo.sh lab2-behavioral-drift`
6. `./scripts/diff_evolution.sh lab2-behavioral-drift lab3-duplicate-implementations`
7. `./scripts/run_evolution_demo.sh lab3-duplicate-implementations`
8. `./scripts/diff_evolution.sh lab3-duplicate-implementations lab4-version-sprawl`
9. `./scripts/run_evolution_demo.sh lab4-version-sprawl`
10. `./scripts/diff_evolution.sh lab4-version-sprawl lab5-runtime-governed-coexistence`
11. `./scripts/run_evolution_demo.sh lab5-runtime-governed-coexistence`
12. `./scripts/diff_evolution.sh lab5-runtime-governed-coexistence lab6-hybrid-adoption`
13. `./scripts/run_evolution_demo.sh lab6-hybrid-adoption`

Expected satisfaction point:
- by the end of lab 6, you should be able to explain how a system can keep evolving without fragmentation, and why that depends on contracts plus runtime enforcement rather than on a one-time redesign

## Questions a reader might ask

### "What am I supposed to learn from this?"

You should leave this lab able to explain:

- how locally valid changes accumulate into drift, duplication, and fragmentation
- why versioning is necessary but only safe when coexistence is governed explicitly
- how runtime resolution restores coherence without requiring a full rewrite

### "Why both Rust and TypeScript?"

Rust is the validated main path because it is the primary implementation standard for these examples.
TypeScript is included in parallel so the same evolution model can be inspected from another language stack without changing the scenario semantics.

### "What should I pay attention to in the output?"

The key signals are:

- `Verdict`
- the six evolution axes
- warnings such as `behavioral_drift`, `duplicate_behavior`, `version_fragmentation`, and `manual_governance_limit`
- the `Runtime Decisions` section, which shows how the system is being resolved under the current conditions
- the `Reader Value` section, which states the intended takeaway for that scenario

### "Why does the lab say `governed` instead of just `coherent`?"

`coherent` is the clean baseline where one contract line already explains the system.
`governed` means the system is still evolving, possibly with multiple versions or legacy internals, but runtime rules are now keeping that evolution aligned instead of letting it fragment.

### "Why use the diff before reading the next lab output?"

The diff is the shortest way to answer the Chapter 11 question, "what changed in the system's evolution model?"
Use `./scripts/diff_evolution.sh <from> <to>` first, then run the next lab report to see how that change affects runtime behavior in detail.

## Hands-on flow

### Lab 11.1: Contract anchor baseline

This is the reference shape:
- one explicit contract lineage
- stable event meaning
- runtime enforcement already active
- consumers sharing one interpretation of behavior

Expected value:
- the verdict is `coherent`
- there are no warnings

### Lab 11.2: Behavioral drift

This lab stretches one contract across different consumer expectations.

Expected value:
- the verdict becomes `at-risk`
- warnings include `behavioral_drift`
- the reader sees that structural compatibility is not enough to preserve one meaning

### Lab 11.3: Duplicate implementations

This lab responds to drift by duplicating the capability across environments.

Expected value:
- warnings include `duplicate_behavior` and `semantic_instability`
- the reader sees how duplication creates parallel behavior that drifts further apart

### Lab 11.4: Version sprawl

This lab introduces multiple versions without clear compatibility governance.

Expected value:
- the verdict becomes `fragmented`
- warnings include `version_fragmentation`
- the reader sees that versioning without explicit coexistence rules becomes another source of drift

### Lab 11.5: Runtime-governed coexistence

This lab keeps multiple versions, but lets the runtime resolve which one is valid for which consumer.

Expected value:
- the verdict becomes `governed`
- fragmentation warnings disappear
- the reader sees that runtime resolution can preserve continuity without forcing synchronized migration

### Lab 11.6: Hybrid adoption without a rewrite

This lab keeps a legacy core but governs the boundaries around it.

Expected value:
- the verdict remains `governed`
- the reader sees that Chapter 11 is not arguing for a rewrite, but for changing how evolution is controlled

## Troubleshooting

- If `cargo` reports dependency resolution failures, run the commands with network access at least once so Cargo can fetch the locked dependencies.
- If `npm test` fails in `ts/`, ensure you are using Node.js 20 or newer.
- If you mistype a lab id, rerun `./scripts/list_labs.sh`.
- If you forget a script’s arguments, use `--help` with `run_evolution_demo.sh`, `diff_evolution.sh`, or `compare_impls.sh`.

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can recognize when evolution is still coherent versus when it has crossed into fragmentation
- you can point to the exact contract, versioning, or runtime-governance choice that changed the outcome
- you can explain why brownfield improvement is possible without a rewrite when boundaries become explicit and enforced
