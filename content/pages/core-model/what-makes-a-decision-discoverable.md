---
ref: what-makes-a-decision-discoverable
title: "What Makes a Decision Discoverable?"
subtitle: "What makes a decision discoverable? A decision becomes discoverable when the system can expose how it reached an outcome instead of only reporting that the outcome happened. In UMA, that means intent, authority, revision, execution, and trace all become first-class architectural surfaces."
macro_area: core-model
content_type: explainer
slug: what-makes-a-decision-discoverable
canonical_url: "https://www.universalmicroservices.com/what-makes-a-decision-discoverable/"
left_nav_group: core-model
chapter_ref: null
seo_description: "Learn what makes a runtime decision discoverable in UMA: visible proposals, authoritative validation, bounded revision, approved execution, and traceable artifacts."
breadcrumbs:
  - "Home"
  - "Core Model"
  - "What Makes a Decision Discoverable?"
related_refs:
  - active-descriptors
  - agent-vs-runtime
  - late-bound-policy-enforcement
  - what-belongs-in-the-runtime-layer
---

## intro

<section class="subpage-hero">
          <h1>What makes a decision discoverable?</h1>
          <p>
            A decision becomes discoverable when the system can expose how it reached an outcome instead of only reporting that the outcome
            happened. In UMA, that means intent, authority, revision, execution, and trace all become first-class architectural surfaces.
          </p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>The short answer</h2>
            <p>
              A discoverable decision is one a person or tool can inspect before and after execution. The system should be able to show the
              proposal, the constraints it could not resolve locally, the authoritative validation response, the approved path, and the
              resulting trace artifacts. Without those surfaces, a runtime may still work, but its decision model remains hidden.
            </p>
            <p>
              This is the step where UMA moves from governed execution into understandable execution. It is not enough for the runtime to
              know why something happened. The system has to make that reasoning legible.
            </p>
          </section>

          <section>
            <h2>Why execution alone is not enough</h2>
            <p>
              Many systems can execute correctly while keeping the actual decision path buried inside code, logs, or implicit behavior. A
              request succeeds, but no one can tell what alternatives were considered, what assumptions were made at the edge, which
              constraints forced a revision, or why the final path was approved. The system acts, but it does not explain itself.
            </p>
            <p>
              Discoverability matters because hidden decision paths turn architecture into folklore. Once the reasoning behind a runtime
              outcome is no longer inspectable, teams have a much harder time reviewing changes, debugging surprising behavior, or safely
              extending the system.
            </p>
          </section>

          <section class="subpage-grid">
            <article class="subpage-card">
              <h3>Projection is useful, not authoritative</h3>
              <p>A local surface may expose likely capabilities and constraints, but it should not pretend it owns final approval.</p>
            </article>
            <article class="subpage-card">
              <h3>Proposals should declare assumptions</h3>
              <p>The system becomes easier to reason about when unresolved constraints stay explicit instead of being hidden in optimistic planning.</p>
            </article>
            <article class="subpage-card">
              <h3>Validation should return guidance</h3>
              <p>Authority is more useful when it explains why a proposal failed and what would make it valid.</p>
            </article>
            <article class="subpage-card">
              <h3>Trace completes the story</h3>
              <p>A successful execution is not fully discoverable until the approved path can be replayed as evidence afterward.</p>
            </article>
          </section>

          <section>
            <h2>What discoverability looks like in practice</h2>
            <p>
              A discoverable system usually reveals a ladder of decision surfaces. First it exposes a capability projection or local view of
              the likely path. Then it emits a proposal that declares assumptions. Next, authority validates that proposal and returns
              structured feedback. If revision is needed, the revision stays bounded and inspectable. Once approval happens, execution
              follows the approved intent instead of silently replanning it. Finally, the trace shows enough evidence to explain the outcome
              after the fact.
            </p>
            <p>
              That ladder matters because it separates useful local reasoning from final authority. Edge planning can still be helpful, but
              it becomes much safer when it no longer pretends to be the last word.
            </p>
          </section>

          <section>
            <h2>Why bounded revision matters</h2>
            <p>
              Many systems become hard to understand because validation and planning collapse into endless negotiation. Each side keeps
              revising the plan until execution finally happens, but no one can explain where the decisive authority really lived. UMA is
              stronger when revision stays explicit and bounded.
            </p>
            <p>
              Bounded revision keeps the architectural roles clearer. Proposal stays proposal. Validation stays authority. Execution stays
              the resolution of approved intent, not a last-minute improvisation.
            </p>
          </section>

          <section>
            <h2>Why trace artifacts matter more than logs</h2>
            <p>
              Logs are often chronological but not architectural. A trace artifact is stronger because it tells the decision story in terms
              the model itself understands: what was proposed, what was rejected, what was revised, what was approved, and what executed.
              That makes post-execution review much more practical.
            </p>
            <p>
              The difference matters for trust, governance, and change review. Teams can audit logs forever and still miss the actual
              architectural question. A structured trace turns the decision into something queryable instead of something merely recorded.
            </p>
          </section>

          <section>
            <h2>What makes a discoverable decision stronger than a hidden one</h2>
            <ul>
              <li>The system can distinguish proposal from approval.</li>
              <li>Constraint failures are explicit enough to be acted on.</li>
              <li>Revision is limited and inspectable instead of endless.</li>
              <li>Execution resolves approved intent rather than silently changing it.</li>
              <li>Trace artifacts make the final path understandable after the fact.</li>
            </ul>
          </section>

          <section>
            <h2>Frequently asked questions</h2>
            <h3>Is discoverability the same as observability?</h3>
            <p>
              No. Observability helps you inspect runtime behavior. Discoverability is narrower and more architectural. It is about whether
              the system exposes its decision path in a structured, queryable way.
            </p>
            <h3>Does every UMA system need the full decision ladder?</h3>
            <p>
              Not always. Smaller systems may not need explicit proposal and revision stages yet. The important part is that authority and
              execution still remain visible once decision complexity starts to grow.
            </p>
            <h3>Why not just let the planner decide everything?</h3>
            <p>
              Because proposal usefulness and execution authority are not the same architectural role. When they blur together, systems may
              look smart while becoming harder to govern.
            </p>
          </section>

          <section class="subpage-callout">
            <strong>Treat discoverability as an architectural property</strong>
            <p>
              This page names the structure. In the book, I take it further into runnable decision ladders so the difference between a
              hidden execution and a queryable decision path becomes concrete. On the site, the best next move is to connect this page to
              runtime authority, workflows, and the live reference application.
            </p>
            <div class="subpage-inline-links">
              <a href="../what-is-a-uma-runtime/">What is a UMA runtime?</a>
              <a href="../what-is-a-workflow/">What is a workflow?</a>
              <a href="../agent-vs-runtime/">Agent vs runtime</a>
              <a href="../diagrams/">Diagrams</a>
              <a href="https://www.universalmicroservices.com/reference-application/">Live reference app</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
