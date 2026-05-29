# Portable Microservices

Portable Microservices are services or capabilities whose core behavior can move across execution environments without being rewritten for each environment.

In UMA, portability is not a slogan. It is tested through contracts, deterministic outputs, parity checks, and runtime-visible execution records.

## What Portability Requires

A portable microservice needs more than a compiled artifact. It needs:

- a small and explicit capability boundary
- inputs and outputs that are stable enough to test
- host concerns kept outside the portable core
- parity checks between native and WebAssembly paths
- a runtime model that can approve or reject execution
- evidence that behavior did not change when the runtime changed

## Why WebAssembly Matters

WebAssembly is useful because it gives portable code a practical execution target. It does not solve architecture by itself.

UMA uses WebAssembly as one tool for keeping business behavior portable while leaving orchestration, trust, and host integration in the runtime layer.

## Repository Proof Path

Use this sequence to inspect portable microservices in code:

1. [Chapter 4: Feature Flag Evaluator](../chapter-04-feature-flag-evaluator/README.md) for the smallest capability boundary.
2. [Chapter 5: Post Fetcher Runtime](../chapter-05-post-fetcher-runtime/README.md) for runtime responsibilities around a pure service.
3. [Chapter 6: Portability Lab](../chapter-06-portability-lab/README.md) for native and WebAssembly parity.
4. [Chapter 13: Portable MCP Runtime](../chapter-13-portable-mcp-runtime/README.md) for a full reference application.

## Related Reading

- [What Is Universal Microservices?](what-is-universal-microservices.md)
- [WebAssembly Portability](webassembly-portability.md)
- [What makes a service portable?](https://www.universalmicroservices.com/what-makes-a-service-portable/)
- [How to prove portability](https://www.universalmicroservices.com/how-to-prove-portability/)
- [Repository README](../README.md)

