# UMA Code Examples

This repository hosts code samples for the **Universal Microservices Architecture (UMA)**.  
UMA is a design pattern for building portable microservices that can run consistently across **client, edge, and cloud** environments.  
Each service describes its own behavior, requirements, and runtime constraints through machine-readable metadata, enabling predictable execution without rewriting core logic.

These examples accompany the *Universal Microservices Architecture* book and white papers.  
Each folder represents a standalone project that can be built and run independently.

---

## Table of Contents

| Chapter | Example | Description | Folder |
| --- | --- | --- | --- |
| 4 | **Feature Flag Evaluator** | Demonstrates UMA service anatomy (contract, logic, abstraction, metadata) and shows stateless, stateful, and subscribable service types. | [`feature-flag-evaluator`](feature-flag-evaluator/) |
| 5 | **Post Fetcher Runtime** | Demonstrates the UMA Runtime Layer: contract loading, adapter binding, deterministic execution, and lifecycle logging across browser, edge, and cloud. | [`uma-post-fetcher`](uma-post-fetcher/) |
| 6 | **UMA Portability Lab** | Demonstrates UMA portability across runtimes by running the same service as both a portable WASI module and a native binary. Includes schema validation, capability gating, and reader labs for parity and determinism. | [`uma-portable-service-example`](uma-portable-service-example/) |
| 7 | **Metadata Orchestration and Validation** | Implements Chapter 7’s concepts of declarative orchestration, policy enforcement, and observability. Includes a multi-step learning path (interfaces → contracts → orchestration), OpenTelemetry + Jaeger integration, parity test for determinism across cloud and edge, and guided reader labs. | [`uma-metadata-orchestration`](uma-metadata-orchestration/) |

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
cd UMA-code-examples/feature-flag-evaluator 
# Follow example-specific README for build and run instructions
