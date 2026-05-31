---
ref: chapter-10-architectural-tradeoffs
title: "Chapter 10 Architectural Tradeoffs Tutorial"
subtitle: "Chapter 10 UMA code example Architectural tradeoffs tutorial This tutorial shows architecture as runtime behavior. You will compare scenarios where service granularity, event semantics, placement rules, and orchestration choices either preserve or degrade coherence."
macro_area: examples
content_type: tutorial
slug: chapter-10-architectural-tradeoffs
canonical_url: "https://www.universalmicroservices.com/examples/chapter-10-architectural-tradeoffs/"
left_nav_group: examples
chapter_ref: "Chapter 10"
seo_description: "Use the Chapter 10 UMA architectural tradeoffs tutorial to compare coherent and degraded designs through runtime-visible warnings and decision axes."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 10: Chapter 10 Architectural Tradeoffs Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 10 UMA code example</p>
          <h1>Architectural tradeoffs tutorial</h1>
          <p>This tutorial shows architecture as runtime behavior. You will compare scenarios where service granularity, event semantics, placement rules, and orchestration choices either preserve or degrade coherence.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Keep the chapter sequence in view so the comparison path stays obvious while you read the tradeoff scenarios.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-10-architectural-tradeoffs">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-09-trust-boundaries/">Previous: Trust Boundaries</a>
              <a href="../chapter-11-evolution-without-fragmentation/">Next: Evolution Without Fragmentation</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>why more services are not automatically better architecture</li><li>how metadata works as a control plane</li><li>how clearer constraints recover coherence after ambiguity or over-orchestration</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.76 or newer</li><li>Node.js 20 or newer</li><li>a checkout of the repository</li></ul>
            
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-10-architectural-tradeoffs</code></pre></li><li><strong>List the architecture labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Validate the baseline</strong><pre class="tutorial-code"><code>./scripts/validate_architecture.sh lab1-baseline</code></pre></li><li><strong>Run the coherent baseline</strong><pre class="tutorial-code"><code>./scripts/run_arch_demo.sh lab1-baseline</code></pre></li><li><strong>Compare over-granular decomposition</strong><pre class="tutorial-code"><code>./scripts/diff_architecture.sh lab1-baseline lab2-over-granular</code></pre></li><li><strong>Run the runtime ambiguity lab</strong><pre class="tutorial-code"><code>./scripts/run_arch_demo.sh lab4-runtime-ambiguity</code></pre></li><li><strong>Run the recovered architecture lab</strong><pre class="tutorial-code"><code>./scripts/run_arch_demo.sh lab6-recovered-architecture</code></pre></li><li><strong>Run the full smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_arch_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>Verdict</li><li>architectural decision axes</li><li>over_granular</li><li>hidden_event_coupling</li><li>runtime_ambiguity</li><li>over_orchestrated</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_arch_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 11 focuses on evolution without fragmentation.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-10-architectural-tradeoffs">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../../learn-uma/chapter-11-microservices-architecture-patterns/">Read Chapter 11: Patterns and Tradeoffs →</a>
              <a href="../chapter-09-trust-boundaries/">Previous: Trust Boundaries</a>
              <a href="../chapter-11-evolution-without-fragmentation/">Next: Evolution Without Fragmentation</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
