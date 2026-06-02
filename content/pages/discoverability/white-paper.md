---
ref: white-paper
title: "Research Papers by Enrico Piovesan"
subtitle: "Published research on distributed systems architecture. composable service models, portable microservices, and execution context coherence."
macro_area: discoverability
content_type: resource
slug: white-paper
canonical_url: "https://www.universalmicroservices.com/discoverability/white-paper/"
left_nav_group: discoverability
chapter_ref: null
seo_description: "Research papers by Enrico Piovesan on distributed systems architecture: CSMA (2023), Universal Microservices Architecture (2024), and ECCA (2025)."
breadcrumbs:
  - "Home"
  - "Discoverability"
  - "Research Papers"
related_refs:
  - about-enrico
  - core-model
  - evolve-uma
---

## intro

<section class="subpage-hero">
  <h1>Research papers</h1>
  <p>
    Three published papers on distributed systems architecture by Enrico Piovesan. Each addresses a distinct problem:
    how services fragment, how portable execution boundaries work, and how governance applies when AI agents enter the
    execution path.
  </p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>CSMA. Composable Software Model Architecture (June 2023)</h2>
    <p>
      CSMA addresses a specific architectural failure mode: distributed systems that fragment because service boundaries
      are defined at the deployment unit rather than at the behavioral contract. The paper introduces the composable
      service model (a structured separation between durable business logic and the runtime that hosts it) and proposes
      machine-readable contracts as the mechanism for making service composition inspectable and governable.
    </p>
    <p>
      The three structural separations it defines (behavior from runtime, contract from implementation, placement from
      business logic) are the conceptual foundation this line of research builds on.
    </p>
    <div class="subpage-inline-links">
      <a href="https://medium.com/@enricopiovesan" rel="noopener">Read on Medium →</a>
    </div>
  </section>

  <section>
    <h2>UMA. Universal Microservices Architecture (August 2024)</h2>
    <p>
      The paper that formalizes the model under its current name. The UMA white paper specifies the portable service
      as a WASM module with an active descriptor, defines the runtime layer's responsibilities (validation, adapter
      binding, trust enforcement, placement, execution evidence), and introduces behavioral equivalence as the
      measurable form of portability.
    </p>
    <p>
      This paper is the architectural specification behind the book and the companion repository. The four structural
      properties it defines (explicit contract, portable binary, governed runtime, parity proof) remain the
      definitional core of UMA.
    </p>
    <div class="subpage-inline-links">
      <a href="https://medium.com/@enricopiovesan" rel="noopener">Read on Medium →</a>
    </div>
  </section>

  <section>
    <h2>ECCA. Execution Context Coherence Architecture (August 2025)</h2>
    <p>
      ECCA addresses the governance question that emerges when AI agents become execution participants: how do trust,
      contract validation, and behavioral evidence apply when the caller is a reasoning model rather than a
      human-written service?
    </p>
    <p>
      The paper introduces execution context coherence as a system-level property: the condition where heterogeneous
      participants (classical services, WASM modules, AI-assisted paths) produce verifiable, governed output regardless
      of which execution context initiated the call. It defines how Model Context Protocol fits into a governed
      execution model.
    </p>
    <div class="subpage-inline-links">
      <a href="https://medium.com/@enricopiovesan" rel="noopener">Read on Medium →</a>
    </div>
  </section>

  <section class="subpage-callout">
    <strong>Want the full model?</strong>
    <div class="subpage-inline-links">
      <a href="../../learn-uma/book/">The UMA book (Apress, 2026)</a>
      <a href="../../core-model/">Core model</a>
      <a href="../about-enrico/">About the author</a>
    </div>
  </section>
</div>

<section id="contacts" class="section contacts-band" data-shared-footer></section>
