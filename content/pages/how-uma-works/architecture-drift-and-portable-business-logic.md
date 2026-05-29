---
ref: architecture-drift-and-portable-business-logic
title: "Architecture Drift and Portable Business Logic"
subtitle: "Architecture drift and portable business logic Architecture drift often begins when the same business behavior is copied into browser code, mobile code, backend code, edge functions, workflow glue, and AI-adjacent automation. UMA attacks that drift by making the durable behavior portable and governed."
macro_area: how-uma-works
content_type: walkthrough
slug: architecture-drift-and-portable-business-logic
canonical_url: "https://www.universalmicroservices.com/architecture-drift-and-portable-business-logic/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "Learn how UMA reduces architecture drift by keeping business behavior portable, contract-shaped, and governed across browser, edge, backend, and AI-assisted runtime paths."
breadcrumbs:
  - "Home"
  - "How Uma Works"
  - "Architecture Drift and Portable Business Logic"
related_refs:
  - incremental-uma-adoption
  - migrating-to-uma-incrementally
  - portable-business-logic
  - runtime-agnostic-architecture
---

## intro

<section class="subpage-hero"><h1>Architecture drift and portable business logic</h1><p>Architecture drift often begins when the same business behavior is copied into browser code, mobile code, backend code, edge functions, workflow glue, and AI-adjacent automation. UMA attacks that drift by making the durable behavior portable and governed.</p></section>

## main

<div class="subpage-body">
          <section><h2>The problem</h2><p>A duplicated rule can start as a harmless optimization. Over time, each copy adapts to local pressures. The web client handles one edge case, the backend handles another, the edge function adds a shortcut, and nobody can point to one authoritative behavior anymore.</p><p>UMA does not promise magic elimination of drift. It gives teams a model for reducing drift by keeping business behavior behind explicit contracts and visible runtime authority.</p></section>
          <section class="subpage-grid"><article class="subpage-card"><h3>Portable core</h3><p>Keep the durable behavior testable outside one host.</p></article><article class="subpage-card"><h3>Runtime wrapper</h3><p>Let validation, adapters, and policy live around the core.</p></article><article class="subpage-card"><h3>Observable parity</h3><p>Compare behavior from outputs and events, not wishful code similarity.</p></article><article class="subpage-card"><h3>Governed evolution</h3><p>Use contracts and runtime decisions to manage version growth.</p></article></section>
          <section><h2>The conversion point</h2><p>This is the pressure that makes the book worth reading. The website can name the pattern, but the book walks through how a system moves from one portable behavior to runtime governance, trust, discoverability, and long-term evolution.</p></section>
          <section class="subpage-callout"><strong>Covered in the book</strong><p>Chapters 4, 6, 10, and 11 show the drift problem from different angles: portable behavior, proof of parity, coherence tradeoffs, and evolution without fragmentation.</p><div class="subpage-inline-links"><a href="../portable-business-logic/">Portable business logic</a><a href="../examples/chapter-04-feature-flag-evaluator/">Chapter 4 example</a><a href="../examples/chapter-10-architectural-tradeoffs/">Chapter 10 example</a><a href="../examples/chapter-11-evolution-without-fragmentation/">Chapter 11 example</a><a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Buy the book</a></div></section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
