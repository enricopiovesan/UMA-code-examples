---
ref: chapter-11-evolution-without-fragmentation
title: "Chapter 11 Evolution Without Fragmentation Tutorial"
subtitle: "Chapter 11 UMA code example Evolution without fragmentation tutorial This tutorial shows what happens after deployment. You will follow drift, duplicated behavior, version sprawl, and runtime-governed recovery without pretending every system can be rewritten."
macro_area: examples
content_type: tutorial
slug: chapter-11-evolution-without-fragmentation
canonical_url: "https://www.universalmicroservices.com/examples/chapter-11-evolution-without-fragmentation/"
left_nav_group: examples
chapter_ref: "Chapter 11"
seo_description: "Chapter 11 UMA tutorial: drift, duplication, version sprawl, and governed coexistence demonstrated through runnable labs and hybrid adoption scenarios."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 11: Chapter 11 Evolution Without Fragmentation Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 11 UMA code example</p>
          <h1>Evolution without fragmentation tutorial</h1>
          <p>This tutorial shows what happens after deployment. You will follow drift, duplicated behavior, version sprawl, and runtime-governed recovery without pretending every system can be rewritten.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Use the links here to move between the evolution chapters without burying the learning path in the footer.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-11-evolution-without-fragmentation">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-10-architectural-tradeoffs/">Previous: Architectural Tradeoffs</a>
              <a href="../chapter-12-discoverable-decisions/">Next: Discoverable Decisions</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>how locally valid changes create fragmentation over time</li><li>why versioning needs explicit coexistence rules</li><li>how runtime governance supports brownfield adoption</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.76 or newer</li><li>Node.js 20 or newer</li><li>a checkout of the repository</li></ul>
            
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-11-evolution-without-fragmentation</code></pre></li><li><strong>List the evolution labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Validate the contract anchor</strong><pre class="tutorial-code"><code>./scripts/validate_evolution.sh lab1-contract-anchor</code></pre></li><li><strong>Run the coherent baseline</strong><pre class="tutorial-code"><code>./scripts/run_evolution_demo.sh lab1-contract-anchor</code></pre></li><li><strong>Inspect behavioral drift</strong><pre class="tutorial-code"><code>./scripts/diff_evolution.sh lab1-contract-anchor lab2-behavioral-drift</code></pre></li><li><strong>Run duplicate implementations</strong><pre class="tutorial-code"><code>./scripts/run_evolution_demo.sh lab3-duplicate-implementations</code></pre></li><li><strong>Run governed coexistence</strong><pre class="tutorial-code"><code>./scripts/run_evolution_demo.sh lab5-runtime-governed-coexistence</code></pre></li><li><strong>Run the full smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_evolution_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>Verdict</li><li>behavioral_drift</li><li>duplicate_behavior</li><li>version_fragmentation</li><li>Runtime Decisions</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_evolution_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 12 turns runtime decisions into discoverable artifacts.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-11-evolution-without-fragmentation">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../../learn-uma/chapter-12-evolving-distributed-systems/">Read Chapter 12: Evolving Systems →</a>
              <a href="../chapter-10-architectural-tradeoffs/">Previous: Architectural Tradeoffs</a>
              <a href="../chapter-12-discoverable-decisions/">Next: Discoverable Decisions</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
