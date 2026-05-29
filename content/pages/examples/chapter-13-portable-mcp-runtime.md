---
ref: chapter-13-portable-mcp-runtime
title: "Chapter 13 Portable MCP Runtime Tutorial"
subtitle: "Chapter 13 UMA code example Portable MCP runtime tutorial This tutorial is the repo reference application. It composes discoverable capabilities into workflows, lets AI propose without becoming authoritative, and explains the execution through CLI, JSON, MCP, and a browser app."
macro_area: examples
content_type: tutorial
slug: chapter-13-portable-mcp-runtime
canonical_url: "https://www.universalmicroservices.com/examples/chapter-13-portable-mcp-runtime/"
left_nav_group: examples
chapter_ref: "Chapter 13"
seo_description: "Use the Chapter 13 portable MCP runtime tutorial to build WASI AI capabilities, run an UMA workflow, inspect JSON reports, and start the MCP server."
breadcrumbs:
  - "Home"
  - "Examples"
  - "Chapter 13: Chapter 13 Portable MCP Runtime Tutorial"
related_refs:
  - examples
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
---

## intro

<section class="subpage-hero tutorial-hero">
          <p class="tutorial-kicker">Chapter 13 UMA code example</p>
          <h1>Portable MCP runtime tutorial</h1>
          <p>This tutorial is the repo reference application. It composes discoverable capabilities into workflows, lets AI propose without becoming authoritative, and explains the execution through CLI, JSON, MCP, and a browser app.</p>
        </section>

## main

<div class="subpage-body tutorial-body">
          <section class="subpage-callout">
            <strong>Tutorial route</strong>
            <p>Keep the final chapter’s navigation close to the lesson so the source, the examples index, and the previous step remain visible up front.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-13-portable-mcp-runtime">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-12-discoverable-decisions/">Previous: Discoverable Decisions</a>
              <a href="../../reference-application/">Live reference application</a>
            </div>
          </section>
          <section>
            <h2>What you will learn</h2>
            <ul><li>how WASM MCP exposes discovery and invocation surfaces</li><li>how the UMA runtime validates and executes selected capabilities</li><li>how AI-backed capabilities participate with explicit fallback reporting</li></ul>
          </section>
          <section>
            <h2>Prerequisites</h2>
            <ul><li>Rust 1.76 or newer</li><li>wasm32-wasip1 target</li><li>Node.js 20 or newer</li><li>npm</li><li>Wasmtime on your PATH</li></ul>
            <p>Run this setup command before the lab if your machine does not already have the target installed.</p><pre class="tutorial-code"><code>rustup target add wasm32-wasip1</code></pre>
          </section>
          <section>
            <h2>Full tutorial</h2>
            <ol class="tutorial-steps"><li><strong>Enter the example</strong><pre class="tutorial-code"><code>cd chapter-13-portable-mcp-runtime</code></pre></li><li><strong>Set up pinned model artifacts</strong><pre class="tutorial-code"><code>./scripts/setup_models.sh</code></pre></li><li><strong>Build the PlannerAI WASI module</strong><pre class="tutorial-code"><code>./scripts/build_planner_ai_wasi.sh</code></pre></li><li><strong>Build the SummarizerAI WASI module</strong><pre class="tutorial-code"><code>./scripts/build_summarizer_ai_wasi.sh</code></pre></li><li><strong>Build the TranslatorFr WASI module</strong><pre class="tutorial-code"><code>./scripts/build_translator_ai_wasi.sh</code></pre></li><li><strong>List workflows</strong><pre class="tutorial-code"><code>./scripts/list_labs.sh</code></pre></li><li><strong>Run the French AI report workflow</strong><pre class="tutorial-code"><code>./scripts/run_lab.sh use-case-2-ai-report</code></pre></li><li><strong>Render the workflow as JSON</strong><pre class="tutorial-code"><code>cargo run --manifest-path rust/Cargo.toml -- render use-case-2-ai-report json</code></pre></li><li><strong>Run the MCP server smoke check</strong><pre class="tutorial-code"><code>./scripts/smoke_mcp_server.sh</code></pre></li><li><strong>Run the full chapter smoke path</strong><pre class="tutorial-code"><code>./scripts/smoke_portable_mcp_labs.sh</code></pre></li></ol>
          </section>
          <section>
            <h2>What to inspect</h2>
            <p>After each command, look for these proof points. They are the signals that connect the code example back to the UMA architecture claim.</p>
            <ul><li>capability selection</li><li>runtime validation result</li><li>structured report JSON</li><li>explicit provider fallback reporting when used</li></ul>
          </section>
          <section>
            <h2>Acceptance check</h2>
            <p>The chapter-level validation path is:</p>
            <pre class="tutorial-code"><code>./scripts/smoke_portable_mcp_labs.sh</code></pre>
            <p>Return to the repository root for the final acceptance gate:</p>
            <pre class="tutorial-code"><code>cd ..
./scripts/smoke_reader_paths.sh</code></pre>
          </section>
          <section>
            <h2>Where to go next</h2>
            <p>This chapter closes the validated learning path and links to the live reference application.</p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-13-portable-mcp-runtime">Open source folder</a>
              <a href="../">All UMA examples</a>
              <a href="../chapter-12-discoverable-decisions/">Previous: Discoverable Decisions</a>
              <a href="../../reference-application/">Live reference application</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
