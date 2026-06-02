---
ref: active-descriptors
title: "Active Descriptors in UMA"
subtitle: "Active descriptors In UMA, a descriptor is not just documentation beside a service. It is metadata the runtime can evaluate to understand the service boundary, validate inputs and outputs, check compatibility, and record why execution was allowed."
macro_area: core-model
content_type: explainer
slug: active-descriptors
canonical_url: "https://www.universalmicroservices.com/core-model/active-descriptors/"
left_nav_group: core-model
chapter_ref: null
seo_description: "Learn how UMA uses active descriptors as runtime-evaluated constraints for contracts, schemas, latency expectations, compatibility, and execution evidence."
breadcrumbs:
  - "Home"
  - "Core Model"
  - "Active Descriptors in UMA"
related_refs:
  - agent-vs-runtime
  - late-bound-policy-enforcement
  - what-belongs-in-the-runtime-layer
  - what-is-a-capability
---

## intro

<section class="subpage-hero">
          <h1>Active descriptors</h1>
          <p>In UMA, a descriptor is not just documentation beside a service. It is metadata the runtime can evaluate to understand the service boundary, validate inputs and outputs, check compatibility, and record why execution was allowed.</p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>The useful idea</h2>
            <p>An active descriptor describes a capability in terms the runtime can use: schemas, emitted events, required inputs, allowed placements, version constraints, and the evidence expected from a run. It is active because it participates in execution decisions.</p>
            <p>This does not mean the contract replaces service logic. The contract is an executable constraint model around the logic, while the service still owns the durable behavior.</p>
          </section>
          <section>
            <h2>Why "active" matters</h2>
            <p>A README beside a service is documentation for humans. An OpenAPI spec sitting in a repository is documentation for other developers. Neither of these is read by the runtime when it decides whether to approve an execution path. They exist to inform. They do not constrain.</p>
            <p>An active descriptor is different. The runtime reads it during a real execution decision. It is a constraint model the system evaluates before approving a path, not a reference artifact a developer consults after something breaks. The word "active" is load-bearing: it means the descriptor has operational effect, not just informational value.</p>
            <p>This distinction matters in practice because documentation drifts and constraint models do not. Or at least: when a constraint model drifts, the system knows about it. When a README drifts, the developer finds out at 2am during an incident.</p>
          </section>
          <section>
            <h2>What an active descriptor contains</h2>
            <p>The shape of a descriptor is specific enough for the runtime to reason with. A fully declared descriptor includes an input schema, an output schema, the events the capability emits, the events it requires, a version declaration, allowed placement targets, latency expectations, error expectations, and evidence expectations: the record of what a valid run is expected to produce.</p>
            <p>Not every field is mandatory on every capability. A simple deterministic evaluator may have no event declarations. A capability that runs at the edge may have explicit latency expectations but relaxed evidence requirements. The descriptor is expressive enough to capture the full contract, and minimal enough that teams can adopt it incrementally.</p>
            <p>What the descriptor does not contain is implementation detail. It does not describe how the capability works internally. It describes what the runtime needs to know to decide whether the capability can participate in a given execution context.</p>
          </section>
          <section>
            <h2>How the runtime uses it</h2>
            <p>When a capability arrives with a descriptor, the runtime follows a specific decision sequence. First, it checks input schema compatibility: does the data flowing into this capability match what the descriptor declares? Second, it checks version constraints against other capabilities already participating in the workflow. Third, it checks placement rules: is this capability allowed to run in this execution environment? Fourth, it validates declared permissions against the trust model.</p>
            <p>If all checks pass, the execution path is approved. The descriptor's evidence expectations then shape what gets recorded after the run: what the system will use to confirm that the capability behaved as declared, not just that it returned a value.</p>
            <p>If any check fails, the path is rejected with a specific reason. The runtime does not proceed optimistically. It does not log a warning and continue. The descriptor is authoritative.</p>
          </section>
          <section>
            <h2>What active descriptors prevent</h2>
            <p>Silent interface drift: when a producer changes its output format but consumers are not updated, systems break at runtime rather than at contract validation. Descriptors make that drift visible before execution reaches production.</p>
            <p>Undeclared dependency coupling: when a service assumes shared state or a side channel that is not in its contract, that assumption is invisible to the runtime and to any team consuming the capability. The descriptor forces every dependency to be declared.</p>
            <p>Invisible permission escalation: when a service accesses a capability it never declared in its contract, the runtime has no way to enforce policy on that access. With descriptors, every permission claim is visible and every undeclared access is rejectable.</p>
          </section>
          <section class="subpage-grid">
            <article class="subpage-card"><h3>Boundary</h3><p>Input and output schemas define the shape of the portable behavior.</p></article>
            <article class="subpage-card"><h3>Compatibility</h3><p>Version and event declarations help the runtime decide whether services can interact.</p></article>
            <article class="subpage-card"><h3>Expectations</h3><p>Latency, error, and evidence expectations make execution easier to inspect.</p></article>
            <article class="subpage-card"><h3>Authority</h3><p>The runtime evaluates the descriptor before it treats a proposed path as approved.</p></article>
          </section>
          <section>
            <h2>Concrete proof</h2>
            <p>The early examples prove the idea in small pieces: Chapter 4 uses a contract around deterministic flag evaluation, Chapter 6 uses a contract to compare native and WASI execution, and Chapter 7 lets contracts and events shape orchestration.</p>
            <p>Chapter 4 is the simplest entry point: one input schema, one output schema, no event dependencies. It shows the minimum viable descriptor for a capability that has predictable behavior. Chapter 6 builds on that by proving the contract holds across two execution environments (native Rust and WASM) without changing the descriptor. Chapter 7 shows descriptors operating as first-class inputs to orchestration, with events shaping the workflow rather than code.</p>
          </section>
          <section>
            <h2>Questions and answers</h2>
            <dl>
              <dt>Is an active descriptor the same as an OpenAPI schema?</dt>
              <dd>It overlaps with schema definition but is broader. OpenAPI describes an HTTP interface. An active descriptor describes the full capability contract including events, placements, version rules, and expected execution evidence, independent of transport. You can have an OpenAPI spec and a UMA descriptor for the same capability. They serve different audiences.</dd>
              <dt>Does every UMA service need a descriptor?</dt>
              <dd>Yes, by design. A capability without a descriptor is not discoverable by the runtime in the UMA model. The descriptor is what makes the capability composable. Without it, the runtime has no basis for approving the capability's participation in a workflow. It is not optional configuration. It is the thing that makes a service a UMA capability.</dd>
            </dl>
          </section>
          <section class="subpage-callout">
            <strong>Covered in the book</strong>
            <p>This page gives the preview. Chapters 4, 6, and 7 explain how descriptors become practical runtime inputs without turning the website into the full book.</p>
            <div class="subpage-inline-links">
              <a href="../examples/chapter-04-feature-flag-evaluator/">Chapter 4 example</a>
              <a href="../examples/chapter-06-portability-lab/">Chapter 6 example</a>
              <a href="../examples/chapter-07-metadata-orchestration/">Chapter 7 example</a>
              <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order the book</a>
              <a href="../../how-uma-works/uma-mcp-runtime-governance/">MCP runtime governance</a>
              <a href="../../learn-uma/chapter-06-uma-runtime-layer/">Chapter 6: UMA runtime layer</a>
              <a href="../../proof/how-to-prove-portability/">How to prove portability</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
