# Universal Microservices Architecture (UMA) Code Examples

This repository hosts code samples for the **Universal Microservices Architecture (UMA)**.  
UMA is a design pattern for building portable microservices that can run consistently across **client, edge, and cloud** environments.  
Each service describes its own behavior, requirements, and runtime constraints through machine-readable metadata, enabling predictable execution without rewriting core logic.

These examples accompany the *Universal Microservices Architecture* book and white papers.  
Each folder represents a standalone project that can be built and run independently.

---

## New To UMA

UMA is a way to build portable services whose behavior stays consistent across client, edge, and cloud runtimes.
No prior UMA knowledge is assumed.

If you are landing here cold, the important idea is:

- the service logic should stay pure and portable
- contracts should describe what the service needs and emits
- the runtime should own placement, policy, adapters, and observability
- architecture should remain understandable as systems grow, evolve, and cross trust boundaries

This repository is not a generic microservices sample pack.
It is a guided progression showing how those ideas build on each other from Chapter 4 through Chapter 13.

## Start Here

If you want the fastest useful entry path as a new developer:

1. Read [chapter-04-feature-flag-evaluator](chapter-04-feature-flag-evaluator/README.md) to understand the smallest portable UMA service.
2. Continue to [chapter-05-post-fetcher-runtime](chapter-05-post-fetcher-runtime/README.md) to see how the runtime wraps a pure service.
3. Continue to [chapter-06-portability-lab](chapter-06-portability-lab/README.md) to see portability proven across native and WASI.
4. Follow Chapters 7 through 13 in order if you want orchestration, trust, architectural tradeoffs, system evolution, discoverable decisions, and portable MCP runtime composition.

If you only want one command to see whether the validated reader path works on your machine:

```bash
./scripts/smoke_reader_paths.sh
```

## Reader Setup

Shared prerequisites for the validated reader flows:

- Rust with the `wasm32-wasip1` target: `rustup target add wasm32-wasip1`
- Node.js 20 or newer for the TypeScript parity paths and optional browser/edge helpers
- `npm` for the TypeScript parity paths and optional browser/edge helpers
- `wasmtime` on your `PATH` for the WASI-backed examples
- Optional: `jq` for manual JSON inspection
- Optional: Deno if you want to experiment with the Chapter 7 edge harness

For a single repo-level verification pass after installing the prerequisites:

```bash
./scripts/smoke_reader_paths.sh
```

This script runs the validated chapter flows in sequence and stops on the first failure.
The same script is also used by the repository CI workflow on pull requests and pushes to `main`.

For measured Rust coverage across the validated reader chapters:

```bash
./scripts/report_rust_coverage.sh
```

To enforce the current minimum Rust coverage floor locally:

```bash
./scripts/check_rust_coverage.sh
```

To simulate a first-time reader running from a clean tracked checkout:

```bash
./scripts/simulate_fresh_reader_checkout.sh
```

---

## Reader Journey

The chapter folders now follow one naming convention:

- `chapter-04-*` through `chapter-13-*`

That gives the repo one visible reading order and makes it easier to move between the book and the code.

The intended thread for a reader is:

| Chapter | Why it matters in the sequence | Folder |
| --- | --- | --- |
| 4 | Start with one portable UMA service and understand its contract, rule evaluation, and deterministic output. | [`chapter-04-feature-flag-evaluator`](chapter-04-feature-flag-evaluator/) |
| 5 | Wrap that pure service logic in a runtime layer that owns validation, adapter binding, and lifecycle evidence. | [`chapter-05-post-fetcher-runtime`](chapter-05-post-fetcher-runtime/) |
| 6 | Prove portability by keeping one contract and one service aligned across native and WASI targets. | [`chapter-06-portability-lab`](chapter-06-portability-lab/) |
| 7 | Let contracts and events create orchestration dynamically, then observe the policy and telemetry around that flow. | [`chapter-07-metadata-orchestration`](chapter-07-metadata-orchestration/) |
| 8 | Watch a service graph emerge from metadata compatibility and inspect its evolution the way a system really grows. | [`chapter-08-service-graph`](chapter-08-service-graph/) |
| 9 | Add trust policy, provenance, and communication boundaries so portability stays governed rather than naive. | [`chapter-09-trust-boundaries`](chapter-09-trust-boundaries/) |
| 10 | Compare good and bad architectural decisions and see how they change runtime coherence. | [`chapter-10-architectural-tradeoffs`](chapter-10-architectural-tradeoffs/) |
| 11 | Follow system evolution over time and see how contracts and runtime governance prevent fragmentation. | [`chapter-11-evolution-without-fragmentation`](chapter-11-evolution-without-fragmentation/) |
| 12 | Make runtime decisions discoverable by exposing projections, proposals, validation feedback, and trace artifacts as queryable system surfaces. | [`chapter-12-discoverable-decisions`](chapter-12-discoverable-decisions/) |
| 13 | Close the path with a Rust-first portable MCP runtime that discovers capabilities, validates agent proposals, coordinates event-driven execution, and produces a structured report from distributed sources. | [`chapter-13-portable-mcp-runtime`](chapter-13-portable-mcp-runtime/) |

