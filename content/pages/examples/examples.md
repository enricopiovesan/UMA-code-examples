---
ref: examples
title: "UMA Examples"
subtitle: "UMA examples The examples are the practical proof layer of Universal Microservices Architecture. They let readers inspect how a portable service is defined, how a runtime wraps it, how orchestration emerges from metadata, and how system evolution and trust decisions become visible rather than hidden inside operational glue. They are the fastest way to evaluate UMA as an execution model instead of treating it as another architectural slogan."
macro_area: examples
content_type: hub
slug: examples
canonical_url: "https://www.universalmicroservices.com/examples/"
left_nav_group: examples
chapter_ref: null
seo_description: "Explore the practical Universal Microservices Architecture examples, including chapter-aligned Rust-first labs, portable MCP runtime composition, and runnable tutorials."
breadcrumbs:
  - "Home"
  - "Examples"
  - "UMA Examples"
related_refs:
  - chapter-04-feature-flag-evaluator
  - chapter-05-post-fetcher-runtime
  - chapter-06-portability-lab
  - chapter-07-metadata-orchestration
---

## intro

<section class="subpage-hero">
          <h1>UMA examples</h1>
          <p>
            The examples are the practical proof layer of Universal Microservices Architecture. They let readers inspect how a portable
            service is defined, how a runtime wraps it, how orchestration emerges from metadata, and how system evolution and trust
            decisions become visible rather than hidden inside operational glue. They are the fastest way to evaluate UMA as an execution
            model instead of treating it as another architectural slogan.
          </p>
        </section>

## main

