---
ref: chapter-08-service-graph
title: "Chapter 8 Service Graph Evolution Tutorial"
subtitle: "Chapter 8 UMA code example Service graph evolution tutorial This tutorial turns system growth into something inspectable. You will run graph scenarios, compare contract changes, and see how compatible services join the graph without upstream rewrites."
macro_area: examples
content_type: tutorial
slug: chapter-08-service-graph
canonical_url: "https://www.universalmicroservices.com/examples/chapter-08-service-graph/"
left_nav_group: examples
chapter_ref: "Chapter 8"
seo_description: "Follow the Chapter 8 UMA service graph tutorial to inspect graph growth, compatibility breaks, and recovery using Rust-first scenario snapshots and diffs."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 8: Chapter 8 Service Graph Evolution Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 8 UMA code example</p>
          <h1>Service graph evolution tutorial</h1>
          <p>This tutorial turns system growth into something inspectable. You will run graph scenarios, compare contract changes, and see how compatible services join the graph without upstream rewrites.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Use the in-page route block to move through the graph progression without relying on footer links for the main sequence.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-08-service-graph">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-07-metadata-orchestration/">Previous: Metadata Orchestration</a>
              <a href="../chapter-09-trust-boundaries/">Next: Trust Boundaries</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>how service graphs emerge from contracts and event compatibility</li><li>how Git-style diffs expose architecture changes</li><li>how broken compatibility fails visibly and recovers predictably</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.76 or newer</li><li>Node.js 20 or newer</li><li>a checkout of the repository</li></ul>
            
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-08-service-graph</code></pre></li><li><strong>List the graph labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Validate the first scenario contracts</strong><pre class="tutorial-code"><code>./scripts/validate_graph_contracts.sh lab1-upload-only</code></pre></li><li><strong>Run the upload-only graph</strong><pre class="tutorial-code"><code>./scripts/run_graph_demo.sh lab1-upload-only</code></pre></li><li><strong>Inspect the contract change to image tagging</strong><pre class="tutorial-code"><code>./scripts/contract_diff.sh lab1-upload-only lab2-image-tagger</code></pre></li><li><strong>Run the indexer graph</strong><pre class="tutorial-code"><code>./scripts/run_graph_demo.sh lab3-indexer</code></pre></li><li><strong>Compare a compatibility break</strong><pre class="tutorial-code"><code>./scripts/graph_diff.sh lab3-indexer lab4-broken-compat</code></pre></li><li><strong>Run the full smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_graph_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>capability lines</li><li>consumes and emits lines</li><li>Edges</li><li>Waiting Consumers</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_graph_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 9 adds explicit trust and runtime enforcement.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-08-service-graph">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../../learn-uma/chapter-09-microservices-to-distributed-systems/">Read Chapter 9: Services to Systems →</a>
              <a href="../chapter-07-metadata-orchestration/">Previous: Metadata Orchestration</a>
              <a href="../chapter-09-trust-boundaries/">Next: Trust Boundaries</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
