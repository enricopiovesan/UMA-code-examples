---
ref: chapter-09-trust-boundaries
title: "Chapter 9 Trust Boundaries Tutorial"
subtitle: "Chapter 9 UMA code example Trust boundaries tutorial This tutorial makes trust decisions visible. The runtime evaluates metadata, permissions, dependency provenance, placement, and communication rules before allowing execution."
macro_area: examples
content_type: tutorial
slug: chapter-09-trust-boundaries
canonical_url: "https://www.universalmicroservices.com/examples/chapter-09-trust-boundaries/"
left_nav_group: examples
chapter_ref: "Chapter 9"
seo_description: "Run the Chapter 9 UMA trust boundaries tutorial to see publisher trust, permissions, provenance, and communication policy produce allow and deny decisions."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 9: Chapter 9 Trust Boundaries Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 9 UMA code example</p>
          <h1>Trust boundaries tutorial</h1>
          <p>This tutorial makes trust decisions visible. The runtime evaluates metadata, permissions, dependency provenance, placement, and communication rules before allowing execution.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Keep the path visible beside the tutorial so the trust rules, source folder, and neighboring chapters are easy to jump to.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-09-trust-boundaries">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-08-service-graph/">Previous: Service Graph Evolution</a>
              <a href="../chapter-10-architectural-tradeoffs/">Next: Architectural Tradeoffs</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>why portable systems cannot inherit trust from location alone</li><li>how undeclared permissions and untrusted dependencies fail before execution</li><li>how trust policy governs communication across services</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.76 or newer</li><li>Node.js 20 or newer</li><li>a checkout of the repository</li></ul>
            
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-09-trust-boundaries</code></pre></li><li><strong>List the trust labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Validate the trusted scenario</strong><pre class="tutorial-code"><code>./scripts/validate_trust.sh lab1-trusted-service</code></pre></li><li><strong>Run the trusted service scenario</strong><pre class="tutorial-code"><code>./scripts/run_trust_demo.sh lab1-trusted-service</code></pre></li><li><strong>Inspect an undeclared permission change</strong><pre class="tutorial-code"><code>./scripts/trust_diff.sh lab1-trusted-service lab2-undeclared-permission</code></pre></li><li><strong>Run the untrusted dependency scenario</strong><pre class="tutorial-code"><code>./scripts/run_trust_demo.sh lab3-untrusted-dependency</code></pre></li><li><strong>Run the restored compliance scenario</strong><pre class="tutorial-code"><code>./scripts/run_trust_demo.sh lab5-restored-compliance</code></pre></li><li><strong>Run the full smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_trust_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>Outcome: allow or deny</li><li>permission.undeclared</li><li>dependency.provenance.untrusted</li><li>communication.forbidden</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_trust_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 10 compares architectural tradeoffs through runtime behavior.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-09-trust-boundaries">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-08-service-graph/">Previous: Service Graph Evolution</a>
              <a href="../chapter-10-architectural-tradeoffs/">Next: Architectural Tradeoffs</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
