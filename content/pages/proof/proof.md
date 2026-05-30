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
seo_description: "Review the evidence surface for UMA with portability proof, benchmark notes, and service portability examples."
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
      Architecture claims that cannot be inspected tend to drift toward assumptions, and assumptions compound. The proof section exists so the UMA portability claim stays falsifiable — something you can run, compare, and verify before deciding whether the model applies to your system.
    </p>
  </section>

  <section>
    <h2>Pages in this area</h2>
    <div class="subpage-grid">
      <article class="subpage-card"><h3><a href="../what-makes-a-service-portable/">What Makes a Service Portable?</a></h3><p>The architectural qualities that make a service viable across runtimes.</p></article>
      <article class="subpage-card"><h3><a href="../how-to-prove-portability/">How to Prove Portability</a></h3><p>The verification path used to make portability concrete.</p></article>
      <article class="subpage-card"><h3><a href="../benchmark-and-footprint/">UMA Benchmark And Footprint Notes</a></h3><p>What the benchmark and footprint evidence tells us.</p></article>
    </div>
  </section>
</div>
