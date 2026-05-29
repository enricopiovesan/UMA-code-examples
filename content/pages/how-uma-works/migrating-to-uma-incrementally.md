---
ref: migrating-to-uma-incrementally
title: "Migrating to UMA Incrementally"
subtitle: "Migrating to UMA incrementally UMA does not require a full-platform rewrite. A practical migration begins with one behavior that is duplicated, hard to govern, or forced to run in more than one place. From there, the team extracts a capability, defines its contract, proves portability, and expands only when the result is useful."
macro_area: how-uma-works
content_type: walkthrough
slug: migrating-to-uma-incrementally
canonical_url: "https://www.universalmicroservices.com/migrating-to-uma-incrementally/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "A practical conceptual guide to incremental UMA migration for greenfield and brownfield systems without requiring a full rewrite."
breadcrumbs:
  - "Home"
  - "How Uma Works"
  - "Migrating to UMA Incrementally"
related_refs:
  - architecture-drift-and-portable-business-logic
  - incremental-uma-adoption
  - portable-business-logic
  - runtime-agnostic-architecture
---

## intro

<section class="subpage-hero">
          <h1>Migrating to UMA incrementally</h1>
          <p>UMA does not require a full-platform rewrite. A practical migration begins with one behavior that is duplicated, hard to govern, or forced to run in more than one place. From there, the team extracts a capability, defines its contract, proves portability, and expands only when the result is useful.</p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>Start with one capability</h2>
            <p>The best first candidate is a rule or decision that already appears in multiple places. Feature flags, eligibility checks, routing decisions, and policy evaluations are common starting points.</p>
            <p>The goal is not to universalize everything. The goal is to find one behavior where portability reduces drift.</p>
          </section>

          <section>
            <h2>Greenfield adoption</h2>
            <p>In a greenfield system, UMA can start as a design discipline. Define the capability, contract, runtime expectations, and validation path before choosing all host-specific details.</p>
            <p>That keeps the first service small enough to understand while still teaching the team the model.</p>
          </section>

          <section>
            <h2>Brownfield adoption</h2>
            <p>In an existing system, begin beside the current implementation. Keep the old path alive, extract the behavior into a portable service, and compare outputs before moving traffic or workflow authority.</p>
            <p>This makes migration observable rather than ideological.</p>
          </section>

          <section>
            <h2>Use a strangler path</h2>
            <p>A strangler path works well when one capability can be routed gradually to the UMA implementation. The surrounding system keeps operating while the runtime earns authority through tests, telemetry, and controlled rollout.</p>
            <p>This approach also helps teams avoid creating a second architecture that nobody trusts.</p>
          </section>

          <section>
            <h2>Incremental rollout</h2>
            <p>Roll out by capability, not by layer. A team can adopt one portable service, one runtime wrapper, one validation flow, and one chapter-aligned proof point before expanding the approach to other parts of the system.</p>
          </section>
          <section class="subpage-grid">
            <article class="subpage-card"><h3>Find drift</h3><p>Look for behavior copied across client, edge, backend, and workflow paths.</p></article>
            <article class="subpage-card"><h3>Extract capability</h3><p>Name the durable behavior before choosing the new runtime shape.</p></article>
            <article class="subpage-card"><h3>Prove parity</h3><p>Compare outputs across the old path and the portable implementation.</p></article>
            <article class="subpage-card"><h3>Expand carefully</h3><p>Move the next behavior only after the runtime path is understandable and governed.</p></article>
          </section>
          <section class="subpage-callout">
            <strong>Want to go deeper?</strong>
            <p>This page keeps migration at a conceptual level. The migration material in the book goes deeper into sequencing, risk, governance, and production patterns. The repository gives concrete labs for proving one step at a time.</p>
            <div class="subpage-inline-links">
              <a href="../how-to-prove-portability/">How to prove portability</a>
              <a href="../what-makes-a-service-portable/">What makes a service portable?</a>
              <a href="../trust-boundaries/">Trust boundaries</a>
              <a href="../examples/chapter-06-portability-lab/">Portability lab example</a>
              <a href="https://github.com/enricopiovesan/UMA-code-examples">Official GitHub examples repository</a>
              <a href="../book/">Book page</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
