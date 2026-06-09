---
ref: wasm-microservices-tutorial-typescript
title: "WASM Microservices Tutorial: TypeScript"
subtitle: "Build a portable microservice runtime in TypeScript using the UMA pattern. This tutorial walks through the Post Fetcher Runtime from Chapter 5: contract enforcement, adapter binding, event ordering, and parity with the Rust implementation."
macro_area: how-uma-works
content_type: tutorial
slug: wasm-microservices-tutorial-typescript
canonical_url: "https://www.universalmicroservices.com/how-uma-works/wasm-microservices-tutorial-typescript/"
left_nav_group: how-uma-works
chapter_ref: chapter-05-post-fetcher-runtime
seo_description: "Build WASM microservices in TypeScript with the UMA Post Fetcher Runtime. Runtime design, contract enforcement, adapter binding, and Rust parity."
breadcrumbs:
  - "Home"
  - "How UMA Works"
  - "WASM Microservices Tutorial: TypeScript"
related_refs:
  - webassembly-microservices-architecture
  - wasm-microservices-tutorial-rust
  - active-descriptors
---

## intro

<section class="subpage-hero">
  <h1>WASM Microservices Tutorial: TypeScript</h1>
  <p>Build a portable microservice runtime in TypeScript using the UMA pattern. This tutorial walks through the Post Fetcher Runtime from Chapter 5: contract enforcement, adapter binding, event ordering, and parity with the Rust implementation.</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>What you will build</h2>
    <p>This tutorial covers building wasm microservices in TypeScript using the UMA runtime discipline. The Post Fetcher Runtime is a complete UMA example: a TypeScript runtime that hosts a pure service, enforces its contract, binds the <code>network.fetch</code> adapter, and records lifecycle metadata. It is Chapter 5 of the UMA book rendered as runnable code.</p>
    <p>By the end of this tutorial you will be able to:</p>
    <ul>
      <li>Explain what belongs in the service layer versus what belongs in the runtime layer</li>
      <li>Run the four guided labs and read the deterministic event log they produce</li>
      <li>Show how <code>compare_impls.sh</code> proves behavioral equivalence between the TypeScript and Rust runtimes</li>
      <li>Describe how the lifecycle record captures which adapter implementation actually ran</li>
    </ul>
    <p>The TypeScript runtime in <code>ts/</code> is a reference implementation kept in parity with the Rust workspace for the core Chapter 5 scenarios. It demonstrates that UMA runtime anatomy is a design discipline, not a language feature. The same contract, the same event ordering, the same adapter binding model. in TypeScript instead of Rust.</p>
  </section>

  <section>
    <h2>Prerequisites</h2>
    <ul>
      <li><strong>Rust 1.77 or newer</strong> plus <code>cargo</code>: the validated Chapter 5 path is Rust-first. the parity comparison requires both runtimes to run</li>
      <li><strong>Node.js 20+</strong> and <strong>npm</strong>: required for the TypeScript reference runtime in <code>ts/</code></li>
      <li><strong>jq</strong>: required for the guided lab checks and golden-fixture comparisons. the scripts fail fast with an explicit message if it is missing</li>
    </ul>
    <p>The browser and edge host sketches under <code>adapters/network/ts-host/</code> and <code>hosts/</code> are illustrative, not validated quick-start paths. You do not need any cloud credentials or external network access: the validated path resolves a checked-in fixture through the host adapter.</p>
  </section>

  <section>
    <h2>Project structure</h2>
    <p>The repository root is a Rust workspace. Each directory corresponds to a layer in the UMA runtime anatomy:</p>
    <ul>
      <li><strong><code>service/</code></strong>: pure normalization logic and service-facing API types. No I/O, no host dependency. This is the portable business logic that must produce identical results regardless of runtime.</li>
      <li><strong><code>contracts/</code></strong>: typed boundary definitions: the adapter capability contract (<code>adapter.network.contract.json</code>), the runtime policy (<code>policy.runtime.json</code>), and the lifecycle metadata schema (<code>metadata.schema.json</code>). These files are the declared contract, not generated artifacts.</li>
      <li><strong><code>adapters/</code></strong>: the <code>network.fetch</code> capability binding plus illustrative TypeScript and browser adapter scaffolding. The runtime selects which adapter implementation to use. the adapter directory defines what implementations exist.</li>
      <li><strong><code>hosts/</code></strong>: cloud and edge host shims. The cloud host is the validated path. The edge host is an illustrative sketch that fails fast with guidance rather than pretending it is turnkey.</li>
      <li><strong><code>ts/</code></strong>: the TypeScript reference runtime. Kept in parity with the Rust workspace for the core Chapter 5 scenarios. Tested with <code>npm test --prefix ts</code>.</li>
      <li><strong><code>runtime/</code></strong>: runtime orchestration, adapter binding, event bus, lifecycle record, and native CLI entrypoint. This is what the labs exercise.</li>
      <li><strong><code>scripts/</code></strong>: guided Chapter 5 lab helpers. Start here.</li>
      <li><strong><code>labs/</code></strong>: lab notes. See <code>labs/README.md</code> for the guided walkthrough.</li>
    </ul>
    <p>The separation between <code>service/</code> and <code>runtime/</code> is the central UMA discipline this chapter demonstrates. The service does not know which adapter ran. The runtime does not contain business logic. The contract layer in <code>contracts/</code> makes the boundary machine-readable.</p>
  </section>

  <section>
    <h2>The runtime's job</h2>
    <p>A UMA runtime has three responsibilities, all visible in the Chapter 5 output:</p>
    <ul>
      <li><strong>Adapter selection and recording.</strong> The runtime chooses which implementation satisfies the <code>network.fetch</code> capability and records that decision in the lifecycle binding. The <code>UMA_ENABLE_RETRY</code> and <code>UMA_ENABLE_CACHE</code> environment variables change the binding record from <code>host-fetch</code> to <code>cache-retry-host-fetch</code> without changing the normalized service output.</li>
      <li><strong>Fail-fast validation.</strong> The runtime validates the request before any side effect. An invalid <code>accept</code> header stops the run before the first <code>fetch_request</code> event is emitted. The service never sees the request.</li>
      <li><strong>Deterministic event ordering.</strong> The runtime emits events in a fixed sequence using a logical clock that increments once per event. The TypeScript runtime in <code>ts/</code> implements and preserves this same ordering, which is why <code>compare_impls.sh</code> can compare the two implementations against the same inputs.</li>
    </ul>
    <p>The <code>ts/</code> directory is a clean implementation of these three responsibilities in TypeScript. It is not a port of the Rust code. it is an independent implementation of the same runtime model. That independence is what the parity check proves.</p>
  </section>

  <section>
    <h2>Running the labs</h2>
    <p>Start by listing available labs:</p>
    <pre><code>./scripts/list_labs.sh</code></pre>
    <p>Then run each lab in order:</p>
    <div class="code-tabs">
      <div class="code-tab-bar">
        <button class="code-tab active" data-tab="ts">TypeScript</button>
        <button class="code-tab" data-tab="rust">Rust</button>
      </div>
      <div class="code-tab-panel active" data-panel="ts">
