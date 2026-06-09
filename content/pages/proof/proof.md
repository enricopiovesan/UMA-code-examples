---
ref: proof
title: "Proof"
subtitle: "The evidence surface for UMA: portability, benchmarks, and footprint."
macro_area: proof
content_type: hub
slug: proof
canonical_url: "https://www.universalmicroservices.com/proof/"
left_nav_group: proof
chapter_ref: null
seo_description: "UMA portability proof: methodology, benchmark measurements, and service portability examples verifying runtime-agnostic distributed systems."
breadcrumbs:
  - "Home"
  - "Proof"
related_refs:
  - what-makes-a-service-portable
  - how-to-prove-portability
  - benchmark-and-footprint
---

## intro

<section class="subpage-hero">
  <h1>Proof</h1>
  <p>
    UMA needs evidence, not only language. This area collects the material that shows portability, behavior size, and runtime tradeoffs
    in a way readers can inspect and compare.
  </p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>What this macro area covers</h2>
    <p>
      These pages show how UMA turns abstract claims into measurable outputs: what is portable, how portability is demonstrated, and
      what the performance and footprint tradeoffs look like in practice.
    </p>
    <p>
      Portability is a claim. This area is about how to make that claim inspectable instead of leaving it as an assumption. The three pages here cover different kinds of proof: what makes a service genuinely portable by design, how to verify that portability holds across runtime environments, and what benchmark and footprint data actually looks like for the UMA code examples.
    </p>
    <p>
      The underlying principle is that proof should be observable. Not "we designed it to be portable" but "here is the same service producing the same output in two different runtimes with the specific differences in the surrounding runtime behavior made explicit." That is the standard the examples in this repository are held to.
    </p>
    <p>
      Architecture claims that cannot be inspected tend to drift toward assumptions, and assumptions compound. The proof section exists so the UMA portability claim stays falsifiable: something you can run, compare, and verify before deciding whether the model applies to your system.
    </p>
    <div class="subpage-inline-links">
      <a href="../../evolve-uma/">Continue to: System Evolution →</a>
    </div>
  </section>

  <section>
    <h2>What "portable" means as a testable claim</h2>
    <p>
      Portable is not a design goal in the UMA sense. It is a testable property of a specific artifact running in a specific set of environments. The test is not whether the code was written with portability in mind, or whether it avoids runtime-specific APIs, or whether the team intends to support multiple runtimes. The test is whether the same compiled binary, given the same inputs, produces the same outputs across each target runtime, with the runtime-introduced differences made explicit.
    </p>
    <p>
      That last clause matters. "Same outputs" does not mean the runtime is invisible. It means the business logic output is stable while runtime metadata (execution timing, host context, instrumentation) may vary by design. A portable service is one where you can distinguish which differences come from the logic and which come from the runtime. When you cannot make that distinction, you do not have portability. You have a black box that happens to work in multiple environments.
    </p>
    <h2>What the benchmark measures</h2>
    <p>
      The benchmark used in this repository runs the same WASM binary across three runtimes: a server-side managed runtime, a local CLI host, and a browser-based execution context. Each run receives identical inputs. The outputs are compared structurally, not as raw bytes. The comparison accounts for runtime-injected fields and normalizes them before asserting equivalence of the business logic result.
    </p>
    <p>
      What the benchmark is not measuring: raw throughput, latency percentiles, or operational efficiency. Those are runtime-dependent by definition and vary with infrastructure, not with the service logic. A benchmark that optimizes for those numbers is measuring the runtime, not the service. The UMA benchmark measures whether the portable unit stays behaviorally stable as the execution context changes. The footprint data (binary size, memory at load, startup cost) is included because those numbers determine which runtimes the service is viable in, not just which runtimes it technically runs on.
    </p>
    <h2>Portability inspectable vs portability assumed</h2>
    <p>
      Most portability claims are architectural. The system was designed to be portable. The interfaces are abstract. The dependencies are injected. These are inputs to portability, not evidence of it. They tell you the team intended the service to be moveable, not that it actually moves.
    </p>
    <p>
      Inspectable portability requires a different kind of artifact: a test harness that runs the same binary in multiple environments and records the comparison, a descriptor that declares the runtime requirements so mismatches surface before deployment, and a build process that produces deterministic output so the binary at test time is the binary at production time. The UMA code examples include all three. The portability claim is not inferred from the design. It is re-verified on every build.
    </p>
    <h2>Runtime-agnostic vs cross-platform</h2>
    <p>
      Runtime-agnostic and cross-platform are often used interchangeably, but they describe different properties. Cross-platform means the software can be compiled or interpreted for different operating systems or hardware architectures. The code is the same. The binary may differ. Runtime-agnostic means the service logic does not depend on any specific execution host. It can be loaded and run by any runtime that conforms to the required interface, without recompilation.
    </p>
    <p>
      WASM is the mechanism that makes runtime-agnostic a concrete property rather than an aspirational one. A WASM binary compiled from a UMA service module does not change between runtimes. The runtime changes around it. The adapter layer (the code that connects the binary to the specific host API) is what varies, and that variation is bounded by the contract. This is what makes the portability claim inspectable: the binary is the constant, and the adapter is the explicit variable. You can read the adapter, test it independently, and verify that it only converts host-specific inputs into the contract interface. It does not change what the service does.
    </p>
  </section>

  <section>
    <h2>Pages in this area</h2>
    <div class="subpage-grid">
      <article class="subpage-card"><h3><a href="what-makes-a-service-portable/">What Makes a Service Portable?</a></h3><p>The architectural qualities that make a service viable across runtimes.</p></article>
      <article class="subpage-card"><h3><a href="how-to-prove-portability/">How to Prove Portability</a></h3><p>The verification path used to make portability concrete.</p></article>
      <article class="subpage-card"><h3><a href="benchmark-and-footprint/">UMA Benchmark And Footprint Notes</a></h3><p>What the benchmark and footprint evidence tells us.</p></article>
    </div>
  </section>
</div>
