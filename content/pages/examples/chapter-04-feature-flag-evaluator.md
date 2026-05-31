---
ref: chapter-04-feature-flag-evaluator
title: "Chapter 4 Feature Flag Evaluator Tutorial"
subtitle: "Chapter 4 UMA code example Feature flag evaluator tutorial This tutorial walks through the first hands-on UMA example: a deterministic feature flag evaluator with one contract, one portable Rust core, a WASI executable, and a TypeScript parity implementation."
macro_area: examples
content_type: tutorial
slug: chapter-04-feature-flag-evaluator
canonical_url: "https://www.universalmicroservices.com/examples/chapter-04-feature-flag-evaluator/"
left_nav_group: examples
chapter_ref: "Chapter 4"
seo_description: "Run the Chapter 4 UMA feature flag evaluator tutorial and learn how one Rust-first portable service keeps deterministic rule behavior aligned with TypeScript parity."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 4: Chapter 4 Feature Flag Evaluator Tutorial"
related_refs:
  - examples
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
  - chapter-07-metadata-orchestration
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 4 UMA code example</p>
          <h1>Feature flag evaluator tutorial</h1>
          <p>This tutorial walks through the first hands-on UMA example: a deterministic feature flag evaluator with one contract, one portable Rust core, a WASI executable, and a TypeScript parity implementation.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Keep the navigation close to the tutorial itself. Use the links below to move between the source folder, the examples index, and the next chapter without hunting through the footer.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-04-feature-flag-evaluator">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-05-post-fetcher-runtime/">Next: Post Fetcher Runtime</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>what belongs in the evaluator contract instead of the host adapter</li><li>how first-match rule evaluation and sticky rollout decisions stay deterministic</li><li>how Rust and TypeScript parity is proven from observable output</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust with the wasm32-wasip1 target</li><li>Wasmtime on your PATH</li><li>Node.js 20 or newer for parity checks</li><li>npm for TypeScript parity and optional adapters</li></ul>
            <p>Run this setup command before the lab if your machine does not already have the target installed.</p><pre class="tutorial-code"><code>rustup target add wasm32-wasip1</code></pre>
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-04-feature-flag-evaluator</code></pre></li><li><strong>List the guided labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Run the country match lab</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab1-country-match</code></pre></li><li><strong>Run the sticky rollout lab</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab2-rollout-match</code></pre></li><li><strong>Run the default fallback lab</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab3-default-fallback</code></pre></li><li><strong>Run the rule language lab</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab4-rule-language</code></pre></li><li><strong>Compare Rust and TypeScript behavior</strong><pre class="tutorial-code"><code>./scripts/compare_impls.sh</code></pre></li><li><strong>Run the chapter smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_flag_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>enabled</li><li>matchedRule</li><li>the same lab returning the same decision in Rust and TypeScript</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_flag_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 5 introduces the runtime layer around a pure service.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-04-feature-flag-evaluator">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../../learn-uma/chapter-05-building-portable-microservices/">Read Chapter 5: Building Portable Microservices →</a>
              <a href="../chapter-05-post-fetcher-runtime/">Next: Post Fetcher Runtime</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
