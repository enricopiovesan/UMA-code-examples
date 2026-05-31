---
ref: chapter-12-discoverable-decisions
title: "Chapter 12 Discoverable Decisions Tutorial"
subtitle: "Chapter 12 UMA code example Discoverable decisions tutorial This tutorial moves from systems that merely execute to systems that can explain. Each lab exposes more of the proposal, validation, revision, execution, and trace lifecycle."
macro_area: examples
content_type: tutorial
slug: chapter-12-discoverable-decisions
canonical_url: "https://www.universalmicroservices.com/examples/chapter-12-discoverable-decisions/"
left_nav_group: examples
chapter_ref: "Chapter 12"
seo_description: "Run the Chapter 12 UMA discoverable decisions tutorial to expose proposal, validation, revision, execution, and trace artifacts as inspectable runtime evidence."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 12: Chapter 12 Discoverable Decisions Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 12 UMA code example</p>
          <h1>Discoverable decisions tutorial</h1>
          <p>This tutorial moves from systems that merely execute to systems that can explain. Each lab exposes more of the proposal, validation, revision, execution, and trace lifecycle.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Keep the decision path in front of the reader so the queryable proof is visible while the tutorial is still on screen.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-12-discoverable-decisions">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-11-evolution-without-fragmentation/">Previous: Evolution Without Fragmentation</a>
              <a href="../chapter-13-portable-mcp-runtime/">Next: Portable MCP Runtime</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>why discoverability is different from execution</li><li>how edge proposals and cloud authority cooperate</li><li>how trace artifacts make final decisions auditable</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.76 or newer</li><li>Node.js 20 or newer</li><li>a checkout of the repository</li></ul>
            
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-12-discoverable-decisions</code></pre></li><li><strong>List the decision labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Validate the first decision scenario</strong><pre class="tutorial-code"><code>./scripts/validate_decisions.sh lab1-capability-projection</code></pre></li><li><strong>Run capability projection</strong><pre class="tutorial-code"><code>./scripts/run_decision_demo.sh lab1-capability-projection</code></pre></li><li><strong>Compare the edge proposal step</strong><pre class="tutorial-code"><code>./scripts/diff_decisions.sh lab1-capability-projection lab2-edge-proposal</code></pre></li><li><strong>Run authority feedback</strong><pre class="tutorial-code"><code>./scripts/run_decision_demo.sh lab3-authority-feedback</code></pre></li><li><strong>Run queryable trace</strong><pre class="tutorial-code"><code>./scripts/run_decision_demo.sh lab6-queryable-trace</code></pre></li><li><strong>Run the full smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_discoverability_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>Verdict</li><li>proposal_hidden</li><li>authority_gap</li><li>Discoverable Surfaces</li><li>Trace</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_discoverability_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 13 applies the model to a portable MCP runtime.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-12-discoverable-decisions">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../../learn-uma/chapter-13-ai-agents-mcp-runtime/">Read Chapter 13: Agents and MCP →</a>
              <a href="../chapter-11-evolution-without-fragmentation/">Previous: Evolution Without Fragmentation</a>
              <a href="../chapter-13-portable-mcp-runtime/">Next: Portable MCP Runtime</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
