---
ref: chapter-07-webassembly-portability-wasm-runtimes
title: "Chapter 7: Portability with WebAssembly and Native Runtimes"
subtitle: "How WebAssembly provides a portable execution boundary, and what UMA adds on top of it."
macro_area: learn-uma
content_type: overview
slug: chapter-07-webassembly-portability-wasm-runtimes
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-07-webassembly-portability-wasm-runtimes/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "How WASM and UMA descriptors make microservice portability inspectable."
breadcrumbs:
  - "Home"
  - "Learn UMA"
  - "Chapter 7: Portability with WebAssembly and Native Runtimes"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
  <h1>Chapter 7: Portability with WebAssembly and Native Runtimes</h1>
  <p>How does WebAssembly provide a portable execution boundary, and what does UMA add on top of it?</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The question this chapter answers</h2>
    <p>WebAssembly has changed what "portable code" means in practice. A WASM binary runs in a browser, a server, an edge node, or a constrained device without recompilation. That is a meaningful guarantee. But it is not the same as a portable service. A portable service needs to carry its own contract — what it needs from the environment, what it produces, and what trace it leaves behind. Without that, portability is a runtime coincidence rather than an architectural property. Chapter 7 draws that line precisely.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>WebAssembly gives you a portable binary. UMA gives you a portable service. The difference is the contract.</p>
    <p>A WASM module runs in multiple environments. A Universal Microservice runs in multiple environments AND carries a descriptor that tells the runtime what it needs, what it produces, and what evidence it should leave behind. The active descriptor travels with the binary. The runtime reads it before execution begins, validates the current environment against it, and refuses to proceed if the requirements are not met.</p>
    <p>Chapter 7 shows how the WASM component model and WASI 0.2 provide the execution substrate, and how active descriptors sit on top of that substrate to make portability inspectable rather than assumed. The chapter builds a service that runs in both a browser WASM runtime and a server-side native runtime, using the same descriptor to govern both executions. The result is portability you can audit — not just portability you can claim.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 6 defined what the runtime layer owns; this chapter shows those responsibilities operating across two structurally different execution environments. The contract model introduced here becomes the basis for the event and orchestration work in Chapter 8.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-06-uma-runtime-layer/">← Chapter 6: The UMA Runtime Layer</a>
      <a href="../chapter-08-service-contracts-events-orchestration/">Chapter 8: Contracts, Events, and Orchestration →</a>
      <a href="../../how-uma-works/webassembly-architecture/">WebAssembly architecture in UMA</a>
      <a href="../../core-model/active-descriptors/">Active descriptors</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order the book on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
