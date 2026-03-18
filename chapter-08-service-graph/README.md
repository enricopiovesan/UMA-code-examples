# Chapter 8: Service Graph Evolution with Git

This example turns Chapter 8 of the UMA book into a reader-facing hands-on sequence.
The focus is not generic Git usage. The focus is using Git-style checkpoints to make UMA system growth visible.

Chapter 8 explains how a UMA system emerges from:

- service contracts
- declared capabilities
- emitted events
- runtime discovery

The labs below let a reader inspect that evolution one commit at a time.

## Learning path position

- Previous: [Chapter 7: Metadata Orchestration Example](../chapter-07-metadata-orchestration/README.md)
- Next: [Chapter 9: Trust Boundaries and Runtime Enforcement](../chapter-09-trust-boundaries/README.md)

---

## Key concepts

- service graphs emerge from contract and event compatibility
- architecture grows by adding services, not rewriting upstream ones
- graph changes should be inspectable as metadata changes
- broken compatibility should fail visibly and recover predictably

---

## Layout

This folder follows the same reader-first structure used in earlier chapters:

```text
chapter-08-service-graph/
  Cargo.toml
  README.md
  contracts/
    schemas/
  src/
    lib.rs
    main.rs
  ts/
    package.json
    src/
  scenarios/
    lab1-upload-only/
    lab2-image-tagger/
    lab3-indexer/
    lab4-broken-compat/
    lab5-fixed-compat/
  scripts/
    list_labs.sh
    run_graph_demo.sh
    graph_snapshot.sh
    graph_diff.sh
    contract_diff.sh
    compare_impls.sh
    run_graph_demo_ts.sh
    validate_graph_contracts.sh
    smoke_graph_labs.sh
```

The validated reader path is the Rust CLI in `src/`.
The secondary implementation is the TypeScript parity path under `ts/`.

---

## Prerequisites

- Rust 1.76 or newer
- Node.js 20 or newer
- a checkout of this repository
- the `wasm32-wasip1` target is not required for this chapter

The Rust CLI is the validated default path.
Node.js is still required for the full smoke run and the TypeScript parity checks.
The TypeScript implementation is kept in parity as a secondary implementation for comparison.
The scenario contracts reference versioned schema files under `contracts/schemas/`.

## Validation status

- Validated path: `./scripts/smoke_graph_labs.sh`
- Main implementation: Rust CLI in `src/`
- Secondary implementation: TypeScript implementation in `ts/`
- Implementation parity check: `./scripts/compare_impls.sh <lab-name>`

---

## Quick start

If you land here as a reader, start by listing the labs:

```bash
cd chapter-08-service-graph
./scripts/list_labs.sh
```

Run a single lab state directly:

```bash
./scripts/run_graph_demo.sh lab1-upload-only
```

Inspect a machine-readable snapshot:

```bash
./scripts/graph_snapshot.sh lab3-indexer
```

Compare two graph states:

```bash
./scripts/graph_diff.sh lab3-indexer lab4-broken-compat
```

Verify Rust and TypeScript stay aligned for one lab:

```bash
./scripts/compare_impls.sh lab3-indexer
```

Inspect the contract-level change between two labs with Git's diff engine:

```bash
./scripts/contract_diff.sh lab1-upload-only lab2-image-tagger
```

Validate the scenario contracts:

```bash
./scripts/validate_graph_contracts.sh
```

Run the full Chapter 8 reader path:

```bash
./scripts/smoke_graph_labs.sh
```

If you want to inspect the TypeScript implementation directly:

```bash
./scripts/run_graph_demo_ts.sh lab1-upload-only
```

## Troubleshooting

- If `cargo` reports dependency resolution failures, run the commands with network access at least once so Cargo can fetch the locked dependencies.
- If `npm test` fails in `ts/`, ensure you are using Node.js 20 or newer.
- If you pass an unknown lab name, rerun `./scripts/list_labs.sh` and use one of the printed scenario ids.
- If you want the raw metadata-level change instead of the graph summary, use `./scripts/contract_diff.sh <from> <to>`.
- If you want to confirm Rust and TypeScript still agree for a scenario, use `./scripts/compare_impls.sh <lab>`.

---

## Reader path

If you are following the chapter as a fresh reader, use this order:

