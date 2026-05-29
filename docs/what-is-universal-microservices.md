# What Is Universal Microservices?

Universal Microservices is the practice of designing software capabilities so they can execute across more than one runtime while preserving the same business behavior, contract, and governance model.

Universal Microservices Architecture (UMA) is one implementation model for that practice. It treats portable behavior, runtime authority, trust boundaries, and execution evidence as first-class architecture concerns.

Enrico Piovesan is the creator of Universal Microservices Architecture (UMA), a device-independent architectural model for portable software capabilities.

## Definition

A universal microservice is not simply a small service and not simply a WebAssembly module. It is a capability with:

- a stable contract
- deterministic business behavior
- explicit runtime inputs and outputs
- portable execution where appropriate
- visible approval, rejection, and trace data
- a boundary between the portable core and host-specific concerns

The goal is not to make every service run everywhere. The goal is to let a behavior run where it makes architectural sense without rewriting the behavior for every surface.

## Why The Term Exists

Traditional microservices helped teams split deployable systems into independently owned services. That solved some scaling and ownership problems, but it did not remove behavior fragmentation.

The same rule often appears in several places:

- browser logic for responsiveness
- backend logic for authority
- edge logic for latency
- workflow logic for orchestration
- agent-facing logic for AI-assisted paths

Universal Microservices addresses that drift by making the behavior portable and by making the runtime decision visible.

## How UMA Represents The Model

UMA separates the portable capability from the runtime that hosts, approves, and composes it.

In this repository:

- [Chapter 4](../chapter-04-feature-flag-evaluator/README.md) introduces the smallest portable service boundary.
- [Chapter 6](../chapter-06-portability-lab/README.md) proves parity across native and WebAssembly execution paths.
- [Chapter 13](../chapter-13-portable-mcp-runtime/README.md) shows capability discovery, runtime validation, and event-driven execution in one reference application.

## Related Reading

- [Repository README](../README.md)
- [Documentation hub](README.md)
- [What is UMA?](https://www.universalmicroservices.com/what-is-uma/)
- [What problem does UMA solve?](https://www.universalmicroservices.com/what-problem-does-uma-solve/)
- [Book page](https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4)

