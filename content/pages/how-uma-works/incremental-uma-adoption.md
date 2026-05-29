---
ref: incremental-uma-adoption
title: "Incremental UMA Adoption"
subtitle: "Incremental UMA adoption UMA does not require a rewrite. It can start with one portable behavior, one contract, or one runtime-governed boundary inside a system that still has legacy services and ordinary deployment infrastructure."
macro_area: how-uma-works
content_type: walkthrough
slug: incremental-uma-adoption
canonical_url: "https://www.universalmicroservices.com/incremental-uma-adoption/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "Learn how UMA can begin with one portable capability and coexist with legacy systems through adapters, governed boundaries, and gradual runtime adoption."
breadcrumbs:
  - "Home"
  - "How Uma Works"
  - "Incremental UMA Adoption"
related_refs:
  - architecture-drift-and-portable-business-logic
  - migrating-to-uma-incrementally
  - portable-business-logic
  - runtime-agnostic-architecture
---

## intro

<section class="subpage-hero"><h1>Incremental UMA adoption</h1><p>UMA does not require a rewrite. It can start with one portable behavior, one contract, or one runtime-governed boundary inside a system that still has legacy services and ordinary deployment infrastructure.</p></section>

## main

<div class="subpage-body">
          <section><h2>Start with the boundary</h2><p>The practical first step is not replacing your platform. It is finding a duplicated behavior or fragile runtime decision and moving that behavior behind an explicit contract and runtime-evaluated boundary.</p><p>Adapters let existing hosts participate while the portable core stays visible.</p></section>
          <section class="subpage-grid"><article class="subpage-card"><h3>One behavior</h3><p>Choose a rule or normalization path that is repeated across environments.</p></article><article class="subpage-card"><h3>One contract</h3><p>Define what the behavior accepts, returns, emits, and depends on.</p></article><article class="subpage-card"><h3>One runtime wrapper</h3><p>Add validation, adapter binding, and lifecycle evidence around it.</p></article><article class="subpage-card"><h3>One migration path</h3><p>Keep legacy internals where needed while governing the boundary.</p></article></section>
          <section><h2>Where the book goes further</h2><p>The full adoption story is not only about starting. It is about avoiding fragmentation as more versions, consumers, and runtime surfaces appear. That is why Chapter 11 matters so much.</p></section>
          <section class="subpage-callout"><strong>Covered in the book</strong><p>Chapters 5 and 6 show the first runtime and portability steps. Chapter 11 explains hybrid adoption without forcing a rewrite.</p><div class="subpage-inline-links"><a href="../examples/chapter-05-post-fetcher-runtime/">Chapter 5 example</a><a href="../examples/chapter-06-portability-lab/">Chapter 6 example</a><a href="../examples/chapter-11-evolution-without-fragmentation/">Chapter 11 example</a><a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Buy the book</a></div></section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