1. `./scripts/list_labs.sh`
2. `./scripts/validate_graph_contracts.sh lab1-upload-only`
3. `./scripts/run_graph_demo.sh lab1-upload-only`
4. `./scripts/contract_diff.sh lab1-upload-only lab2-image-tagger`
5. `./scripts/compare_impls.sh lab2-image-tagger`
6. `./scripts/run_graph_demo.sh lab2-image-tagger`
7. `./scripts/compare_impls.sh lab3-indexer`
8. `./scripts/run_graph_demo.sh lab3-indexer`
9. `./scripts/graph_diff.sh lab3-indexer lab4-broken-compat`
10. `./scripts/compare_impls.sh lab4-broken-compat`
11. `./scripts/run_graph_demo.sh lab4-broken-compat`
12. `./scripts/compare_impls.sh lab5-fixed-compat`
13. `./scripts/run_graph_demo.sh lab5-fixed-compat`

That flow mirrors the chapter idea:

- start with one event-producing service
- add compatible services
- observe graph growth
- break compatibility
- restore it and see the graph recover

---

## Questions a reader might ask

### "Where is the Git part if all lab states are in one folder?"

This repo ships the Chapter 8 labs as scenario directories so the reader can run everything from one checkout.
The Git-oriented inspection step is:

- `./scripts/contract_diff.sh <from> <to>` for contract changes
- `./scripts/graph_diff.sh <from> <to>` for graph changes

This keeps the chapter runnable while still letting the reader inspect architectural evolution with Git's diff model.

### "Which implementation should I treat as the main one?"

Use the Rust CLI through the `scripts/` entry points.
Those scripts call `cargo run --locked` and `cargo test --locked`, so the Chapter 8 quick-start path is Rust-first and reproducible on a clean machine.

TypeScript lives under `ts/` and is kept in parity through `./scripts/compare_impls.sh`.

### "What should I pay attention to in the output?"

The most important lines are:

- `capability: ...` to see what the service claims it can do
- `consumes: ...` and `emits: ...` to see event compatibility
- `Edges` to see the active service graph
- `Waiting Consumers` to see where compatibility has broken

### "How do I know if the hands-on gave me value?"

You got value from the Chapter 8 lab if you can explain all three of these points after running it:

- the upload service reports an event instead of orchestrating downstream work
- adding a compatible service changes the graph without editing upstream services
- a metadata mismatch removes graph edges and turns a downstream service into a waiting consumer

If you cannot explain those three outcomes from the command outputs, reread Labs 8.2 through 8.4 and compare:

- `./scripts/contract_diff.sh lab1-upload-only lab2-image-tagger`
- `./scripts/graph_diff.sh lab3-indexer lab4-broken-compat`
- `./scripts/compare_impls.sh lab3-indexer`

---

## Hands-on flow

### Lab 8.1: One Service, One Event

Start with only `upload-service`.
It accepts an image and emits `image.uploaded`.

Reader outcome:

- one service node exists
- one event is visible
- no downstream consumer is present

Suggested commands:

```bash
./scripts/validate_graph_contracts.sh lab1-upload-only
./scripts/run_graph_demo.sh lab1-upload-only
./scripts/graph_snapshot.sh lab1-upload-only
```

Expected graph:

```text
Scenario: lab1-upload-only

Services
- upload-service v1.0.0
  emits: image.uploaded (contracts/schemas/image-uploaded.event.v1.json)

Edges
- none
```

Architectural point:
The system begins with an observable event, not a hardcoded workflow.

### Lab 8.2: Add the First Compatible Service

Add `image-tagger`.
It declares that it can consume `image.uploaded` and provide `media.tagging`, then emits `image.tagged`.

Reader outcome:

- the runtime discovers a new compatible service
- the graph grows without orchestration code changes

Suggested commands:

```bash
./scripts/validate_graph_contracts.sh lab2-image-tagger
./scripts/contract_diff.sh lab1-upload-only lab2-image-tagger
./scripts/run_graph_demo.sh lab2-image-tagger
./scripts/graph_snapshot.sh lab2-image-tagger
./scripts/graph_diff.sh lab1-upload-only lab2-image-tagger
```

Expected graph:

```text
Edges
- upload-service -> image.uploaded -> image-tagger
```

Architectural point:
The first edge in the system comes from metadata compatibility.

### Lab 8.3: Extend the Graph Without Editing Existing Services

