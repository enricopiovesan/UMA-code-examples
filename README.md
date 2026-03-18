# Universal Microservices Architecture (UMA) Code Examples

This repository hosts code samples for the **Universal Microservices Architecture (UMA)**.  
UMA is a design pattern for building portable microservices that can run consistently across **client, edge, and cloud** environments.  
Each service describes its own behavior, requirements, and runtime constraints through machine-readable metadata, enabling predictable execution without rewriting core logic.

These examples accompany the *Universal Microservices Architecture* book and white papers.  
Each folder represents a standalone project that can be built and run independently.

---

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

- `chapter-04-*` through `chapter-11-*`

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

Chapter 3 is intentionally not a full lab in this repository. It is the conceptual bridge into Chapter 4, which is the first validated hands-on entry point.

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

## Working In The Repo

Build and test examples locally:

```bash
git clone https://github.com/enricopiovesan/UMA-code-examples.git
cd UMA-code-examples
# Follow example-specific README for build and run instructions
```

The chapter READMEs call out where a path is fully validated versus where a browser or edge host is still illustrative scaffolding.

Contributor expectations for keeping that bar are documented in [CONTRIBUTING.md](CONTRIBUTING.md).
