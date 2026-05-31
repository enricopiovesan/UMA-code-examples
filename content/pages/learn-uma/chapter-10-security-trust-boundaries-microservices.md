---
ref: chapter-10-security-trust-boundaries-microservices
title: "Chapter 10: Security and Trust Boundaries in UMA"
subtitle: "How to enforce trust in a system where the same service executes in multiple, structurally different runtime environments."
macro_area: learn-uma
content_type: overview
slug: chapter-10-security-trust-boundaries-microservices
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-10-security-trust-boundaries-microservices/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "How UMA enforces per-execution trust across multi-runtime microservice systems."
breadcrumbs:
  - "Home"
  - "Learn UMA"
  - "Chapter 10: Security and Trust Boundaries in UMA"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
  <h1>Chapter 10: Security and Trust Boundaries in UMA</h1>
  <p>How do you enforce trust in a system where the same service executes in multiple, structurally different runtime environments?</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The question this chapter answers</h2>
    <p>In a single-runtime system, trust is typically a deployment-time decision: if the service is running, it is authorized to run. That assumption breaks in a multi-runtime system. The same service binary may execute in a browser, on a cloud node, at an edge location, or inside an AI agent — each with different authority, different data access, and different policy requirements. Chapter 10 addresses how trust is defined, carried, and enforced when deployment-time authorization is no longer sufficient.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>In a multi-runtime system, trust must be a per-execution decision enforced by the runtime layer. UMA defines trust as a graph property: not whether a service is trusted in isolation, but whether the runtime that executed it was authorized to do so for that specific capability, in that specific context, under that specific policy.</p>
    <p>Active descriptors carry trust requirements alongside capability declarations. Before execution begins, the runtime validates that it has the authority to fulfill those requirements — that it can satisfy the capability's stated trust constraints given the current execution context. A runtime that cannot satisfy them refuses to execute rather than proceeding with degraded guarantees.</p>
    <p>Chapter 10 builds on the system model from Chapter 9 to show how trust requirements propagate through a service graph. It shows what happens when a high-trust capability is invoked through a low-trust runtime, and how the descriptor model makes that failure explicit and early rather than implicit and late.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 9 established the system properties that emerge at scale; this chapter adds the security model that keeps those properties valid under adversarial conditions. Chapter 11 continues with the broader architectural patterns that govern UMA systems in production.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-09-microservices-to-distributed-systems/">← Chapter 9: From Services to Systems</a>
      <a href="../chapter-11-microservices-architecture-patterns/">Chapter 11: Microservices Architecture Patterns →</a>
      <a href="../../evolve-uma/trust-boundaries/">Trust boundaries in UMA</a>
      <a href="../../evolve-uma/runtime-provenance-and-trust/">Runtime provenance and trust</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order the book on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
