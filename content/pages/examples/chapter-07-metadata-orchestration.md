---
ref: chapter-07-metadata-orchestration
title: "Chapter 7 Metadata Orchestration Tutorial"
subtitle: "Chapter 7 UMA code example Metadata orchestration tutorial This tutorial shows orchestration emerging from contracts and events instead of hardcoded workflow steps. The Rust cloud runner is the validated path, with TypeScript kept in parity."
macro_area: examples
content_type: tutorial
slug: chapter-07-metadata-orchestration
canonical_url: "https://www.universalmicroservices.com/examples/chapter-07-metadata-orchestration/"
left_nav_group: examples
chapter_ref: "Chapter 7"
seo_description: "Use the Chapter 7 UMA metadata orchestration tutorial to run a contract-driven flow where bindings, policy, validation, and telemetry are visible."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 7: Chapter 7 Metadata Orchestration Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 7 UMA code example</p>
          <h1>Metadata orchestration tutorial</h1>
          <p>This tutorial shows orchestration emerging from contracts and events instead of hardcoded workflow steps. The Rust cloud runner is the validated path, with TypeScript kept in parity.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Keep the primary navigation alongside the lesson so the contract and policy links are visible before you reach the footer.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-07-metadata-orchestration">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-06-portability-lab/">Previous: Portability Lab</a>
              <a href="../chapter-08-service-graph/">Next: Service Graph Evolution</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>how emits and subscribes create runtime bindings</li><li>how policy fail-open and fail-closed modes affect execution</li><li>how CloudEvents and telemetry prove orchestration behavior</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.76 or newer</li><li>Wasmtime 20 or newer</li><li>Node.js 20 or newer</li><li>jq and yq are optional</li></ul>
            
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-07-metadata-orchestration</code></pre></li><li><strong>List the guided labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Run the baseline cloud flow</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab1-baseline-cloud-flow</code></pre></li><li><strong>Verify Rust and TypeScript parity</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab2-rust-ts-parity</code></pre></li><li><strong>Run the fail-closed policy lab</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab3-policy-fail-closed</code></pre></li><li><strong>Run the telemetry audit lab</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab4-telemetry-audit</code></pre></li><li><strong>Run the chapter smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_orchestration_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>binding.created</li><li>policy.violation</li><li>validation.passed</li><li>telemetry.ok</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_orchestration_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 8 makes service graph evolution inspectable.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-07-metadata-orchestration">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-06-portability-lab/">Previous: UMA Portability Lab</a>
              <a href="../chapter-08-service-graph/">Next: Service Graph Evolution</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
