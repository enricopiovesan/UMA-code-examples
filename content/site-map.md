# UMA Site Map

This file is the canonical ordering and grouping source for the UMA website.
It defines semantic page groups, not HTML output.

## Rules

- Home is managed separately.
- Every other page belongs to one macro area.
- Chapter pages live under `Examples`.
- Stable URLs take precedence over title changes.
- The generator should read this file to build navigation and page order.

## Macro Areas

### Why UMA
- `why-uma`
- `what-problem-does-uma-solve`
- `what-is-uma`
- `why-universal-microservices-exist`
- `what-is-a-universal-microservice`
- `from-stack-ownership-to-behavior-ownership`

### Core Model
- `core-model`
- `what-is-a-capability`
- `what-is-a-workflow`
- `what-is-a-uma-runtime`
- `what-belongs-in-the-runtime-layer`
- `active-descriptors`
- `late-bound-policy-enforcement`
- `what-makes-a-decision-discoverable`
- `what-is-wasm-mcp`
- `agent-vs-runtime`

### How UMA Works
- `how-uma-works`
- `runtime-agnostic-architecture`
- `portable-business-logic`
- `architecture-drift-and-portable-business-logic`
- `webassembly-architecture`
- `webassembly-microservices-architecture`
- `wasm-microservices-tutorial-rust`
- `wasm-microservices-tutorial-typescript`
- `migrating-to-uma-incrementally`
- `incremental-uma-adoption`
- `uma-production-readiness`

### Proof
- `proof`
- `what-makes-a-service-portable`
- `how-to-prove-portability`
- `benchmark-and-footprint`

### Learning Path
- `learn-uma`
- `learning-path`
- `book`
- `end-to-end-feature-flag-example`

### Hands-On Examples
- `examples`
- `chapter-04-feature-flag-evaluator`
- `chapter-05-post-fetcher-runtime`
- `chapter-06-portability-lab`
- `chapter-07-metadata-orchestration`
- `chapter-08-service-graph`
- `chapter-09-trust-boundaries`
- `chapter-10-architectural-tradeoffs`
- `chapter-11-evolution-without-fragmentation`
- `chapter-12-discoverable-decisions`
- `chapter-13-portable-mcp-runtime`

### System Evolution
- `evolve-uma`
- `contract-driven-orchestration`
- `service-graph-evolution`
- `how-systems-evolve-without-fragmentation`
- `what-makes-a-system-coherent`
- `trust-boundaries`
- `runtime-provenance-and-trust`
- `ai-native-runtime-governance`
- `mcp-wasm-ai-native-microservices`

### Discovery and References
- `discoverability`
- `faq`
- `glossary`
- `diagrams`
- `about-enrico`
- `blog`
- `reference-application`
- `white-paper`

### Comparisons and Tradeoffs
- `comparisons`
- `uma-vs-serverless`
- `uma-vs-modular-monolith`
- `uma-vs-traditional-microservices`
- `common-criticisms-and-tradeoffs-of-uma`
- `why-software-architecture-keeps-fragmenting`
- `wasm-vs-docker-kubernetes`
- `microservices-without-kubernetes`

## Ordering Notes

- Primary ordering should follow the learning journey from `Why UMA` to `Hands-On Examples` and then outward to evolution and references.
- The `Examples` area should be subdivided by chapter sequence when rendered in navigation.
- The footer should draw from the macro areas, not from ad hoc page lists.
