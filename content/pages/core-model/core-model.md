---
ref: core-model
title: "Core Model"
subtitle: "The core UMA primitives: capability, workflow, runtime, descriptor, policy, and decision discoverability."
macro_area: core-model
content_type: hub
slug: core-model
canonical_url: "https://www.universalmicroservices.com/core-model/"
left_nav_group: core-model
chapter_ref: null
seo_description: "Explore UMA's core model: capability boundaries, workflow orchestration, runtime rules, and how decisions stay discoverable."
breadcrumbs:
  - "Home"
  - "Core Model"
related_refs:
  - what-is-a-capability
  - what-is-a-workflow
  - what-is-a-uma-runtime
  - what-belongs-in-the-runtime-layer
  - active-descriptors
  - late-bound-policy-enforcement
  - what-makes-a-decision-discoverable
  - what-is-wasm-mcp
  - agent-vs-runtime
---

## intro

<section class="subpage-hero">
  <h1>Core Model</h1>
  <p>
    UMA becomes practical when its building blocks are explicit. This area defines the units that stay stable across runtimes and the
    rules that keep behavior, policy, and orchestration understandable.
  </p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>What this macro area covers</h2>
    <p>
      These pages describe the vocabulary used to model capabilities, workflows, runtime responsibilities, and the active descriptors
      that let a system remain discoverable without turning the runtime into a black box.
    </p>
  </section>

  <section>
    <h2>Pages in this area</h2>
    <div class="subpage-grid">
      <article class="subpage-card"><h3><a href="../what-is-a-capability/">What Is a Capability in UMA?</a></h3><p>The portable unit of behavior that UMA tries to keep coherent.</p></article>
      <article class="subpage-card"><h3><a href="../what-is-a-workflow/">What Is a Workflow in UMA?</a></h3><p>How capability execution becomes an inspectable progression of steps.</p></article>
      <article class="subpage-card"><h3><a href="../what-is-a-uma-runtime/">What Is a UMA Runtime?</a></h3><p>The governed execution layer that decides where behavior can run.</p></article>
      <article class="subpage-card"><h3><a href="../what-belongs-in-the-runtime-layer/">What Belongs in the Runtime Layer?</a></h3><p>The responsibilities that should remain runtime-owned.</p></article>
      <article class="subpage-card"><h3><a href="../active-descriptors/">Active Descriptors in UMA</a></h3><p>How descriptors keep runtime capabilities visible and queryable.</p></article>
      <article class="subpage-card"><h3><a href="../late-bound-policy-enforcement/">Late-Bound Policy Enforcement in UMA</a></h3><p>How policy stays authoritative without freezing the model too early.</p></article>
      <article class="subpage-card"><h3><a href="../what-makes-a-decision-discoverable/">What Makes a Decision Discoverable?</a></h3><p>Why runtime decisions need to be explainable after the fact.</p></article>
      <article class="subpage-card"><h3><a href="../what-is-wasm-mcp/">What Is WASM MCP in UMA?</a></h3><p>How discovery and runtime integration meet in a portable model.</p></article>
      <article class="subpage-card"><h3><a href="../agent-vs-runtime/">Agent vs Runtime in UMA</a></h3><p>Why agents and runtimes are related but not interchangeable.</p></article>
    </div>
  </section>
</div>
