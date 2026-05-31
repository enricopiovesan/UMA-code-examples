---
ref: white-paper
title: "UMA White Papers"
subtitle: "Research papers behind the UMA model: CSMA (2023), UMA (2024), and ECCA (2025)."
macro_area: discoverability
content_type: resource
slug: white-paper
canonical_url: "https://www.universalmicroservices.com/discoverability/white-paper/"
left_nav_group: discoverability
chapter_ref: null
seo_description: "The research papers behind Universal Microservices Architecture: Composable Software Model Architecture (2023), UMA (2024), and Execution Context Coherence Architecture (2025)."
breadcrumbs:
  - "Home"
  - "Discoverability"
  - "UMA White Papers"
related_refs:
  - about-enrico
  - core-model
  - evolve-uma
---

## intro

<section class="subpage-hero">
  <h1>UMA White Papers</h1>
  <p>
    Three research papers document the thinking behind Universal Microservices Architecture — from the initial composable
    service model through the full UMA specification to execution context coherence. Each paper is available to read online
    and represents a point in the model's development.
  </p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>CSMA — Composable Software Model Architecture (June 2023)</h2>
    <p>
      The first paper in the series. CSMA establishes the foundational argument: that distributed systems fragment because
      service boundaries are defined at the deployment unit rather than at the behavioral contract. The paper introduces
      the composable service model — a structured separation between durable business logic and the runtime that hosts it
      — and proposes machine-readable contracts as the mechanism for making service composition inspectable and governable.
    </p>
    <p>
      CSMA is the conceptual origin of what became UMA. It does not yet use WebAssembly as the execution boundary, but
      the three structural separations it defines — behavior from runtime, contract from implementation, placement from
      business logic — carry through unchanged into the full UMA model.
    </p>
    <div class="subpage-inline-links">
      <a href="https://medium.com/@enricopiovesan" rel="noopener">Read on Medium →</a>
    </div>
  </section>

  <section>
    <h2>UMA — Universal Microservices Architecture (August 2024)</h2>
    <p>
      The paper that formalizes the model under its current name. The UMA white paper specifies the portable service
      as a WASM module with an active descriptor, defines the runtime layer's responsibilities (validation, adapter
      binding, trust enforcement, placement, execution evidence), and introduces behavioral equivalence as the
      measurable form of portability.
    </p>
    <p>
      This paper is the architectural specification behind the book and the companion repository. The four structural
      properties it defines — explicit contract, portable binary, governed runtime, parity proof — remain the
      definitional core of UMA.
    </p>
    <div class="subpage-inline-links">
      <a href="https://medium.com/@enricopiovesan" rel="noopener">Read on Medium →</a>
    </div>
  </section>

  <section>
    <h2>ECCA — Execution Context Coherence Architecture (August 2025)</h2>
    <p>
      The third paper extends the UMA model to AI-native execution paths. ECCA addresses the governance question that
      emerges when AI agents become execution participants: how do trust, contract validation, and behavioral evidence
      apply when the caller is a reasoning model rather than a human-written service?
    </p>
    <p>
      ECCA introduces execution context coherence as the system-level property that UMA's runtime governance produces
      across heterogeneous participants — classical services, WASM modules, and AI-assisted paths — and defines how
      Model Context Protocol (MCP) fits into that governance model.
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
