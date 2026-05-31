---
ref: uma-production-readiness
title: "UMA Production Readiness"
subtitle: "UMA production readiness Production readiness in UMA is not just the question of whether portable code can run. The stronger question is whether the runtime can govern that code under real trust, versioning, observability, and deployment constraints."
macro_area: how-uma-works
content_type: walkthrough
slug: uma-production-readiness
canonical_url: "https://www.universalmicroservices.com/uma-production-readiness/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "Understand what production readiness means in UMA across security, versioning, governance, observability, deployment, and runtime authority."
breadcrumbs:
  - "Home"
  - "How Uma Works"
  - "UMA Production Readiness"
related_refs:
  - architecture-drift-and-portable-business-logic
  - incremental-uma-adoption
  - migrating-to-uma-incrementally
  - portable-business-logic
---

## intro

<section class="subpage-hero">
          <h1>UMA production readiness</h1>
          <p>Production readiness in UMA is not just the question of whether portable code can run. The stronger question is whether the runtime can govern that code under real trust, versioning, observability, and deployment constraints.</p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>Security and trust</h2>
            <p>A portable service can create new security pressure because the same behavior may run in different contexts. UMA treats trust boundaries, permissions, provenance, and authority as part of the runtime model.</p>
            <p>The runtime must know what is allowed before it executes.</p>
          </section>

          <section>
            <h2>Versioning and compatibility</h2>
            <p>A production UMA system needs clear contracts, compatibility checks, and versioned evolution. Without that discipline, portability can turn into version sprawl.</p>
            <p>The service graph should make change visible rather than leaving teams to discover drift through incidents.</p>
          </section>

          <section>
            <h2>Governance</h2>
            <p>Governance means deciding who can expose a capability, who can call it, what evidence is required, and which runtime path is authoritative. UMA makes those questions architecture questions, not only operations questions.</p>
            <p>This is especially important when AI-assisted workflows participate in discovery or proposal.</p>
          </section>

          <section>
            <h2>Observability</h2>
            <p>Observability should show more than whether a process is alive. A UMA runtime should expose capability selection, validation decisions, approvals, rejections, execution paths, and relevant traces.</p>
            <p>That evidence is what makes runtime decisions explainable.</p>
          </section>

          <section>
            <h2>Deployment</h2>
            <p>Deployment readiness depends on the hosts involved. Browser, edge, cloud, WASI, and workflow execution each introduce different constraints. UMA does not erase those differences. It gives teams a model for deciding where a capability should run and why.</p>
          </section>
          <section class="subpage-grid">
            <article class="subpage-card"><h3>Security</h3><p>Trust, permissions, and provenance must travel with the runtime decision.</p></article>
            <article class="subpage-card"><h3>Versioning</h3><p>Contracts and compatibility need to be visible as the system evolves.</p></article>
            <article class="subpage-card"><h3>Governance</h3><p>Capability exposure and execution authority need explicit ownership.</p></article>
            <article class="subpage-card"><h3>Observability</h3><p>The runtime should explain decisions, not only report activity.</p></article>
          </section>
          <section>
            <h2>The three readiness gates</h2>
            <p>Production readiness in UMA has three concrete gates. Until all three are met, the system is not production-ready in the UMA sense — it may be deployable, but it is not governed.</p>
            <p><strong>Contract coverage.</strong> Every capability that participates in a production execution path must have an active descriptor with a validated schema. Not a file that exists; a file that the runtime reads, evaluates, and enforces on every invocation. Coverage means the descriptor is wired into the runtime decision path, not sitting in a repository as documentation.</p>
            <p><strong>Parity proof.</strong> Every portable service must have a CI-enforced equivalence test across at least two runtimes. The proof must run on every push. A parity test that runs locally but is not in CI is not a parity proof — it is a local experiment. Runtimes diverge silently; the only way to catch drift before production is to compare outputs automatically on every change.</p>
            <p><strong>Runtime governance.</strong> The runtime layer must enforce trust, validate contracts, and record execution evidence across all deployment paths — not just the production server path. This includes edge deployments, AI-assisted execution paths, and any path where the same portable binary runs in a context other than the primary deployment. Governance that covers only one path creates a false confidence boundary. The system is only as governed as its least-governed execution path.</p>
          </section>

          <section>
            <h2>Common gaps teams hit before production</h2>
            <p>Most teams that reach a late-stage pre-production review have artifacts in place but enforcement missing. Three patterns recur.</p>
            <p><strong>Descriptors written but not enforced.</strong> The runtime reads the descriptor but does not block on violations. This is worse than having no descriptor: it creates the appearance of governance while providing none of the protection. A descriptor the runtime ignores is documentation. A descriptor the runtime enforces on every invocation is a contract. These are not the same thing, and the gap between them is invisible until an incident exposes it.</p>
            <p><strong>Parity proofs written but not in CI.</strong> Tests exist locally and pass. They are not configured to run on every push. The result is that the proofs go stale silently — a change to the core or the adapter breaks behavioral equivalence, and nobody knows until the divergence surfaces in production behavior. A parity proof that does not run continuously is not a proof; it is a snapshot.</p>
            <p><strong>Governance present in the server path but absent in the edge or AI-assisted path.</strong> The primary deployment has a fully governed runtime. The edge deployment or the AI agent tool executor runs the same binary without a runtime layer, without contract validation, without execution evidence. This is the most dangerous gap because the system tests well against the governed path and fails in production only when traffic hits the ungoverned path. Partial governance is worse than no governance because it makes the system appear production-ready when it is not.</p>
          </section>

          <section>
            <h2>What production readiness is not</h2>
            <p>Production readiness in UMA is a continuous enforcement posture, not a set of artifacts. Three common misreadings need direct correction.</p>
            <p>It is not "the WASM binary compiles." A binary that compiles proves the toolchain works. It does not prove that the behavior is governed, that the contract is enforced, or that the parity across runtimes holds. Compilation is a precondition, not a gate.</p>
            <p>It is not "the descriptor file exists." A descriptor file in a repository is documentation. The gate is whether the runtime reads that descriptor and acts on it — approves or rejects the execution path based on what the descriptor declares. Existence without enforcement is not production readiness.</p>
            <p>It is not "we ran the parity test once." A parity test run at a point in time proves behavioral equivalence at that point. It does not prove it on the next change, or the change after that. Production readiness requires CI enforcement: the test runs on every push to main, blocks merges on failure, and is treated as a first-class gate rather than an optional check.</p>
            <p>The definition is: a CI that proves contract coverage, parity, and governance on every change, and a runtime that enforces those properties on every execution. Anything less is a system that is on its way to production readiness, not there yet.</p>
          </section>

          <section class="subpage-callout">
            <strong>Want to go deeper?</strong>
            <p>This page answers the production question at a conceptual level. The book goes further into governance, deployment patterns, runtime controls, and implementation guidance. The repository shows the proof artifacts that make those discussions concrete.</p>
            <div class="subpage-inline-links">
              <a href="../trust-boundaries/">Trust boundaries</a>
              <a href="../what-makes-a-system-coherent/">What makes a system coherent?</a>
              <a href="../what-makes-a-decision-discoverable/">What makes a decision discoverable?</a>
              <a href="../examples/chapter-09-trust-boundaries/">Trust boundaries example</a>
              <a href="https://github.com/enricopiovesan/UMA-code-examples">Official GitHub examples repository</a>
              <a href="../book/">Book page</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
