---
ref: chapter-06-uma-runtime-layer
title: "Ch.6: The UMA Runtime Layer"
subtitle: "What the runtime layer owns, and why that ownership must be explicit."
macro_area: learn-uma
content_type: overview
slug: chapter-06-uma-runtime-layer
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-06-uma-runtime-layer/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "What the UMA runtime layer owns and why explicit ownership matters."
breadcrumbs:
  - "Home"
  - "Learn UMA"
  - "Chapter 6: The UMA Runtime Layer"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
  <h1>Chapter 6: The UMA Runtime Layer</h1>
  <p>What does the runtime layer own, and why must that ownership be explicit?</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The question this chapter answers</h2>
    <p>Most microservice architectures have a runtime layer in practice — it just isn't declared. Configuration loading, request validation, adapter logic, and execution logging end up distributed across frameworks, libraries, and service code. When those responsibilities are scattered, portability becomes accidental: the service works in a given environment because the environment assumes it, not because the contract says so. Chapter 6 defines where the line is, and why drawing it explicitly changes what you can safely promise about a service.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>The runtime layer is not middleware. It is the governed boundary between the portable service and the host environment. It owns six responsibilities that should never live inside the service: input validation against the active descriptor, adapter binding, trust enforcement, placement decisions, policy evaluation, and execution evidence recording.</p>
    <p>When any of these leak into the service, portability fails — not because the code stops compiling, but because the service accumulates hidden assumptions about its environment. A service that validates its own input has implicitly assumed it knows what environment it is running in. A service that makes its own placement decisions has bypassed the governance layer entirely.</p>
    <p>The chapter works through each of the six responsibilities in turn, showing the specific failure mode that results when that responsibility is misplaced. It then shows how a runtime that explicitly owns all six enables a service to be dropped into a new environment — browser, cloud, edge, or AI agent — without modification.</p>
  </section>

  <section>
    <h2>What the runtime layer owns — and what it doesn't</h2>
    <p>The runtime layer owns validation, adapter binding, trust enforcement, placement decisions, and execution evidence. Each of these is a host concern, not a business logic concern. Validation means checking inputs against the active descriptor before the service sees them — not inside the service, before. Adapter binding means resolving the service's declared interface to the concrete resources available in the current environment — file handles, network sockets, credential providers — without the service knowing which adapter was chosen. Trust enforcement means verifying that the caller has the authority the descriptor requires for this invocation. Placement decisions mean determining whether the current environment satisfies the descriptor's constraints before loading the service. Execution evidence means recording what happened — what inputs were received, what outputs were produced, what trust level was in effect — in a form that can be audited independently of the service's own logs.</p>
    <p>The runtime layer does not own business logic, schema definition, or capability versioning. These belong to the service and its descriptor respectively. When a runtime takes responsibility for business logic — by adding transformation rules in the adapter layer, for instance — it creates a situation where the service's behavior in that runtime is different from its behavior in any other runtime, which is precisely the failure mode the architecture is designed to prevent. When a runtime takes responsibility for schema definition, the contract is no longer portable: it lives in the runtime configuration, not in the artifact, which means deploying the service to a new runtime requires re-configuring the schema. The boundary is drawn where it is for a reason, and crossing it has predictable consequences.</p>
    <p>This separation is what makes the runtime replaceable. A service that offloads all host concerns to the runtime layer is not coupled to any specific runtime implementation. The same binary can be governed by a browser runtime, a cloud function host, and an edge runtime, each of which implements the six responsibilities in the way that makes sense for that environment, without the service needing to change. The runtime is swappable; the service is durable. This is the inversion that UMA's runtime model produces, and the chapter shows why it doesn't happen automatically — it requires the boundary to be drawn explicitly and enforced consistently.</p>
  </section>

  <section>
    <h2>Why the runtime must be explicit</h2>
    <p>Implicit runtime behavior is the primary source of invisible governance decisions. When a framework makes a placement decision by convention — running a function locally in development and remotely in production, for instance — that decision is real, consequential, and invisible unless you know the convention. When trust enforcement happens through middleware that was added to the stack three years ago and isn't documented in any current architecture review, the enforcement is happening, but nobody reading the service code would know it. Convention-over-configuration produces systems where the answers to governance questions exist, but are distributed across framework documentation, deployment scripts, and institutional memory rather than in a form any runtime can inspect.</p>
    <p>UMA requires every runtime decision to be inspectable. This is not a philosophical preference — it's a practical requirement for systems that need to operate across multiple execution surfaces. When the same service runs in a browser, an edge node, and a cloud function, the runtime decisions for each surface are different: different adapters, different trust levels, different placement constraints. If those decisions are implicit in each surface's framework configuration, there is no single place to verify that all three surfaces are making equivalent decisions. If they're recorded as evidence artifacts, the comparison is straightforward and automatable.</p>
    <p>The chapter shows what this looks like in code. Each of the six runtime responsibilities has a corresponding evidence record: what was validated, what adapter was bound, what trust level was enforced, what placement decision was made, and what the execution produced. These records are not logs in the observability sense — they are governance artifacts in the audit sense. A compliance review doesn't need to instrument the runtime or add tracing to the service; the evidence is already there, structured, and associated with the specific descriptor version that governed the execution. This is what explicit runtime behavior enables: governance that works at the speed of the runtime, not at the speed of the compliance review cycle.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 5 built a portable service; this chapter defines the layer that makes portability enforceable. The runtime model established here is the foundation for the WebAssembly execution boundary in Chapter 7.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-05-building-portable-microservices/">← Chapter 5: Building Portable Microservices</a>
      <a href="../chapter-07-webassembly-portability-wasm-runtimes/">Chapter 7: Portability with WebAssembly →</a>
      <a href="../../core-model/what-is-a-uma-runtime/">What is a UMA runtime?</a>
      <a href="../../core-model/what-belongs-in-the-runtime-layer/">What belongs in the runtime layer?</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order the book on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
