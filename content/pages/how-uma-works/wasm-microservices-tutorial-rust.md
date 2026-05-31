---
ref: wasm-microservices-tutorial-rust
title: "WASM Microservices Tutorial: Rust"
subtitle: "Build a portable WebAssembly microservice in Rust using the UMA pattern. This tutorial walks through the Feature Flag Evaluator from Chapter 4: pure service core, WASI adapter, contract definition, and runtime validation."
macro_area: how-uma-works
content_type: tutorial
slug: wasm-microservices-tutorial-rust
canonical_url: "https://www.universalmicroservices.com/how-uma-works/wasm-microservices-tutorial-rust/"
left_nav_group: how-uma-works
chapter_ref: chapter-04-feature-flag-evaluator
seo_description: "Step-by-step tutorial: build a portable WASM microservice in Rust with WASI. Based on the UMA Feature Flag Evaluator from Chapter 4 of the UMA book."
breadcrumbs:
  - "Home"
  - "How UMA Works"
  - "WASM Microservices Tutorial: Rust"
related_refs:
  - webassembly-microservices-architecture
  - active-descriptors
---

## intro

<section class="subpage-hero">
  <h1>WASM Microservices Tutorial: Rust</h1>
  <p>Build a portable WebAssembly microservice in Rust using the UMA pattern. This tutorial walks through the Feature Flag Evaluator from Chapter 4: pure service core, WASI adapter, contract definition, and runtime validation.</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>What you will build</h2>
    <p>The Feature Flag Evaluator is a service that decides whether a named flag is enabled for a given user context. It reads a flag definition and a context object, evaluates rules in first-match-wins order, and returns a deterministic decision: which flag, whether it is enabled, and which rule matched.</p>
    <p>The evaluator core is written in pure Rust with no I/O dependencies. It compiles to a single <code>.wasm</code> binary. That binary runs via <code>wasmtime</code> on the command line, in a browser via a JavaScript WASI polyfill, and in a Node-based edge or cloud adapter — same binary, same output, across every host. The core never changes between deployments. Only the adapter changes.</p>
    <p>This is the UMA portability claim made concrete: business logic that is deterministic by construction, not by convention. The rollout hash, the rule language, and the default fallback all live inside the portable core. No adapter can deviate from them.</p>
  </section>

  <section>
    <h2>Prerequisites</h2>
    <ul>
      <li>Rust toolchain via <a href="https://rustup.rs/">rustup</a>. Verify with <code>cargo --version</code>.</li>
      <li>The <code>wasm32-wasip1</code> compilation target: <code>rustup target add wasm32-wasip1</code></li>
      <li><code>wasmtime</code> on your <code>PATH</code>. On macOS: <code>brew install wasmtime</code>. On Linux: download a release from the <a href="https://github.com/bytecodealliance/wasmtime/releases">Wasmtime releases page</a> and add it to your <code>PATH</code>. Verify with <code>wasmtime --version</code>.</li>
      <li><code>jq</code> on your <code>PATH</code> (used by the lab scripts for output comparison).</li>
      <li>Node.js 20 or newer and <code>npm</code> for the TypeScript parity path. Verify with <code>node --version</code>.</li>
    </ul>
    <p>The repo also ships a pinned <code>wasmtime</code> copy at <code>.bin/</code> used by CI. If <code>wasmtime</code> is not on your global <code>PATH</code>, the scripts will fall back to that copy.</p>
  </section>

  <section>
    <h2>Project structure</h2>
    <p>The chapter directory follows the canonical UMA service anatomy. Every directory has a single, non-negotiable responsibility:</p>
    <ul>
      <li><strong><code>core/</code></strong> — pure business logic. A Rust library crate (<code>ff_eval_core</code>) with no dependencies beyond the standard library. This is the only code that gets compiled to WASM. It has no I/O, no network access, no filesystem access. Deterministic by construction.</li>
      <li><strong><code>contracts/</code></strong> — typed boundary. JSON Schema files (<code>input.schema.json</code>, <code>output.schema.json</code>) that define what the evaluator accepts and what it promises to return. The contract is the stable surface; everything else can change.</li>
      <li><strong><code>adapters/</code></strong> — runtime translation. Thin host-specific wrappers that know how to feed input to the WASM module and read its output. Three adapters are provided: <code>browser/</code> (JavaScript WASI polyfill), <code>edge/</code> (Node-based Cloudflare-style worker), <code>cloud/</code> (Node-based Lambda-style handler). Each adapter is less than 100 lines. None owns any rule semantics.</li>
      <li><strong><code>wasi-app/</code></strong> — the WASI CLI host. A second Rust crate (<code>ff_eval_wasi_app</code>) that reads JSON from stdin, calls the core evaluation function, and writes JSON to stdout. This is the binary that compiles to <code>ff_eval_wasi_app.wasm</code>.</li>
    </ul>
    <p>The separation matters because it is the architectural proof. If rule semantics ever leaked into an adapter, the TypeScript parity tests would diverge. The contract enforces the boundary; the directory structure makes the boundary visible.</p>
  </section>

  <section>
    <h2>The service core</h2>
    <p>All evaluation logic lives in <code>core/src/lib.rs</code>. The rule language supports <code>==</code>, <code>!=</code>, <code>&lt;</code>, <code>&lt;=</code>, <code>&gt;</code>, <code>&gt;=</code>, <code>in</code>, <code>&amp;&amp;</code>, and <code>||</code>. Identifiers resolve from the context object. String and numeric literals are supported. A built-in <code>rollout(p)</code> function performs a deterministic FNV-1a hash of <code>flag.key + ":" + context.userId</code> and returns true if the hash normalized to [0, 1) is less than <code>p</code>.</p>
    <p>Rollout decisions are sticky: the same flag key and user ID always produce the same bucket. This is not a runtime property — it is baked into the hash algorithm in the core. No adapter can change it. No deployment configuration can change it. That stickiness guarantee is what makes the rollout trustworthy at scale.</p>
    <p>The core has no <code>Cargo.toml</code> dependencies beyond the standard library. No <code>serde</code>, no <code>tokio</code>, no runtime. This is deliberate. Every dependency added to the core is a dependency that must compile to WASM, a potential source of non-determinism, and a reason for binary size to grow. The WASI app crate is where <code>serde</code> and <code>serde_json</code> live — on the boundary, not in the logic.</p>
  </section>

  <section>
    <h2>Building and running</h2>
    <p>The validated Chapter 4 reader path is two commands:</p>
    <pre><code>cargo test --locked -p ff_eval_core