Chapter 3 is intentionally not a full lab in this repository. It is the conceptual bridge into Chapter 4, which is the first validated hands-on entry point.

## Learning Path

Use this table if you want a chapter-by-chapter tutorial path instead of browsing the folders ad hoc:

| Step | Chapter | First command | What you should understand before moving on |
| --- | --- | --- | --- |
| 1 | 4 | `cd chapter-04-feature-flag-evaluator && ./scripts/run_lab.sh lab1-country-match` | what a portable UMA service contract looks like and why deterministic behavior matters |
| 2 | 5 | `cd chapter-05-post-fetcher-runtime && ./scripts/run_lab.sh lab1-cloud-golden-path` | what belongs in a runtime layer versus a pure service |
| 3 | 6 | `cd chapter-06-portability-lab && ./scripts/run_lab.sh lab1-native-wasm-parity` | how portability is proven from observable output rather than assumed from shared code |
| 4 | 7 | `cd chapter-07-metadata-orchestration && ./scripts/run_lab.sh lab1-baseline-cloud-flow` | how contracts and events create orchestration without hardcoded workflow logic |
| 5 | 8 | `cd chapter-08-service-graph && ./scripts/run_graph_demo.sh lab1-upload-only` | how service graphs emerge from compatibility and evolve over time |
| 6 | 9 | `cd chapter-09-trust-boundaries && ./scripts/run_trust_demo.sh lab1-trusted-service` | how portability must be governed by trust, provenance, and communication policy |
| 7 | 10 | `cd chapter-10-architectural-tradeoffs && ./scripts/run_arch_demo.sh lab1-baseline` | how architectural choices change runtime coherence and system quality |
| 8 | 11 | `cd chapter-11-evolution-without-fragmentation && ./scripts/run_evolution_demo.sh lab1-contract-anchor` | how systems keep evolving without fragmenting when contracts and runtime governance stay explicit |
| 9 | 12 | `cd chapter-12-discoverable-decisions && ./scripts/run_decision_demo.sh lab1-capability-projection` | how a UMA system becomes discoverable when decisions, validation, and traces are exposed as artifacts |
| 10 | 13 | `cd chapter-13-portable-mcp-runtime && ./scripts/run_lab.sh use-case-1-basic-report` | how MCP discovery, runtime validation, agent proposals, and event-driven capabilities compose a portable structured-report experience |

---

## Chapter Status

