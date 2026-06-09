---
ref: chapter-13-ai-agents-mcp-runtime
title: "Ch.13: Agents, MCP, and Reasoning"
subtitle: "How does UMA's runtime model extend to AI-native execution, where an agent rather than a human initiates service invocation?"
macro_area: learn-uma
content_type: overview
slug: chapter-13-ai-agents-mcp-runtime
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-13-ai-agents-mcp-runtime/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "Chapter 13: AI agents and MCP runtime in UMA. How AI participates in distributed execution while the runtime stays authoritative and auditable."
breadcrumbs:
  - "Home"
  - "Learn UMA"
  - "Chapter 13: Agents, MCP, and the Runtime of Reasoning"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
  <h1>Chapter 13: Agents, MCP, and the Runtime of Reasoning</h1>
  <p>How does UMA's runtime model extend to AI-native execution, where an agent rather than a human initiates service invocation?</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The question this chapter answers</h2>
    <p>When an AI agent calls a service, the governance requirements do not relax. they become more important. The agent cannot be audited by reading its code the way a deterministic caller can. Its execution path depends on model state, prompt context, and prior tool results. If the services it calls do not enforce their own contracts, the agent becomes the de facto authority over what is valid input and what constitutes a successful call. That is the wrong place for that authority to live. Chapter 13 addresses how UMA's runtime model applies when the caller is an agent.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>The governance requirements for agent-initiated invocations are the same as for any other invocation: the service must declare its contract, the runtime must validate input against that contract, trust must be enforced at the boundary, and execution evidence must be recorded. The caller being an agent rather than a deterministic process does not change any of these requirements.</p>
    <p>Model Context Protocol (MCP) gives agents a structured mechanism for discovering and invoking capabilities. A capability exposed over MCP has a name, a parameter schema, and a response format: the same elements that appear in a UMA descriptor. UMA gives those MCP-exposed capabilities a portable boundary that survives the shift from deterministic to AI-influenced execution paths. The runtime remains the authority over what is valid. the agent is a well-governed participant.</p>
    <p>Chapter 13 builds a portable MCP-compatible service and shows the runtime governing agent-initiated invocations the same way it governs any other execution. The service does not know or care that its caller is an agent. The contract is honored either way.</p>
  </section>

  <section>
    <h2>Why AI agents are a new class of runtime participant</h2>
    <p>Classical service callers are deterministic. They call specific capabilities with specific inputs at specific times, and those decisions are encoded in code that was reviewed before deployment. When a caller misbehaves (sends invalid inputs, calls capabilities in wrong sequence, exceeds its declared authority) that behavior can be traced to a specific code path, reviewed, and corrected. The review happens before deployment, not after execution.</p>
    <p>AI agents are non-deterministic in a structurally different way. The decision of what to call, with what inputs, at what time is made by a reasoning model at inference time. That decision depends on model state, prompt context, the results of prior tool calls, and inference-time variation that is not reproducible from inputs alone. The agent's behavior cannot be fully reviewed before deployment because its execution decisions do not exist until runtime. A governance model that relies on pre-deployment review of caller behavior cannot apply to AI agents in the same way it applies to deterministic callers.</p>
    <p>This changes what "enforcement" means. For deterministic callers, enforcement can include pre-deployment review that catches bad decisions before they reach production. For AI agents, enforcement must happen at the runtime boundary (every time, for every invocation) because there is no pre-deployment review step that can substitute for it. The runtime is the only point in the system where every invocation decision, regardless of who made it, can be evaluated against a consistent set of declared requirements.</p>
  </section>

  <section>
    <h2>How UMA's runtime governance extends to AI-native paths</h2>
    <p>The runtime treats the AI agent as a caller like any other. Before a capability executes, the runtime evaluates the invocation against the capability's active descriptor: contract requirements, trust policy, input validation, data classification constraints. The caller being an AI agent rather than a deterministic process does not alter this evaluation. The descriptor does not have a carve-out for agent-initiated calls, and neither does the trust model.</p>
    <p>An agent's use of MCP to discover and invoke capabilities does not bypass governance. MCP gives the agent a structured interface for capability discovery and invocation: it describes what capabilities are available, what they accept, and what they return. UMA gives those capabilities an active descriptor that the runtime enforces regardless of how the invocation arrived. The agent calls the capability through MCP. the runtime evaluates the descriptor before execution proceeds. The two layers are complementary, not redundant.</p>
    <p>What this produces is a governance model that is uniform across execution paths. A capability invoked by a batch processing pipeline, a browser-side workflow, and an AI agent in a reasoning loop is governed by the same descriptor, evaluated by the same runtime, producing the same class of execution evidence. The evidence records for an agent-initiated invocation and a classical service invocation have the same structure. This uniformity is what makes Chapter 13's reference system different from a typical MCP server implementation: governance is not an add-on applied to classical paths and omitted from agent paths. It is a property of the execution model itself.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 12 established how UMA systems evolve without drift. Chapter 13 applies that stability guarantee to a new class of execution environment: one where the caller is non-deterministic and behavioral drift pressure is higher. Chapter 14 assembles the full system: portable services, runtime governance, trust boundaries, and MCP-compatible endpoints running together as an integrated reference application.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-12-evolving-distributed-systems/">← Chapter 12: Evolving and Adapting UMA Systems</a>
      <a href="../chapter-14-uma-reference-application/">Chapter 14: The Reference Experience →</a>
      <a href="../../evolve-uma/ai-native-runtime-governance/">AI-Native Runtime Governance</a>
      <a href="../../evolve-uma/mcp-wasm-ai-native-microservices/">MCP, WASM, and AI-Native Microservices</a>
      <a href="../examples/chapter-13-portable-mcp-runtime/">Chapter 13 code examples</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
