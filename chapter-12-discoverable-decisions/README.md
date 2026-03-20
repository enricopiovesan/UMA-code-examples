# Chapter 12: Discoverable Decisions

This example turns Chapter 12 of the UMA book into a reader-facing hands-on lab.
The focus is not on adding another protocol demo. The focus is on showing how a UMA system becomes queryable, navigable, and auditable when decisions are exposed as artifacts instead of being buried inside execution.

Chapter 12 is about:

- moving from executable systems to understandable systems
- exposing capability metadata and model context without losing governance
- separating proposal, validation, revision, and execution
- using partial projections for local reasoning without pretending they are authoritative
- producing trace artifacts that explain why the final outcome happened
- treating planner, validator, and executor as functions rather than fixed components

## Learning path position

- Previous: [Chapter 11: Evolution Without Fragmentation](../chapter-11-evolution-without-fragmentation/README.md)
- Next: [Chapter 13: Portable MCP Runtime](../chapter-13-portable-mcp-runtime/README.md)

## Key concepts

- discoverability is not the same thing as execution
- edge planning can be useful even when authority remains in the cloud
- proposals must declare their assumptions and unresolved constraints explicitly
- validation should explain constraint failures in a way the planner can use
- execution should resolve approved intent, not silently replan it
- trace artifacts are what make architectural decisions inspectable after the fact

## Layout

```text
chapter-12-discoverable-decisions/
  README.md
  scenarios/
    lab1-capability-projection/
    lab2-edge-proposal/
    lab3-authority-feedback/
    lab4-single-revision/
    lab5-approved-execution/
    lab6-queryable-trace/
  rust/
    Cargo.toml
    src/
  ts/
    package.json
    src/
  scripts/
    list_labs.sh
    run_decision_demo.sh
    validate_decisions.sh
    diff_decisions.sh
    compare_impls.sh
    run_decision_demo_ts.sh
    smoke_discoverability_labs.sh
```

## Prerequisites

- Rust 1.76 or newer
- Node.js 20 or newer
- a checkout of this repository

No external services are required for the validated path.

## Validation status

- Validated path: `./scripts/smoke_discoverability_labs.sh`
- Main implementation: Rust CLI under `rust/`
- Secondary implementation: TypeScript reference implementation under `ts/`
- Implementation parity check: `./scripts/compare_impls.sh <lab>`

## Quick start

```bash
cd chapter-12-discoverable-decisions
./scripts/list_labs.sh
./scripts/run_decision_demo.sh lab1-capability-projection
./scripts/diff_decisions.sh lab1-capability-projection lab2-edge-proposal
./scripts/compare_impls.sh lab1-capability-projection
./scripts/smoke_discoverability_labs.sh
```

The Chapter 12 scripts resolve paths from their own location, so the same commands also work from the repo root with the `chapter-12-discoverable-decisions/` prefix.

If you want to inspect the TypeScript implementation directly:

```bash
./scripts/run_decision_demo_ts.sh lab1-capability-projection
```

The TypeScript renderer prints the same reader-facing sections as Rust so you can compare implementations without losing explanatory detail.

## Reader path

Use this order if you are following Chapter 12 as a first-time reader:

1. `./scripts/list_labs.sh`
2. `./scripts/validate_decisions.sh lab1-capability-projection`
3. `./scripts/run_decision_demo.sh lab1-capability-projection`
4. `./scripts/diff_decisions.sh lab1-capability-projection lab2-edge-proposal`
5. `./scripts/run_decision_demo.sh lab2-edge-proposal`
6. `./scripts/diff_decisions.sh lab2-edge-proposal lab3-authority-feedback`
7. `./scripts/run_decision_demo.sh lab3-authority-feedback`
8. `./scripts/diff_decisions.sh lab3-authority-feedback lab4-single-revision`
9. `./scripts/run_decision_demo.sh lab4-single-revision`
10. `./scripts/diff_decisions.sh lab4-single-revision lab5-approved-execution`
11. `./scripts/run_decision_demo.sh lab5-approved-execution`
12. `./scripts/diff_decisions.sh lab5-approved-execution lab6-queryable-trace`
13. `./scripts/run_decision_demo.sh lab6-queryable-trace`

