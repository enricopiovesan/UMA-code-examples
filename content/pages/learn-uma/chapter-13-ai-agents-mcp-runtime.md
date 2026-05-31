---
ref: chapter-13-ai-agents-mcp-runtime
title: "Chapter 13: Agents, MCP, and the Runtime of Reasoning"
subtitle: "How does UMA's runtime model extend to AI-native execution, where an agent rather than a human initiates service invocation?"
macro_area: learn-uma
content_type: overview
slug: chapter-13-ai-agents-mcp-runtime
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-13-ai-agents-mcp-runtime/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "UMA runtime governance for AI agents and MCP-compatible services."
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
    <p>When an AI agent calls a service, the governance requirements do not relax — they become more important. The agent cannot be audited by reading its code the way a deterministic caller can. Its execution path depends on model state, prompt context, and prior tool results. If the services it calls do not enforce their own contracts, the agent becomes the de facto authority over what is valid input and what constitutes a successful call. That is the wrong place for that authority to live. Chapter 13 addresses how UMA's runtime model applies when the caller is an agent.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>The governance requirements for agent-initiated invocations are the same as for any other invocation: the service must declare its contract, the runtime must validate input against that contract, trust must be enforced at the boundary, and execution evidence must be recorded. The caller being an agent rather than a deterministic process does not change any of these requirements.</p>
    <p>Model Context Protocol (MCP) gives agents a structured mechanism for discovering and invoking capabilities. A capability exposed over MCP has a name, a parameter schema, and a response format — the same elements that appear in a UMA descriptor. UMA gives those MCP-exposed capabilities a portable boundary that survives the shift from deterministic to AI-influenced execution paths. The runtime remains the authority over what is valid; the agent is a well-governed participant.</p>
    <p>Chapter 13 builds a portable MCP-compatible service and shows the runtime governing agent-initiated invocations the same way it governs any other execution. The service does not know or care that its caller is an agent. The contract is honored either way.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 12 established how UMA systems evolve without drift. Chapter 13 applies that stability guarantee to a new class of execution environment — one where the caller is non-deterministic and behavioral drift pressure is higher. Chapter 14 assembles the full system: portable services, runtime governance, trust boundaries, and MCP-compatible endpoints running together as an integrated reference application.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-12-evolving-distributed-systems/">← Chapter 12: Evolving and Adapting UMA Systems</a>
      <a href="../chapter-14-uma-reference-application/">Chapter 14: The Reference Experience →</a>
      <a href="../../evolve-uma/ai-native-runtime-governance/">AI-Native Runtime Governance</a>
      <a href="../../evolve-uma/mcp-wasm-ai-native-microservices/">MCP, WASM, and AI-Native Microservices</a>
      <a href="../examples/chapter-13-portable-mcp-runtime/">Chapter 13 code examples</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Get the book on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
