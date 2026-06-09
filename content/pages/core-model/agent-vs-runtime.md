---
ref: agent-vs-runtime
title: "Agent vs Runtime in UMA"
subtitle: "Agent vs runtime in UMA One of the easiest ways to misunderstand Universal Microservices Architecture is to assume that once agents enter the picture, they become the real authority of the system. UMA makes the opposite claim. Agents can help discover, rank, and compose capabilities, but the runtime still decides what is valid, safe, and executable."
macro_area: core-model
content_type: explainer
slug: agent-vs-runtime
canonical_url: "https://www.universalmicroservices.com/agent-vs-runtime/"
left_nav_group: core-model
chapter_ref: null
seo_description: "Agents propose, the runtime validates: how UMA separates AI agent intent from governed execution to keep distributed system behavior auditable and explainable."
breadcrumbs:
  - "Home"
  - "Core Model"
  - "Agent vs Runtime in UMA"
related_refs:
  - active-descriptors
  - late-bound-policy-enforcement
  - what-belongs-in-the-runtime-layer
  - what-is-a-capability
---

## intro

<section class="subpage-hero">
          <h1>Agent vs runtime in UMA</h1>
          <p>
            One of the easiest ways to misunderstand Universal Microservices Architecture is to assume that once agents enter the picture,
            they become the real authority of the system. UMA makes the opposite claim. Agents can help discover, rank, and compose
            capabilities, but the runtime still decides what is valid, safe, and executable.
          </p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>The short answer</h2>
            <p>
              In UMA, an agent is a reasoning component. It interprets a goal, looks at available capabilities, and proposes a path. The
              runtime is the execution authority. It validates contracts, checks constraints, enforces trust and placement rules, and
              decides whether the proposal is acceptable.
            </p>
            <p>
              That distinction matters because modern systems increasingly include components that can generate plans dynamically. If those
              plans are treated as self-justifying, architecture turns into improvisation. UMA avoids that by keeping a hard boundary
              between proposing work and authorizing work.
            </p>
            <ul>
              <li><strong>Agent:</strong> proposes what might work.</li>
              <li><strong>Runtime:</strong> decides what is actually allowed to run.</li>
              <li><strong>System:</strong> stays governed because planning and authority are not collapsed into one layer.</li>
            </ul>
          </section>

          <section>
            <h2>What an agent is for</h2>
            <p>
              Agents are useful when a system has many capabilities and no single fixed workflow fits every situation. They can help
              interpret goals, identify candidate capabilities, rank plausible options, and suggest a sequence that might satisfy the
              request. That is valuable because it lets the system adapt to context instead of relying only on predetermined paths.
            </p>
            <p>
              But adaptability is not the same thing as authority. An agent is good at proposing. It is not, by itself, a stable place to
              define policy, trust, placement, compatibility, or approval rules. Those concerns need a layer that is more explicit and more
              governable than a generated plan.
            </p>
          </section>

          <section>
            <h2>What the runtime is for</h2>
            <p>
              The runtime exists to turn a capability model into governed execution. It knows what is available, what is compatible, what
              is allowed in the current environment, and what must be rejected even if a proposal looks plausible. It can also explain what
              happened, which matters just as much as the execution itself.
            </p>
            <p>
              That means the runtime is the place where architectural rules stay explicit. It is where discovery becomes validation,
              ranking becomes selection, and a proposed workflow becomes an approved one. Without that layer, an agent can still produce
              output, but the system becomes harder to audit, reproduce, and trust.
            </p>
          </section>

          <section class="subpage-grid">
            <article class="subpage-card">
              <h3>Agent responsibility</h3>
              <p>Interpret the goal, explore available capabilities, and propose a plausible path.</p>
            </article>
            <article class="subpage-card">
              <h3>Runtime responsibility</h3>
              <p>Validate contracts, enforce constraints, and decide whether the proposal can actually run.</p>
            </article>
            <article class="subpage-card">
              <h3>Agent strength</h3>
              <p>Adaptability, ranking, and dynamic composition when the exact path cannot be fully hardwired in advance.</p>
            </article>
            <article class="subpage-card">
              <h3>Runtime strength</h3>
              <p>Authority, repeatability, evidence, and governance across environments and capability boundaries.</p>
            </article>
          </section>

          <section>
            <h2>If you remember one rule</h2>
            <p>
              A good UMA system can benefit from an agent without becoming agent-governed. That is the practical test. If the planner goes
              away, the runtime should still be coherent. If the runtime goes away, the planner should not be trusted to define the system
              alone.
            </p>
            <p>
              That rule is important because many systems sound flexible only as long as nobody asks who is accountable for the final
              decision. UMA keeps that answer simple: the runtime is accountable for approval and execution, even when the proposal was
              generated dynamically.
            </p>
          </section>

          <section>
            <h2>Why this distinction matters more with AI</h2>
            <p>
              AI systems make the agent side more powerful, but they also make the boundary more important. As soon as the planning layer is
              model-driven, it becomes even more dangerous to let the proposal layer silently become the control layer. A strong answer is
              still needed to the question: who gets to say yes?
            </p>
            <p>
              UMA’s answer is that the runtime says yes or no. The agent can suggest. The runtime can accept, reject, or replace that
              suggestion according to contracts, placement rules, trust boundaries, and current system conditions. That keeps the system
              explainable even when planning becomes more fluid.
            </p>
          </section>

          <section>
            <h2>What goes wrong when the distinction disappears</h2>
            <p>
              When teams blur the line between agent and runtime, failures start to look intelligent instead of architectural. A proposal is
              accepted because it seems reasonable, not because it was validated. A capability is invoked because it looked relevant, not
              because it matched the active constraints. Placement becomes accidental. Trust becomes inferred. Reproducibility collapses
              because nobody can clearly say which part of the system was merely suggestive and which part was authoritative.
            </p>
            <p>
              That is one reason many “AI-native” system stories still feel thin from an architectural standpoint. They show dynamic plans,
              but not governed execution. UMA tries to keep that second part visible.
            </p>
            <p>
              Discoverability strengthens that same boundary. Once proposal, validation, revision, and trace become inspectable artifacts,
              the system can explain not only that authority exists, but how it actually shaped the result.
            </p>
          </section>

          <section>
            <h2>How Chapter 13 makes this visible</h2>
            <p>
              The Chapter 13 reference experience is useful because it shows this boundary directly instead of leaving it implicit. The
              planner can rank capabilities. The runtime still validates the choice. If a proposed path violates the active rules, the
              runtime can reject it. The resulting workflow is not whatever the planner imagined. It is what the runtime approved.
            </p>
            <p>
              That is the practical version of the architectural claim on this page. The live demo is not just an AI workflow viewer. It is
              a demonstration of runtime authority in the presence of dynamic planning.
            </p>
            <p>
              That makes the page useful for two kinds of readers. One reader wants the concept: what is the difference between agent and
              runtime? Another reader wants proof: can I see the proposal accepted or rejected by a governed layer? Chapter 13 is the proof
              surface for that second question.
            </p>
          </section>

          <section>
            <h2>A useful design test</h2>
            <p>
              If you want to know whether your system is treating agents and runtimes clearly, ask a simple question: if the planner proposes
              a valid-looking but disallowed path, what enforces the rejection? If the answer is unclear, the architecture probably needs a
              stronger runtime story.
            </p>
            <p>
              A second question helps too: can you explain, after execution, why one capability was approved and another was rejected? If
              that answer only exists inside the planner’s prompt or hidden reasoning, the system is still missing the governed layer UMA is
              trying to make explicit.
            </p>
          </section>

          <section>
            <h2>Frequently asked questions</h2>
            <h3>Does UMA require an agent?</h3>
            <p>
              No. UMA does not require an agent in order to be coherent. A system can still use explicit workflows, fixed capability
              selection, and strong runtime governance. Agents become useful when the system needs dynamic selection or proposal behavior.
            </p>
            <h3>Can an agent ever be authoritative?</h3>
            <p>
              It can feel authoritative at the product layer, but in a governed UMA system it should still pass through runtime approval.
              Otherwise the system loses the architectural distinction that keeps dynamic planning from turning into hidden control.
            </p>
            <h3>Why not let the agent decide everything if it performs well?</h3>
            <p>
              Because performance and authority are different concerns. A planner may often suggest a good path, but a governed system still
              needs explicit validation, trust enforcement, and reproducible approval. Those are runtime responsibilities, not prompt
              outcomes.
            </p>
          </section>

          <section class="subpage-callout">
            <strong>See the distinction in practice</strong>
            <p>
              If this page makes sense conceptually, the next step is to watch the live reference app. It makes the planner, runtime, and
              capability flow visible step by step. If you want the deeper architectural argument behind that design, the book is where that
              fuller model belongs.
            </p>
            <div class="subpage-inline-links">
              <a href="https://www.universalmicroservices.com/reference-application/">Live reference app</a>
              <a href="../what-is-a-capability/">What is a capability?</a>
              <a href="../what-is-a-uma-runtime/">What is a UMA runtime?</a>
              <a href="../what-makes-a-decision-discoverable/">What makes a decision discoverable?</a>
              <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order the book</a>
              <a href="../runtime-agnostic-architecture/">Runtime-agnostic architecture</a>
              <a href="../what-is-uma/">What is UMA?</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
