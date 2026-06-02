---
ref: chapter-14-uma-reference-application
title: "Ch.14: The Reference Experience"
subtitle: "What does a complete UMA system look like when all the pieces (portable services, runtime layer, contracts, trust, and AI-native execution) are assembled together?"
macro_area: learn-uma
content_type: overview
slug: chapter-14-uma-reference-application
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-14-uma-reference-application/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "The complete UMA reference application: all pieces assembled."
breadcrumbs:
  - "Home"
  - "Learn UMA"
  - "Chapter 14: The Reference Experience"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
  <h1>Chapter 14: The Reference Experience</h1>
  <p>What does a complete UMA system look like when all the pieces (portable services, runtime layer, contracts, trust, and AI-native execution) are assembled together?</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The question this chapter answers</h2>
    <p>Every earlier chapter addressed one dimension of the UMA model: capability boundaries, contracts, runtime placement, trust enforcement, evolution, agent governance. Chapter 14 answers the question those chapters defer: what does the full system look like when all of those dimensions are operating at once? Not another isolated example: the complete model running as an integrated system, with every architectural decision visible and explained.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>The reference application is the answer to "show me the whole thing." It runs capabilities with active descriptors, a runtime that enforces governance across all invocations, adapters for multiple host environments, and MCP-compatible endpoints for AI agent access. Every piece is where it is for a reason that earlier chapters established. Chapter 14 walks through the architecture decision by decision. not as a tour, but as a justification. Each element is explained in terms of what it prevents: what breaks if the descriptor is passive, what drifts if the runtime is advisory, what fails if the adapter is baked into the capability.</p>
    <p>This makes the reference application a diagnostic tool as much as a demonstration. A team building their first UMA system can use it as a structural template. A team already running UMA can compare their decisions against a system where every tradeoff has been made explicit. The live reference application is available at this site.</p>
  </section>

  <section>
    <h2>What the reference application demonstrates</h2>
    <p>The reference application is not a toy example and not a simplified illustration. It is the complete UMA system as described in chapters 1–13, assembled and running: portable services with active descriptors, a runtime layer that enforces governance on every invocation, MCP-exposed capabilities invokable by AI agents, contract-driven orchestration that assembles execution paths from declared event contracts, trust enforcement evaluated at execution time against each capability's declared requirements, and execution evidence stored as queryable artifacts.</p>
    <p>Every architectural claim made in earlier chapters has a corresponding running component in the reference application. The claim that trust is a per-execution decision corresponds to a runtime evaluation that runs before every invocation, not a deployment-time configuration that grants blanket authority. The claim that orchestration is contract-derived rather than hardcoded corresponds to a workflow that was assembled from event declarations, not a pipeline script. The claim that agent-initiated invocations are governed the same as classical invocations corresponds to a test that invokes the same capability from both a deterministic script and an AI agent and compares the execution evidence produced in each case.</p>
    <p>This grounding is deliberate. Architecture books describe systems. the reference application is the proof that the system described in the book can be built and can work as described. A team that finds a gap between the book's claims and the reference application's behavior has found a genuine architectural question worth investigating, not just a documentation inconsistency.</p>
  </section>

  <section>
    <h2>How to use the reference application</h2>
    <p>The companion repository's chapter-13 lab is the recommended entry point. Run it with the provided script and observe the complete system initializing: runtime layer starting, capabilities registering their descriptors, MCP endpoints becoming available, the governance layer beginning to record execution evidence. The startup sequence itself reflects the architectural order of operations established across the book's chapters.</p>
    <p>After the system is running, inspect the active descriptors directly. Each capability's descriptor is a readable artifact that states what the capability requires, what it emits, what trust level it demands, and what data classifications it handles. Reading the descriptors for a small set of capabilities gives a concrete sense of what "machine-readable contract" means at the level of a running system rather than a conceptual description.</p>
    <p>Trace an invocation through the governance layer by calling a capability directly and then examining the execution evidence record it produces. The record identifies the capability, the runtime that executed it, the trust context that was evaluated, the inputs it received, and the outputs it produced. Compare that record to the evidence produced by an AI agent invoking the same capability through the MCP interface. The two records have the same structure, confirming that the governance model does not distinguish between caller types. That structural equivalence is the reference application's core architectural proof. and the thing most worth verifying before using the book's model as the basis for a production system design.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 13 showed UMA governance applied to AI-native execution. Chapter 14 closes the book by showing the model whole. every concept from the preceding chapters present and accountable in a single running system.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-13-ai-agents-mcp-runtime/">← Chapter 13: Agents, MCP, and the Runtime of Reasoning</a>
      <a href="../">Back to Learn UMA</a>
      <a href="../../reference-application/">Live Reference Application</a>
      <a href="../examples/chapter-13-portable-mcp-runtime/">Chapter 13 code examples</a>
      <a href="../book/">Book overview</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
