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

---

## Chapter Status

| Chapter | Main implementation | Validated path | Optional paths |
| --- | --- | --- | --- |
| 4 | Rust | `cargo test` plus `./scripts/run_vectors.sh` | none |
| 5 | Rust | `cargo test --workspace` plus native cloud host smoke | browser and edge sketches |
| 6 | Rust | native plus WASM parity lab | none |
| 7 | Rust and TypeScript | `./scripts/smoke_orchestration_labs.sh` with Rust as the validated default | browser and edge helper harnesses |
| 8 | Rust and TypeScript | `./scripts/smoke_graph_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |
| 9 | Rust and TypeScript | `./scripts/smoke_trust_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |
| 10 | Rust and TypeScript | `./scripts/smoke_arch_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |
| 11 | Rust and TypeScript | `./scripts/smoke_evolution_labs.sh` with Rust as the validated default | TypeScript implementation kept in parity and exposed directly for comparison |

Validated path means the commands are exercised by the repo smoke script and are the first path a reader should trust.

---

## Table of Contents

| Chapter | Example | Description | Folder |
| --- | --- | --- | --- |
| 4 | **Feature Flag Evaluator** | Demonstrates UMA service anatomy (contract, logic, abstraction, metadata) and shows stateless, stateful, and subscribable service types. | [`feature-flag-evaluator`](feature-flag-evaluator/) |
| 5 | **Post Fetcher Runtime** | Demonstrates the UMA Runtime Layer: contract loading, adapter binding, deterministic execution, and lifecycle logging across browser, edge, and cloud. | [`uma-post-fetcher`](uma-post-fetcher/) |
| 6 | **UMA Portability Lab** | Demonstrates UMA portability across runtimes by running the same service as both a portable WASI module and a native binary. Includes schema validation, capability gating, and reader labs for parity and determinism. | [`uma-portable-service-example`](uma-portable-service-example/) |
| 7 | **Metadata Orchestration and Validation** | Implements Chapter 7’s concepts of declarative orchestration, policy enforcement, and observability with a Rust-first validated cloud runner, a TypeScript parity runner, and optional browser and edge helper harnesses. | [`uma-metadata-orchestration`](uma-metadata-orchestration/) |
| 8 | **Service Graph Evolution with Git** | Demonstrates Chapter 8 with a Rust-first hands-on lab showing how UMA service graphs emerge through contracts, events, and runtime discovery across Git-style checkpoints. Includes a parallel TypeScript implementation. | [`chapter-8-service-graph`](chapter-8-service-graph/) |
| 9 | **Trust Boundaries and Runtime Enforcement** | Demonstrates Chapter 9’s trust model with a Rust-first lab that validates service identity, permissions, dependency provenance, and communication policy across five trust-boundary scenarios. Includes a parallel TypeScript implementation. | [`chapter-9-trust-boundaries`](chapter-9-trust-boundaries/) |
| 10 | **Architectural Tradeoffs and Runtime Coherence** | Demonstrates Chapter 10 through a Rust-first hands-on lab that compares coherent and degraded architectural choices across capability boundaries, events, metadata, placement, and orchestration. Includes a parallel TypeScript implementation. | [`chapter-10-architectural-tradeoffs`](chapter-10-architectural-tradeoffs/) |
| 11 | **Evolution Without Fragmentation** | Demonstrates Chapter 11 through a Rust-first hands-on lab that follows behavioral drift, duplication, version sprawl, and runtime-governed coexistence across an evolving system. Includes a parallel TypeScript implementation. | [`chapter-11-evolution-without-fragmentation`](chapter-11-evolution-without-fragmentation/) |

---

## Usage

Each example folder includes:
- `README.md` — build and run instructions  
- `contracts/` — UMA and adapter contracts  
- `src/` — core service logic  
- `runtime/` — example loaders and adapters for browser, edge, and cloud  
- `metadata/` — lifecycle or execution logs (where applicable)

Build and test examples locally:

```bash
git clone https://github.com/enricopiovesan/UMA-code-examples.git
cd UMA-code-examples
# Follow example-specific README for build and run instructions
```

The chapter READMEs call out where a path is fully validated versus where a browser or edge host is still illustrative scaffolding.
