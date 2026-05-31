---
ref: chapter-08-service-contracts-events-orchestration
title: "Chapter 8: Contracts, Events, and Orchestration"
subtitle: "How explicit contracts and declared events make orchestration emerge from the system rather than being hardcoded into it."
macro_area: learn-uma
content_type: overview
slug: chapter-08-service-contracts-events-orchestration
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-08-service-contracts-events-orchestration/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "How UMA contracts and declared events replace hardcoded orchestration."
breadcrumbs:
  - "Home"
  - "Learn UMA"
  - "Chapter 8: Contracts, Events, and Orchestration"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
  <h1>Chapter 8: Contracts, Events, and Orchestration</h1>
  <p>How do explicit contracts and declared events make orchestration emerge from the system rather than being hardcoded into it?</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The question this chapter answers</h2>
    <p>Hardcoded orchestration is one of the most common sources of fragility in microservice systems. Service A calls service B with a known schema, at a known endpoint, in a known sequence. When any of those assumptions changes, the orchestration breaks — and the coupling is often invisible until it does. The question Chapter 8 answers is not how to make orchestration more resilient by adding retries and circuit breakers. It is how to make orchestration a property of the declared system rather than a property of the code that wires it together.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>Traditional orchestration is brittle because it is hardcoded: service A calls service B with a known schema. UMA replaces this with contract-driven composition: services declare what events they emit and require, and the runtime builds execution paths from those declarations.</p>
    <p>The result is orchestration that can be inspected, validated, and reconfigured without touching the services themselves. Adding a new step in a workflow means declaring a new event contract and publishing a service that satisfies it — not modifying the orchestrator. Removing a step means withdrawing the contract declaration. The runtime recomputes the execution graph from what is currently declared, not from what a developer wired together at build time.</p>
    <p>Chapter 8 builds a multi-service workflow from declared contracts and shows, at each step, where a hardcoded approach would have introduced coupling. The examples use real event types with real validation to make the difference concrete.</p>
  </section>

  <section>
    <h2>Why hardwired orchestration is a liability</h2>
    <p>When workflow steps are coded as explicit function calls or pipeline configuration, the workflow becomes a second artifact to maintain alongside the services. A change to the order of steps, the addition of a conditional branch, or the substitution of one service for another each requires a deployment — not just of the changed service, but of the orchestration layer that wires them together.</p>
    <p>This liability compounds as execution surfaces multiply. The workflow that coordinates backend-to-backend calls does not translate directly to edge-to-workflow invocation or to an AI-agent-to-service path. Each new execution surface requires a rewrite of the orchestration logic, because the orchestration is coupled to the execution model, not derived from the capabilities themselves. Systems that start with two or three services and a simple pipeline accumulate orchestration debt that becomes visible only when the fourth or fifth surface is added and the pipeline cannot adapt without a significant rewrite.</p>
    <p>The coupling is also often invisible. When service A calls service B at a specific endpoint with a specific schema, that dependency does not appear in either service's contract — it appears only in the orchestration code. Auditing the system for dependencies requires reading the orchestrator, not reading the services. This means that changes to service B can silently break the orchestration without any contract violation being reported.</p>
  </section>

  <section>
    <h2>How contract-driven orchestration works</h2>
    <p>In the UMA model, a capability declares the events it emits and the events it requires. These declarations are part of the active descriptor, not separate workflow configuration. The runtime reads those declarations and assembles valid execution paths from them — a workflow is not a script that a developer writes; it is a solution the runtime finds given the set of declared event contracts currently in the system.</p>
    <p>This changes the operational model for workflow evolution. Adding a new capability to the system makes it automatically available for orchestration wherever its declared event requirements can be satisfied, without modifying any existing workflow definition. Removing a capability withdraws its event declarations; the runtime recomputes execution paths from what remains. A team can add, replace, or retire workflow steps by managing capability declarations rather than by editing orchestration code.</p>
    <p>The runtime's role is to find valid execution paths that satisfy every declared event dependency, validate that those paths meet the trust and governance requirements of each participating capability, and record execution evidence for each invocation. This means orchestration validation happens before execution — the system knows whether a valid path exists before attempting to run it — rather than failing mid-execution when a hardcoded endpoint is unavailable or a schema assumption breaks.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 7 established portable execution across environments; this chapter shows how multiple portable services compose into governed workflows. Chapter 9 extends that to system-level properties that emerge as service graphs grow.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-07-webassembly-portability-wasm-runtimes/">← Chapter 7: Portability with WebAssembly</a>
      <a href="../chapter-09-microservices-to-distributed-systems/">Chapter 9: From Services to Systems →</a>
      <a href="../../core-model/what-is-a-workflow/">What is a UMA workflow?</a>
      <a href="../../evolve-uma/contract-driven-orchestration/">Contract-driven orchestration</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order the book on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
