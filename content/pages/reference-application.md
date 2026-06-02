---
ref: reference-application
title: "UMA Reference Application"
subtitle: "A live, runnable implementation of Universal Microservices Architecture. The reference application demonstrates portable services, governed runtimes, contract enforcement, and behavioral equivalence across execution contexts."
macro_area: null
content_type: reference
slug: reference-application
canonical_url: "https://www.universalmicroservices.com/reference-application/"
left_nav_group: reference-application
seo_description: "The UMA reference application: runnable proof of portable microservices, WebAssembly execution, contract enforcement, and runtime governance across browser, edge, and cloud."
breadcrumbs:
  - "Home"
  - "Reference Application"
related_refs:
  - active-descriptors
  - webassembly-microservices-architecture
  - wasm-microservices-tutorial-rust
---

## intro

<section class="subpage-hero">
          <h1>UMA Reference Application</h1>
          <p>
            A live, runnable implementation of Universal Microservices Architecture. The reference application demonstrates portable services,
            governed runtimes, contract enforcement, and behavioral equivalence across execution contexts.
          </p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>What the reference application is</h2>
            <p>
              The reference application is a single runnable system that demonstrates UMA's core claims in working code. It is not a slide deck,
              a diagram, or a set of aspirational descriptions. Every architectural claim it makes is backed by code that runs, output that can
              be inspected, and CI checks that execute on every commit.
            </p>
            <p>
              Specifically, the application demonstrates portable service boundaries that stay coherent across execution contexts,
              explicit contracts that define behavior without depending on any one runtime, adapter binding that keeps host capabilities
              outside the portable core, runtime governance that validates, approves, and records execution decisions, and behavioral
              equivalence across native, WASI, TypeScript, browser, edge, and cloud paths.
            </p>
            <p>
              The examples repository is the companion to the UMA book. It is structured so that each chapter proves a distinct architectural
              claim, and so the claims compound as the reader progresses from Chapter 4 through Chapter 13.
            </p>
          </section>

          <section class="subpage-grid">
            <article class="subpage-card">
              <h3>What it demonstrates</h3>
              <p>Portable service boundaries, explicit contracts, adapter binding, runtime governance, and behavioral equivalence: each proven with runnable output, not just described.</p>
            </article>
            <article class="subpage-card">
              <h3>How to verify it</h3>
              <p>Reader Smoke CI and Business Logic Coverage CI run on every commit. The 100% business logic coverage badge means the portable core is verified independently of any host environment.</p>
            </article>
          </section>

          <section>
            <h2>What it proves</h2>
            <p>
              Each chapter example proves a specific UMA claim with runnable output. The claims are not described and left for the reader to
              accept. They are verified by CI on every commit to the repository.
            </p>
            <p>
              The CI badges at the top of the repository reflect that state:
            </p>
            <ul>
              <li><strong>Reader Smoke</strong>: validates the complete guided reader path end to end</li>
              <li><strong>Business Logic Coverage</strong>: reports 100% coverage of the portable core, independently of any host</li>
              <li><strong>Benchmark Proof</strong>: validates the published footprint numbers against the actual binaries</li>
            </ul>
            <p>
              These badges are live:
              <a href="https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/reader-smoke.yml">Reader Smoke CI</a> ·
              <a href="https://github.com/enricopiovesan/UMA-code-examples/actions/workflows/business-logic-coverage.yml">Business Logic Coverage CI</a>
            </p>
          </section>

          <section>
            <h2>Chapter walkthrough</h2>
            <p>
              The three foundational chapters each prove a distinct layer of the UMA model.
            </p>

            <h3>Chapter 4: Feature Flag Evaluator</h3>
            <p>
              The smallest portable UMA service boundary. The core is written in Rust: a pure evaluator with one contract, deterministic rule
              semantics, and a WASI CLI. A TypeScript implementation runs the same contract in parallel so that parity can be checked from
              observable output rather than inferred from shared intent. Labs cover country matching, rollout percentages, default fallback,
              and the full rule language. The smoke script <code>./scripts/smoke_flag_labs.sh</code> builds the WASI evaluator, runs the Rust
              unit tests, runs the TypeScript parity tests, and compares outputs across every guided lab.
            </p>

            <h3>Chapter 5: Post Fetcher Runtime</h3>
            <p>
              The runtime layer around a pure service. This chapter makes explicit what belongs in the runtime versus what belongs in the
              service: the runtime selects and records the <code>network.fetch</code> adapter binding, validates input before side effects
              happen, and produces lifecycle metadata that makes execution auditable. The validated path is hermetic. It resolves a
              checked-in fixture through the host adapter rather than opening a localhost server. A TypeScript reference runtime mirrors the
              same scenarios for cross-language comparison. The smoke script is <code>./scripts/smoke_runtime_labs.sh</code>.
            </p>

            <h3>Chapter 6: Portability Lab</h3>
            <p>
              Multi-runtime behavioral equivalence under one contract. One Rust crate and one contract drive both a native binary and a
              WASI binary. Parity is asserted from emitted <code>image.analyzed</code> events, not by trusting that shared code implies
              shared behavior. Contract parameters change behavior without code edits. Target-specific capabilities such as native telemetry
              stay explicit without contaminating the portable path. The lab parity script checks event digests across both targets.
            </p>
          </section>

          <section>
            <h2>How to run it</h2>
            <p>Prerequisites for the validated reader path:</p>
            <ul>
              <li>Rust with the <code>wasm32-wasip1</code> target: <code>rustup target add wasm32-wasip1</code></li>
              <li>Wasmtime 20 or newer on your <code>PATH</code></li>
              <li>Node.js 20 or newer (for TypeScript parity paths)</li>
              <li><code>npm</code></li>
              <li>Optional: <code>jq</code> for digest comparisons and golden-fixture checks</li>
            </ul>
            <p>Run Chapter 4 (Feature Flag Evaluator):</p>
            <pre><code>cd chapter-04-feature-flag-evaluator
