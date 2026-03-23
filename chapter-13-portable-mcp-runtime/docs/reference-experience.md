# Chapter 13 Reference Experience

## Purpose

The Chapter 13 reference experience exists to make the final UMA runtime model visible, concrete, and testable.

It is not a generic app and not a production orchestration platform. It is a small but complete system whose purpose is to prove the final architectural ideas through one understandable flow.

## What the experience must prove

- capabilities are defined through contracts
- discovery happens at runtime
- compatibility and constraints determine valid execution paths
- execution is validated before each step
- composition is not hardwired
- the runtime can adapt when constraints or availability change
- the agent can propose, but the runtime remains authoritative
- `PlannerAI` and `SummarizerAI` can execute through real runtime-hosted WASI modules
- if either AI capability falls back, the fallback stays explicit in the execution trace
- MCP is exposed through a real server surface rather than only through diagrams
- the same model stays portable across WASM-oriented environments

## Primary scenario

Generate a structured report in French from distributed source fragments.

The experience should:
- start from a goal and execution context
- discover candidate capabilities
- let the agent propose a path
- validate or reject that proposal explicitly
- execute capability steps
- emit events between steps
- produce a final French report

## Why this scenario

This scenario is strong enough to demonstrate:

- multiple distributed sources
- dynamic capability selection
- path expansion through translation and enrichment
- fallback when one capability is unavailable
- agent-assisted planning with authoritative runtime validation

It is also small enough to stay teachable.
