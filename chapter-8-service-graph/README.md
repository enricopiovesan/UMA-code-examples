# Chapter 8: Service Graph Evolution with Git

This example turns Chapter 8 of the UMA book into a reader-facing hands-on sequence.
The focus is not generic Git usage. The focus is using Git-style checkpoints to make UMA system growth visible.

Chapter 8 explains how a UMA system emerges from:

- service contracts
- declared capabilities
- emitted events
- runtime discovery

The labs below let a reader inspect that evolution one commit at a time.

---

## Learning goals

By the end of this hands-on section, the reader should be able to:

- start from a single UMA service that emits an event
- add compatible services without hand-written orchestration
- inspect how the service graph changes across commits
- see how metadata compatibility determines system shape
- understand how a broken contract fractures the graph
- restore compatibility and verify that the graph recovers

---

## Folder layout

This folder follows the same reader-first structure used in earlier chapters:

```text
chapter-8-service-graph/
  README.md
  contracts/
    schemas/
  runtime/
    graph.mjs
    graph_diff.mjs
    graph_lib.mjs
    validate.mjs
    graph.test.mjs
  scenarios/
    lab1-upload-only/
    lab2-image-tagger/
    lab3-indexer/
    lab4-broken-compat/
    lab5-fixed-compat/
  scripts/
    run_graph_demo.sh
    graph_snapshot.sh
    graph_diff.sh
```

The runtime prints a simple graph snapshot so the reader can see which services and events are currently connected.

---

## Prerequisites

- Node.js 20 or newer
- a checkout of this repository

No extra packages are required for this Chapter 8 scaffold.
The runtime, validator, and tests use only the built-in Node standard library.
The scenario contracts reference versioned schema files under `contracts/schemas/`.

---

## Quick start

Run each lab state directly:

```bash
cd chapter-8-service-graph
./scripts/run_graph_demo.sh lab1-upload-only
./scripts/run_graph_demo.sh lab2-image-tagger
./scripts/run_graph_demo.sh lab3-indexer
./scripts/run_graph_demo.sh lab4-broken-compat
./scripts/run_graph_demo.sh lab5-fixed-compat
```

Inspect a machine-readable snapshot:

```bash
./scripts/graph_snapshot.sh lab3-indexer
```

Compare two graph states:

```bash
./scripts/graph_diff.sh lab3-indexer lab4-broken-compat
```

Validate the scenario contracts:

```bash
./scripts/validate_graph_contracts.sh
```

Run the full Chapter 8 reader path:

```bash
./scripts/smoke_graph_labs.sh
```

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
upload-service
  emits -> image.uploaded
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
./scripts/run_graph_demo.sh lab2-image-tagger
./scripts/graph_snapshot.sh lab2-image-tagger
./scripts/graph_diff.sh lab1-upload-only lab2-image-tagger
```

Expected graph:

```text
upload-service
  emits -> image.uploaded
image-tagger
  consumes -> image.uploaded
  emits -> image.tagged
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
./scripts/run_graph_demo.sh lab3-indexer
./scripts/graph_snapshot.sh lab3-indexer
./scripts/graph_diff.sh lab2-image-tagger lab3-indexer
```

Expected graph:

```text
upload-service
  emits -> image.uploaded
image-tagger
  consumes -> image.uploaded
  emits -> image.tagged
metadata-indexer
  consumes -> image.tagged
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
./scripts/run_graph_demo.sh lab4-broken-compat
./scripts/graph_snapshot.sh lab4-broken-compat
./scripts/graph_diff.sh lab3-indexer lab4-broken-compat
```

Expected graph:

```text
upload-service
  emits -> image.uploaded
image-tagger
  emits -> image-tagged-renamed
metadata-indexer
  waiting for -> image.tagged
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
./scripts/run_graph_demo.sh lab5-fixed-compat
./scripts/graph_snapshot.sh lab5-fixed-compat
./scripts/graph_diff.sh lab4-broken-compat lab5-fixed-compat
```

Expected graph:

```text
upload-service
  emits -> image.uploaded
image-tagger
  consumes -> image.uploaded
  emits -> image.tagged
metadata-indexer
  consumes -> image.tagged
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

---

## Reader notes

- Keep the runtime output small and readable. The value in this chapter is the graph change, not framework complexity.
- Every lab should show both the runtime view and the Git diff.
- Validate the contracts before rendering the graph so bad metadata fails early.
- Keep capability ids, event names, and schema references aligned across labs so the graph only changes when the chapter intends it to.
- Avoid adding manual orchestration code between labs unless the exercise is explicitly about anti-patterns.
- If browser or edge variants are added later, mark them as validated only after they have the same reader-quality pass as the earlier chapters.

---

## Next implementation steps

This scaffold is runnable, but it is still intentionally small.
The next improvements would be:

- add contract-level tests for schema version drift and capability incompatibility
- add more graph assertions around capability conflicts and version drift
- add Git tags if you want each lab to exist as an immutable checkpoint

That would bring Chapter 8 fully in line with the production quality of the earlier chapters.
