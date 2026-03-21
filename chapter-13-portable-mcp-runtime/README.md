# Chapter 13: Portable MCP Runtime

This example turns Chapter 13 of the UMA book into a reference lab.

The point of Chapter 13 is not to add another generic workflow demo. The point is to show how a UMA runtime, MCP discovery, event-driven capabilities, WASM portability, and agent-assisted reasoning combine into one coherent execution flow.

## Learning path position

- Previous: [Chapter 12: Discoverable Decisions](../chapter-12-discoverable-decisions/README.md)
- This chapter closes the validated repo learning path with a Rust-first reference experience.

## Key concepts

- requests define intent, not a fixed workflow
- MCP exposes capability descriptors and discovery surfaces
- the agent proposes a path, but the runtime validates it
- capabilities execute through contracts rather than hardwired service calls
- events make the transformation between steps visible
- WASM portability keeps execution behavior stable across runtimes

## Layout

```text
chapter-13-portable-mcp-runtime/
  README.md
  AGENTS.md
  docs/
  contracts/
  examples/
  rust/
  app/
  scripts/
```

## Prerequisites

- Rust 1.76 or newer
- a checkout of this repository
- optional: `wasm-pack` if you want to build the browser shell against the Rust WASM package later

No external services are required for the validated CLI path.

## Validation status

- Validated path: `./scripts/smoke_portable_mcp_labs.sh`
- Main implementation: Rust under `rust/`
- Browser shell: optional `app/` scaffold that can consume the same runtime report model
- Chapter 13 is intentionally Rust-first and does not maintain a duplicate TypeScript runtime

## Quick start

```bash
cd chapter-13-portable-mcp-runtime
./scripts/list_labs.sh
./scripts/run_lab.sh use-case-1-basic-report
./scripts/run_lab.sh use-case-2-ai-report
./scripts/run_lab.sh use-case-3-french-report
./scripts/run_lab.sh use-case-5-agent-validation
./scripts/smoke_portable_mcp_labs.sh
```

The scripts also work from the repo root if you prefix them with `chapter-13-portable-mcp-runtime/`.

If you want the machine-readable report for one scenario, use:

```bash
cargo run --manifest-path rust/Cargo.toml -- render use-case-3-french-report json
```

## Reader path

Use this order if you are following Chapter 13 as a first-time reader:

1. `./scripts/list_labs.sh`
2. `./scripts/run_lab.sh use-case-1-basic-report`
3. `./scripts/run_lab.sh use-case-2-ai-report`
4. `./scripts/run_lab.sh use-case-3-french-report`
5. `./scripts/run_lab.sh use-case-4-runtime-adapts`
6. `./scripts/run_lab.sh use-case-5-agent-validation`

Expected satisfaction point:
- by the end of use case 5, you should be able to explain how the agent can suggest a path while the runtime still owns compatibility, validation, and execution authority

## Questions a reader might ask

### "What is the system actually doing?"

It is generating a structured report from distributed source fragments. Depending on the goal and constraints, it discovers capabilities, chooses a valid path, executes intermediate enrichment and summarization steps, translates to French if required, and formats the result.

### "What makes this different from a workflow engine?"

The path is not hardcoded in advance. The runtime discovers candidates, the agent proposes a path, and the runtime validates or rejects each proposed step based on contracts and context.

### "What should I pay attention to in the output?"

The key sections are:

- `Goal`
- `Discovery`
- `Agent Proposal`
- `Rejected Candidates`
- `Execution Timeline`
- `Events`
- `Final Output`

### "Where is MCP in this chapter?"

MCP is the discoverability and invocation surface. It exposes capability descriptors and acts as the entry point through which capabilities are queried and invoked. It does not own business execution logic.

### "Why is Rust the only runtime implementation here?"

Chapter 13 is the reference experience. The strongest version is one authoritative Rust/WASM implementation rather than a duplicated parity path that would add surface area without improving architectural clarity.

## Hands-on flow

### Use Case 1: Basic report generation

Expected value:
- a valid local path is discovered and executed
- the output is structured, but not translated

### Use Case 2: Same goal, different path through changed constraints

Expected value:
- the runtime accepts the AI summarizer when local-only constraints are lifted
- the final report is still French, but the selected path is materially different
- the reader sees that the application goal stayed the same while capability resolution changed

### Use Case 3: Localized output with translation

Expected value:
- the runtime inserts an additional translator capability because the goal requires French output
- the final output remains structurally consistent

### Use Case 4: Capability unavailable, runtime adapts

Expected value:
- one capability becomes unavailable
- the runtime either chooses a degraded valid path or explains why it cannot fully satisfy the request

### Use Case 5: Agent proposes, runtime validates

Expected value:
- the agent proposes an AI summarizer
- the runtime rejects it under local-only constraints
- the runtime selects a valid deterministic summarizer instead

## Optional app shell

The browser shell under `app/` is a reader-facing visualization layer for the same Chapter 13 reports.
It uses generated JSON fixtures from the Rust runtime so the timeline and graph stay aligned with the authoritative implementation.

```bash
./scripts/export_app_fixtures.sh
cd app
python3 -m http.server 4173
```

Then open [http://localhost:4173](http://localhost:4173).

The shell also exposes a direct link to the machine-readable JSON for the currently selected scenario, so the CLI and browser views can be inspected side by side.

## Troubleshooting

- If `cargo` fails to resolve dependencies, run the command again once the lockfile dependencies have been fetched locally.
- If you mistype a lab id, rerun `./scripts/list_labs.sh`.
- If you want the raw scenario ids only, use `./scripts/list_labs.sh --ids-only`.

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can explain the difference between capability discovery and capability execution
- you can show how an agent participates without becoming authoritative
- you can point to the events, validation results, and final report that explain why the chosen path happened