Add `metadata-indexer`.
It consumes `image.tagged` and persists searchable metadata.

Reader outcome:

- the graph deepens
- upstream services remain unchanged
- growth happens by addition, not rewiring

Suggested commands:

```bash
./scripts/validate_graph_contracts.sh lab3-indexer
./scripts/contract_diff.sh lab2-image-tagger lab3-indexer
./scripts/run_graph_demo.sh lab3-indexer
./scripts/graph_snapshot.sh lab3-indexer
./scripts/graph_diff.sh lab2-image-tagger lab3-indexer
```

Expected graph:

```text
Edges
- image-tagger -> image.tagged -> metadata-indexer
- upload-service -> image.uploaded -> image-tagger
```

Architectural point:
A UMA system can evolve without reopening stable upstream services.

### Lab 8.4: Break Compatibility

Introduce a deliberate mismatch by renaming an event or capability identifier.

Reader outcome:

- discovery no longer links all services
- the graph fractures in a visible, testable way

Suggested commands:

```bash
./scripts/validate_graph_contracts.sh lab4-broken-compat
./scripts/contract_diff.sh lab3-indexer lab4-broken-compat
./scripts/run_graph_demo.sh lab4-broken-compat
./scripts/graph_snapshot.sh lab4-broken-compat
./scripts/graph_diff.sh lab3-indexer lab4-broken-compat
```

Expected graph:

```text
Edges
- upload-service -> image.uploaded -> image-tagger

Waiting Consumers
- metadata-indexer waiting for image.tagged
```

Architectural point:
Metadata is not documentation around the architecture.
It is the architecture.

### Lab 8.5: Restore Compatibility

Fix the event or capability mismatch and rerun the graph.

Reader outcome:

- the graph is restored
- the repair is visible in Git history
- the runtime reconnects services automatically

Suggested commands:

```bash
./scripts/validate_graph_contracts.sh lab5-fixed-compat
./scripts/contract_diff.sh lab4-broken-compat lab5-fixed-compat
./scripts/run_graph_demo.sh lab5-fixed-compat
./scripts/graph_snapshot.sh lab5-fixed-compat
./scripts/graph_diff.sh lab4-broken-compat lab5-fixed-compat
```

Expected graph:

```text
Edges
- image-tagger -> image.tagged -> metadata-indexer
- upload-service -> image.uploaded -> image-tagger
```

Architectural point:
System recovery comes from restoring declarative consistency, not rewriting orchestration logic.

---

## Git alignment

The current scaffold stores each lab as a scenario folder so the reader can run everything from one branch.
If you want the book repo to mirror the chapter more literally, these five scenarios map cleanly to Git tags:

- `chapter8-lab1-upload-only` -> `lab1-upload-only`
- `chapter8-lab2-image-tagger` -> `lab2-image-tagger`
- `chapter8-lab3-indexer` -> `lab3-indexer`
- `chapter8-lab4-broken-compat` -> `lab4-broken-compat`
- `chapter8-lab5-fixed-compat` -> `lab5-fixed-compat`

That gives you both teaching modes:

- one branch for easy local execution
- tags later if you want immutable chapter checkpoints
- Git-style inspection now, using `contract_diff.sh` and `graph_diff.sh`

---

## Reader notes

- Keep the runtime output small and readable. The value in this chapter is the graph change, not framework complexity.
- Every lab should show both the runtime view and the Git diff.
- Validate the contracts before rendering the graph so bad metadata fails early.
- Keep capability ids, event names, and schema references aligned across labs so the graph only changes when the chapter intends it to.
- Avoid adding manual orchestration code between labs unless the exercise is explicitly about anti-patterns.
- If browser or edge variants are added later, mark them as validated only after they have the same reader-quality pass as the earlier chapters.

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can explain how an event-first service graph grows without explicit orchestration code
- you can point to the exact metadata change that created or removed a graph edge
- you can explain why restoring declarative consistency restores the system graph

---

## Next implementation steps

This scaffold is runnable, but it is still intentionally small.
The next improvements would be:

- add contract-level tests for schema version drift and capability incompatibility
- add more graph assertions around capability conflicts and version drift
- add Git tags if you want each lab to exist as an immutable checkpoint

That would bring Chapter 8 fully in line with the production quality of the earlier chapters.
