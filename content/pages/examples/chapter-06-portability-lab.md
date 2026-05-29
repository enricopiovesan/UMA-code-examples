---
ref: chapter-06-portability-lab
title: "Chapter 6 UMA Portability Lab Tutorial"
subtitle: "Chapter 6 UMA code example UMA portability lab tutorial This tutorial proves portability through observable behavior. The native runner and WASI runner emit the same image.analyzed payload while target-specific telemetry remains explicit."
macro_area: examples
content_type: tutorial
slug: chapter-06-portability-lab
canonical_url: "https://www.universalmicroservices.com/examples/chapter-06-portability-lab/"
left_nav_group: examples
chapter_ref: "Chapter 6"
seo_description: "Run the Chapter 6 UMA portability tutorial and compare the same image analyzer behavior across native Rust and WASI with shared event payloads."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 6: Chapter 6 UMA Portability Lab Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-07-metadata-orchestration
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 6 UMA code example</p>
          <h1>UMA portability lab tutorial</h1>
          <p>This tutorial proves portability through observable behavior. The native runner and WASI runner emit the same image.analyzed payload while target-specific telemetry remains explicit.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Use the links up front so the portability path feels like a guided sequence instead of a footer scavenger hunt.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-06-portability-lab">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-05-post-fetcher-runtime/">Previous: Post Fetcher Runtime</a>
              <a href="../chapter-07-metadata-orchestration/">Next: Metadata Orchestration</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>how one contract drives native and WASI execution</li><li>why parity should be checked through emitted events</li><li>how native-only capabilities stay outside the portable path</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.77 or newer</li><li>rustup target add wasm32-wasip1</li><li>Wasmtime 20 or newer</li><li>jq</li></ul>
            <p>Run this setup command before the lab if your machine does not already have the target installed.</p><pre class="tutorial-code"><code>rustup target add wasm32-wasip1</code></pre>
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-06-portability-lab</code></pre></li><li><strong>List the guided labs</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Compare native and WASI events</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab1-native-wasm-parity</code></pre></li><li><strong>Compute shared payload digests</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab2-shared-payload-digest</code></pre></li><li><strong>Exercise failure paths and capability gates</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab3-failure-paths-and-capability-gates</code></pre></li><li><strong>Check Rust and TypeScript reference parity</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh lab4-rust-ts-reference-parity</code></pre></li><li><strong>Run the chapter smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_portability_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>identical image.analyzed payloads</li><li>matching shared payload digests</li><li>gpu.telemetry.reported only on the native path</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_portability_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>Chapter 7 uses contracts and events to create orchestration.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-06-portability-lab">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-05-post-fetcher-runtime/">Previous: Post Fetcher Runtime</a>
              <a href="../chapter-07-metadata-orchestration/">Next: Metadata Orchestration</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