./scripts/smoke_flag_labs.sh</code></pre>
    <p>The first command compiles the core library and runs its unit test suite. Tests cover equality comparisons, rollout bucketing, the <code>in</code> operator, numeric comparisons, logical operators, and the default fallback. All tests run on the native host — no WASM runtime required at this step.</p>
    <p>The smoke script is the full acceptance path. It builds the WASI evaluator (<code>cargo build --release --target wasm32-wasip1 -p ff_eval_wasi_app</code>), runs the Rust unit tests, installs the TypeScript parity implementation dependencies, runs the TypeScript parity tests, executes the JSON vector suite, and compares Rust and TypeScript outputs across all four guided labs. If the smoke gate passes, the chapter is verified end-to-end.</p>
    <p>To build the WASM binary directly:</p>
    <pre><code>cargo build --release --target wasm32-wasip1 -p ff_eval_wasi_app</code></pre>
    <p>The compiled module is written to <code>target/wasm32-wasip1/release/ff_eval_wasi_app.wasm</code>.</p>
  </section>

  <section>
    <h2>Running the labs</h2>
    <p>Four labs exercise progressively richer evaluation scenarios. Run them in order:</p>
    <pre><code>./scripts/run_lab.sh lab1-country-match
./scripts/run_lab.sh lab2-rollout-match
./scripts/run_lab.sh lab3-default-fallback
./scripts/run_lab.sh lab4-rule-language</code></pre>
    <ul>
      <li><strong>lab1-country-match</strong> — a direct equality rule: <code>country == 'CA'</code>. First-match-wins returns rule 0. The simplest possible evaluation path.</li>
      <li><strong>lab2-rollout-match</strong> — the <code>rollout(0.20)</code> function. The country rule misses; the rollout rule hits for this specific user ID. Demonstrates sticky bucketing.</li>
      <li><strong>lab3-default-fallback</strong> — no rule matches. The evaluator returns the flag's <code>default</code> value. <code>matchedRule</code> is null.</li>
      <li><strong>lab4-rule-language</strong> — demonstrates the <code>in</code> operator, numeric comparison, and logical <code>&amp;&amp;</code> and <code>||</code> in a single flag definition.</li>
    </ul>
    <p>Every lab produces the same output whether run via the Rust WASI binary, the TypeScript implementation, or fed directly through <code>wasmtime</code>. That invariant is what the labs are designed to surface — not the output itself, but the fact that the output is identical regardless of host.</p>
    <p>To see the available labs: <code>./scripts/list_labs.sh</code></p>
  </section>

  <section>
    <h2>The TypeScript parity path</h2>
    <p>A parallel TypeScript implementation lives in the <code>ts/</code> directory. It implements the same contract, the same rule language semantics, and the same FNV-1a rollout hash. It is maintained as a parity implementation, not as a reference. The Rust WASI binary is the validated default path.</p>
    <p>To run a lab against the TypeScript implementation:</p>
    <pre><code>./scripts/run_lab.sh --impl ts lab2-rollout-match</code></pre>
    <p>To prove behavioral equivalence across all labs:</p>
    <pre><code>./scripts/compare_impls.sh</code></pre>
    <p>This script runs every lab through both implementations and diffs the outputs. If they diverge, the parity contract is broken and the diff shows exactly where. If they match, you have a cross-language proof that the contract is the behavior — not the implementation.</p>
    <p>This is the portability proof. Two implementations in different languages, compiled independently, producing bit-for-bit identical outputs because the contract defines the semantics and neither implementation owns them.</p>
  </section>

  <section>
    <h2>What this demonstrates</h2>
    <p>The WASM binary compiled from <code>core/</code> and <code>wasi-app/</code> runs identically in three contexts without recompilation:</p>
    <ul>
      <li><strong>wasmtime CLI</strong> — pipe JSON to stdin, read JSON from stdout. The host is a native process. Full WASI 0.1 capability surface.</li>
      <li><strong>Browser</strong> — the <code>adapters/browser/ff.js</code> polyfill provides a minimal WASI implementation in JavaScript. The same <code>.wasm</code> file fetched over HTTP, stdin and stdout wired through JavaScript buffers.</li>
      <li><strong>Edge or cloud</strong> — Node's built-in <code>wasi</code> module instantiates the module in a Cloudflare Worker-style or Lambda-style handler. The adapter is the only code that knows it is running in Node.</li>
    </ul>
    <p>In every case the core is identical. The rule evaluation, the rollout hash, and the output schema do not change. What changes is the thin adapter layer: how input arrives, how output is returned, and which WASI capabilities the host provides.</p>
    <p>This is the architectural argument for UMA in practice. The durable behavior — the logic worth maintaining, testing, and versioning — lives in a single artifact. The runtime context is a deployment detail. Changing where the service runs does not require changing what the service does.</p>
  </section>

  <section class="subpage-grid">
    <article class="subpage-card">
      <h3>Rust core</h3>
      <p>A library crate with no I/O dependencies. Implements the rule language, the FNV-1a rollout hash, and the first-match-wins evaluation loop. Compiles to WASM. Has no knowledge of stdin, stdout, HTTP, or any host capability. Tested on the native host via <code>cargo test</code>.</p>
    </article>
    <article class="subpage-card">
      <h3>WASM host</h3>
      <p>Any process that can run a WASI module: wasmtime on the command line, Node's built-in <code>wasi</code> module in an edge worker, a JavaScript WASI polyfill in a browser. The host provides the I/O surface — stdin, stdout, clocks — and nothing else. It does not interpret flag rules. It does not own rollout semantics.</p>
    </article>
  </section>

  <section class="subpage-callout">
    <strong>Source code and further reading</strong>
    <p>The complete Chapter 4 source is in the <a href="https://github.com/enricopiovesan/UMA-code-examples">UMA code examples repository</a> under <code>chapter-04-feature-flag-evaluator/</code>. For the architectural context behind the WASM execution model and the WASI system interface, see the <a href="../webassembly-microservices-architecture/">WebAssembly Microservices Architecture</a> page.</p>
    <div class="subpage-inline-links">
      <a href="https://github.com/enricopiovesan/UMA-code-examples">GitHub: UMA Code Examples</a>
      <a href="../webassembly-microservices-architecture/">WebAssembly Microservices Architecture</a>
      <a href="../wasm-microservices-tutorial-typescript/">WASM Microservices Tutorial: TypeScript</a>
      <a href="../../proof/what-makes-a-service-portable/">What makes a service portable?</a>
      <a href="../../learn-uma/chapter-05-building-portable-microservices/">Chapter 5: building portable microservices</a>
    </div>
  </section>
</div>
