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
    <h2>The three requirements for genuine portability</h2>
    <p>The first requirement is the absence of hidden runtime dependencies. This means more than avoiding framework lock-in. It means the service must not reach outside its declared interface for anything: no direct filesystem access, no environment variable reads in business logic, no ambient network calls, no assumptions about which secrets are available at startup. Every external dependency the service needs must enter through the contract interface as an explicit input. The practical test is whether the service can be run in a sandbox that provides nothing except what the descriptor declares — and still produce correct outputs. If it can't, it has hidden dependencies, and those dependencies will break portability on the first execution surface that doesn't replicate the original host's environment.</p>
    <p>The second requirement is a machine-readable contract that exists as a computable artifact, not prose documentation. Prose documentation describes what the service does for a human reader. A machine-readable descriptor tells the runtime what the service requires before it executes. These are different things with different consequences. A runtime that reads a descriptor can validate inputs before execution, enforce placement constraints, and refuse to proceed if trust requirements aren't met. A runtime that reads prose documentation can do none of those things automatically. The descriptor isn't a nice-to-have — it's the artifact that makes governance automatable rather than manual.</p>
    <p>The third requirement is a parity proof: a CI-enforced test that runs the same inputs through the service in at least two structurally different runtimes and asserts behavioral equivalence. "Structurally different" is important here — running the same binary in two slightly different configurations of the same runtime doesn't prove cross-environment portability. The parity proof typically runs the native implementation alongside the WASM-compiled version, using the same fixture set, and fails the build if outputs diverge. This is the requirement that makes portability a falsifiable claim rather than an architectural aspiration. Without it, portability is asserted. With it, portability is tested, and the test result is a build artifact that can be audited.</p>
  </section>

  <section>
    <h2>What the chapter builds</h2>
    <p>The feature flag evaluator introduced in Chapter 4 as a narrative example becomes the hands-on subject of Chapter 5. The evaluator takes a user context and a flag configuration as inputs and returns a boolean with a confidence annotation. It's a deliberately simple rule — but simple enough to build completely, which is what makes it useful as a teaching example. The chapter takes it through the full UMA treatment: Rust implementation, WASM compilation, active descriptor authoring, and parity proof construction.</p>
    <p>The Rust implementation is written to satisfy the first requirement: no hidden dependencies in the core logic. The chapter shows what that process looks like when you start from a working service that does have hidden dependencies — reading flag configurations from a file path embedded in the code, for instance — and refactor it to accept those configurations as explicit inputs. This is not a trivial refactor in real systems, and the chapter doesn't pretend it is. It walks through the dependency audit, identifies every assumption the code makes about its environment, and shows the interface changes required to make those assumptions explicit.</p>
    <p>The active descriptor is authored alongside the code, not generated from it. The chapter explains why: a generated descriptor reflects what the code does; an authored descriptor reflects what the service intends to guarantee. Those are different things. The authored descriptor is the contract; the code is the implementation. The parity proof is the final step — a test that runs the native binary and the WASM-compiled version against the same fixture set and asserts that every output matches. By the end of the chapter, the reader has a working example of all three requirements satisfied, a passing CI run, and a concrete artifact to build on in the chapters that follow.</p>
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
