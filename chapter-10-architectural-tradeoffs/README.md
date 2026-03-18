# Chapter 10: Architectural Tradeoffs and Runtime Coherence

This example turns Chapter 10 of the UMA book into a reader-facing hands-on lab.
The focus is not on building one more service. The focus is on showing how architectural decisions change runtime behavior, clarity, and system coherence.

Chapter 10 is about:

- architecture as runtime behavior rather than only static structure
- metadata as a control plane
- capability boundaries and service granularity
- event semantics and hidden coupling
- runtime placement and deterministic selection
- avoiding over-orchestration while keeping systems governable

## Key concepts

- architecture quality should be visible in runtime behavior, not only in diagrams
- metadata works as a control plane for service selection and coordination
- more services or more orchestration is not automatically better architecture
- recovery comes from clearer constraints, not more complexity

## Layout

```text
chapter-10-architectural-tradeoffs/
  README.md
  scenarios/
    lab1-baseline/
    lab2-over-granular/
    lab3-hidden-event-coupling/
    lab4-runtime-ambiguity/
    lab5-over-orchestrated/
    lab6-recovered-architecture/
  rust/
    Cargo.toml
    src/
  ts/
    package.json
    src/
  scripts/
    list_labs.sh
    run_arch_demo.sh
    validate_architecture.sh
    diff_architecture.sh
    compare_impls.sh
    run_arch_demo_ts.sh
    smoke_arch_labs.sh
```

## Prerequisites

- Rust 1.76 or newer
- Node.js 20 or newer
- a checkout of this repository

No external services are required for the validated path.

## Validation status

- Validated path: `./scripts/smoke_arch_labs.sh`
- Main implementation: Rust CLI under `rust/`
- Secondary implementation: TypeScript reference implementation under `ts/`
- Implementation parity check: `./scripts/compare_impls.sh <lab>`

## Quick start

```bash
cd chapter-10-architectural-tradeoffs
./scripts/list_labs.sh
./scripts/run_arch_demo.sh lab1-baseline
./scripts/diff_architecture.sh lab1-baseline lab2-over-granular
./scripts/compare_impls.sh lab1-baseline
./scripts/smoke_arch_labs.sh
```

If you want to inspect the TypeScript implementation directly:

```bash
./scripts/run_arch_demo_ts.sh lab1-baseline
```

The TypeScript renderer prints the same reader-facing sections as Rust so you can compare implementations without losing explanatory detail.

## Reader path

Use this order if you are following Chapter 10 as a first-time reader:

1. `./scripts/list_labs.sh`
2. `./scripts/validate_architecture.sh lab1-baseline`
3. `./scripts/run_arch_demo.sh lab1-baseline`
4. `./scripts/diff_architecture.sh lab1-baseline lab2-over-granular`
5. `./scripts/run_arch_demo.sh lab2-over-granular`
6. `./scripts/diff_architecture.sh lab2-over-granular lab3-hidden-event-coupling`
7. `./scripts/run_arch_demo.sh lab3-hidden-event-coupling`
8. `./scripts/diff_architecture.sh lab3-hidden-event-coupling lab4-runtime-ambiguity`
9. `./scripts/run_arch_demo.sh lab4-runtime-ambiguity`
10. `./scripts/diff_architecture.sh lab4-runtime-ambiguity lab5-over-orchestrated`
11. `./scripts/run_arch_demo.sh lab5-over-orchestrated`
12. `./scripts/diff_architecture.sh lab5-over-orchestrated lab6-recovered-architecture`
13. `./scripts/run_arch_demo.sh lab6-recovered-architecture`

Expected satisfaction point:
- by the end of lab 6, you should be able to explain not just which scenario is better, but which metadata and runtime choices made it better

## Questions a reader might ask

### "What am I supposed to learn from this?"

You should leave this lab able to explain:

- why architecture quality in UMA depends on metadata and runtime rules, not only on service count
- how over-granularity, vague events, and ambiguous placement degrade behavior without obvious code failures
- why recovery comes from clearer constraints, not from adding more orchestration

### "Why both Rust and TypeScript?"

Rust is the validated main path because it is the primary implementation standard for these examples.
TypeScript is included in parallel so the same architectural model can be inspected from another language stack without changing the scenario semantics.

### "What should I pay attention to in the output?"

The key signals are:

- `Verdict`
- the six architectural decision axes
- warnings such as `over_granular`, `hidden_event_coupling`, `runtime_ambiguity`, and `over_orchestrated`
- the diff output showing which decision axes changed between labs
- the `Reader Value` section, which states the intended architectural takeaway for that scenario

### "Why use the diff before reading the next lab output?"

The diff is the shortest way to answer the Chapter 10 question, "what architectural decision changed here?"
Use `./scripts/diff_architecture.sh <from> <to>` first, then run the next lab report to see how that decision affects runtime behavior in detail.

## Hands-on flow

### Lab 10.1: Coherent baseline

This is the reference shape:
- one meaningful capability per service
- stable domain events
- concise metadata
- distributed event progression

Expected value:
- the verdict is `coherent`
- there are no architectural warnings

### Lab 10.2: Over granular decomposition

This lab splits a coherent flow into too many narrow capability steps.

Expected value:
- the system still works conceptually
- the warning `over_granular` appears
- the reader sees that more services is not automatically better architecture

### Lab 10.3: Hidden event coupling

This lab replaces stable domain facts with a vague workflow event.

Expected value:
- the warning `hidden_event_coupling` appears
- multiple consumers depend on implied semantics rather than explicit facts

### Lab 10.4: Runtime ambiguity

This lab introduces overlapping analyzer capabilities without deterministic selection metadata.

Expected value:
- the verdict becomes `ambiguous`
- the warning `runtime_ambiguity` appears

### Lab 10.5: Over orchestrated workflow

This lab adds a central coordinator that dictates too much of the flow.

Expected value:
- the verdict becomes `fragile`
- warnings include `over_orchestrated`, `metadata_bloat`, and `state_drift`
- metadata quality also degrades because orchestration detail leaks into configuration

### Lab 10.6: Recovered architecture

This lab restores stable events, focused capability boundaries, and deterministic selection.

Expected value:
- the warnings disappear
- the verdict returns to `coherent`

## Troubleshooting

- If `cargo` reports dependency resolution failures, run the commands with network access at least once so Cargo can fetch the locked dependencies.
- If `npm test` fails in `ts/`, ensure you are using Node.js 20 or newer.
- If you mistype a lab id, rerun `./scripts/list_labs.sh`.
- If you forget a script’s arguments, use `--help` with `run_arch_demo.sh`, `diff_architecture.sh`, or `compare_impls.sh`.

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can identify when a UMA system is coherent versus merely functional
- you can point to the exact metadata or runtime choice that caused a warning
- you can explain why the recovery path improves architecture without simply adding more services or more orchestration
