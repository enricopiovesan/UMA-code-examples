# UMA Code Examples

This repository hosts code samples for the **Universal Microservices Architecture (UMA)**.  
UMA is a design pattern for building portable microservices that can run consistently across **client, edge, and cloud** environments.  
Each service describes its own behavior, requirements, and runtime constraints through machine-readable metadata, enabling predictable execution without rewriting core logic.

These examples accompany the *Universal Microservices Architecture* book and white papers.  
Each folder represents a standalone project that can be built and run independently.

---

## Reader Setup

Shared prerequisites for the validated reader flows:

- Rust with the `wasm32-wasip1` target: `rustup target add wasm32-wasip1`
- Node.js 20 or newer for the optional browser and edge helpers
- `npm` for the optional browser and edge helpers
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

## Table of Contents

| Chapter | Example | Description | Folder |
| --- | --- | --- | --- |
| 4 | **Feature Flag Evaluator** | Demonstrates UMA service anatomy (contract, logic, abstraction, metadata) and shows stateless, stateful, and subscribable service types. | [`feature-flag-evaluator`](feature-flag-evaluator/) |
| 5 | **Post Fetcher Runtime** | Demonstrates the UMA Runtime Layer: contract loading, adapter binding, deterministic execution, and lifecycle logging across browser, edge, and cloud. | [`uma-post-fetcher`](uma-post-fetcher/) |
| 6 | **UMA Portability Lab** | Demonstrates UMA portability across runtimes by running the same service as both a portable WASI module and a native binary. Includes schema validation, capability gating, and reader labs for parity and determinism. | [`uma-portable-service-example`](uma-portable-service-example/) |
| 7 | **Metadata Orchestration and Validation** | Implements Chapter 7’s concepts of declarative orchestration, policy enforcement, and observability with a Rust-first validated cloud runner, plus optional browser and edge helper harnesses. | [`uma-metadata-orchestration`](uma-metadata-orchestration/) |
| 8 | **Service Graph Evolution with Git** | Demonstrates Chapter 8 with a Rust-first hands-on lab showing how UMA service graphs emerge through contracts, events, and runtime discovery across Git-style checkpoints. | [`chapter-8-service-graph`](chapter-8-service-graph/) |
| 9 | **Trust Boundaries and Runtime Enforcement** | Demonstrates Chapter 9’s trust model with a Rust-first lab that validates service identity, permissions, dependency provenance, and communication policy across five trust-boundary scenarios. | [`chapter-9-trust-boundaries`](chapter-9-trust-boundaries/) |

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
