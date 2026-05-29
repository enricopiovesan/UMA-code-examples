---
ref: active-descriptors
title: "Active Descriptors in UMA"
subtitle: "Active descriptors In UMA, a descriptor is not just documentation beside a service. It is metadata the runtime can evaluate to understand the service boundary, validate inputs and outputs, check compatibility, and record why execution was allowed."
macro_area: core-model
content_type: explainer
slug: active-descriptors
canonical_url: "https://www.universalmicroservices.com/active-descriptors/"
left_nav_group: core-model
chapter_ref: null
seo_description: "Learn how UMA uses active descriptors as runtime-evaluated constraints for contracts, schemas, latency expectations, compatibility, and execution evidence."
breadcrumbs:
  - "Home"
  - "Core Model"
  - "Active Descriptors in UMA"
related_refs:
  - agent-vs-runtime
  - late-bound-policy-enforcement
  - what-belongs-in-the-runtime-layer
  - what-is-a-capability
---

## intro

<section class="subpage-hero">
          <h1>Active descriptors</h1>
          <p>In UMA, a descriptor is not just documentation beside a service. It is metadata the runtime can evaluate to understand the service boundary, validate inputs and outputs, check compatibility, and record why execution was allowed.</p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>The useful idea</h2>
            <p>An active descriptor describes a capability in terms the runtime can use: schemas, emitted events, required inputs, allowed placements, version constraints, and the evidence expected from a run. It is active because it participates in execution decisions.</p>
            <p>This does not mean the contract replaces service logic. The contract is an executable constraint model around the logic, while the service still owns the durable behavior.</p>
          </section>
          <section class="subpage-grid">
            <article class="subpage-card"><h3>Boundary</h3><p>Input and output schemas define the shape of the portable behavior.</p></article>
            <article class="subpage-card"><h3>Compatibility</h3><p>Version and event declarations help the runtime decide whether services can interact.</p></article>
            <article class="subpage-card"><h3>Expectations</h3><p>Latency, error, and evidence expectations make execution easier to inspect.</p></article>
            <article class="subpage-card"><h3>Authority</h3><p>The runtime evaluates the descriptor before it treats a proposed path as approved.</p></article>
          </section>
          <section>
            <h2>Concrete proof</h2>
            <p>The early examples prove the idea in small pieces: Chapter 4 uses a contract around deterministic flag evaluation, Chapter 6 uses a contract to compare native and WASI execution, and Chapter 7 lets contracts and events shape orchestration.</p>
          </section>
          <section class="subpage-callout">
            <strong>Covered in the book</strong>
            <p>This page gives the preview. Chapters 4, 6, and 7 explain how descriptors become practical runtime inputs without turning the website into the full book.</p>
            <div class="subpage-inline-links">
              <a href="../examples/chapter-04-feature-flag-evaluator/">Chapter 4 example</a>
              <a href="../examples/chapter-06-portability-lab/">Chapter 6 example</a>
              <a href="../examples/chapter-07-metadata-orchestration/">Chapter 7 example</a>
              <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Buy the book</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