<div class="subpage-body">
          <section class="subpage-callout">
            <strong>How to navigate the examples</strong>
            <p>
              Start with Chapter 4 and move forward in order if you want the cleanest learning path. Each tutorial page keeps the source
              folder, the examples index, and the sibling chapter links near the top so the navigation is part of the page instead of
              hidden in the footer.
            </p>
            <div class="subpage-inline-links">
              <a href="chapter-04-feature-flag-evaluator/">Start with Chapter 4</a>
              <a href="chapter-07-metadata-orchestration/">Jump to orchestration</a>
              <a href="chapter-12-discoverable-decisions/">See discoverable decisions</a>
            </div>
          </section>

          <section id="why-the-examples-matter">
            <h2>Why the examples matter</h2>
            <p>
              Architecture claims are easy to make and hard to validate. The examples exist to close that gap. They give the reader a way
              to compare intention against output, contracts against runtime decisions, and design coherence against the real drift that
              appears once a system grows.
            </p>
            <p>
              The key thing to evaluate is not “can this code run somewhere else?” It is whether one behavior stays coherent while compute
              can happen in many places and the runtime decides where logic runs.
            </p>
          </section>

          <section id="before-and-after-repo-reading">
            <h2>A before-and-after way to read the repo</h2>
            <p>
              Before UMA, a team often ends up with the same business rule scattered across browser logic, edge handling, backend checks,
              and workflow glue. After UMA, the question becomes whether one portable behavioral unit can stay intact while the runtime
              takes responsibility for placement, validation, trust, and workflow approval.
            </p>
            <p>
              That is the lens to use when reading the examples. Do not ask only whether the lab runs. Ask whether the architecture still
              knows what the durable behavior is and why the runtime approved the path that produced the result.
            </p>
          </section>

          <section id="what-makes-these-examples-different">
            <h2>What makes these examples different</h2>
            <p>
              These examples are not generic demos attached to a book launch. Each one exists to make a specific architectural question
              visible. What does a portable service boundary look like? What belongs in the runtime layer? How does metadata shape
              orchestration? When do trust and compatibility become part of the architecture rather than operational cleanup?
            </p>
            <p>
              That makes the examples valuable even outside the book flow. A reader can inspect one example in isolation and still learn
              something important about portability, runtime governance, or long-term system design. Used in sequence, they become a
              practical walk through the model.
            </p>
          </section>

          <section id="example-props" class="subpage-grid">
            <article class="subpage-card">
              <h3>Chapter-aligned</h3>
              <p>The examples map directly to the later chapters so the learning path continues from explanation into runnable material.</p>
            </article>
            <article class="subpage-card">
              <h3>Rust-first</h3>
              <p>The primary validated path uses Rust for the core runtime and architecture examples.</p>
            </article>
            <article class="subpage-card">
              <h3>TypeScript parity</h3>
              <p>Selected chapters include TypeScript parity paths to make comparison easier for readers coming from different stacks.</p>
            </article>
            <article class="subpage-card">
              <h3>Reader-tested flows</h3>
              <p>Scripts, smoke checks, and learning-path validation make the examples easier to follow chapter by chapter.</p>
            </article>
          </section>

          <section id="tutorials">
            <h2>Runnable tutorials for every code example</h2>
            <p>
              Each chapter example now has a dedicated tutorial page with its own canonical URL, metadata, source link, validation
              command, and repository-level acceptance check. The pages are grouped below so the learning path is easier to scan and
              easier to follow.
            </p>
          </section>

          <section id="foundations">
            <h2>Foundations</h2>
            <div class="subpage-grid tutorial-link-grid">
              <article class="subpage-card">
                <h3><a href="chapter-04-feature-flag-evaluator/">Chapter 4: Feature Flag Evaluator</a></h3>
                <p>Build and run the deterministic Rust-first evaluator, then prove TypeScript parity from output.</p>
              </article>
              <article class="subpage-card">
                <h3><a href="chapter-05-post-fetcher-runtime/">Chapter 5: Post Fetcher Runtime</a></h3>
                <p>Inspect validation, adapter binding, event ordering, and lifecycle evidence around a pure service.</p>
              </article>
              <article class="subpage-card">
                <h3><a href="chapter-06-portability-lab/">Chapter 6: Portability Lab</a></h3>
                <p>Compare native and WASI execution through the same emitted image analysis payload.</p>
              </article>
            </div>
          </section>

          <section id="orchestration-and-trust">
            <h2>Orchestration and trust</h2>
            <div class="subpage-grid tutorial-link-grid">
              <article class="subpage-card">
                <h3><a href="chapter-07-metadata-orchestration/">Chapter 7: Metadata Orchestration</a></h3>
                <p>Run the contract-driven orchestration flow and inspect binding, policy, schema, and telemetry signals.</p>
              </article>
              <article class="subpage-card">
                <h3><a href="chapter-08-service-graph/">Chapter 8: Service Graph Evolution</a></h3>
                <p>Use graph snapshots and diffs to watch compatible services join, break, and recover.</p>
              </article>
              <article class="subpage-card">
                <h3><a href="chapter-09-trust-boundaries/">Chapter 9: Trust Boundaries</a></h3>
                <p>See trust metadata, permissions, dependency provenance, and communication policy produce runtime decisions.</p>
              </article>
            </div>
          </section>

          <section id="evolution-and-discoverability">
            <h2>Evolution and discoverability</h2>
            <div class="subpage-grid tutorial-link-grid">
              <article class="subpage-card">
                <h3><a href="chapter-10-architectural-tradeoffs/">Chapter 10: Architectural Tradeoffs</a></h3>
                <p>Compare coherent and degraded designs using runtime-visible warnings and architectural decision axes.</p>
              </article>
              <article class="subpage-card">
                <h3><a href="chapter-11-evolution-without-fragmentation/">Chapter 11: Evolution Without Fragmentation</a></h3>
                <p>Follow drift, duplication, version sprawl, coexistence, and hybrid adoption through runnable scenarios.</p>
              </article>
              <article class="subpage-card">
                <h3><a href="chapter-12-discoverable-decisions/">Chapter 12: Discoverable Decisions</a></h3>
                <p>Expose proposal, authority feedback, revision, execution, and trace artifacts as queryable proof.</p>
              </article>
              <article class="subpage-card">
                <h3><a href="chapter-13-portable-mcp-runtime/">Chapter 13: Portable MCP Runtime</a></h3>
                <p>Build WASI AI capabilities, run the UMA workflow, inspect JSON reports, and smoke the MCP server.</p>
              </article>
            </div>
          </section>

          <section id="what-each-tutorial-page-includes">
            <h2>What each tutorial page includes</h2>
            <p>
              Every tutorial page now keeps the navigation close to the content: an in-page route block, the validation commands, the
              source folder link, and the next step in the sequence. That makes the structure easier to scan without sending readers to
              the footer first.
            </p>
          </section>

          <section id="what-you-can-do">
            <h2>What you can do with them</h2>
            <p>
              Use the examples to test the book’s architectural claims against concrete behavior. You can run a minimal service, trace a
              runtime wrapper, compare implementations, follow orchestration decisions, and see where trust and compatibility start to
              matter.
            </p>
            <p>
              They also give teams a way to discuss architecture with evidence. Instead of debating portability or coherence in abstract
              terms, readers can point to actual contracts, runtime outputs, and learning-path transitions.
            </p>
            <p>
              That proof should include measurable tradeoffs, not just screenshots. The benchmark notes now publish local artifact sizes
              and repeated execution timings for the early portability chapters so readers can evaluate UMA with numbers as well as
              examples.
            </p>
          </section>

          <section id="how-the-repository-is-organized">
            <h2>How the repository is organized</h2>
            <p>
              The repository follows the same broad progression as the learning path. Earlier chapters keep the system intentionally small.
              Later chapters introduce orchestration, service graphs, trust boundaries, and governed evolution. That organization helps the
              reader understand not just individual samples, but the architectural consequences of adding more runtime responsibility over
              time.
            </p>
            <p>
              The latest steps in that progression are the Chapter 12 lab on discoverable decisions and the Chapter 13 portable MCP runtime,
              where projections, proposals, validation feedback, approved execution, MCP discovery, and event-driven composition become
              first-class outputs instead of hidden runtime behavior.
            </p>
            <p>
              By then, the examples are proving something stronger than execution. They are proving that a governed system can expose its
              own reasoning surfaces as reusable architectural artifacts.
            </p>
          </section>

          <section id="why-rust-first-matters">
            <h2>Why Rust-first matters here</h2>
            <p>
              The validated default path uses Rust because it makes the portable runtime story concrete and rigorous. TypeScript parity is
              included where it adds value for comparison, but the repository stays centered on one primary validated path so the learning
              experience remains consistent and defensible.
            </p>
          </section>

          <section id="what-the-closing-examples-prove">
            <h2>What the closing examples prove</h2>
            <p>
              The Chapter 12 example matters because it demonstrates a final architectural shift: a system should not only execute correctly,
              it should expose why it acted. That lab makes decision stages queryable so readers can inspect capability projection, proposal
              shape, authority feedback, bounded revision, approved execution, and full traceability in one sequence.
            </p>
            <p>
              Chapter 13 then carries that idea into a richer reference experience. The runtime discovers distributed capabilities through
              MCP-style descriptors, evaluates compatibility and constraints, accepts or rejects agent proposals, coordinates event-driven
              execution, and produces a structured French report without collapsing back into a hardwired workflow.
            </p>
            <p>
              If you want one conceptual lens that makes the Chapter 13 behavior easier to read, focus on the distinction between planning
              and authority. The agent can help propose a workflow, but the runtime still governs what is approved and executed.
            </p>
            <p>
              That is also the cleanest demonstration of the UMA promise: not write once, run everywhere, but write once, run where it
              makes sense under visible runtime authority.
            </p>
          </section>

          <section id="frequently-asked-question">
            <h2>Frequently asked question</h2>
            <h3>Do I need to run every example to understand UMA?</h3>
            <p>
              No, but the examples are where the architectural claims become easiest to verify. Even running a few of the core chapters
              gives much more confidence in the model than reading about it alone.
            </p>
          </section>

          <section id="repository-entry-point" class="subpage-callout">
            <strong>Repository entry point</strong>
            <p>
              The examples live in the public repository and follow the same story arc as the site and the book. That makes the repository
              the right place to continue after this overview. If you want the full design reasoning behind that progression, the book takes
              the same examples and connects them into one architectural sequence rather than a set of isolated labs.
            </p>
            <div class="subpage-inline-links">
              <a href="https://github.com/enricopiovesan/UMA-code-examples">GitHub repository</a>
              <a href="../benchmark-and-footprint/">Benchmark and footprint notes</a>
              <a href="../what-makes-a-service-portable/">What makes a service portable?</a>
              <a href="../how-to-prove-portability/">How to prove portability</a>
              <a href="../what-is-a-capability/">What is a capability?</a>
              <a href="../what-makes-a-decision-discoverable/">What makes a decision discoverable?</a>
              <a href="../what-is-a-workflow/">What is a workflow?</a>
              <a href="../what-is-wasm-mcp/">What is WASM MCP?</a>
              <a href="../learning-path/">Learning Path</a>
              <a href="../agent-vs-runtime/">Agent vs runtime</a>
              <a href="../#hands-on">Hands-on</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