Expected satisfaction point:
- by the end of lab 6, you should be able to explain how UMA turns a runtime decision into a discoverable artifact trail instead of a hidden execution side effect

## Questions a reader might ask

### "What am I supposed to learn from this?"

You should leave this lab able to explain:

- why a partial edge projection is useful without being authoritative
- how proposals, validation guidance, and revision cycles make change visible
- why a traceable execution summary matters more than a raw success/failure result

### "Why both Rust and TypeScript?"

Rust is the validated main path because it is the primary implementation standard for these examples.
TypeScript is included in parallel so the same decision model can be inspected from another language stack without changing the scenario semantics.

### "Why is lab 2 already `discoverable` if the trace is still missing?"

Because Chapter 12 treats discoverability as a ladder, not a binary switch.
The system becomes `discoverable` once the proposal and authority boundary are explicit and queryable enough that a reader can inspect intent before execution.
It only becomes `governed` when the trace chain is complete and the approved decision can be replayed end to end without hidden steps.

### "What should I pay attention to in the output?"

The key signals are:

- `Verdict`
- the six decision axes
- warnings such as `proposal_hidden`, `authority_gap`, and `partial_trace`
- the `Discoverable Surfaces` section, which shows what edge and cloud can each expose
- the `Trace` section, which shows whether the system produced first-class evidence or just a result

### "Why use the diff before reading the next lab output?"

The diff is the shortest way to answer the Chapter 12 question, "what part of the decision lifecycle became more visible?"
Use `./scripts/diff_decisions.sh <from> <to>` first, then run the next lab report to see how that changed the discoverability model in detail.

### "How do I just get the raw lab ids for scripting?"

Use:

```bash
./scripts/list_labs.sh --ids-only
```

The default `list_labs.sh` output is reader-oriented and includes a title and summary for each lab.

### "Is this supposed to be a full MCP server?"

No.
This lab uses MCP-style discoverability as an architectural shape, not as a dependency on live protocol infrastructure.
The point is to make capability metadata, proposals, constraints, and traces queryable and machine-usable in a controlled way.

## Hands-on flow

### Lab 12.1: Capability projection

This lab starts with execution that works but is not yet discoverable.

Expected value:
- the verdict is `opaque`
- warnings show that the system still hides proposal and authority decisions
- the reader sees the difference between "the runtime knows" and "the system can explain"

### Lab 12.2: Edge proposal

This lab introduces an explicit proposal generated from the edge projection.

Expected value:
- the verdict becomes `discoverable`
- the reader sees assumptions and unresolved constraints stated directly instead of being implied

### Lab 12.3: Authority feedback

This lab sends the proposal to authoritative validation and returns structured guidance.

Expected value:
- violations are exposed explicitly
- the reader sees that validation is not just a gate, but a source of actionable reasoning

### Lab 12.4: Single revision

This lab performs one controlled revision cycle instead of endless negotiation.

Expected value:
- the proposal becomes acceptable without collapsing planning and authority into one step
- the reader sees why bounded negotiation matters for architectural clarity

### Lab 12.5: Approved execution

This lab resolves the approved proposal into concrete placement and capability selection.

Expected value:
- execution happens under authority instead of local improvisation
- the warning `partial_trace` remains, showing that execution alone still does not make the system fully inspectable

### Lab 12.6: Queryable trace

This lab adds the full trace and query surface for the final decision path.

Expected value:
- the verdict becomes `governed`
- warnings disappear
- the reader sees the complete path from edge hypothesis to authoritative outcome

## Troubleshooting

- If `cargo` reports dependency resolution failures, run the commands with network access at least once so Cargo can fetch the locked dependencies.
- If `npm test` fails in `ts/`, ensure you are using Node.js 20 or newer.
- If you mistype a lab id, rerun `./scripts/list_labs.sh`.
- If you forget a script’s arguments, use `--help` with `run_decision_demo.sh`, `diff_decisions.sh`, or `compare_impls.sh`.

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can distinguish a discoverable system from one that merely executes correctly
- you can point to the exact proposal, validation, revision, and trace artifacts that shaped the outcome
- you can explain why authority and discoverability strengthen each other instead of competing
