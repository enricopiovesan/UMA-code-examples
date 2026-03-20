# Architecture Decisions

## Primary implementation language

Rust is the authoritative implementation language for Chapter 13.

TypeScript may be used only for the browser shell or integration glue. There is no duplicate TypeScript runtime path for this chapter.

## Runtime model

- the runtime is written in Rust
- the runtime is designed to compile to WASM
- capability implementations are modeled as WASM-oriented contracts and resolvable runtime modules
- the MCP-facing surface is represented through runtime discovery and invocation APIs
- eventing is used for capability interaction and visibility

## AI integration

- the planner agent is deterministic and local for the MVP
- a model-backed summarization capability may exist as an optional execution candidate
- the runtime remains authoritative for validation and execution

## UX decision

The app uses a split-screen layout:

- left: execution timeline
- right: 3D capability graph

The graph is there to explain the system, not to decorate it.

## Interpretive rule

Every implementation choice should be judged against one question:

Does this make Chapter 13 clearer?
