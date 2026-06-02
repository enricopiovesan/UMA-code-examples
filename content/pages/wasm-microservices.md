---
ref: wasm-microservices
title: "WASM Microservices"
subtitle: "WebAssembly microservices are portable services compiled to WASM: sandboxed, runtime-agnostic, and deployable across browser, edge, cloud, and AI-native environments without recompilation."
macro_area: null
content_type: hub
slug: wasm-microservices
canonical_url: "https://www.universalmicroservices.com/wasm-microservices/"
left_nav_group: null
chapter_ref: null
seo_description: "What WebAssembly microservices are, how they work, and why WASM is the right execution boundary for portable distributed systems. Practical guides for Rust and TypeScript."
breadcrumbs:
  - "Home"
  - "WASM Microservices"
related_refs:
  - webassembly-microservices-architecture
  - wasm-microservices-tutorial-rust
  - wasm-microservices-tutorial-typescript
  - wasm-vs-docker-kubernetes
  - what-is-wasm-mcp
---

## intro

<section class="subpage-hero">
  <h1>WASM microservices</h1>
  <p>WebAssembly microservices are portable services compiled to WASM: sandboxed, runtime-agnostic, and deployable across browser, edge, cloud, and AI-native environments without recompilation.</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>What a WASM microservice is</h2>
    <p>A WASM microservice is a service whose business logic is compiled to a WebAssembly module rather than packaged as a container image or deployed as a native process. The module is a self-contained binary: it imports only the capabilities it explicitly declares, exposes a typed interface via the WASM Component Model, and executes inside a sandboxed virtual machine hosted by a WASM runtime.</p>
    <p>This is different from a traditional container-based microservice in three concrete ways:</p>
    <ul>
      <li><strong>Isolation model.</strong> A container process shares the host OS kernel and has ambient access to the network, filesystem, and environment unless explicitly restricted. A WASM module has no ambient authority. It can only use the capabilities the host explicitly passes at startup. The permission surface is opt-in, not opt-out.</li>
      <li><strong>Portability scope.</strong> A container image is tied to an OS and CPU architecture combination. The same WASM module runs without recompilation in a browser, a Cloudflare Worker, a wasmtime process on Linux, and a WasmEdge instance at the edge. The execution semantics are defined by the WASM specification, not by the host OS.</li>
      <li><strong>Artifact size and startup time.</strong> A minimal WASM module is measured in kilobytes. A container image is typically measured in tens to hundreds of megabytes. Cold start times for WASM are sub-millisecond at the edge. Container cold starts are seconds. This difference matters for high-frequency invocations, edge deployments, and cost-sensitive workloads.</li>
    </ul>
    <p>The service contract is expressed using WIT (WebAssembly Interface Types), which supports typed records, variants, streams, and resources. The interface is machine-readable and embedded in the artifact itself, so a runtime can inspect what a module expects and produces before running it.</p>
  </section>

  <section>
    <h2>Why WASM is the right execution boundary</h2>
    <p>The case for WASM as a service execution boundary is not about performance alone, though near-native execution speed and sub-millisecond cold starts are real properties. The stronger argument is the combination of sandboxing, portability, and interface determinism in a single binary format.</p>
    <p><strong>Sandboxing.</strong> The WASM security model is capability-based by specification, not by convention. A module compiled to WASM cannot open a network socket, read a file, or access environment variables unless the host runtime explicitly provides those capabilities at startup. This is not a policy layer bolted on top. It is the default behavior of the execution model. The permission surface of a running service is auditable by inspecting what capabilities the host grants, not by auditing the service's source code for ambient calls.</p>
    <p><strong>Portability.</strong> The same binary runs across runtimes. In practice this means: the same module that handles a Cloudflare Worker HTTP request handles a wasmtime process invocation and a browser WebAssembly call. Behavior is deterministic: the same module, same inputs, same outputs regardless of host OS or CPU architecture. This is a stronger guarantee than "same Dockerfile on different machines."</p>
    <p><strong>Size.</strong> A Rust-compiled WASM module for a stateless business logic service is typically 200–800 KB. A comparable container image with a language runtime is 50–200 MB. The size difference affects distribution time, cold start time, and cost at scale. At the edge, where modules may be distributed to hundreds of points of presence, this is not a marginal concern.</p>
    <p><strong>Interface determinism.</strong> WIT interfaces are embedded in the component binary. The type system covers records, variants, options, lists, resources, and streams, which is enough to express real service contracts. A WASM runtime can verify interface compatibility before executing. This is structurally different from REST APIs or gRPC where the contract lives in a separate file that may drift from the implementation.</p>
  </section>

  <section>
    <h2>WASM microservices vs Docker and Kubernetes</h2>
    <p>WASM and containers solve overlapping but distinct problems. Choosing between them depends on the workload, the deployment target, and the operational constraints in place.</p>
    <p><strong>Use WASM when:</strong></p>
    <ul>
      <li>The service is stateless and compute-oriented: request handling, transformation, validation, inference.</li>
      <li>You need sub-millisecond cold starts for edge deployments, event-driven invocations, or high-frequency functions.</li>
      <li>You are deploying to multiple runtimes (browser, edge, server) and want a single binary artifact.</li>
      <li>The service's permission surface must be explicit and auditable by default.</li>
      <li>Binary distribution size and startup latency are operational constraints.</li>
    </ul>
    <p><strong>Use containers when:</strong></p>
    <ul>
      <li>The service depends on full Linux userspace tooling, mature language runtimes that do not compile well to WASM, or shared filesystem semantics.</li>
      <li>You need to run existing workloads without modifying them. Containerizing an existing binary is lower friction than recompiling to WASM.</li>
      <li>Stateful services with complex I/O patterns that exceed current WASI capability coverage.</li>
      <li>Your team's operational tooling (observability, debugging, incident response) is built around container primitives.</li>
    </ul>
    <p>In realistic architectures, WASM and containers coexist. The edge and serverless layers increasingly run WASM. The server-side coordination, stateful storage, and legacy integration layers run in containers. The question is not which replaces the other but which execution boundary fits each service's requirements.</p>
    <p>Kubernetes does not directly support WASM workloads today. Runwasi (a containerd shim for WASM) allows Kubernetes to schedule WASM modules alongside containers, but it is still maturing. For pure WASM deployments at scale, purpose-built WASM platforms (Cloudflare Workers, Fastly Compute, Fermyon Spin) are the current production-grade option.</p>
  </section>

  <section>
    <h2>How to build a WASM microservice</h2>
    <p>Two toolchain paths cover most production use cases: Rust and TypeScript via AssemblyScript. Both compile to WASM modules compatible with WASI 0.2 and the Component Model.</p>
    <p><strong>Rust path.</strong> Rust has the most complete WASM toolchain. The <code>wasm32-wasip2</code> compilation target (available from Rust 1.78+) produces WASI 0.2 components. The <code>cargo component</code> toolchain handles WIT interface generation and component bundling. Output binaries are small (typically under 1 MB for non-trivial services) because Rust has no garbage collector and compiles to minimal WASM. The tradeoff is Rust's learning curve: ownership and borrow checking add friction for teams new to the language.</p>
    <p><strong>TypeScript path.</strong> AssemblyScript is a strict subset of TypeScript that compiles to WASM. Teams already writing TypeScript can adopt AssemblyScript with moderate friction. Binary sizes are larger than Rust but smaller than GC-based languages. WASI 0.2 support in AssemblyScript is maturing. As of 2024, WASI 0.1 is the stable target. WASI 0.2 support is in active development.</p>
    <p>For step-by-step walkthroughs, see the linked tutorials below.</p>
  </section>

  <section>
    <h2>WASM microservices in UMA</h2>
    <p>UMA uses WASM as the execution boundary for portable business logic. The WASM module provides the portability guarantee: compile once, run anywhere the WASM spec is implemented. UMA's active descriptors and runtime governance layer provide what WASM alone does not: contract definition, placement policy, version constraints, and audit evidence.</p>
    <p>In UMA, a WASM microservice is not just a module. It is a module paired with a descriptor that declares:</p>
    <ul>
      <li>The input and output schemas the module expects and produces.</li>
      <li>The deployment targets where the module is allowed to run (browser, edge, server, or a specific runtime class).</li>
      <li>The version constraints governing compatibility with other services in the workflow.</li>
      <li>The evidence requirements a successful run must satisfy to be considered compliant.</li>
    </ul>
    <p>The runtime evaluates the descriptor before scheduling execution. If the current environment does not satisfy the declared constraints, the module is not placed there. This is runtime governance: policy enforcement at execution time, not at deployment time as a one-off check.</p>
    <p>The same WASM module runs without recompilation across the three UMA deployment contexts: browser (via the browser's native WASM API), edge (via Cloudflare Workers or Fastly Compute), and server (via wasmtime or WasmEdge inside a container). What changes between contexts is the capability surface the host provides. The module's behavior is invariant.</p>
    <p>WASM portability plus descriptor-driven governance is what makes WASM microservices production-ready in complex distributed systems rather than just in toy examples. WASM provides the mechanism. UMA provides the model.</p>
  </section>

  <section class="subpage-callout">
    <strong>Go deeper</strong>
    <div class="subpage-inline-links">
      <a href="../how-uma-works/webassembly-microservices-architecture/">WebAssembly Microservices Architecture</a>
      <a href="../how-uma-works/wasm-microservices-tutorial-rust/">WASM Microservices Tutorial: Rust</a>
      <a href="../how-uma-works/wasm-microservices-tutorial-typescript/">WASM Microservices Tutorial: TypeScript</a>
      <a href="../comparisons/wasm-vs-docker-kubernetes/">WASM vs Docker and Kubernetes</a>
      <a href="../core-model/what-is-wasm-mcp/">What is WASM MCP</a>
    </div>
  </section>
</div>
