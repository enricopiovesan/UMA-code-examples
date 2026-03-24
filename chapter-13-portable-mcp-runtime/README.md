# Chapter 13: Portable MCP Runtime

Chapter 13 is the repo's reference application.

It shows how UMA composes a workflow from discoverable capabilities instead of hardwiring a pipeline:

- `WASM MCP` exposes capabilities
- `PlannerAI` proposes when ranking is needed
- `UMA runtime` validates what is allowed
- the selected capabilities execute step by step
- the result is produced as a structured report

[Try the live demo](https://www.universalmicroservices.com/reference-application/)

![Chapter 13 reference app preview](../book-site/assets/ref-app.png)

## Key concepts

- `WASM MCP` as a real discovery and invocation surface
- runtime-built workflows composed from capabilities
- AI participation without AI authority
- runtime-hosted WASI capability execution
- one execution explained through CLI, JSON, and browser views

## What This Chapter Proves

- MCP can be a real discovery and invocation surface, not just a diagram label
- workflows can be built from capabilities at runtime
- AI can participate without becoming authoritative
- Rust/WASM can stay the primary implementation path
- the same execution can be explained in CLI, JSON, and browser views

## What Is Real Here

- real Rust UMA runtime
- real Rust stdio MCP server
- real runtime reports exported as JSON
- real runtime-hosted AI capability paths for:
  - `PlannerAI`
  - `SummarizerAI`
  - `TranslatorFr`
- real browser reference app built from those runtime reports

If a model-backed provider is unavailable, the runtime falls back explicitly instead of failing silently. That fallback is reported in the execution data.

## Layout

```text
chapter-13-portable-mcp-runtime/
  README.md
  AGENTS.md
  app/
  contracts/
  docs/
  examples/
  models/
  planner-ai-wasi/
  rust/
  scripts/
  summarizer-ai-wasi/
  translator-ai-wasi/
```

## Validation status

- validated Rust-first runtime path
- validated Rust stdio MCP server
- validated exported browser fixtures and reference app build
- explicit fallback reporting when a model-backed provider is unavailable

## Reader path

If you want the fastest useful path:

1. try the live app
2. run one AI workflow in the CLI
3. inspect the JSON report
4. run the MCP server

Recommended order:

```bash
cd chapter-13-portable-mcp-runtime
./scripts/setup_models.sh
./scripts/build_planner_ai_wasi.sh
./scripts/build_summarizer_ai_wasi.sh
./scripts/build_translator_ai_wasi.sh
./scripts/run_lab.sh use-case-2-ai-report
cargo run --manifest-path rust/Cargo.toml -- render use-case-2-ai-report json
./scripts/run_mcp_server.sh
```

## Key Workflows

The browser reference app focuses on the workflows that best explain the chapter:

- `use-case-2-ai-report`
  - French AI report
  - best first workflow
- `use-case-5-agent-validation`
  - runtime overrides a planner proposal
  - best workflow for understanding runtime authority
- `use-case-6-ai-executive-briefing`
  - same capability model, different output path

The CLI still includes the other use cases for testing and comparison.

## Prerequisites

- Rust 1.76+
- `wasm32-wasip1` target
- Node.js 20+
- `npm`
- `wasmtime` on your `PATH`

## Quick start

```bash
cd chapter-13-portable-mcp-runtime
./scripts/setup_models.sh
./scripts/build_planner_ai_wasi.sh
./scripts/build_summarizer_ai_wasi.sh
./scripts/build_translator_ai_wasi.sh
./scripts/list_labs.sh
./scripts/run_lab.sh use-case-2-ai-report
./scripts/smoke_mcp_server.sh
./scripts/smoke_portable_mcp_labs.sh
```

The scripts also work from the repo root with the chapter path prefixed.

## CLI Surfaces

List workflows:

```bash
./scripts/list_labs.sh
```

Run one workflow:

```bash
./scripts/run_lab.sh use-case-2-ai-report
```

Validate one workflow:

```bash
./scripts/validate_lab.sh use-case-2-ai-report
```

Render one report as JSON:

```bash
cargo run --manifest-path rust/Cargo.toml -- render use-case-2-ai-report json
```

Run the MCP server:

```bash
./scripts/run_mcp_server.sh
```

The MCP server exposes:

- `list_scenarios`
- `describe_scenario`
- `list_capabilities`
- `run_scenario`
- `validate_scenario`
- `render_report`

## Browser Reference App

The browser app is not a separate implementation of the runtime.

It is a reader-facing explanation layer over the same Chapter 13 reports:

- execution narrative
- execution graph
- workflow JSON
- workflow info modal

Local run:

```bash
./scripts/export_app_fixtures.sh
cd app
npm install
npm run dev
```

Do not use `python3 -m http.server` for this app. It uses Vite and G6.

Published app:

- [https://www.universalmicroservices.com/reference-application/](https://www.universalmicroservices.com/reference-application/)

GitHub source:

- [https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-13-portable-mcp-runtime](https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-13-portable-mcp-runtime)

## Model Setup

`./scripts/setup_models.sh` downloads the pinned chapter-local model artifacts and verifies checksums.

`./scripts/build_planner_ai_wasi.sh`, `./scripts/build_summarizer_ai_wasi.sh`, and `./scripts/build_translator_ai_wasi.sh` build the runtime-hosted AI modules the Rust runtime can invoke.

## Questions A Reader Should Be Able To Answer

By the end of this chapter, a reader should be able to explain:

- what a capability is
- what a workflow is
- what `WASM MCP` does
- what `PlannerAI` does
- why `UMA runtime` remains authoritative
- how the final result was produced

## Troubleshooting

- If the app fixtures are missing, run `./scripts/export_app_fixtures.sh`
- If model-backed AI is unavailable, check `./scripts/setup_models.sh` and the WASI build scripts
- If the app fails to load, run it through Vite with `npm run dev`
- If you need the raw workflow ids again, run `./scripts/list_labs.sh`

## Value check

By the end of this chapter, a reader should be able to explain:

- what a capability is
- what a workflow is
- what `WASM MCP` does
- what `PlannerAI` does
- why `UMA runtime` remains authoritative
- how the final result was produced

## Learning path position

- Previous: [Chapter 12: Discoverable Decisions](../chapter-12-discoverable-decisions/README.md)
- This chapter closes the repo’s validated learning path
