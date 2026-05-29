---
ref: portable-business-logic
title: "Portable Business Logic"
subtitle: "Portable business logic Portable business logic is the part of the system that should outlast frameworks, hosts, and deployment patterns. In UMA, that logic is treated as a durable service boundary with explicit inputs, outputs, and contracts, rather than as code trapped inside one application layer."
macro_area: how-uma-works
content_type: walkthrough
slug: portable-business-logic
canonical_url: "https://www.universalmicroservices.com/portable-business-logic/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "Understand portable business logic through the UMA model: what should stay durable, what should stay explicit, and how to avoid stack-bound rewrites."
breadcrumbs:
  - "Home"
  - "How Uma Works"
  - "Portable Business Logic"
related_refs:
  - architecture-drift-and-portable-business-logic
  - incremental-uma-adoption
  - migrating-to-uma-incrementally
  - runtime-agnostic-architecture
---

## intro

<section class="subpage-hero">
          <h1>Portable business logic</h1>
          <p>
            Portable business logic is the part of the system that should outlast frameworks, hosts, and deployment patterns. In UMA, that
            logic is treated as a durable service boundary with explicit inputs, outputs, and contracts, rather than as code trapped inside
            one application layer.
          </p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>What portable business logic actually means</h2>
            <p>
              Portable business logic is not just shared code or a convenience library. It is the part of the system that preserves rules,
              decisions, and domain meaning even when the surrounding runtime changes. In UMA, that logic is treated as a durable service
              boundary with explicit contracts, instead of being trapped inside one frontend, backend, or middleware layer.
            </p>
            <p>
              This distinction matters because software teams often say they want reuse while continuing to let each stack reinterpret the
              same behavior in its own way. A portable service model is stricter than that. It requires the meaning of the service to stay
              stable while the runtime around it adapts.
            </p>
            <p>
              Another way to frame it is that portable business logic is one practical expression of
              <a href="../from-stack-ownership-to-behavior-ownership/">behavior ownership</a>. The architecture stops treating each stack
              as the natural owner of the rule and starts treating the rule itself as the thing that deserves a durable center.
            </p>
          </section>

          <section>
            <h2>What should be portable</h2>
            <p>
              The most valuable behavior in a product is the part that encodes rules, decisions, and domain meaning. That is the behavior
              worth preserving. User-interface concerns, infrastructure integration, transport details, and host capabilities still matter,
              but they should not own the domain logic that makes the service useful.
            </p>
            <p>
              If the core business behavior is allowed to drift between browser code, backend code, and edge logic, the system eventually
              stops having one truth. Portable business logic is the answer to that problem. It gives the team one durable expression of the
              service, while still allowing runtime-specific adaptation around it.
            </p>
          </section>

          <section>
            <h2>Why contracts matter so much</h2>
            <p>
              Contracts are what make portability governable instead of aspirational. They give the service a visible shape: inputs,
              outputs, constraints, and expectations. Without them, “shared logic” tends to become a soft promise that each runtime
              interprets differently.
            </p>
            <p>
              With explicit contracts, the logic can be preserved, versioned, compared, validated, and tested across environments. That is
              one of the biggest differences between portable business logic and a loosely shared utility package.
            </p>
          </section>

          <section class="subpage-grid">
            <article class="subpage-card">
              <h3>Keep the core stable</h3>
              <p>Portable logic should not change just because the system moved from browser to edge or from edge to cloud.</p>
            </article>
            <article class="subpage-card">
              <h3>Make bindings visible</h3>
              <p>Capabilities, adapters, and runtime bindings should surround the logic instead of being fused into it.</p>
            </article>
            <article class="subpage-card">
              <h3>Version the behavior</h3>
              <p>A portable service boundary is easier to test, compare, evolve, and reason about than duplicated stack-owned logic.</p>
            </article>
            <article class="subpage-card">
              <h3>Reduce stack ownership</h3>
              <p>The architecture becomes more coherent when rules belong to the service model rather than to the frontend or backend toolchain.</p>
            </article>
          </section>

          <section>
            <h2>Why teams benefit</h2>
            <p>
              When the logic stays portable, teams spend less time reconciling behavior across implementations and more time improving the
              product. It also becomes easier to reason about change because the durable business meaning of the service stops moving every
              time the surrounding runtime changes.
            </p>
            <p>
              That benefit compounds over time. The longer a product lives, the more expensive behavioral duplication becomes. Portable
              business logic reduces that future cost by making the rules of the system easier to preserve and easier to govern.
            </p>
            <p>
              It also improves cross-team clarity. Instead of one team owning the “real” rule in one stack and another team recreating it
              elsewhere, the service boundary itself becomes the common reference point.
            </p>
          </section>

          <section>
            <h2>What portability does not mean</h2>
            <ul>
              <li>It does not mean every service should execute in every environment.</li>
              <li>It does not mean UI concerns disappear into the service layer.</li>
              <li>It does not mean runtime bindings can be ignored.</li>
              <li>It does not mean data access, trust, and policy stop mattering.</li>
              <li>It means the core business behavior is no longer owned by one stack.</li>
            </ul>
          </section>

          <section>
            <h2>Where teams usually go wrong</h2>
            <p>
              A common failure mode is to preserve only part of the logic while letting runtime code fill in the rest through implicit
              assumptions. Another is to call something portable while it still depends on local framework state, hidden host bindings, or
              unspoken data-shape assumptions.
            </p>
            <p>
              Portable business logic works only when the service boundary is explicit enough that the runtime can surround it without
              rewriting it. That means teams need to be honest about what belongs inside the service and what belongs outside it.
            </p>
          </section>

          <section>
            <h2>Frequently asked questions</h2>
            <h3>Is portable business logic the same as shared code?</h3>
            <p>
              Not exactly. Shared code can still be vague, under-governed, or tied to local assumptions. Portable business logic is shared
              through a clear service boundary with explicit contracts and a runtime story around it.
            </p>
            <h3>Does portable business logic remove the need for runtime-specific code?</h3>
            <p>
              No. Runtime-specific code still matters for validation, transport, trust, adaptation, and integration. The point is that this
              code should stay around the service instead of silently becoming the place where the business behavior is redefined.
            </p>
          </section>

          <section class="subpage-callout">
            <strong>Continue</strong>
            <p>
              Portable business logic becomes much clearer once you connect it to behavior ownership and the runtime model around it. The
              book goes deeper into that progression by showing where the portable core stops and the governed system begins.
            </p>
            <div class="subpage-inline-links">
              <a href="../from-stack-ownership-to-behavior-ownership/">From stack ownership to behavior ownership</a>
              <a href="../what-is-a-capability/">What is a capability?</a>
              <a href="../runtime-agnostic-architecture/">Runtime-agnostic architecture</a>
              <a href="../webassembly-architecture/">WebAssembly architecture</a>
              <a href="../examples/">Examples</a>
              <a href="../diagrams/">Diagrams</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
