# GitHub Discussions Topics For Universal Microservices Architecture

These are production-ready GitHub Discussions drafts. Use them in the repository Discussions area to create searchable, question-oriented pages around UMA concepts.

## Topic 1: What Problem Does Universal Microservices Architecture Solve?

Category: Ideas

Title: What problem does Universal Microservices Architecture solve?

Body:

Universal Microservices Architecture (UMA) addresses behavior fragmentation across runtime surfaces.

In many systems, the same business behavior is rewritten in the browser, backend, edge layer, workflow engine, and AI-assisted path. That duplication makes behavior harder to govern, test, and explain.

UMA explores a different boundary: a portable capability with explicit contracts, runtime validation, trust rules, and execution traces.

Useful starting points:

- [Repository README](../../README.md)
- [What Is Universal Microservices?](../what-is-universal-microservices.md)
- [Chapter 4: Feature Flag Evaluator](../../chapter-04-feature-flag-evaluator/README.md)
- [Chapter 13: Portable MCP Runtime](../../chapter-13-portable-mcp-runtime/README.md)

Discussion prompts:

- Where does behavior drift show up in your systems?
- Which runtime surfaces duplicate logic today?
- What evidence would you need before moving behavior into a portable capability?

## Topic 2: UMA vs Traditional Microservices

Category: Ideas

Title: Universal Microservices vs traditional microservices

Body:

Traditional microservices focus on independently deployable services. UMA focuses on portable capabilities whose behavior can remain consistent across runtime surfaces.

This is not a replacement argument. It is a boundary question: when should behavior live behind a normal service API, and when should it become a portable capability governed by a runtime?

Useful starting points:

- [Universal Microservices vs Traditional Microservices](../universal-microservices-vs-microservices.md)
- [UMA vs traditional microservices](https://www.universalmicroservices.com/uma-vs-traditional-microservices/)
- [Chapter 6: Portability Lab](../../chapter-06-portability-lab/README.md)

Discussion prompts:

- Which service boundaries should remain conventional services?
- Which behaviors are copied across frontend, backend, edge, or workflow code?
- What tradeoffs would make portability worth the added modeling discipline?

## Topic 3: UMA vs Serverless

Category: Ideas

Title: Universal Microservices vs serverless

Body:

Serverless platforms help with managed execution and scaling. UMA focuses on portable behavior, runtime authority, and execution evidence.

A UMA capability can run on serverless infrastructure, but the architecture question is different: can the behavior keep the same contract and governance model when the runtime changes?

Useful starting points:

- [Universal Microservices vs Serverless](../universal-microservices-vs-serverless.md)
- [What belongs in the runtime layer?](https://www.universalmicroservices.com/what-belongs-in-the-runtime-layer/)
- [Chapter 5: Post Fetcher Runtime](../../chapter-05-post-fetcher-runtime/README.md)

Discussion prompts:

- Where has serverless helped your system?
- Where has provider-specific glue created behavior drift?
- Which functions would benefit from stronger portability tests?

## Topic 4: Why WebAssembly Matters For Portable Architecture

Category: Ideas

Title: Why WebAssembly matters for portable architecture

Body:

WebAssembly gives portable code a practical execution target. UMA adds the architecture around that execution: contracts, runtime validation, trust boundaries, and traces.

Compiling to WebAssembly is not enough by itself. The portable core must stay small, deterministic, and separated from host-specific concerns.

Useful starting points:

- [WebAssembly Portability](../webassembly-portability.md)
- [WebAssembly architecture](https://www.universalmicroservices.com/webassembly-architecture/)
- [Chapter 6: Portability Lab](../../chapter-06-portability-lab/README.md)

Discussion prompts:

- Which WebAssembly use cases need stronger architectural boundaries?
- What should stay outside the module and inside the host runtime?
- How should teams test parity between native and WebAssembly behavior?

## Topic 5: Portable Business Logic

Category: Ideas

Title: What makes business logic portable?

Body:

Portable business logic is behavior that can execute in more than one runtime without being rewritten for each one.

In UMA, portability requires more than a shared library. It requires a stable contract, deterministic behavior, runtime separation, and evidence that execution remained equivalent.

Useful starting points:

- [Portable Microservices](../portable-microservices.md)
- [What makes a service portable?](https://www.universalmicroservices.com/what-makes-a-service-portable/)
- [Chapter 4: Feature Flag Evaluator](../../chapter-04-feature-flag-evaluator/README.md)

Discussion prompts:

- What logic in your systems is most often duplicated?
- What host concerns make portability difficult?
- What tests would prove that a capability stayed portable?

## Topic 6: Runtime Portability And Runtime Authority

Category: Ideas

Title: Runtime portability and runtime authority

Body:

UMA separates portable behavior from runtime authority. The capability performs deterministic work. The runtime discovers, validates, approves, rejects, composes, and records execution.

That distinction matters when workflows involve AI agents. An agent can propose a path, but the runtime remains responsible for deciding whether execution is allowed.

Useful starting points:

- [Chapter 13: Portable MCP Runtime](../../chapter-13-portable-mcp-runtime/README.md)
- [Agent vs runtime](https://www.universalmicroservices.com/agent-vs-runtime/)
- [What is a UMA runtime?](https://www.universalmicroservices.com/what-is-a-uma-runtime/)

Discussion prompts:

- What should an agent be allowed to propose?
- What should only the runtime be allowed to approve?
- What trace data should exist after a workflow runs?

