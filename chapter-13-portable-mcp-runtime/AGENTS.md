# AGENTS.md

## Project goal

Build the Chapter 13 UMA reference experience as a Rust-first, WASM-first lab.

This chapter is the point where UMA, MCP, event-driven capability composition, and agent-assisted reasoning are shown working together in one visible system. The lab must keep that flow concrete, inspectable, and small enough to understand.

## What this chapter must demonstrate

- requests enter through an MCP-facing surface
- capabilities are exposed through contracts, not fixed integrations
- the agent reasons over capability descriptors, not hardwired service identities
- the UMA runtime remains authoritative for compatibility, validation, and execution
- capabilities communicate through explicit events
- execution can cross browser, edge, and cloud-oriented runtimes through WASM-compatible contracts
- a final French report can emerge from multiple distributed sources and intermediate capability steps

## Primary scenario

Generate a structured report in French from distributed sources.

The request should:
- start from multiple source fragments
- discover candidate capabilities
- allow agent proposal
- validate or reject the proposal explicitly
- execute a valid path
- enrich and format the result
- show the final French output

## Implementation model

- Rust is the authoritative implementation language
- Rust code should be written so it can compile to WASM
- TypeScript may be used only for the browser shell or integration glue
- there is no duplicate TypeScript runtime path for Chapter 13

## Product constraints

- do not hardwire a fixed workflow
- do not bypass contract validation
- do not let the agent bypass runtime authority
- do not hide rejected candidates or validation reasons
- do not overbuild infrastructure for the MVP
- prefer in-memory state and deterministic fixtures
- prefer clarity over framework cleverness

## UX model

The chapter app uses a split-screen experience.

Left:
- execution timeline
- goal
- discovery
- proposal
- validation
- execution events
- final output

Right:
- minimal 3D graph
- MCP node
- agent
- runtime
- capability nodes
- active path highlighting

## Naming rules

Use these terms consistently:

- capability
- contract
- constraint
- event
- context
- resolver
- runtime
- MCP node
- agent

## Definition of done

A task is done only if:

- it makes the Chapter 13 runtime story clearer
- the behavior is visible or inspectable
- selected capabilities can be explained
- rejected capabilities include a reason
- validation is explicit
- the result is aligned with `docs/`
- the implementation does not reintroduce fixed pipeline logic
