---
ref: portable-business-logic
title: "Portable Business Logic Across Browser, Server, and Edge"
subtitle: "How to write a pricing rule, validation check, or eligibility formula once and run it identically everywhere without copy-paste."
macro_area: use-cases
content_type: concept
slug: portable-business-logic
canonical_url: "https://www.universalmicroservices.com/use-cases/portable-business-logic/"
left_nav_group: use-cases
seo_description: "Write business logic once and run it on browser, server, edge, and AI agents without duplicating code. How UMA portable services solve the multi-environment problem."
breadcrumbs:
  - "Home"
  - "Use Cases"
  - "Portable Business Logic"
related_refs:
  - core-model
  - wasm-microservices
  - how-uma-works
---

## intro

<section class="subpage-hero">
  <h1>Portable business logic across browser, server, and edge</h1>
  <p>How to write a pricing rule, validation check, or eligibility formula once and run it identically everywhere without copy-paste.</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The problem: logic drift across environments</h2>
    <p>Most non-trivial products apply the same business rules in more than one place. A pricing formula runs on the server when generating an invoice, in the browser when showing a live total, and at the CDN edge when personalizing a landing page. A validation rule runs in a form input handler, in an API endpoint, and in a background job that reprocesses historical records.</p>
    <p>When those rules live in the same codebase, they start identical. After a few months they do not. The backend version gains a special case for a new product category. The browser version keeps an old rounding behavior because the team forgot to update it. The edge version is a rough approximation written in a different language by a different team under time pressure. At some point, the three versions agree on most inputs and silently disagree on the rest.</p>
    <p>That divergence is not a discipline failure. It is what happens when the same logic is owned by different layers, tested in different pipelines, and deployed on different release cadences.</p>
  </section>

  <section>
    <h2>Why it keeps happening</h2>
    <p>The standard response to this problem is a shared library. Extract the rule into a package, import it everywhere, and update one place. In a single-language monorepo, this works reasonably well. In a real production system, it fails in predictable ways.</p>
    <p>The browser cannot import a Node.js module with database access patterns baked in. The edge function runtime does not support the same standard library as the server. The AI agent calling the rule at inference time needs a stateless, sandboxed invocation with no ambient I/O. Each environment has different constraints, and a shared library written for one environment tends to accumulate assumptions that make it unusable in the others.</p>
    <p>The result is that teams maintain separate implementations. The implementations start aligned and drift over time. Drift is discovered late, usually in production, usually after the business has already made decisions based on inconsistent outputs.</p>
  </section>

  <section>
    <h2>How UMA solves it</h2>
    <p>A UMA portable service packages business logic as a WebAssembly module paired with an active descriptor. The WASM module contains the compiled logic. The descriptor declares what the module expects as input, what it produces as output, and what capabilities it requires from the host. The module has no ambient I/O access by default. It cannot open sockets, read files, or touch environment variables unless the host explicitly grants those capabilities at startup.</p>
    <p>Because the module targets the WASM binary format, not a specific language runtime or OS, the same artifact runs in a browser via the native WebAssembly API, in a Cloudflare Worker or Fastly Compute instance at the edge, in a wasmtime process on a server, and inside an AI agent tool invocation. The execution semantics are defined by the WASM specification. The behavior is deterministic across all four targets from the same binary.</p>
    <p>The active descriptor is what makes the service governable rather than just portable. It carries the typed interface contract, versioning information, and the capability surface. A runtime can inspect the descriptor before executing to verify that its environment satisfies what the module needs. This moves compatibility checks from runtime surprises to load-time validation.</p>
    <p>Ownership of the rule stays with one team. That team ships one artifact. Every environment runs that artifact. When the rule changes, every environment gets the updated behavior on the next deployment without coordination across teams.</p>
  </section>

  <section>
    <h2>Concrete example: a pricing rule across three targets</h2>
    <p>Consider a pricing rule that applies volume discounts, promotional codes, and region-specific tax rates. The rule takes a cart snapshot and a customer context object as inputs and returns a priced total with a breakdown of applied adjustments.</p>
    <p>Written as a UMA portable service:</p>
    <ul>
      <li><strong>In the browser.</strong> The WASM module is loaded once at page load. The pricing rule runs locally on every cart update with no round trip. The customer sees accurate totals while editing quantity fields, applying promo codes, or switching shipping regions. The module works offline. It does not call the server for every recalculation.</li>
      <li><strong>On the server.</strong> The same module runs inside the order processing service at checkout. The server hosts a wasmtime runtime, loads the module from the service registry, and invokes it with the final cart state before writing the order record. The output is bit-for-bit identical to what the browser showed the customer, because both ran the same module.</li>
      <li><strong>At the CDN edge.</strong> The same module runs inside a Cloudflare Worker to personalize pricing previews on category pages for logged-in users. The edge worker fetches the module from the registry, instantiates it with the customer's regional context, and renders the adjusted price directly in the response. No origin request is needed for the pricing calculation.</li>
    </ul>
    <p>When the finance team adds a new discount tier in January, the team that owns the pricing service ships one updated WASM artifact. The browser, the server, and the edge all pick up the change on their next module load. There is no cross-team coordination. There is no risk that the browser version misses the update because it lives in a different repository.</p>
  </section>

  <section>
    <h2>Where to go next</h2>
    <p>Portable business logic is one application of the broader UMA model. The approach works because the core model separates what a service does from where it runs, and because WASM provides an execution boundary that holds across environments without recompilation.</p>

    <div class="callout-box">
      <h3>Go deeper</h3>
      <ul>
        <li><a href="/core-model/">The UMA core model</a> — how active descriptors and portable services fit together as an architecture.</li>
        <li><a href="/wasm-microservices/">WASM microservices</a> — the execution model that makes environment-agnostic deployment possible.</li>
        <li><a href="/how-uma-works/">How UMA works</a> — end-to-end walkthrough of the runtime, registry, and governance layer.</li>
        <li><a href="https://www.amazon.com/" target="_blank" rel="noopener">Pre-order on Amazon</a> — the full treatment of portable service design, with worked examples in Rust and TypeScript.</li>
      </ul>
    </div>
  </section>
</div>
