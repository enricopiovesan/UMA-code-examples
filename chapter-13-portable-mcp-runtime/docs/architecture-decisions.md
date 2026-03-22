# Architecture Decisions

## Primary implementation language

Rust is the authoritative implementation language for Chapter 13.

TypeScript may be used only for the browser shell or integration glue. There is no duplicate TypeScript runtime path for this chapter.

## Runtime model

- the runtime is written in Rust
- the runtime is designed to compile to WASM
- capability implementations are modeled as WASM-oriented contracts and resolvable runtime modules
- the MCP-facing surface is implemented as a real stdio JSON-RPC server in Rust
- eventing is used for capability interaction and visibility

## AI integration

- the planner agent is deterministic and local for the MVP
- the planner agent must remain behind a provider boundary so a model-backed implementation can be added later without changing runtime authority
- `SummarizerAI` is modeled as a runtime-hosted AI capability with a stable contract
- until the runtime-hosted provider is bound, fallback is allowed for continuity, but the fallback must be explicit in reports and UI
- the runtime remains authoritative for validation and execution

## UX decision

The app uses a split-screen layout:

- left: execution timeline
- right: 3D capability graph

The graph is there to explain the system, not to decorate it.

## Interpretive rule

Every implementation choice should be judged against one question:

Does this make Chapter 13 clearer?