<pre><code># Install TypeScript runtime dependencies
cd ts &amp;&amp; npm install &amp;&amp; cd ..

# Run each lab (TypeScript reference runtime)
./scripts/run_lab.sh lab1-cloud-golden-path
./scripts/run_lab.sh lab2-header-validation-fail-fast
./scripts/run_lab.sh lab3-adapter-binding-and-wrappers
./scripts/run_lab.sh lab4-rust-ts-parity

# Run TypeScript unit tests separately
npm test --prefix ts</code></pre>
      </div>
      <div class="code-tab-panel" data-panel="rust">
<pre><code># Build the Rust workspace (required for parity comparison)
cargo build --release -p post_fetcher_runtime

# Run the smoke path (builds Rust, installs TS deps, runs all labs)
./scripts/smoke_runtime_labs.sh

# Prove behavioral equivalence between Rust and TypeScript runtimes
./scripts/compare_impls.sh

# Run Rust unit tests
cargo test --locked</code></pre>
      </div>
    </div>
    <p>What each lab exercises:</p>
    <ul>
      <li><strong>lab1-cloud-golden-path</strong>: runs the validated cloud host path and compares the output against the checked-in golden fixture. The expected log signal is <code>Integration test passed: output matches golden fixture.</code></li>
      <li><strong>lab2-header-validation-fail-fast</strong>: feeds an invalid <code>accept</code> header into the native CLI path and proves that validation stops the run before any <code>fetch_request</code> event is emitted. This is the clearest demonstration of fail-fast contract enforcement.</li>
      <li><strong>lab3-adapter-binding-and-wrappers</strong>: enables the retry and cache wrappers via environment variables and verifies that the runtime binding record changes to <code>cache-retry-host-fetch</code>. The normalized post output is identical to lab1. only the binding record changes.</li>
      <li><strong>lab4-rust-ts-parity</strong>: runs both the Rust and TypeScript runtimes against the validated Chapter 5 scenarios and compares their summarized runtime behavior. This is the parity proof.</li>
    </ul>
    <p>After lab3, you should be able to explain how the runtime validates input, chooses an adapter implementation, and records that decision without changing the pure service logic. That is the stated satisfaction point for Chapter 5.</p>
    <p>To run the full validated smoke path:</p>
    <pre><code>./scripts/smoke_runtime_labs.sh</code></pre>
    <p>Expected terminal signals: <code>Integration test passed: output matches golden fixture.</code> and <code>Chapter 5 smoke run completed successfully.</code></p>
  </section>

  <section>
    <h2>The parity proof</h2>
    <p>The parity check is what distinguishes Chapter 5 from a standard tutorial:</p>
    <pre><code>./scripts/compare_impls.sh</code></pre>
    <p>This script runs both the Rust workspace and the TypeScript reference runtime against the same validated Chapter 5 inputs and compares their summarized runtime behavior. Behavioral equivalence is the proof of portability. If two independently written runtimes (one in Rust, one in TypeScript) produce the same event log, the same binding record, and the same normalized output for the same inputs, then the portable behavior lives in the service and the contract, not in the runtime implementation.</p>
    <p>This is a property you can verify, not a claim you have to take on faith. The <code>compare_impls.sh</code> script makes the check reproducible and CI-friendly. The TypeScript runtime tests are separately verifiable with <code>npm test --prefix ts</code>.</p>
    <p>The runtime is deliberately deterministic: no timers or random values influence event ordering. The logical clock increments once per emitted event. These constraints are what make the comparison tractable.</p>
  </section>

  <section>
    <h2>What the TypeScript path demonstrates</h2>
    <p>A runtime does not have to be a WASM host to participate in the UMA model. The TypeScript runtime in <code>ts/</code> is a Node.js process. It hosts the same pure service logic, enforces the same contract, and produces the same observable behavior. The portable behavior lives in the service and the contract. the runtime is an adapter to a different host ecosystem.</p>
    <p>This has a practical implication: the UMA discipline applies at the architecture level, not at the WASM toolchain level. Teams working in TypeScript ecosystems can adopt the runtime model (contract enforcement, adapter binding, deterministic event ordering, lifecycle recording) without compiling anything to WASM. The WASM compilation step adds the strongest portability guarantee (run anywhere without recompilation), but the architecture discipline delivers value independently of that step.</p>
    <p>Same contract. Different host. Provably equivalent behavior. That is the point of <code>ts/</code>.</p>
  </section>

  <section>
    <h2>Why TypeScript for WASM microservices</h2>
    <p>Most WebAssembly tutorials default to Rust because Rust has the most complete WASM toolchain, the smallest output binaries, and the best WASI 0.2 support. But TypeScript teams do not need to switch languages to adopt the WASM microservices architecture discipline. The UMA model separates the portable service layer from the runtime adapter layer. That separation applies regardless of whether the service core compiles to a <code>.wasm</code> binary or runs as a Node.js module.</p>
    <p>What the TypeScript path demonstrates is that the architecture discipline (not the WASM binary format) is the core value. A TypeScript microservice that enforces its contract at the boundary, records adapter decisions in a lifecycle log, and validates inputs before emitting side effects is architecturally equivalent to a Rust WASM module. The portable behavior lives in the service contract. The runtime is an implementation detail.</p>
    <p>For teams that do need to cross runtime boundaries (running the same logic in a browser and on a server) the step from this TypeScript runtime to a compiled WASM binary is additive. The contract, the adapter model, and the event ordering discipline transfer directly. You are not rewriting. you are compiling the service core to a different target.</p>
  </section>

  <section>
    <h2>Contract enforcement in practice</h2>
    <p>The contract layer in <code>contracts/</code> is the most important output of this tutorial. Three files define the boundary:</p>
    <ul>
      <li><strong><code>adapter.network.contract.json</code></strong>: specifies what the <code>network.fetch</code> capability must accept and return. Any adapter implementation that satisfies this contract is interchangeable. The runtime does not care whether the adapter makes a real HTTP request, hits a fixture, or invokes a cache. It cares that the adapter returns a response that matches the schema.</li>
      <li><strong><code>policy.runtime.json</code></strong>: declares which adapter implementations are allowed, what retry and cache wrappers are permitted, and what environment variables can change the binding. This is the governance layer. It makes the adapter selection rules explicit rather than implicit in application code.</li>
      <li><strong><code>metadata.schema.json</code></strong>: defines the shape of the lifecycle record that every run produces. The record captures which adapter was selected, which wrappers were applied, and the event sequence. This is the audit surface.</li>
    </ul>
    <p>These contracts are not generated from code. They are the specification that both the Rust and TypeScript implementations must satisfy. The <code>compare_impls.sh</code> script verifies that they do. A drift between the two implementations shows up as a diff, not as a runtime surprise in production.</p>
  </section>

  <section>
    <h2>Next steps</h2>
    <p>After completing this tutorial:</p>
    <ul>
      <li><strong>Compare with the Rust runtime</strong>: the Rust workspace is the validated default path for Chapter 5. Reading both implementations side by side clarifies which choices are language-specific and which are architecture-level constraints. See the <a href="../wasm-microservices-tutorial-rust/">WASM Microservices Tutorial: Rust</a>.</li>
      <li><strong>Chapter 6: UMA Portability Lab</strong>: the next chapter takes the same portable service and runs it across multiple host environments in a single lab, making the portability guarantee concrete rather than theoretical.</li>
      <li><strong>Active Descriptors</strong>: the contract layer in <code>contracts/</code> is a simplified version of the full active descriptor model. See the <a href="../../active-descriptors/">Active Descriptors</a> page for the complete governance and placement model that builds on this foundation.</li>
    </ul>
  </section>

  <section class="subpage-grid">
    <article class="subpage-card">
      <h3>TypeScript runtime role</h3>
      <p>The <code>ts/</code> runtime enforces the same three responsibilities as the Rust runtime: adapter selection and recording, fail-fast validation before side effects, and deterministic event ordering via a logical clock. It is an independent implementation of the runtime model, not a port. Its existence proves the model is a design discipline, not a Rust API.</p>
    </article>
    <article class="subpage-card">
      <h3>Portability proof</h3>
      <p><code>./scripts/compare_impls.sh</code> runs both runtimes against the same inputs and compares their outputs. Behavioral equivalence between two independent implementations (one in Rust, one in TypeScript) is machine-verifiable evidence that the portable behavior lives in the service and the contract. The runtime is an adapter. the contract is the constant.</p>
    </article>
  </section>

  <section class="subpage-callout">
    <strong>Source code and Rust tutorial</strong>
    <p>The full Chapter 5 source code, including the TypeScript reference runtime, all lab scripts, and the golden fixtures, is on GitHub. The Rust tutorial covers the same chapter from the validated Rust-first path.</p>
    <div class="subpage-inline-links">
      <a href="https://github.com/enricopiovesan/UMA-code-examples">GitHub: UMA Code Examples</a>
      <a href="../wasm-microservices-tutorial-rust/">WASM Microservices Tutorial: Rust</a>
      <a href="../../proof/what-makes-a-service-portable/">What makes a service portable?</a>
      <a href="../../learn-uma/chapter-07-webassembly-portability-wasm-runtimes/">Chapter 7: WebAssembly portability and WASM runtimes</a>
    </div>
  </section>
</div>
