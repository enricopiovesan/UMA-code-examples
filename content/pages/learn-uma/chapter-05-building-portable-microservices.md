---
ref: chapter-05-building-portable-microservices
title: "Chapter 5: Building UMA Services"
subtitle: "What it takes to build a service that is genuinely portable rather than just framework-independent."
macro_area: learn-uma
content_type: overview
slug: chapter-05-building-portable-microservices
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-05-building-portable-microservices/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "Three requirements for genuine portability: no hidden deps, declared contract, parity proof."
breadcrumbs:
  - "Home"
  - "Learn UMA"
  - "Chapter 5: Building UMA Services"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
  <h1>Chapter 5: Building UMA Services</h1>
  <p>Framework independence is not portability. A service is genuinely portable only when it has no hidden runtime dependencies, carries a machine-readable contract, and can be verified across at least two environments.</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>What genuine portability requires</h2>
    <p>Most services that claim to be portable are framework-independent — they don't lock to a specific web framework or cloud SDK. That's a necessary condition, not a sufficient one. A service can be framework-independent and still make implicit assumptions about the filesystem, the clock, the network, or the environment variables available at startup. Those assumptions are hidden runtime dependencies. They don't show up in the interface contract, but they break portability the moment the service runs somewhere those assumptions don't hold. Building a genuinely portable service requires making every dependency explicit, declaring the contract in a form the runtime can validate, and proving behavioral equivalence across environments rather than asserting it.</p>
  </section>

  <section>
    <h2>Three requirements for a portable service</h2>
    <p>The chapter builds a concrete example — a feature flag evaluator — and uses it to demonstrate each requirement in turn.</p>
    <p>The first requirement is <strong>no hidden runtime dependencies</strong> in the core logic. The portable service must be a pure function of its inputs: given the same input, it produces the same output regardless of where it runs. Any dependency on external state — environment variables, filesystem paths, ambient credentials — must be injected through the contract interface, not accessed directly. The chapter shows how to audit a service for hidden dependencies and what the refactoring pattern looks like when they're found.</p>
    <p>The second requirement is a <strong>machine-readable descriptor</strong>. The contract has to be declared in a structured form the runtime can parse and act on. This means inputs and outputs are typed and named, not inferred from code. Placement constraints and trust requirements are explicit fields, not comments. The descriptor is what separates a portable service from a portable library — the runtime can reason about a service without executing it.</p>
    <p>The third requirement is a <strong>parity proof across at least two environments</strong>. Portability is a testable claim. The chapter shows how to construct a behavioral equivalence test that runs the same inputs through the native implementation and the WASM-compiled version, compares outputs deterministically, and fails the build if they diverge. This is the proof that makes portability falsifiable rather than aspirational.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 4 establishes why metadata-driven contracts are the architectural unit that enables portable orchestration. Chapter 6 takes the portability proof further: how to verify behavioral equivalence across runtimes as a first-class CI artifact.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-04-from-soa-to-metadata-driven-services/">Chapter 4: From SOA to Metadata</a>
      <a href="/how-uma-works/portable-business-logic/">Portable business logic</a>
      <a href="/proof/what-makes-a-service-portable/">What makes a service portable?</a>
      <a href="/examples/chapter-04-feature-flag-evaluator/">Chapter 4 feature flag example</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Buy the book on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
