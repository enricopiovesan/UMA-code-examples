# Universal Microservices Architecture (UMA) Code Examples

[![Reader Smoke](https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/reader-smoke.yml/badge.svg)](https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/reader-smoke.yml)
[![Book Site](https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/book-site-pages.yml/badge.svg)](https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/book-site-pages.yml)
[![Live Reference App](https://img.shields.io/badge/live-reference%20app-0a7f5a)](https://www.universalmicroservices.com/reference-application/)
[![Learn UMA](https://img.shields.io/badge/book-universalmicroservices.com-1f6feb)](https://www.universalmicroservices.com/)

This repository is the hands-on code companion for **Universal Microservices Architecture (UMA)**.

It is not a generic microservices sample pack. It is a guided progression showing how UMA moves from one portable service to runtime-governed systems, discoverable decisions, and a full reference application.

[Buy the book and learn more about UMA](https://www.universalmicroservices.com/)

[Try the live UMA Reference APP](https://www.universalmicroservices.com/reference-application/)

![UMA Reference APP preview](book-site/assets/ref-app.png)

## What UMA Is

UMA is an architectural model for building portable services whose behavior stays consistent across client, edge, and cloud runtimes.

The core idea is:

- service logic stays pure and portable
- contracts describe what a capability needs and emits
- the runtime owns placement, policy, adapters, and observability
- systems stay understandable as they evolve

If you want the conceptual introduction, start here:

- [https://www.universalmicroservices.com/](https://www.universalmicroservices.com/)

If you want the most concrete end-to-end demo first, start here:

- [https://www.universalmicroservices.com/reference-application/](https://www.universalmicroservices.com/reference-application/)

## What This Repo Contains

The repo follows the book’s chapter order:

- `chapter-04-*` through `chapter-13-*`

Each chapter is a standalone lab or reference implementation. The path starts small and ends with the Chapter 13 portable MCP runtime reference app.

## Start Here

If you are new, use this order:

1. read Chapter 4’s README and run the first lab
2. continue chapter by chapter
3. use Chapter 13 as the reference application that ties the model together

If you want one quick confidence check for the repo:

```bash
./scripts/smoke_reader_paths.sh
```

## Reader Setup

Common prerequisites for the validated reader path:

- Rust with `wasm32-wasip1`
- Node.js 20+
- `npm`
- `wasmtime` on your `PATH`
- optional: `jq`

Useful repo-level commands:

```bash
./scripts/smoke_reader_paths.sh
./scripts/report_rust_coverage.sh
./scripts/check_rust_coverage.sh
./scripts/simulate_fresh_reader_checkout.sh
```

## Reader Journey

| Chapter | Example | Why it matters | First command |
| --- | --- | --- | --- |
| 4 | **Feature Flag Evaluator** | Start with one portable UMA service and understand its contract and deterministic behavior. | `cd chapter-04-feature-flag-evaluator && ./scripts/run_lab.sh lab1-country-match` |
| 5 | **Post Fetcher Runtime** | See what belongs in the runtime layer around a pure service. | `cd chapter-05-post-fetcher-runtime && ./scripts/run_lab.sh lab1-cloud-golden-path` |
| 6 | **UMA Portability Lab** | Prove portability across native and WASI targets. | `cd chapter-06-portability-lab && ./scripts/run_lab.sh lab1-native-wasm-parity` |
| 7 | **Metadata Orchestration and Validation** | Watch orchestration emerge from contracts and events. | `cd chapter-07-metadata-orchestration && ./scripts/run_lab.sh lab1-baseline-cloud-flow` |
| 8 | **Service Graph Evolution with Git** | See a service graph emerge and evolve. | `cd chapter-08-service-graph && ./scripts/run_graph_demo.sh lab1-upload-only` |
| 9 | **Trust Boundaries and Runtime Enforcement** | Add trust, provenance, and communication policy. | `cd chapter-09-trust-boundaries && ./scripts/run_trust_demo.sh lab1-trusted-service` |
| 10 | **Architectural Tradeoffs and Runtime Coherence** | Compare coherent and degraded architecture choices. | `cd chapter-10-architectural-tradeoffs && ./scripts/run_arch_demo.sh lab1-baseline` |
| 11 | **Evolution Without Fragmentation** | Follow system evolution without losing coherence. | `cd chapter-11-evolution-without-fragmentation && ./scripts/run_evolution_demo.sh lab1-contract-anchor` |
| 12 | **Discoverable Decisions** | Make runtime decisions queryable and inspectable. | `cd chapter-12-discoverable-decisions && ./scripts/run_decision_demo.sh lab1-capability-projection` |
| 13 | **Portable MCP Runtime** | See the full UMA reference application: MCP discovery, runtime validation, agent proposals, event-driven execution, and structured output. | `cd chapter-13-portable-mcp-runtime && ./scripts/run_lab.sh use-case-2-ai-report` |

Chapter 3 is intentionally not a full lab here. It is the conceptual bridge into Chapter 4, which is the first validated hands-on entry point.

## Chapter 13 Call To Action

If you only try one thing before reading deeper, use Chapter 13:

- live app: [https://www.universalmicroservices.com/reference-application/](https://www.universalmicroservices.com/reference-application/)
- repo chapter: [chapter-13-portable-mcp-runtime](chapter-13-portable-mcp-runtime/README.md)

That is the clearest single place to see UMA as:

- capability discovery
- runtime-governed workflow composition
- agent participation without agent authority
- portable execution explained in both CLI and browser form

## How To Use The Repo

- Use the README inside the chapter you are currently following.
- If you want the validated end-to-end repo path, use `./scripts/smoke_reader_paths.sh`.
- If you want contribution guidance, read [CONTRIBUTING.md](CONTRIBUTING.md).

## Learn More

- Book and overview: [https://www.universalmicroservices.com/](https://www.universalmicroservices.com/)
- Live reference app: [https://www.universalmicroservices.com/reference-application/](https://www.universalmicroservices.com/reference-application/)