| Chapter | Main implementation | Validated path | Optional paths |
| --- | --- | --- | --- |
| 4 | Rust and TypeScript | `./scripts/smoke_flag_labs.sh` with Rust as the validated default | browser, edge, and cloud adapters remain illustrative host examples |
| 5 | Rust and TypeScript | `./scripts/smoke_runtime_labs.sh` with Rust as the validated default | browser and edge sketches |
| 6 | Rust and TypeScript | `./scripts/smoke_portability_labs.sh` with Rust as the validated default | none |
| 7 | Rust and TypeScript | `./scripts/smoke_orchestration_labs.sh` with Rust as the validated default | browser and edge helper harnesses |
| 8 | Rust and TypeScript | `./scripts/smoke_graph_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |
| 9 | Rust and TypeScript | `./scripts/smoke_trust_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |
| 10 | Rust and TypeScript | `./scripts/smoke_arch_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |
| 11 | Rust and TypeScript | `./scripts/smoke_evolution_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |
| 12 | Rust and TypeScript | `./scripts/smoke_discoverability_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |
| 13 | Rust | `./scripts/smoke_portable_mcp_labs.sh` with Rust as the validated default | optional browser shell under `app/` consumes Rust-generated fixtures; no duplicate TypeScript runtime |

Validated path means the commands are exercised by the repo smoke script and are the first path a reader should trust.

---

## Chapter Index

| Chapter | Example | Description | Folder |
| --- | --- | --- | --- |
| 4 | **Feature Flag Evaluator** | Demonstrates UMA service anatomy with a Rust-first evaluator core, WASI CLI, guided reader labs, and a TypeScript parity implementation that mirrors the same rule semantics. | [`chapter-04-feature-flag-evaluator`](chapter-04-feature-flag-evaluator/) |
| 5 | **Post Fetcher Runtime** | Demonstrates the UMA Runtime Layer with a Rust-first reader lab covering deterministic cloud execution, fail-fast validation, adapter binding, lifecycle recording, and a TypeScript reference runtime kept in parity for the core scenarios. Browser and edge files remain illustrative sketches. | [`chapter-05-post-fetcher-runtime`](chapter-05-post-fetcher-runtime/) |
| 6 | **UMA Portability Lab** | Demonstrates UMA portability across runtimes by running the same Rust service as both a portable WASI module and a native binary. Includes guided reader labs for parity, payload digests, capability-boundary failure paths, and a TypeScript reference implementation of the shared analysis logic. | [`chapter-06-portability-lab`](chapter-06-portability-lab/) |
| 7 | **Metadata Orchestration and Validation** | Implements Chapter 7’s concepts of declarative orchestration, policy enforcement, and observability with a Rust-first validated cloud runner, a TypeScript parity runner, and optional browser and edge helper harnesses. | [`chapter-07-metadata-orchestration`](chapter-07-metadata-orchestration/) |
| 8 | **Service Graph Evolution with Git** | Demonstrates Chapter 8 with a Rust-first hands-on lab showing how UMA service graphs emerge through contracts, events, and runtime discovery across Git-style checkpoints. Includes a parallel TypeScript implementation. | [`chapter-08-service-graph`](chapter-08-service-graph/) |
| 9 | **Trust Boundaries and Runtime Enforcement** | Demonstrates Chapter 9’s trust model with a Rust-first lab that validates service identity, permissions, dependency provenance, and communication policy across five trust-boundary scenarios. Includes a parallel TypeScript implementation. | [`chapter-09-trust-boundaries`](chapter-09-trust-boundaries/) |
| 10 | **Architectural Tradeoffs and Runtime Coherence** | Demonstrates Chapter 10 through a Rust-first hands-on lab that compares coherent and degraded architectural choices across capability boundaries, events, metadata, placement, and orchestration. Includes a parallel TypeScript implementation. | [`chapter-10-architectural-tradeoffs`](chapter-10-architectural-tradeoffs/) |
| 11 | **Evolution Without Fragmentation** | Demonstrates Chapter 11 through a Rust-first hands-on lab that follows behavioral drift, duplication, version sprawl, and runtime-governed coexistence across an evolving system. Includes a parallel TypeScript implementation. | [`chapter-11-evolution-without-fragmentation`](chapter-11-evolution-without-fragmentation/) |
| 12 | **Discoverable Decisions** | Demonstrates Chapter 12 through a Rust-first hands-on lab that moves from hidden execution to queryable projections, structured proposals, authoritative validation feedback, bounded revision, approved execution, and full decision traces. Includes a parallel TypeScript implementation. | [`chapter-12-discoverable-decisions`](chapter-12-discoverable-decisions/) |
| 13 | **Portable MCP Runtime** | Demonstrates Chapter 13 through a Rust-first reference experience that discovers capabilities through MCP-style descriptors, validates deterministic agent proposals, coordinates event-driven capability execution, and produces a structured French report from distributed sources. Includes an optional browser visualization shell driven by Rust-generated fixtures. | [`chapter-13-portable-mcp-runtime`](chapter-13-portable-mcp-runtime/) |

---

## Chapter README Format

Each validated chapter README now follows the same reader-first spine:

- `## Key concepts`
- `## Prerequisites`
- `## Validation status`
- `## Quick start`
- `## Reader path`
- `## Questions a reader might ask`
- `## Layout`
- `## Troubleshooting`
- `## Value check`

Some chapters add domain-specific sections such as contracts, diff tools, or reports, but that core shape should stay stable.

Each chapter README should also make it obvious where the reader came from and where they should go next in the learning path.

## Working In The Repo

Build and test examples locally:

```bash
git clone https://github.com/enricopiovesan/UMA-code-examples.git
cd UMA-code-examples
# Follow example-specific README for build and run instructions
```

The chapter READMEs call out where a path is fully validated versus where a browser or edge host is still illustrative scaffolding.

Contributor expectations for keeping that bar are documented in [CONTRIBUTING.md](CONTRIBUTING.md).
