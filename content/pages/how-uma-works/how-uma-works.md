---
ref: how-uma-works
title: "How UMA Works"
subtitle: "The operating model that keeps services portable, governed, and evolvable without fragmenting the system."
macro_area: how-uma-works
content_type: hub
slug: how-uma-works
canonical_url: "https://www.universalmicroservices.com/how-uma-works/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "See how UMA keeps services portable while still making runtime placement, portability, adoption, and readiness explicit."
breadcrumbs:
  - "Home"
  - "How UMA Works"
related_refs:
  - runtime-agnostic-architecture
  - portable-business-logic
  - architecture-drift-and-portable-business-logic
  - webassembly-architecture
  - migrating-to-uma-incrementally
  - incremental-uma-adoption
  - uma-production-readiness
  - uma-mcp-runtime-governance
---

## intro

<section class="subpage-hero">
  <h1>How UMA Works</h1>
  <p>
    UMA is not just a vocabulary. It is a way to organize behavior so it can move across execution contexts without turning into a
    different system each time it moves.
  </p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>What this macro area covers</h2>
    <p>
      These pages show how UMA connects portability, runtime placement, adoption, and readiness into a single operating model for
      software that must keep working as it grows.
    </p>
    <p>
      Understanding the UMA model is one thing. Seeing how it works in a real system is another. This area covers the practical side: what runtime-agnostic architecture looks like in code, how portable business logic is structured so it can survive outside its original host, how WebAssembly provides a concrete execution boundary, and how teams start adopting UMA without rewriting everything at once.
    </p>
    <p>
      The most common entry point is not a full adoption. It is one behavior that is duplicated across two runtimes. The practical first step is to extract that behavior behind an explicit contract and prove it produces identical outputs in both environments. That proof is the foundation for the rest. It makes portability an inspectable claim instead of an assumption.
    </p>
    <p>
      This area also covers architecture drift — what happens when behavior duplication goes unmanaged — and what production readiness looks like when a UMA approach is being evaluated for a real system. These are not theoretical concerns. They are the questions teams ask before committing to any new architectural model.
    </p>
    <div class="subpage-inline-links">
      <a href="../../learn-uma/">Continue to: Learn UMA →</a>
    </div>
  </section>

  <section>
    <h2>From source to execution: the actual mechanism</h2>
    <p>
      A UMA service starts as source code in any language that compiles to WebAssembly. The developer writes business logic, not runtime integration code. There are no SDK calls to a specific host, no imports of runtime-specific libraries, no assumptions about how the function will be invoked. The service module defines its interface — what it accepts and what it returns — through a typed contract that is specified separately from the implementation.
    </p>
    <p>
      Compilation produces a WASM binary. That binary is the portable artifact. It does not contain host bindings. It cannot call arbitrary system APIs. The execution environment it will run in is entirely outside its scope — which is the point. The binary is a sealed behavior unit. Given the same inputs, it will produce the same outputs regardless of what is running around it.
    </p>
    <h2>The descriptor layer</h2>
    <p>
      Alongside the binary, the build process produces a service descriptor. The descriptor is machine-readable metadata that specifies the contract: what types the service accepts, what it returns, what trust conditions it requires, what runtime capabilities it needs (memory bounds, execution time limits, I/O permissions). The descriptor is not documentation — it is the specification the runtime reads before loading the binary.
    </p>
    <p>
      This is the layer that most architectural descriptions of UMA skip over, and it is the load-bearing one. The descriptor is what allows the runtime to make deployment decisions without executing the service. It is what allows the service graph to be validated for compatibility before any service runs. It is what allows trust policies to be applied at load time rather than inferred from runtime behavior. Without the descriptor, WASM portability is just a compilation target. With it, portability becomes a governed property of a system.
    </p>
    <h2>The adapter: controlled surface between binary and host</h2>
    <p>
      The runtime cannot call the WASM binary directly using arbitrary host conventions. The binary expects inputs in the contract's type format. The host has its own invocation model — HTTP request objects, message queue payloads, event structures — that does not match the contract type format. The adapter is the code that converts between them.
    </p>
    <p>
      Adapters are runtime-specific and thin by design. A server-side HTTP adapter converts an incoming request into the input type the contract specifies, invokes the binary, and converts the output type back into an HTTP response. A browser adapter does the same for browser event inputs. The adapter does not contain business logic. It cannot — any business logic in the adapter is logic that the portable binary does not own, which means it is logic that will have to be duplicated or diverge when the service moves to a different runtime.
    </p>
    <p>
      The boundary between adapter and binary is the actual runtime boundary in UMA. Everything inside the binary is portable. Everything inside the adapter is host-specific. Making that boundary explicit and enforced is what distinguishes UMA from an architecture where "portable" is an intention rather than a constraint.
    </p>
    <h2>Why the separation is the load-bearing decision</h2>
    <p>
      The compile → descriptor → adapter → runtime chain is not an implementation detail. It is the reason the system's other properties hold. Portability holds because the binary has no host dependencies. Inspectability holds because the descriptor makes requirements explicit before execution. Governance holds because the runtime reads the descriptor and enforces the contract at load time, not at design time.
    </p>
    <p>
      When teams adopt frameworks that blur this separation — embedding runtime-specific calls in business logic, encoding contract requirements as runtime configuration rather than service-level descriptors, letting adapters grow to include logic — they get a system that runs in multiple environments but is not actually portable. Moving the service requires changing the service. The test for whether the separation is real is simple: can you swap the adapter for a different runtime without touching the binary or the descriptor? If yes, the boundary is where UMA requires it to be. If no, the boundary has drifted into the logic.
    </p>
  </section>

  <section>
    <h2>Pages in this area</h2>
    <div class="subpage-grid">
      <article class="subpage-card"><h3><a href="runtime-agnostic-architecture/">Runtime-agnostic Architecture</a></h3><p>The architectural stance that keeps behavior portable across runtimes.</p></article>
      <article class="subpage-card"><h3><a href="portable-business-logic/">Portable Business Logic</a></h3><p>What stays inside the capability boundary when the runtime changes.</p></article>
      <article class="subpage-card"><h3><a href="architecture-drift-and-portable-business-logic/">Architecture Drift and Portable Business Logic</a></h3><p>Why portability matters when teams and runtimes change over time.</p></article>
      <article class="subpage-card"><h3><a href="webassembly-architecture/">WebAssembly Architecture and UMA</a></h3><p>The place where UMA and WASM meet in the site narrative.</p></article>
      <article class="subpage-card"><h3><a href="migrating-to-uma-incrementally/">Migrating to UMA Incrementally</a></h3><p>How to move toward UMA without a big-bang rewrite.</p></article>
      <article class="subpage-card"><h3><a href="incremental-uma-adoption/">Incremental UMA Adoption</a></h3><p>Adoption patterns that keep the transition measurable and reversible.</p></article>
      <article class="subpage-card"><h3><a href="uma-production-readiness/">UMA Production Readiness</a></h3><p>What needs to be true before the model is ready for production use.</p></article>
      <article class="subpage-card"><h3><a href="uma-mcp-runtime-governance/">UMA Runtime Governance for MCP</a></h3><p>How contract validation, trust enforcement, and execution evidence apply to AI agent tool calls.</p></article>
    </div>
  </section>
</div>
