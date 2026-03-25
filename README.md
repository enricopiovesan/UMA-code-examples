
<img width="1419" height="868" alt="Screenshot 2026-03-25 at 11 39 52 AM" src="https://github.com/user-attachments/assets/793ea8d4-0d98-4dd9-b693-10ed2859c07a" />
[![Reader Smoke](https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/reader-smoke.yml/badge.svg)](https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/reader-smoke.yml)
[![Book Site](https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/book-site-pages.yml/badge.svg)](https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/book-site-pages.yml)
[![Live Reference App](https://img.shields.io/badge/live-reference%20app-0a7f5a)](https://www.universalmicroservices.com/reference-application/)
[![Learn UMA](https://img.shields.io/badge/book-universalmicroservices.com-1f6feb)](https://www.universalmicroservices.com/)
[![UMA Blog](https://img.shields.io/badge/blog-Medium-12100E)](https://medium.com/the-rise-of-device-independent-architecture)
# Universal Microservices Architecture (UMA) Code Examples

![Universal Microservices Architecture reference app banner](book-site/assets/ref-app.png)


This repository is the runnable companion for **Universal Microservices Architecture (UMA)**.

UMA is an architectural model for keeping one business behavior coherent across browser, edge, backend, workflow, and AI-assisted execution surfaces. The repo exists to prove that model with code, not just describe it.

[Buy the book and learn more about UMA](https://www.universalmicroservices.com/)

[Try the live UMA Reference APP](https://www.universalmicroservices.com/reference-application/)

![UMA Reference APP preview](book-site/assets/ref-app.png)

## Why This Repo Exists

Most teams do not struggle because they cannot deploy services. They struggle because the same business behavior keeps getting rewritten across runtimes:

- once in the browser for responsiveness
- again in the backend for authority
- again in the edge layer for latency
- again in workflow glue
- and now again in AI-assisted paths

UMA is a response to that fragmentation problem.

If you want the shortest site explanation of that gap, start here:

- [What problem does UMA solve?](https://www.universalmicroservices.com/what-problem-does-uma-solve/)

## What Makes UMA Different

UMA is not trying to replace deployment platforms, service meshes, or WebAssembly tooling.

It is trying to answer a different architectural question:

> How do you keep one behavior portable, governed, and explainable as execution moves across many runtimes?

The repo demonstrates a few specific answers:

- a portable service boundary that stays recognizable across runtimes
- explicit contracts instead of hidden behavior in framework glue
- a runtime that discovers, validates, approves, and explains execution
- workflows built from capabilities rather than hardcoded stack-shaped assumptions
- agent participation without agent authority

If you want the comparison page first:

- [UMA vs traditional microservices](https://www.universalmicroservices.com/uma-vs-traditional-microservices/)

## What You Can Try Today

If you want one fast proof point:

1. Open the [live reference app](https://www.universalmicroservices.com/reference-application/).
2. Play the workflow.
3. Watch the runtime approve the path and explain the result.

If you want the runnable repo path:

1. Start with Chapter 4.
2. Follow the examples chapter by chapter.
3. End with Chapter 13, where the full reference application ties the model together.

## Start Here

### Fastest conceptual path

- [What is UMA?](https://www.universalmicroservices.com/what-is-uma/)
- [What problem does UMA solve?](https://www.universalmicroservices.com/what-problem-does-uma-solve/)
- [What is a capability?](https://www.universalmicroservices.com/what-is-a-capability/)
- [What is a workflow?](https://www.universalmicroservices.com/what-is-a-workflow/)
- [What is a UMA runtime?](https://www.universalmicroservices.com/what-is-a-uma-runtime/)
- [Agent vs runtime](https://www.universalmicroservices.com/agent-vs-runtime/)

### Fastest runnable path

```bash
./scripts/smoke_reader_paths.sh
```

### Best single demo

- [Chapter 13 README](chapter-13-portable-mcp-runtime/README.md)
- [Live UMA Reference APP](https://www.universalmicroservices.com/reference-application/)

## Reader Journey

| Chapter | Example | What it proves | First command |
| --- | --- | --- | --- |
| 4 | [**Feature Flag Evaluator**](chapter-04-feature-flag-evaluator/README.md) | The smallest portable UMA service with one contract and deterministic output. | `cd chapter-04-feature-flag-evaluator && ./scripts/run_lab.sh lab1-country-match` |
| 5 | [**Post Fetcher Runtime**](chapter-05-post-fetcher-runtime/README.md) | What belongs in the runtime layer around a pure service. | `cd chapter-05-post-fetcher-runtime && ./scripts/run_lab.sh lab1-cloud-golden-path` |
| 6 | [**UMA Portability Lab**](chapter-06-portability-lab/README.md) | How portability is proven instead of assumed. | `cd chapter-06-portability-lab && ./scripts/run_lab.sh lab1-native-wasm-parity` |
| 7 | [**Metadata Orchestration and Validation**](chapter-07-metadata-orchestration/README.md) | How orchestration can emerge from contracts, metadata, and events. | `cd chapter-07-metadata-orchestration && ./scripts/run_lab.sh lab1-baseline-cloud-flow` |
| 8 | [**Service Graph Evolution with Git**](chapter-08-service-graph/README.md) | How compatibility and change shape a visible service graph. | `cd chapter-08-service-graph && ./scripts/run_graph_demo.sh lab1-upload-only` |
| 9 | [**Trust Boundaries and Runtime Enforcement**](chapter-09-trust-boundaries/README.md) | How trust, provenance, and policy remain explicit around portable execution. | `cd chapter-09-trust-boundaries && ./scripts/run_trust_demo.sh lab1-trusted-service` |
| 10 | [**Architectural Tradeoffs and Runtime Coherence**](chapter-10-architectural-tradeoffs/README.md) | How a system can still function while becoming architecturally incoherent. | `cd chapter-10-architectural-tradeoffs && ./scripts/run_arch_demo.sh lab1-baseline` |
| 11 | [**Evolution Without Fragmentation**](chapter-11-evolution-without-fragmentation/README.md) | How drift, coexistence, and version sprawl can stay governed instead of becoming chaos. | `cd chapter-11-evolution-without-fragmentation && ./scripts/run_evolution_demo.sh lab1-contract-anchor` |
| 12 | [**Discoverable Decisions**](chapter-12-discoverable-decisions/README.md) | How proposal, validation, approval, and trace become queryable system artifacts. | `cd chapter-12-discoverable-decisions && ./scripts/run_decision_demo.sh lab1-capability-projection` |
| 13 | [**Portable MCP Runtime**](chapter-13-portable-mcp-runtime/README.md) | The full UMA reference application: capability discovery, runtime validation, agent proposals, event-driven execution, and structured output. | `cd chapter-13-portable-mcp-runtime && ./scripts/run_lab.sh use-case-2-ai-report` |

## Repo Structure

- `chapter-04-*` through `chapter-13-*`
  - validated labs and reference implementations aligned with the learning path
- `book-site/`
  - the public site and concept pages at [universalmicroservices.com](https://www.universalmicroservices.com/)
- `scripts/`
  - reader smoke, coverage, and repo-quality helpers

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

## If You Want To Evaluate UMA Honestly

The best path is not to read one slogan and decide.

Use this order instead:

1. Read the problem framing on the site.
2. Try the live reference app.
3. Inspect one early chapter and one late chapter in the repo.
4. Then decide whether the full book is worth your time.

That is exactly why the site, the repo, and the book are kept connected. The site gives you the framing, the repo gives you the proof, and the book goes deeper into the model, the tradeoffs, and the design sequence behind it.

## Learn More

- Book and site: [https://www.universalmicroservices.com/](https://www.universalmicroservices.com/)
- Live reference app: [https://www.universalmicroservices.com/reference-application/](https://www.universalmicroservices.com/reference-application/)
- Medium blog: [https://medium.com/the-rise-of-device-independent-architecture](https://medium.com/the-rise-of-device-independent-architecture)
- Contribution guide: [CONTRIBUTING.md](CONTRIBUTING.md)

## License

This repository is available under either of these licenses, at your option:

- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE)
