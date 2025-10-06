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
