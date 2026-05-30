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
  </section>

  <section>
    <h2>Pages in this area</h2>
    <div class="subpage-grid">
      <article class="subpage-card"><h3><a href="../runtime-agnostic-architecture/">Runtime-agnostic Architecture</a></h3><p>The architectural stance that keeps behavior portable across runtimes.</p></article>
      <article class="subpage-card"><h3><a href="../portable-business-logic/">Portable Business Logic</a></h3><p>What stays inside the capability boundary when the runtime changes.</p></article>
      <article class="subpage-card"><h3><a href="../architecture-drift-and-portable-business-logic/">Architecture Drift and Portable Business Logic</a></h3><p>Why portability matters when teams and runtimes change over time.</p></article>
      <article class="subpage-card"><h3><a href="../webassembly-architecture/">WebAssembly Architecture and UMA</a></h3><p>The place where UMA and WASM meet in the site narrative.</p></article>
      <article class="subpage-card"><h3><a href="../migrating-to-uma-incrementally/">Migrating to UMA Incrementally</a></h3><p>How to move toward UMA without a big-bang rewrite.</p></article>
      <article class="subpage-card"><h3><a href="../incremental-uma-adoption/">Incremental UMA Adoption</a></h3><p>Adoption patterns that keep the transition measurable and reversible.</p></article>
      <article class="subpage-card"><h3><a href="../uma-production-readiness/">UMA Production Readiness</a></h3><p>What needs to be true before the model is ready for production use.</p></article>
    </div>
  </section>
</div>
