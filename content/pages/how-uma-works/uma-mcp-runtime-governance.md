---
ref: uma-mcp-runtime-governance
title: "UMA Runtime Governance for MCP"
subtitle: "How UMA applies contract validation, trust enforcement, and execution evidence to AI agent tool calls via Model Context Protocol."
macro_area: how-uma-works
content_type: overview
slug: uma-mcp-runtime-governance
canonical_url: "https://www.universalmicroservices.com/how-uma-works/uma-mcp-runtime-governance/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "UMA governance for MCP tool calls: contract validation, trust boundaries, and execution evidence for AI agents invoking WebAssembly services."
breadcrumbs:
  - "Home"
  - "How UMA Works"
  - "UMA Runtime Governance for MCP"
related_refs:
  - how-uma-works
  - what-is-wasm-mcp
  - mcp-wasm-ai-native-microservices
  - chapter-13-ai-agents-mcp-runtime
  - what-belongs-in-the-runtime-layer
---

## intro

<section class="subpage-hero">
  <h1>UMA Runtime Governance for MCP</h1>
  <p>
    Model Context Protocol gives AI agents a structured way to discover and invoke tools. UMA's runtime governance layer applies
    to those invocations the same way it applies to any other capability call: contract validation before execution, trust
    enforcement at the boundary, and execution evidence after the fact. The result is AI tool invocation that is inspectable
    and governable. not a black box that bypasses the architectural model.
  </p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>What MCP adds to the AI execution path</h2>
    <p>
      Without MCP, an AI agent calling a tool is an opaque side effect: the model decides what to call, the call happens,
      and nothing in the system can inspect whether the call was valid, authorized, or consistent with the declared contract.
      Trust, versioning, and behavioral evidence are all absent.
    </p>
    <p>
      MCP introduces structure at the discovery layer. A server exposes a list of tools with names, descriptions, and input
      schemas. The agent uses that manifest to decide what to call and how to call it. This is structurally similar to what
      UMA's active descriptor does for capabilities: a machine-readable surface that makes behavior discoverable before
      invocation rather than opaque until execution.
    </p>
    <p>
      The gap MCP does not close on its own: it describes tools, but it does not validate contracts, enforce trust policy,
      or produce execution evidence at the business-logic level. Those are runtime concerns. UMA's runtime layer fills them.
    </p>
  </section>

  <section>
    <h2>How UMA's runtime layer governs MCP tool calls</h2>
    <p>
      In the UMA model, an MCP server is not the execution boundary: it is the discovery surface. The portable WASM service
      is the execution boundary. The runtime layer sits between the two and applies governance at every invocation:
    </p>
    <ul>
      <li>
        <strong>Contract validation</strong> (the runtime reads the active descriptor before the WASM module executes.
        If the agent's tool call does not satisfy the declared input schema, version constraint, or placement rule, the
        runtime rejects it with a specific reason) not a generic error.
      </li>
      <li>
        <strong>Trust enforcement</strong> (the runtime evaluates the trust policy in effect for the current execution
        context. An AI agent's invocation can be granted a different trust level than a human-initiated call to the same
        capability) and that difference is explicit, not assumed.
      </li>
      <li>
        <strong>Adapter binding</strong>: the portable WASM service has no knowledge of whether it is being called by
        an AI agent, a backend service, or a browser client. The runtime resolves the correct adapter for the current
        context, keeping that decision outside the service's business logic.
      </li>
      <li>
        <strong>Execution evidence</strong> (the runtime records what was called, under what trust level, with what
        inputs, and what the outcome was. This evidence exists as a queryable artifact after the fact) not as a log
        line that requires parsing.
      </li>
    </ul>
  </section>

  <section>
    <h2>Why this matters for AI-assisted execution paths</h2>
    <p>
      AI agents introduce a new class of caller: one whose invocation pattern is non-deterministic, whose intent is
      inferred rather than declared, and whose trust level is ambiguous by default. Without governance at the runtime
      layer, every tool the agent can call becomes an implicit trust grant with no audit trail.
    </p>
    <p>
      UMA treats the AI agent as a caller like any other: one that must satisfy the same contract requirements and
      trust policy as a classical service. The agent's use of MCP to discover and invoke tools does not bypass
      governance. it becomes the structured entry point through which governance is applied.
    </p>
    <p>
      This also means the same portable WASM service can be invoked by a classical service, a workflow orchestrator,
      and an AI agent. with the runtime enforcing the appropriate trust boundary for each caller type, without the
      service itself needing to know which caller type it is receiving.
    </p>
  </section>

  <section>
    <h2>The portable MCP runtime in Chapter 13</h2>
    <p>
      Chapter 13 of the UMA book builds this as a complete runnable system. The portable MCP runtime is the reference
      implementation of UMA's governance model applied to AI-native execution: an MCP server that exposes capabilities
      as tools, a runtime layer that validates contracts and enforces trust before each WASM invocation, and an
      evidence layer that makes every agent decision queryable after the fact.
    </p>
    <p>
      The companion repository's
      <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-13-portable-mcp-runtime">chapter-13 lab</a>
      is the runnable proof. It is not a diagram or a conceptual sketch: it is the complete system assembled and
      exercised with passing CI.
    </p>
    <div class="subpage-inline-links">
      <a href="../../learn-uma/chapter-13-ai-agents-mcp-runtime/">Chapter 13 overview</a>
      <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-13-portable-mcp-runtime" rel="noopener">Chapter 13 lab on GitHub</a>
    </div>
  </section>

  <section>
    <h2>MCP governance vs MCP description</h2>
    <p>
      A common pattern in current MCP implementations is to treat the tool manifest as both the description and the
      governance surface. The tool schema describes what the tool accepts. the server decides whether to execute.
      This collapses two distinct concerns into one layer.
    </p>
    <p>
      UMA separates them deliberately. The MCP manifest is the discovery surface (what the agent can see and request.
      The active descriptor is the governance surface) what the runtime validates before execution proceeds. These
      are different documents with different audiences: the manifest is for the AI model, the descriptor is for the
      runtime. Conflating them produces either over-permissive execution (governance is only as strict as the manifest
      schema) or over-restrictive discovery (the manifest carries runtime constraints the agent cannot interpret).
    </p>
  </section>

  <section>
    <h2>What this looks like in practice</h2>
    <p>
      A request arrives from an AI agent via MCP tool call. The sequence in a governed UMA runtime:
    </p>
    <ol>
      <li>The MCP server receives the tool invocation and maps it to a capability by name and version</li>
      <li>The runtime reads the capability's active descriptor. input schema, placement constraints, trust requirements</li>
      <li>The runtime evaluates the caller's trust level against the descriptor's trust policy</li>
      <li>If valid: the runtime binds the appropriate adapter and executes the WASM module</li>
      <li>If invalid: the runtime rejects with a structured reason. schema mismatch, trust violation, version incompatibility</li>
      <li>In either case: the runtime records execution evidence. caller, capability, version, trust level, outcome, timestamp</li>
    </ol>
    <p>
      The AI agent receives a tool result or a structured error. The system has a complete audit record of what was
      requested, whether it was permitted, and what happened. independent of whether the agent's reasoning was correct.
    </p>
  </section>

  <section class="subpage-callout">
    <strong>Go deeper</strong>
    <div class="subpage-inline-links">
      <a href="../../core-model/what-is-wasm-mcp/">What is WASM MCP in UMA?</a>
      <a href="../../evolve-uma/mcp-wasm-ai-native-microservices/">MCP and WebAssembly: AI-native microservices</a>
      <a href="../../core-model/what-belongs-in-the-runtime-layer/">What belongs in the runtime layer?</a>
      <a href="../../learn-uma/chapter-13-ai-agents-mcp-runtime/">Chapter 13. Agents, MCP, and the Runtime of Reasoning</a>
      <a href="../../learn-uma/book/">The UMA book</a>
    </div>
  </section>
</div>

<section id="contacts" class="section contacts-band" data-shared-footer></section>
