---
ref: chapter-05-post-fetcher-runtime
title: "Chapter 5 Post Fetcher Runtime Tutorial"
subtitle: "Chapter 5 UMA code example Post fetcher runtime tutorial This tutorial shows what the UMA runtime layer owns around a pure service: validation, adapter selection, deterministic event ordering, and lifecycle evidence."
macro_area: examples
content_type: tutorial
slug: chapter-05-post-fetcher-runtime
canonical_url: "https://www.universalmicroservices.com/examples/chapter-05-post-fetcher-runtime/"
left_nav_group: examples
chapter_ref: "Chapter 5"
seo_description: "Follow the Chapter 5 UMA post fetcher runtime tutorial to see validation, adapter binding, event ordering, and lifecycle metadata around a pure Rust service."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 5: Chapter 5 Post Fetcher Runtime Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-06-portability-lab
  - chapter-07-metadata-orchestration
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 5 UMA code example</p>
          <h1>Post fetcher runtime tutorial</h1>
          <p>This tutorial shows what the UMA runtime layer owns around a pure service: validation, adapter selection, deterministic event ordering, and lifecycle evidence.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Use the links below to move through the tutorial sequence without dropping to the footer first.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-05-post-fetcher-runtime">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-04-feature-flag-evaluator/">Previous: Feature Flag Evaluator</a>
              <a href="../chapter-06-portability-lab/">Next: UMA Portability Lab</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>what belongs in service logic versus runtime logic</li><li>why validation should stop execution before side effects happen</li><li>how lifecycle metadata records the network.fetch adapter binding</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.77 or newer</li><li>cargo</li><li>jq</li><li>Node.js and npm for TypeScript parity</li></ul>
            
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-05-post-fetcher-runtime</code></pre></li><li><strong>List the guided labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Run the golden cloud host path</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab1-cloud-golden-path</code></pre></li><li><strong>Run fail-fast header validation</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab2-header-validation-fail-fast</code></pre></li><li><strong>Inspect adapter binding and wrappers</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab3-adapter-binding-and-wrappers</code></pre></li><li><strong>Verify Rust and TypeScript parity</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab4-rust-ts-parity</code></pre></li><li><strong>Run the chapter smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_runtime_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>output.events</li><li>lifecycle.bindings</li><li>final lifecycle.state</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_runtime_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 6 proves portability across native and WASI targets.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-05-post-fetcher-runtime">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../../learn-uma/chapter-06-uma-runtime-layer/">Read Chapter 6: The Runtime Layer →</a>
              <a href="../chapter-04-feature-flag-evaluator/">Previous: Feature Flag Evaluator</a>
              <a href="../chapter-06-portability-lab/">Next: UMA Portability Lab</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
