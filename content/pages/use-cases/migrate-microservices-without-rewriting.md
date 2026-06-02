---
ref: migrate-microservices-without-rewriting
title: "Migrate Microservices Without Rewriting Them"
subtitle: "How to modernise a distributed system one service at a time without breaking what works or committing to a full rewrite."
macro_area: use-cases
content_type: concept
slug: migrate-microservices-without-rewriting
canonical_url: "https://www.universalmicroservices.com/use-cases/migrate-microservices-without-rewriting/"
left_nav_group: use-cases
seo_description: "Migrate microservices incrementally without a full rewrite. How UMA portable services let you modernise one service at a time while keeping the rest running."
breadcrumbs:
  - "Home"
  - "Use Cases"
  - "Migrate Without Rewriting"
related_refs:
  - core-model
  - how-uma-works
  - comparisons
---

## intro

<section class="subpage-hero">
          <h1>Migrate microservices without rewriting them</h1>
          <p>
            Your system has forty services. Some are fine. Some are a liability. You cannot stop the world to fix them,
            and you have been burned by big-bang rewrites before. There is a way through that does not require either
            tolerance for the status quo or a year-long freeze on everything else.
          </p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>The problem: modernisation paralysis</h2>
            <p>
              Most teams reach a point where the cost of changing a service outweighs the cost of tolerating its problems.
              Not because the problems are small, but because change is expensive when the service is deeply coupled to
              everything around it. Callers depend on internal implementation details. The deployment pipeline is shared
              with things that cannot move. The team that owns it has been restructured twice since it was built.
            </p>
            <p>
              The result is a system that grows around its worst parts. New capabilities get bolted onto old foundations.
              The services that most need attention are the ones least safe to touch. That is modernisation paralysis,
              and it is not a failure of will. It is a structural problem that comes from tight coupling between services
              that should have been independent.
            </p>
            <p>
              Every migration plan that assumes you can touch one service in isolation without defining what isolation
              actually means is going to hit the same wall.
            </p>
          </section>

          <section>
            <h2>Why big-bang rewrites always fail</h2>
            <p>
              The appeal of the full rewrite is understandable. Start clean, apply lessons learned, remove the accumulated
              debt in one coordinated effort. In practice, it fails for the same reason every time.
            </p>
            <p>
              The old system keeps running while the new one is being built. Requirements continue to change. The two
              systems diverge. Feature parity becomes a moving target. By the time the new system is nearly ready, the
              scope of what it needs to match has grown. The cutover date slips. Pressure builds to declare victory
              before the system is actually ready. Then the new system ships with different bugs and the same structural
              problems because the underlying architecture was not examined, only re-implemented.
            </p>
            <p>
              The rewrite also concentrates risk. You go from a known-bad system to an unknown-risk one in a single step.
              There is no incremental confidence, no gradual validation under real load, and no fallback once the switch
              is made.
            </p>
          </section>

          <section>
            <h2>The incremental path UMA enables</h2>
            <p>
              Universal Microservices Architecture addresses this at the level of service contracts rather than service
              implementations. A UMA-portable service expresses what it does through a stable, runtime-readable descriptor
              rather than through its internal structure. That descriptor is what the rest of the system depends on, not
              the implementation behind it.
            </p>
            <p>
              When that contract is in place, you can replace the implementation without touching anything that calls it.
              The runtime resolves the capability by descriptor. Callers do not know or care whether the service behind
              the descriptor has been rewritten in a new language, moved to a different host, or restructured internally.
              The contract is what the system depends on.
            </p>
            <p>
              This is the mechanism that makes incremental migration real rather than theoretical. You are not asking
              teams to coordinate a simultaneous change across dozens of services. You are asking one team to wrap one
              service in a portable contract, validate the descriptor, and migrate the implementation behind it at
              their own pace. The rest of the system is unaffected.
            </p>
            <p>
              The portable contract also removes the hidden coupling that makes rewrites fail. When the implementation
              detail is no longer the surface that callers depend on, you can change the implementation without
              negotiating with every downstream team.
            </p>
          </section>

          <section>
            <h2>What this looks like in practice</h2>
            <p>
              One team owns a legacy pricing service. It is in the wrong language, running on infrastructure the
              organisation wants to retire, and it has not been meaningfully changed in three years because the blast
              radius of any change is too large to reason about.
            </p>
            <p>
              The team wraps the service in a UMA capability descriptor. The descriptor expresses what the service does
              in terms the runtime can discover and validate. Existing callers are updated to resolve against the
              descriptor rather than a hard-coded endpoint. This is the only cross-team coordination required at this
              stage, and it is a change in how the service is addressed, not in what it does.
            </p>
            <p>
              Once the descriptor is in place and validated under production traffic, the team migrates the implementation
              behind it. New language, new infrastructure, new internal structure. The rest of the system sees no change
              because it was already depending on the contract rather than the implementation.
            </p>
            <p>
              The team can roll back at any point during the migration by swapping the implementation behind the descriptor.
              There is no cutover moment where the old system must be switched off before the new one is confirmed working.
              Confidence is built incrementally, under real conditions, with a clear fallback.
            </p>
            <p>
              Apply that pattern to each of the forty services, one at a time, at whatever pace makes sense given team
              capacity and risk tolerance. The system as a whole continues to operate throughout. Each migration is
              isolated. No single team is blocked waiting for another.
            </p>
          </section>

          <section>
            <h2>Where to go next</h2>
            <p>
              The mechanism behind portable contracts and capability descriptors is covered in the core model pages.
              The comparisons section covers how this approach differs from service mesh patterns, API gateway strategies,
              and other migration frameworks that address the problem at a different layer.
            </p>
            <p>
              If you are evaluating whether this is the right approach for a specific system, the core model gives you
              the vocabulary to reason about what a portable contract actually contains, and what the runtime needs to
              resolve one reliably.
            </p>
          </section>

          <section class="subpage-callout">
            <strong>Read the underlying model</strong>
            <p>
              The incremental migration path depends on how UMA defines portable service contracts and runtime resolution.
              The core model pages cover both in detail. The book extends that into a full treatment of how capability
              governance works across a live distributed system.
            </p>
            <div class="subpage-inline-links">
              <a href="../../core-model/">Core Model</a>
              <a href="../../comparisons/">Comparisons</a>
              <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order on Amazon</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
