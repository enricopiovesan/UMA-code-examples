---
ref: end-to-end-feature-flag-example
title: "End-to-End UMA Example: Feature Flag Service"
subtitle: "End-to-end UMA example: feature flag service A feature flag service is a useful end-to-end UMA example because the business behavior is small, testable, and easy to duplicate badly. The same decision often appears in frontend checks, backend authority, edge routing, and workflow logic. UMA turns that repeated rule into one governed capability."
macro_area: learn-uma
content_type: onboarding
slug: end-to-end-feature-flag-example
canonical_url: "https://www.universalmicroservices.com/end-to-end-feature-flag-example/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "Follow an end-to-end UMA feature flag service from requirement to capability, contract, build, browser execution, cloud execution, and AI-assisted execution."
breadcrumbs:
  - "Home"
  - "Learn Uma"
  - "End-to-End UMA Example: Feature Flag Service"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
          <h1>End-to-end UMA example: feature flag service</h1>
          <p>A feature flag service is a useful end-to-end UMA example because the business behavior is small, testable, and easy to duplicate badly. The same decision often appears in frontend checks, backend authority, edge routing, and workflow logic. UMA turns that repeated rule into one governed capability.</p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>1. Requirement</h2>
            <p>The product needs to decide whether a feature is enabled for a user. The decision may depend on country, account, rollout percentage, or a fallback rule. The important part is that every runtime should agree on the same outcome.</p>
          </section>

          <section>
            <h2>2. Capability</h2>
            <p>The capability is feature flag evaluation. It is narrow enough to test and important enough to keep coherent. That makes it a good first Universal Microservice candidate.</p>
          </section>

          <section>
            <h2>3. Contract</h2>
            <p>The contract defines the input context, flag rules, and decision output. A reader should be able to inspect the contract without needing to understand a specific UI framework or cloud provider.</p>
            <p>The contract is where the service becomes understandable before it becomes deployable.</p>
          </section>

          <section>
            <h2>4. Build</h2>
            <p>The portable core implements deterministic evaluation. In the repository, the Rust-first path is the authoritative implementation, with TypeScript parity where present to make comparison visible.</p>
          </section>

          <section>
            <h2>5. Browser execution</h2>
            <p>A browser can use the capability when low-latency local decisions are appropriate. The runtime still has to decide which data is safe and which decisions require backend authority.</p>
          </section>

          <section>
            <h2>6. Cloud execution</h2>
            <p>Cloud execution can provide authority, auditability, and integration with backend systems. UMA does not force the choice. It makes the runtime decision explicit.</p>
          </section>

          <section>
            <h2>7. AI-assisted execution</h2>
            <p>An AI-assisted flow may propose a flag change, request an evaluation, or summarize rollout behavior. The runtime remains authoritative over validation and execution. The agent participates, but it does not become the system of record.</p>
          </section>
          <section class="subpage-grid">
            <article class="subpage-card"><h3>Website</h3><p>Conceptual walkthrough of the end-to-end model.</p></article>
            <article class="subpage-card"><h3>GitHub</h3><p>Runnable examples and validation scripts.</p></article>
            <article class="subpage-card"><h3>Book</h3><p>Methodology, tradeoffs, governance, and production guidance.</p></article>
            <article class="subpage-card"><h3>Runtime</h3><p>The authority that decides where execution belongs.</p></article>
          </section>
          <section class="subpage-callout">
            <strong>Want to go deeper?</strong>
            <p>This walkthrough tells the complete story at a high level. The Chapter 4 example lets you run the evaluator, and the book connects the same pattern to contracts, runtime governance, and later production concerns.</p>
            <div class="subpage-inline-links">
              <a href="../examples/chapter-04-feature-flag-evaluator/">Feature flag tutorial</a>
              <a href="../what-is-a-universal-microservice/">What is a Universal Microservice?</a>
              <a href="../what-is-a-capability/">What is a capability?</a>
              <a href="../agent-vs-runtime/">Agent vs runtime</a>
              <a href="https://github.com/enricopiovesan/UMA-code-examples">Official GitHub examples repository</a>
              <a href="../book/">Book page</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
