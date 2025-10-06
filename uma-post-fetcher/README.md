# UMA Post Fetcher Example

This repository contains a minimal but production‑quality example service for Chapter 5 of the UMA book.  The goal is to demonstrate how the UMA runtime layer loads contracts, binds an adapter at runtime, enforces deterministic execution and persists a small lifecycle record.  A single compiled WebAssembly component runs unmodified in three different hosts: a browser, an edge worker and a cloud CLI.

## Overview

The service implements a simple HTTP fetcher.  It accepts a JSON document describing a request and a `runId` and returns a normalized representation of a [JSONPlaceholder](https://jsonplaceholder.typicode.com) post along with a deterministic event log.  The UMA runtime layer drives the service through a well‑defined lifecycle, ensures deterministic ordering of events and persists metadata about the run.

The repository is organised into several parts:

| Path | Purpose |
| --- | --- |
| `contracts/` | JSON contracts for the service interface, the network adapter, the runtime policy and the lifecycle metadata schema. |
| `service/` | Pure application logic (normalisation functions, models and API wrapper). |
| `runtime/` | The UMA runtime implementation (loader, adapter manager, thread manager, event bus and metadata). |
| `adapters/` | Portable capability adapters for network fetch.  Each adapter exposes the same WIT interface but uses different host binding strategies. |
| `hosts/` | Thin shims for running the compiled WebAssembly in different environments (cloud CLI, edge/Node worker and browser). |
| `tests/` | Integration test scripts and fixtures. |

## Service Contract

The service accepts an input document shaped like this:

```json
{
  "request": {
    "url": "https://jsonplaceholder.typicode.com/posts/1",
    "headers": {
      "accept": "application/json"
    }
  },
  "runId": "demo-001"
}
```

It returns a JSON object containing a normalised post and a deterministic event log.  An example response on a successful run:

```json
{
  "normalizedPost": {
    "id": 1,
    "userId": 1,
    "title": "<string>",
    "body": "<string>"
  },
  "events": [
    { "t": "0", "type": "start", "data": { "runId": "demo-001" } },
    { "t": "1", "type": "fetch_request", "data": { "url": "<string>" } },
    { "t": "2", "type": "fetch_response", "data": { "status": 200 } },
    { "t": "3", "type": "normalized", "data": { "id": 1 } },
    { "t": "4", "type": "end", "data": {} }
  ]
}
```

In error cases (non‑2xx status or parse error) the `normalizedPost` field is set to `null` and the last event contains an `error` key describing the condition.

### Adapter Capability Contract

The network adapter exposes a single capability, `network.fetch`, defined in `adapter.network.contract.json`.  It describes an idempotent `fetch` operation that accepts a URL, HTTP method (only `GET` is supported in this example) and optional headers.  The response includes a status code, headers and a raw body in bytes.  The UMA runtime selects an implementation at runtime based on the policy and persists the decision in the lifecycle record.  See `contracts/adapter.network.contract.json` for the full schema.

### Runtime Policy

The default runtime policy (`contracts/policy.runtime.json`) drives adapter selection, binding semantics, observability and lifecycle states.  It instructs the runtime to attempt to bind a `wasi-http` implementation first and fall back to a host‑provided fetch.  Binding of required capabilities is eager, and the runtime persists adapter selection decisions.  If you wish to override the default policy or experiment with different behaviours (for example, prefer a custom adapter, enable caching or adjust observability), you can create your own policy JSON file.  In this example the policy file serves as documentation; the runtime loader does not currently parse it at runtime, but in a full UMA implementation the loader would read the policy and act accordingly.

### Lifecycle Metadata Schema

The runtime persists a lifecycle record after each run.  The record includes the service name and version, a reference to the policy used, the bindings chosen for each capability, the final state, a logical clock (equal to the number of emitted events) and the event log itself.  See `contracts/metadata.schema.json` for the full schema.

## Getting Started

This repository demonstrates a complete example of a **UMA** service and runtime layer.  It includes contracts, pure service logic, a runtime that selects and binds adapters, event logging and lifecycle metadata, plus host shims for running the same compiled WebAssembly component in a cloud CLI, an edge worker and a web browser.

### Prerequisites

To build and run the example you will need:

* A recent Rust toolchain (tested with Rust 1.74 or later).  Install via [rustup](https://rustup.rs/).
* The `wasm32‑wasi` target added to your toolchain (`rustup target add wasm32-wasi`).
* [`wasmtime`](https://wasmtime.dev/) or another WASI runtime for running the cloud CLI example.  For outbound HTTP requests under WASI you must use a runtime built with the [wasi‑http](https://github.com/WebAssembly/wasi-http) preview interface enabled (for example, the `wasmtime-http` wrapper from the `wasi-experimental-http` project).  Otherwise the `WasiHttpAdapter` will fail and the runtime will fall back to host fetch in other environments.
* [npm](https://www.npmjs.com/) for building the browser and edge hosts, plus [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) if you prefer to generate JS bindings.

### Build the Workspace

Clone this repository and build all crates:

```sh
git clone <repo-url>
cd uma-post-fetcher
cargo build --workspace --release
```

This command builds the service, runtime and adapter crates for your native platform.  To build the WebAssembly component for WASI:

```sh
rustup target add wasm32-wasi # only needed once
cargo build -p uma_runtime --release --target wasm32-wasi
```

The resulting Wasm file will be placed at `target/wasm32-wasi/release/uma_runtime.wasm`.

#### Optional adapters: enabling retries and caching

The runtime can wrap the underlying network adapter with additional behaviours.  Two wrappers are provided out of the box:

* **RetryAdapter** – transparently retries failed requests a fixed number of times.  The number of retries is hard‑coded to three in this example and no back‑off delays are used so that execution remains deterministic.
* **CacheAdapter** – caches responses by URL for the lifetime of the adapter.  Subsequent requests for the same URL return the cached response instead of hitting the network.

You can enable these wrappers by setting environment variables when running the service.  For example, to enable both retries and caching for the cloud host:

```sh
export UMA_ENABLE_RETRY=1
export UMA_ENABLE_CACHE=1
bash hosts/cloud/run.sh
```

The adapter manager reads these variables at runtime and wraps the selected adapter accordingly.  If both retry and cache are enabled, the cache adapter wraps the retry adapter.  You can apply the same variables when running the edge or browser examples.

Currently the retry wrapper does not emit individual events for each retry attempt; only the final outcome (success or error) is recorded.  Extending the event bus to log `retry_attempt` events is a potential enhancement.

### Running on Multiple Targets

The same compiled component can run in a cloud CLI, an edge worker and a web browser.  The `hosts/` directory contains scripts and templates for each environment.

#### Cloud CLI (WASI)

Use the provided shell script to run the WebAssembly module via `wasmtime`:

```sh
cd uma-post-fetcher
bash hosts/cloud/run.sh
```

This script compiles the runtime for `wasm32‑wasi` (if needed) and invokes it with a sample input using `wasmtime`.  The output JSON and lifecycle record are printed to stdout.

#### Edge Worker (Deno/Node)

To run the example in an edge environment such as Deno or Node, you need to generate JavaScript bindings for the WebAssembly module.  One approach is to use `wasm-pack`:

```sh
cd uma-post-fetcher
wasm-pack build runtime --target nodejs --out-dir hosts/edge/pkg --out-name uma_runtime
```

Then execute the script in `hosts/edge/run.ts` with Node (you may need to enable ECMAScript modules):

```sh
node hosts/edge/run.ts
```

This script loads the Wasm module, constructs an input document and logs the service output and lifecycle record.  If you prefer Deno, build the package with `--target deno` and run it using `deno run`.

#### Browser

For the browser host, use `wasm-pack` to build for the web target and the Vite project in `adapters/network/ts-host` to serve the application:

```sh
cd uma-post-fetcher
wasm-pack build runtime --target web --out-dir adapters/network/ts-host/src/pkg --out-name uma_runtime
cd adapters/network/ts-host
npm install
npm run dev
```

Open the reported URL (typically `http://localhost:5173`) in your browser.  The console will display the service output and lifecycle record.  The UI can be customised by editing `public/index.html` and `src/host.ts`.

### Testing

Unit tests cover the service and runtime logic.  Run them with:

```sh
cargo test --workspace
```

Integration test scripts live under `tests/integration`.  The cloud integration script exercises the WASI build; browser and edge tests require building the corresponding packages first.

### Extending the Example

This repository is meant as a starting point for exploring UMA concepts.  You can extend it in several ways:

* **Retry and Backoff** – Add a runtime policy that instructs the adapter manager to retry failed fetches with exponential backoff.  The core logic in `service` remains pure; only the runtime changes.
* **Header Validation and Size Limits** – Inspect and validate request headers in the service contract and enforce response size limits in the adapter implementation.
* **Caching** – Implement an optional caching adapter for edge or browser hosts.  The runtime can select a cache adapter before falling back to network fetch.

### Environment variables

The runtime reads a few environment variables to control optional behaviour at runtime.  These variables allow you to enable the retry and cache wrappers without changing any code or configuration files:

| Variable | Description |
| --- | --- |
| `UMA_ENABLE_RETRY` | When set (to any value), wraps the selected network adapter in a `RetryAdapter` that retries failed requests up to three times. |
| `UMA_ENABLE_CACHE` | When set, wraps the selected network adapter in a `CacheAdapter` that caches responses by URL for the lifetime of the adapter. |
| `UMA_POLICY_PATH` | Not yet implemented.  Would instruct the runtime loader to read a custom policy JSON file instead of the default embedded policy. |

Unset variables mean the corresponding wrappers are disabled.  You can set these variables on a per‑invocation basis to experiment with different behaviours without recompiling the service.

### WASI HTTP support

When compiled for the `wasm32` target, the adapter manager attempts to select a `WasiHttpAdapter` if no custom adapter is provided.  This adapter uses the experimental `wasi-experimental-http-client` crate to send outbound HTTP requests via the WASI preview 2 `wasi:http` interface.  Your host runtime must support this interface; otherwise the adapter will return an error and the runtime will fall back to a host‑provided fetch in browser and edge environments.
* **Batch Requests** – Modify the service API to accept a list of URLs and normalise multiple posts in a single run.  Extend the event bus to log events for each request deterministically.
* **New Capabilities** – Define additional capability contracts (e.g. `storage.put`, `queue.publish`) under `contracts/` and provide corresponding adapters.  Update the runtime policy to select implementations at runtime.

When extending the example, maintain the principles of determinism, pure service logic and clear separation between capabilities (adapters) and business logic.  Document new contracts and policies in the `README` or separate design notes so that readers understand the service’s behaviour and extension points.

### Tests and Coverage

Comprehensive unit tests are provided for both the service and runtime crates under the `src/tests.rs` files.  These tests cover the normalisation logic, error handling, event bus, lifecycle record construction and the top‑level `run_json` function using a stub network adapter.  To run the tests:

```sh
cargo test --workspace
```

Code in the repository is documented with Rust doc comments.  You can generate HTML documentation with:

```sh
cargo doc --workspace --open
```

This will open the API documentation in your browser, where you can explore the modules, traits and structs exposed by the service, runtime and adapter crates.

The core logic is deliberately pure and deterministic: it does not depend on timers or random values.  A logical clock increments with each emitted event, guaranteeing consistent ordering across hosts.

## Extensibility

The example is intentionally simple to highlight the UMA runtime responsibilities.  The runtime can be extended to support retries, caching, header allowlists and more.  See the section in the book and the comments in the code for suggested extension points.