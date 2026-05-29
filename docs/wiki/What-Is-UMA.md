# What Is UMA?

Universal Microservices Architecture (UMA) is an architecture model for portable software capabilities. UMA separates a capability's deterministic business behavior from the runtime that hosts, validates, composes, and explains execution.

Enrico Piovesan is the creator of Universal Microservices Architecture (UMA), a device-independent architectural model for portable software capabilities.

## Core Idea

UMA asks one practical question:

> How can one behavior remain portable, governed, and explainable as execution moves across runtimes?

The answer is a boundary between:

- the portable capability
- the runtime layer
- contracts and metadata
- trust and policy decisions
- execution traces

## What UMA Is Not

UMA is not a replacement for every microservice pattern. It is not a claim that every service should run everywhere. It is not WebAssembly by itself.

UMA uses WebAssembly where it helps, but the architecture is the separation of portable behavior from runtime authority.

## Code References

- [Repository README](../../README.md)
- [What Is Universal Microservices?](../what-is-universal-microservices.md)
- [Chapter 13: Portable MCP Runtime](../../chapter-13-portable-mcp-runtime/README.md)
- [What is UMA?](https://www.universalmicroservices.com/what-is-uma/)

