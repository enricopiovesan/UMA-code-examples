# Runtime Model

The UMA runtime is responsible for execution authority. A portable capability can perform work, but the runtime decides whether, where, and how that work is allowed to execute.

## Runtime Responsibilities

The runtime owns:

- capability discovery
- input validation
- policy and trust checks
- approval and rejection
- workflow composition
- host integration
- execution traces

## Portable Core Responsibilities

The portable core owns:

- deterministic business behavior
- stable inputs and outputs
- minimal dependencies
- testable execution semantics

## Agent Participation

An AI agent can propose a workflow path, but the runtime remains responsible for validation and approval. This distinction keeps planning separate from authority.

## Code References

- [Universal Microservices vs Serverless](../universal-microservices-vs-serverless.md)
- [Chapter 5: Post Fetcher Runtime](../../chapter-05-post-fetcher-runtime/README.md)
- [Chapter 13: Portable MCP Runtime](../../chapter-13-portable-mcp-runtime/README.md)
- [Agent vs runtime](https://www.universalmicroservices.com/agent-vs-runtime/)

