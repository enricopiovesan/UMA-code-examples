---
ref: chapter-10-security-trust-boundaries-microservices
title: "Ch.10: Security and Trust in UMA"
subtitle: "How to enforce trust in a system where the same service executes in multiple, structurally different runtime environments."
macro_area: learn-uma
content_type: overview
slug: chapter-10-security-trust-boundaries-microservices
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-10-security-trust-boundaries-microservices/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "Chapter 10: security and trust boundaries in microservices. How UMA enforces per-execution trust across multi-runtime systems, beyond perimeter models."
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
    <p>In a single-runtime system, trust is typically a deployment-time decision: if the service is running, it is authorized to run. That assumption breaks in a multi-runtime system. The same service binary may execute in a browser, on a cloud node, at an edge location, or inside an AI agent. each with different authority, different data access, and different policy requirements. Chapter 10 addresses how trust is defined, carried, and enforced when deployment-time authorization is no longer sufficient.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>In a multi-runtime system, trust must be a per-execution decision enforced by the runtime layer. UMA defines trust as a graph property: not whether a service is trusted in isolation, but whether the runtime that executed it was authorized to do so for that specific capability, in that specific context, under that specific policy.</p>
    <p>Active descriptors carry trust requirements alongside capability declarations. Before execution begins, the runtime validates that it has the authority to fulfill those requirements, that it can satisfy the capability's stated trust constraints given the current execution context. A runtime that cannot satisfy them refuses to execute rather than proceeding with degraded guarantees.</p>
    <p>Chapter 10 builds on the system model from Chapter 9 to show how trust requirements propagate through a service graph. It shows what happens when a high-trust capability is invoked through a low-trust runtime, and how the descriptor model makes that failure explicit and early rather than implicit and late.</p>
  </section>

  <section>
    <h2>Why trust must be a per-execution runtime decision</h2>
    <p>Trust assigned at deployment time is too coarse. The same service may handle requests from trusted internal callers and untrusted external callers within the same deployment. Granting trust to the service at deployment time conflates the two. A request from an internal orchestrator that has been through authentication and authorization carries different risk than a request arriving from an edge node that has not. Deployment-time trust cannot distinguish between them.</p>
    <p>Trust assigned at the network layer (mutual TLS, service mesh identity) is also too coarse. mTLS authenticates the caller's identity but says nothing about the specific capability being invoked, the data classification of the inputs, or the policy requirements of the operation. A caller that is network-authenticated may still be invoking a capability with inputs that exceed its declared authority. Network-layer trust is a necessary condition for security, not a sufficient one.</p>
    <p>UMA makes trust a property of each execution context, evaluated by the runtime at invocation time against the capability's declared trust requirements. The evaluation happens before the capability executes. not as a post-hoc audit, but as a precondition. A capability that requires high-trust context will not execute in a low-trust runtime regardless of network-layer authentication. This is what makes the trust model composable: each capability's requirements are declared independently, and the runtime enforces them independently, without requiring a central policy authority that knows about every possible invocation combination in advance.</p>
  </section>

  <section>
    <h2>What trust enforcement looks like in practice</h2>
    <p>The capability's active descriptor declares its trust requirements: minimum caller trust level, permitted caller identities, and data sensitivity classification for inputs and outputs. These are not documentation fields. they are evaluated by the runtime before the WASM module executes. The runtime compares the current execution context against these declared requirements and either proceeds with execution or refuses.</p>
    <p>A denied invocation produces a structured rejection with a reason code: the specific requirement that was not satisfied, the trust level that was presented, and the trust level that was required. This is different from a generic 403 response. The structured rejection is itself an execution artifact: it carries enough information for a security audit to determine whether the denial was a correct enforcement action or a misconfiguration. Generic HTTP 403s are not auditable in this sense. they indicate that access was denied but not why, under what policy, or whether the policy was correctly stated in the first place.</p>
    <p>This also means trust decisions are composable across a service graph. When a high-trust capability is invoked via a chain that passes through a lower-trust intermediate capability, the runtime evaluates each invocation in the chain against that capability's declared requirements. A trust downgrade in the middle of a call chain is detected at the point where it occurs, not at the point where the final capability refuses execution. Chapter 10 shows this chain evaluation with a three-service example and explains why detecting the violation early (at the intermediate capability boundary) is architecturally significant.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 9 established the system properties that emerge at scale. this chapter adds the security model that keeps those properties valid under adversarial conditions. Chapter 11 continues with the broader architectural patterns that govern UMA systems in production.</p>
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
