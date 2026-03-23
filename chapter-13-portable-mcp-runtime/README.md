# Chapter 13: Portable MCP Runtime

This example turns Chapter 13 of the UMA book into a reference lab.

The point of Chapter 13 is not to add another generic workflow demo. The point is to show how a UMA runtime, a real MCP discovery surface, event-driven capabilities, WASM portability, and agent-assisted reasoning combine into one coherent execution flow.

## Learning path position

- Previous: [Chapter 12: Discoverable Decisions](../chapter-12-discoverable-decisions/README.md)
- This chapter closes the validated repo learning path with a Rust-first reference experience.

## Key concepts

- requests define intent, not a fixed workflow
- MCP exposes capability descriptors and invocation surfaces through a real stdio server
- the agent proposes a path, but the runtime validates it
- `PlannerAI` and `SummarizerAI` are modeled as explicit AI capabilities
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

What is real in this chapter today:

- the UMA runtime is real Rust code
- the browser shell is fed by real runtime reports
- the MCP server is a real stdio JSON-RPC server in Rust
- `PlannerAI` can run as a real runtime-hosted WASI planning capability once its chapter-local model artifacts are installed and the module is built
- `SummarizerAI` can run as a real runtime-hosted WASI summarization capability once its chapter-local model artifacts are installed and the module is built

What is not yet model-backed:

- if either runtime-hosted AI provider is missing or unavailable, the runtime falls back to the deterministic implementation instead of failing the whole scenario
- the fallback is reported explicitly in the machine-readable report and in the execution step that used it

What is already pinned for the real model path:

- `SummarizerAI` uses the chapter-local model manifest at `models/manifest.json`
- `PlannerAI` uses the chapter-local model manifest at `models/planner/manifest.json`
- both runtime-hosted model paths use pinned ONNX artifacts
- setup is handled by `./scripts/setup_models.sh` with checksum validation

## Validation status

- Validated path: `./scripts/smoke_portable_mcp_labs.sh`
- Main implementation: Rust under `rust/`
- Browser shell: optional `app/` scaffold that can consume the same runtime report model
- Chapter 13 is intentionally Rust-first and does not maintain a duplicate TypeScript runtime

## Quick start

```bash
cd chapter-13-portable-mcp-runtime
./scripts/setup_models.sh
./scripts/build_planner_ai_wasi.sh
./scripts/build_summarizer_ai_wasi.sh
./scripts/list_labs.sh
./scripts/run_lab.sh use-case-1-basic-report
./scripts/run_lab.sh use-case-2-ai-report
./scripts/run_lab.sh use-case-3-french-report
./scripts/run_lab.sh use-case-5-agent-validation
./scripts/run_lab.sh use-case-6-ai-executive-briefing
./scripts/smoke_mcp_server.sh
./scripts/smoke_portable_mcp_labs.sh
```

The scripts also work from the repo root if you prefix them with `chapter-13-portable-mcp-runtime/`.

`./scripts/setup_models.sh` downloads the pinned Chapter 13 ONNX summarizer artifacts into `models/`
and planner artifacts into `models/planner/`, then verifies their SHA-256 checksums.
The manifests are committed; the binary model files remain local.

`./scripts/build_planner_ai_wasi.sh` compiles the separate `PlannerAI` WASI module that the
Rust Chapter 13 runtime can invoke when a model-backed planner is selected.

`./scripts/build_summarizer_ai_wasi.sh` compiles the separate `SummarizerAI` WASI module that the
Rust Chapter 13 runtime can invoke when `SummarizerAI` is selected.

If you want the machine-readable report for one scenario, use:

```bash
cargo run --manifest-path rust/Cargo.toml -- render use-case-3-french-report json
```

If you want to run the Chapter 13 MCP server directly, use:

```bash
./scripts/run_mcp_server.sh
```

It serves the chapter through stdio JSON-RPC and exposes:

- `list_scenarios`
- `describe_scenario`
- `list_capabilities`
- `run_scenario`
- `validate_scenario`
- `render_report`

## Reader path

Use this order if you are following Chapter 13 as a first-time reader:

1. `./scripts/list_labs.sh`
2. `./scripts/run_lab.sh use-case-1-basic-report`
3. `./scripts/run_lab.sh use-case-2-ai-report`
4. `./scripts/run_lab.sh use-case-3-french-report`
5. `./scripts/run_lab.sh use-case-4-runtime-adapts`
6. `./scripts/run_lab.sh use-case-5-agent-validation`
7. `./scripts/run_lab.sh use-case-6-ai-executive-briefing`

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

MCP is the discoverability and invocation surface. In this repo, it is a real stdio JSON-RPC server implemented in Rust. It exposes capability descriptors and runtime tools, and acts as the entry point through which capabilities are queried and scenarios are invoked. It does not own business execution logic.

### "Why is Rust the only runtime implementation here?"

Chapter 13 is the reference experience. The strongest version is one authoritative Rust/WASM implementation rather than a duplicated parity path that would add surface area without improving architectural clarity.

### "Is the agent a real AI model?"

It can be.

If the chapter-local planner model artifacts are installed and the WASI planner module is built,
`PlannerAI` executes as a real runtime-hosted ranking planner.
If that provider is missing, the runtime falls back to the deterministic local planner.

What remains authoritative either way is still the UMA runtime: contracts, compatibility,
validation, and execution do not become model-controlled.

### "What happens when SummarizerAI is selected today?"

The runtime still resolves the real `SummarizerAI` contract and validates it normally.
If the chapter-local model artifacts are installed and the WASI summarizer module is built,
`SummarizerAI` executes as a real runtime-hosted extractive summarizer.
If either piece is missing, execution falls back automatically to a deterministic summarization provider.

That fallback is not hidden:

- the execution report records the provider, mode, and fallback reason
- the browser shell can show the same note in the transformation step

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
The graph panel is rendered with G6 rather than hand-built DOM edges, so playback and workflow highlighting stay clearer as the scenario changes.
Because the app now imports `@antv/g6`, it must be served through Vite rather than a plain static file server.
The browser shell automatically focuses on the workflows that actually exercise the real runtime-hosted AI path:

- `use-case-2-ai-report`
- `use-case-6-ai-executive-briefing`

The deterministic-only, degraded, and fallback-validation scenarios remain in the chapter CLI, tests, and fixtures, but they are not shown in the app selector by default because they do not invoke the real model-backed AI path end to end.

```bash
./scripts/export_app_fixtures.sh
cd app
npm install
npm run dev
```

Then open the local Vite URL shown in the terminal, typically [http://localhost:5173](http://localhost:5173).

Do not use `python3 -m http.server` for the G6 app. A plain static server cannot resolve the `@antv/g6` module import.

The shell also exposes a direct link to the machine-readable JSON for the currently selected scenario, so the CLI and browser views can be inspected side by side.
The human-readable CLI report intentionally suppresses generic "not relevant" discovery noise so the reader sees the meaningful runtime decisions first. The full raw event/discovery detail remains available in the JSON report.

## Troubleshooting

- If `cargo` fails to resolve dependencies, run the command again once the lockfile dependencies have been fetched locally.
- If you mistype a lab id, rerun `./scripts/list_labs.sh`.
- If you want the raw scenario ids only, use `./scripts/list_labs.sh --ids-only`.

## Value check

If this hands-on worked, you should finish it with three concrete gains:

- you can explain the difference between capability discovery and capability execution
- you can show how an agent participates without becoming authoritative
- you can point to the events, validation results, and final report that explain why the chosen path happened