./scripts/smoke_flag_labs.sh</code></pre>
            <p>Run Chapter 5 (Post Fetcher Runtime):</p>
            <pre><code>cd chapter-05-post-fetcher-runtime
./scripts/smoke_runtime_labs.sh</code></pre>
            <p>
              Both scripts are the chapter acceptance paths used during repo-level validation. Browser, edge, and cloud adapter paths in
              both chapters remain illustrative host examples. They demonstrate the adapter binding model but are not part of the validated
              smoke paths.
            </p>
          </section>

          <section>
            <h2>The CI proof</h2>
            <p>
              Two CI workflows run on every commit and represent the proof methodology UMA requires.
            </p>
            <p>
              <strong>Reader Smoke CI</strong> executes the complete guided reader path from checkout to final lab output. It validates that
              a reader following the documented steps gets the expected results without manual intervention.
            </p>
            <p>
              <strong>Business Logic Coverage CI</strong> measures coverage of the portable Rust core independently of any host. The 100%
              business logic coverage badge means every branch of the portable evaluator and service logic is exercised by tests that do not
              depend on a specific runtime host. This is how UMA separates the proof of portable logic from the proof of runtime integration.
            </p>
            <p>
              That separation is not incidental. It is the proof methodology the UMA model requires: business behavior must be verifiable
              independently of execution context. The CI structure enforces that constraint mechanically rather than relying on convention
              or manual review.
            </p>
          </section>

          <section>
            <h2>What the GitHub repository contains</h2>
            <p>
              The repository is organized to make each proof surface findable without navigating a large codebase:
            </p>
            <ul>
              <li><code>capability-descriptors/</code>: shared contract definitions used across chapters</li>
              <li><code>chapter-04-feature-flag-evaluator/</code> through <code>chapter-13-portable-mcp-runtime/</code>: the full learning path in chapter sequence</li>
              <li><code>benchmarks/</code>: generated proof artifacts for the published benchmark and footprint notes</li>
              <li><code>coverage/</code>: business logic coverage reports</li>
              <li><code>scripts/</code>: reader smoke, coverage, and repo-quality helpers</li>
            </ul>
            <p>
              The full repository is at
              <a href="https://github.com/enricopiovesan/UMA-code-examples">github.com/enricopiovesan/UMA-code-examples</a>.
            </p>
          </section>

          <section>
            <h2>Frequently asked questions</h2>

            <h3>Is this production code?</h3>
            <p>
              No. This is a reference implementation. Its purpose is to prove the UMA model with running code that readers can inspect,
              fork, and challenge. Production use of UMA requires adapting the patterns to your specific runtime constraints, trust model,
              and deployment context. The reference application makes no claims about production readiness of any specific implementation.
            </p>

            <h3>Where do I start?</h3>
            <p>
              Chapter 4 is the validated entry point. It is the smallest complete example: one portable service, one contract, one smoke
              script, and a TypeScript parity check. The reader path from Chapter 4 onward is the sequence the CI validates, so starting
              there gives you the path with the strongest proof backing. If you want the full picture first, Chapter 13 contains the
              complete reference application that ties every chapter together.
            </p>
          </section>

          <section class="subpage-callout">
            <strong>Inspect the proof or run it yourself</strong>
            <p>
              The GitHub repository contains the full examples, CI configuration, and smoke scripts. The tutorial walks through the
              WebAssembly execution model the reference application uses.
            </p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples">GitHub: UMA Code Examples</a>
              <a href="/wasm-microservices-tutorial-rust/">WebAssembly Microservices Tutorial (Rust)</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
