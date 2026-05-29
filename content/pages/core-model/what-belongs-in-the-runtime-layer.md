---
ref: what-belongs-in-the-runtime-layer
title: "What Belongs in the Runtime Layer?"
subtitle: "What belongs in the runtime layer? In UMA, the runtime layer owns the governed conditions around execution. The service keeps the durable business behavior. The runtime keeps the decisions that must remain visible when that behavior is validated, adapted, and allowed to run."
macro_area: core-model
content_type: explainer
slug: what-belongs-in-the-runtime-layer
canonical_url: "https://www.universalmicroservices.com/what-belongs-in-the-runtime-layer/"
left_nav_group: core-model
chapter_ref: null
seo_description: "Learn what belongs in the runtime layer in UMA: validation, adapter binding, policy, lifecycle evidence, and the governed conditions around execution."
breadcrumbs:
  - "Home"
  - "Core Model"
  - "What Belongs in the Runtime Layer?"
related_refs:
  - active-descriptors
  - agent-vs-runtime
  - late-bound-policy-enforcement
  - what-is-a-capability
---

## intro

<section class="subpage-hero">
          <h1>What belongs in the runtime layer?</h1>
          <p>
            In UMA, the runtime layer owns the governed conditions around execution. The service keeps the durable business behavior. The
            runtime keeps the decisions that must remain visible when that behavior is validated, adapted, and allowed to run.
          </p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>The short answer</h2>
            <p>
              The runtime layer should own validation, adapter selection, policy enforcement, lifecycle evidence, and the execution context
              around the service. It should not quietly become the place where the service’s core business rule is rewritten.
            </p>
            <p>
              This distinction matters because many systems claim to preserve a portable service while still allowing runtime code to carry
              the rule that actually determines the outcome. Once that happens, the service boundary stops being the durable center of the
              architecture.
            </p>
          </section>

          <section>
            <h2>What should stay inside the service</h2>
            <p>
              The service should keep the business behavior that must remain semantically stable across hosts. That usually means rules,
              normalization logic, decision semantics, and the contract-shaped meaning of the result. This is the part of the system that
              portability is trying to preserve.
            </p>
            <p>
              If the service boundary is clean, the same behavior can be tested, compared, and reasoned about without depending on one
              transport, one adapter, or one deployment surface.
            </p>
          </section>

          <section>
            <h2>What the runtime should own instead</h2>
            <p>
              The runtime should own the questions around the service rather than the rule inside it. Is the request valid? Which adapter
              is allowed to satisfy the capability? Which policy applies? Should execution fail fast before side effects happen? What
              evidence should be recorded so the run can be explained afterward?
            </p>
            <p>
              Those concerns are not secondary infrastructure details. They are part of the architecture because they determine how the
              system turns portable behavior into governed execution.
            </p>
          </section>

          <section class="subpage-grid">
            <article class="subpage-card">
              <h3>Validation</h3>
              <p>Input checks, constraint checks, and fail-fast rules should happen before side effects are allowed to begin.</p>
            </article>
            <article class="subpage-card">
              <h3>Adapter binding</h3>
              <p>The runtime should decide which host capability or adapter implementation satisfies the requested behavior.</p>
            </article>
            <article class="subpage-card">
              <h3>Policy and trust</h3>
              <p>Permissions, placement, trust boundaries, and environment-specific rules belong around the service, not inside it.</p>
            </article>
            <article class="subpage-card">
              <h3>Lifecycle evidence</h3>
              <p>The runtime should keep enough observable evidence to explain what path actually ran and why.</p>
            </article>
          </section>

          <section>
            <h2>Why fail-fast validation belongs there</h2>
            <p>
              One of the clearest runtime responsibilities is stopping invalid execution before the system causes side effects. If the
              runtime has enough context to know that a request is malformed, incompatible, or disallowed, the architecture is stronger
              when that rejection happens before the adapter path even begins.
            </p>
            <p>
              This is not only about safety. It is about keeping the system honest. A runtime that validates too late often forces cleanup
              logic to carry architectural meaning that should have stayed visible from the start.
            </p>
          </section>

          <section>
            <h2>Why adapter binding should stay visible</h2>
            <p>
              Adapter selection is easy to hide in helper code, but that usually makes the architecture less legible. If a capability can
              be satisfied through different bindings or wrappers, the system is easier to review when the runtime owns that decision and
              records it explicitly.
            </p>
            <p>
              That makes the system more explainable to both people and tooling. The service result can stay stable while the runtime still
              shows which adapter path actually participated.
            </p>
          </section>

          <section>
            <h2>Why lifecycle evidence matters</h2>
            <p>
              A runtime layer becomes much more valuable once it can show what happened instead of merely claiming success. Event ordering,
              binding records, validation failures, and final state markers help the architecture stay auditable instead of turning into an
              opaque control flow.
            </p>
            <p>
              This does not require a heavy observability platform to matter. Even a small service becomes easier to trust when the runtime
              leaves behind enough evidence to prove which governed path actually ran.
            </p>
          </section>

          <section>
            <h2>What the runtime layer should not become</h2>
            <ul>
              <li>It should not absorb the core business rule because the host code feels more convenient.</li>
              <li>It should not become a vague utility layer with hidden architectural decisions.</li>
              <li>It should not hide adapter choice when that choice affects trust, policy, or evidence.</li>
              <li>It should not wait until after side effects to perform validation it already knows how to do.</li>
              <li>It should not make the service boundary harder to see.</li>
            </ul>
          </section>

          <section>
            <h2>A practical design test</h2>
            <p>
              Ask whether your system can point to the exact place where validation happens, the exact place where adapter choice is made,
              and the exact place where execution evidence is recorded. If those answers are scattered across helpers and host-specific
              code, the runtime layer still exists, but the architecture has not owned it cleanly.
            </p>
            <p>
              Another useful test is this: if the service behavior stays the same, can the runtime change validation rules, adapter
              wrappers, or lifecycle recording without forcing a rewrite of the core logic? If yes, the separation is probably healthy.
            </p>
          </section>

          <section>
            <h2>Frequently asked questions</h2>
            <h3>Does the runtime layer replace the service?</h3>
            <p>
              No. The service remains the durable behavioral center. The runtime layer exists around it so execution conditions stay
              explicit instead of leaking back into the service logic.
            </p>
            <h3>Why not keep adapter binding inside the host application?</h3>
            <p>
              You can, but the architecture becomes harder to inspect. UMA is stronger when adapter choice is treated as a visible runtime
              concern rather than an implementation accident.
            </p>
            <h3>Is lifecycle evidence only useful in large systems?</h3>
            <p>
              No. It becomes more valuable at scale, but even a small service is easier to trust when the runtime can explain what it did
              and what it chose.
            </p>
          </section>

          <section class="subpage-callout">
            <strong>Follow the runtime boundary further</strong>
            <p>
              This page isolates one practical question the chapter sequence makes important: what the runtime should own once the service
              itself stays clean. In the book, I push that separation further into execution traces, adapter design, and the evolving
              responsibilities of the runtime as systems grow. On the site, the best next step is to connect this idea to the UMA runtime
              and to the runnable examples.
            </p>
            <div class="subpage-inline-links">
              <a href="../what-is-a-uma-runtime/">What is a UMA runtime?</a>
              <a href="../runtime-agnostic-architecture/">Runtime-agnostic architecture</a>
              <a href="../what-makes-a-service-portable/">What makes a service portable?</a>
              <a href="../examples/">Examples</a>
              <a href="../diagrams/">Diagrams</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
