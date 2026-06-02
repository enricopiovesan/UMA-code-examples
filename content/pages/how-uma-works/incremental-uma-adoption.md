---
ref: incremental-uma-adoption
title: "Incremental UMA Adoption"
subtitle: "Incremental UMA adoption UMA does not require a rewrite. It can start with one portable behavior, one contract, or one runtime-governed boundary inside a system that still has legacy services and ordinary deployment infrastructure."
macro_area: how-uma-works
content_type: walkthrough
slug: incremental-uma-adoption
canonical_url: "https://www.universalmicroservices.com/incremental-uma-adoption/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "Learn how UMA can begin with one portable capability and coexist with legacy systems through adapters, governed boundaries, and gradual runtime adoption."
breadcrumbs:
  - "Home"
  - "How Uma Works"
  - "Incremental UMA Adoption"
related_refs:
  - architecture-drift-and-portable-business-logic
  - migrating-to-uma-incrementally
  - portable-business-logic
  - runtime-agnostic-architecture
---

## intro

<section class="subpage-hero"><h1>Incremental UMA adoption</h1><p>UMA does not require a rewrite. It can start with one portable behavior, one contract, or one runtime-governed boundary inside a system that still has legacy services and ordinary deployment infrastructure.</p></section>

## main

<div class="subpage-body">
          <section>
            <h2>Start with the boundary</h2>
            <p>The practical first step is not replacing your platform. It is finding a behavior that is duplicated across services, environments, or teams (a validation rule, a pricing calculation, a feature flag evaluator) and moving that behavior behind an explicit contract and a runtime-evaluated boundary.</p>
            <p>The rest of the stack does not change. The legacy services still run. The existing deployment infrastructure still operates. The only thing that changes is that one behavior, previously embedded in the host, is now expressed as a portable service with a declared contract, wrapped by a runtime that validates, binds adapters, and records lifecycle evidence. Adapters let existing hosts participate without modification: the host that was previously calling an internal function now calls through an adapter that the runtime selects.</p>
            <p>This is incremental adoption in the most literal sense: one boundary moves, the rest stays.</p>
          </section>

          <section class="subpage-grid">
            <article class="subpage-card"><h3>One behavior</h3><p>Choose a rule or normalization path that is repeated across environments: a validation function called in three places, a pricing rule embedded in two services, a flag evaluator reimplemented for each deployment target.</p></article>
            <article class="subpage-card"><h3>One contract</h3><p>Define what the behavior accepts, returns, emits, and depends on. The contract is the source of truth for what the portable core does. not documentation, not convention, but a machine-readable descriptor that the runtime enforces.</p></article>
            <article class="subpage-card"><h3>One runtime wrapper</h3><p>Add validation, adapter binding, and lifecycle evidence around the portable core. The runtime selects the right adapter for the host, validates input before side effects happen, and records what ran and why.</p></article>
            <article class="subpage-card"><h3>One migration path</h3><p>Keep legacy internals where needed while governing the boundary. The first milestone is not eliminating legacy code: it is making one boundary explicit, which creates the proof that the pattern works in your context.</p></article>
          </section>

          <section>
            <h2>Three common entry points</h2>
            <p>Most teams find one of three behaviors easiest to move first, because each is already isolated in intent even if not in implementation:</p>
            <p><strong>A shared validation rule.</strong> Input validation logic that is duplicated across services, with slight variations that cause inconsistencies at the edges. The rule has clear inputs and outputs, no side effects, and its inconsistency is already causing bugs. Expressing it as a portable service with a single contract eliminates the drift immediately. The existing callers each get a thin adapter. the rule itself becomes testable in isolation against the contract.</p>
            <p><strong>A pricing engine or rate calculator.</strong> Business logic that must produce the same result regardless of whether it runs on a server, in a browser, or at an edge node. This is the canonical UMA target: behavior that cannot afford inconsistency across contexts. Wrapping it as a portable service with a runtime-selected adapter for each deployment target makes the equivalence provable rather than assumed. The Chapter 6 portability lab demonstrates this pattern directly: one contract, two execution targets (native and WASI), parity checked from emitted events.</p>
            <p><strong>A feature flag evaluator.</strong> Evaluation logic that determines which code paths run based on rules, user attributes, or rollout percentages. This is the entry point used in Chapter 4 of the book. deliberately chosen because it is small enough to implement completely, complex enough to have meaningful contract semantics (country matching, percentage rollout, default fallback), and important enough that inconsistency has visible consequences. The WASI CLI built in Chapter 4 can be called from any host with a one-line adapter without changing the evaluator.</p>
          </section>

          <section>
            <h2>What the first milestone looks like</h2>
            <p>The first milestone in incremental UMA adoption is not a complete platform migration. It is a before-and-after proof: one behavior that was previously embedded in a host, now running as a portable service behind a contract, with a runtime that produces lifecycle evidence, and a parity check that confirms the output matches the previous implementation.</p>
            <p>That proof serves two purposes. Technically, it validates that the adapter model works for your specific host and deployment context, that the runtime can select the right adapter, that the contract semantics match what the host needs, and that the portable core passes the same cases the legacy implementation handled. Organizationally, it demonstrates that the approach is not speculative. The smoke script runs, the CI passes, and the output is inspectable.</p>
            <p>From that first milestone, adoption proceeds incrementally: each subsequent capability that crosses runtime boundaries is a candidate for the same treatment. The system does not need to commit to a full rewrite. It accumulates portable boundaries at the rate that makes sense for the team, while the legacy surfaces continue to function through adapters.</p>
          </section>

          <section>
            <h2>Avoiding fragmentation as adoption grows</h2>
            <p>Incremental adoption introduces a risk: if each team that adopts UMA independently develops its own contract conventions, its own adapter patterns, and its own runtime configuration, the result is a new form of fragmentation. not the fragmentation of duplicated logic, but the fragmentation of incompatible portable boundaries.</p>
            <p>The antidote is shared contract definitions and a consistent runtime governance model. When capability descriptors are defined in a shared location and referenced by multiple services, the runtime can enforce compatibility across the system rather than just within each service. Chapter 11 in the book covers this directly: how to structure adoption so that each new portable boundary integrates with the existing governance model rather than creating a parallel one.</p>
          </section>

          <section class="subpage-callout">
            <strong>See the entry points in running code</strong>
            <p>Chapters 5 and 6 show the first runtime and portability steps: the smallest complete examples of an incremental boundary move. Chapter 11 addresses the governance model that prevents fragmentation as adoption grows.</p>
            <div class="subpage-inline-links">
              <a href="../../core-model/what-is-a-capability/">What is a capability</a>
              <a href="../migrating-to-uma-incrementally/">Migrating to UMA incrementally</a>
              <a href="../../learn-uma/chapter-05-building-portable-microservices/">Chapter 5: Building portable microservices</a>
              <a href="../../proof/what-makes-a-service-portable/">What makes a service portable</a>
              <a href="../../why-uma/what-problem-does-uma-solve/">What problem does UMA solve</a>
              <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Universal Microservices Architecture (book)</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
