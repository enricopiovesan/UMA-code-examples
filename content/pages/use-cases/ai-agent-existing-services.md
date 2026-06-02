---
ref: ai-agent-existing-services
title: "AI Agent Integration Without Rewriting Your Services"
subtitle: "How to let AI agents call existing microservices without building separate adapters or duplicating business logic."
macro_area: use-cases
content_type: concept
slug: ai-agent-existing-services
canonical_url: "https://www.universalmicroservices.com/use-cases/ai-agent-existing-services/"
left_nav_group: use-cases
seo_description: "Let AI agents call existing microservices without wrappers or duplicated logic. How UMA runtime governance makes services AI-callable without rewriting them."
breadcrumbs:
  - "Home"
  - "Use Cases"
  - "AI Agent Integration"
related_refs:
  - core-model
  - what-is-wasm-mcp
  - evolve-uma
---

## intro

<section class="subpage-hero">
  <h1>AI Agent Integration Without Rewriting Your Services</h1>
  <p>How to let AI agents call existing microservices without building separate adapters or duplicating business logic.</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The problem</h2>
    <p>You have a pricing service. It has been in production for two years. It handles discount rules, tier logic, promotional overrides, and a handful of edge cases that were hard to get right the first time. It works. Your team trusts it.</p>
    <p>Now you are building an AI agent that needs to quote a price during a conversation. The agent needs to call your pricing service. The service was not built for AI callers. It speaks REST. It expects an authenticated HTTP request with a specific payload shape. It returns a JSON blob that your front-end knows how to read but that an LLM context window does not.</p>
    <p>So you face a choice: write a wrapper that translates between the agent and the service, or rewrite the service to expose a new interface. Neither option is good. The wrapper duplicates knowledge about what the service does. The rewrite risks the correctness you already have.</p>
    <p>This problem is not unique to pricing services. It appears any time an AI agent needs to call business logic that was written for a different caller model. Authentication workflows, inventory checks, compliance validators, approval routing — the same mismatch appears across the stack.</p>
  </section>

  <section>
    <h2>Why the usual approach breaks</h2>
    <p>The instinctive fix is to wrap the service. Write a thin adapter that the agent calls. The adapter calls the real service. Problem solved, at least for now.</p>
    <p>The wrapper approach accumulates hidden costs. Every wrapper is a new surface to maintain. When the underlying service changes its contract, the wrapper breaks silently or requires synchronized updates. When a second agent needs the same service, you write a second wrapper or share the first one and couple two agents together. When you need to audit what the agent did and why, the wrapper is a gap in the trace.</p>
    <p>The deeper problem is that wrappers push logic in the wrong direction. The business rule that determines when a discount applies belongs in the pricing service. If the wrapper starts encoding conditions — "only call the service if the cart total is above X" — you now have business logic in the adapter layer, and the original service no longer describes its own behavior completely.</p>
    <p>Rewriting the service for AI callers has the opposite problem. You add an MCP endpoint or a tool schema on top of existing logic, which sounds clean until the two interfaces drift. The REST API evolves for one set of callers. The MCP surface evolves for agents. They describe the same underlying behavior but through different contracts, and keeping them in sync becomes its own coordination overhead.</p>
  </section>

  <section>
    <h2>How UMA solves it</h2>
    <p>UMA separates service behavior from the caller model at the runtime layer, not the service layer. The pricing service does not need to know it is being called by an agent. The agent does not need to understand the service transport protocol. The UMA runtime sits between them and handles translation, policy enforcement, and traceability.</p>
    <p>A UMA-governed service is described by a machine-readable descriptor: what it accepts, what it returns, what policies govern its execution, and what its contract version is. That descriptor is the stable surface. The service core — the business logic — does not change. What changes is which runtime is hosting the call.</p>
    <p>When an agent needs to invoke the pricing service, it calls a tool that is backed by the UMA runtime. The runtime reads the service descriptor, validates the input, routes the call, enforces policy (rate limits, authorization, audit logging), and returns output in a shape the agent can use. The service itself is unchanged. The agent does not need a custom wrapper. The runtime provides the translation layer that both sides can trust.</p>
    <p>This is runtime governance applied to AI integration. The behavior stays in the service. The governance lives in the runtime. The agent gets a stable, policy-enforced interface without anyone rewriting business logic to accommodate a new caller.</p>
  </section>

  <section>
    <h2>What this looks like in practice</h2>
    <p>Consider a pricing service packaged as a WASM module with an active descriptor. The descriptor declares the service input schema, output schema, allowed callers, and audit requirements. The module implements the pricing logic in a pure function: input in, output out, no side effects, no external calls.</p>
    <p>An AI agent running inside a UMA-aware MCP host needs to quote a price. The host exposes the pricing service as an available tool based on the descriptor. The agent calls the tool. The runtime instantiates the WASM module, passes the validated input, and returns the output. The agent receives a structured price quote it can reason about.</p>
    <p>From the agent perspective: a tool call returned a result. From the service perspective: a valid input arrived and a correct output was returned. From the runtime perspective: a governed call was logged, validated against policy, and executed against a versioned artifact. No wrappers were written. No logic was duplicated. The audit trail is complete.</p>
    <p>This pattern extends to services that are not yet packaged as WASM. The UMA runtime can wrap an existing REST service in a governed adapter defined by a descriptor, without touching the service code. The descriptor becomes the contract. The runtime enforces it. The AI caller gets the same trust guarantees regardless of what is underneath.</p>
  </section>

  <section>
    <h2>Where to go next</h2>
    <p>The core model page describes how descriptors, capabilities, and runtime governance fit together. The MCP and WASM page shows how UMA integrates with the Model Context Protocol to make services agent-callable by construction. The book covers this integration in depth, including how to structure descriptors for AI callers and how the runtime handles policy enforcement across agent-driven workflows.</p>
  </section>

  <section class="subpage-callout">
    <strong>Learn more</strong>
    <div class="subpage-inline-links">
      <a href="../../core-model/">Core Model: capabilities, descriptors, and runtime governance</a>
      <a href="../../core-model/what-is-wasm-mcp/">MCP and WASM: making services agent-callable</a>
      <a href="../../learn-uma/book/">Pre-order on Amazon</a>
    </div>
  </section>
</div>
