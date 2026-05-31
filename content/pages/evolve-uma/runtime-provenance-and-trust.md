---
ref: runtime-provenance-and-trust
title: "Runtime Provenance and Trust in UMA"
subtitle: "Runtime provenance and trust Portable behavior does not automatically become trusted behavior. UMA makes publisher identity, declared permissions, dependency provenance, placement, and communication rules visible to the runtime."
macro_area: evolve-uma
content_type: walkthrough
slug: runtime-provenance-and-trust
canonical_url: "https://www.universalmicroservices.com/runtime-provenance-and-trust/"
left_nav_group: evolve-uma
chapter_ref: null
seo_description: "Learn how UMA makes provenance, permissions, dependencies, and trust boundaries explicit in runtime decisions instead of hiding them in perimeter infrastructure."
breadcrumbs:
  - "Home"
  - "Evolve Uma"
  - "Runtime Provenance and Trust in UMA"
related_refs:
  - ai-native-runtime-governance
  - contract-driven-orchestration
  - how-systems-evolve-without-fragmentation
  - service-graph-evolution
---

## intro

<section class="subpage-hero"><h1>Runtime provenance and trust</h1><p>Portable behavior does not automatically become trusted behavior. UMA makes publisher identity, declared permissions, dependency provenance, placement, and communication rules visible to the runtime.</p></section>

## main

<div class="subpage-body">
          <section>
            <h2>The architectural shift</h2>
            <p>Traditional systems often inherit trust from location: inside the network, inside the cluster, inside the gateway. UMA asks a different question: what does this module declare, who published it, what does it depend on, and what is it allowed to communicate with?</p>
            <p>That makes trust part of the system model instead of a perimeter assumption. The difference matters because location-based trust is invisible: a service inside the perimeter is trusted, but the system has no record of why, what it was allowed to do, or whether the decision to allow it was ever evaluated. When something goes wrong — a misconfigured service, an undeclared dependency, a lateral move inside the network — there is nothing in the execution record to reconstruct what happened.</p>
            <p>Provenance changes that. When the runtime records who published a service, which permissions it declared, which dependencies it resolved, and which communication paths it was allowed to follow, that record becomes the basis for both runtime enforcement and post-hoc audit. The question is no longer "did the request come from inside the network?" but "was this execution authorized, and can we prove it?"</p>
          </section>

          <section class="subpage-grid">
            <article class="subpage-card"><h3>Publisher</h3><p>Who produced the service matters to runtime approval. A service from an unrecognized publisher — or one whose identity cannot be verified against the declared module signature — fails before execution begins.</p></article>
            <article class="subpage-card"><h3>Permissions</h3><p>Requested capability access must match declared permissions. A service that requests <code>network.fetch</code> without declaring it in its descriptor is denied at the adapter binding stage, not at the network layer.</p></article>
            <article class="subpage-card"><h3>Dependencies</h3><p>Provenance and checksums can affect whether a service is trusted. A service whose declared dependencies resolve to modules with unknown provenance or mismatched checksums can be held at the evaluation stage before any execution occurs.</p></article>
            <article class="subpage-card"><h3>Communication</h3><p>Event compatibility is not enough if trust policy rejects the path. Two services can share a compatible event schema but have a trust policy that forbids the communication path between them — the runtime enforces both.</p></article>
          </section>

          <section>
            <h2>Provenance in a multi-runtime system</h2>
            <p>In a single-runtime system, provenance is simple: one runtime approved the execution, and you can inspect its logs. In a multi-runtime system — browser, edge, cloud, or a mix of on-device and server — the same service may run under different runtimes with different trust policies. The question of whether a decision was authorized becomes: authorized by which runtime, under which policy, with which dependency graph resolved at what point in time?</p>
            <p>UMA treats trust as a property of the execution graph, not of individual services. A service is not trusted in the abstract. It is trusted by a specific runtime in a specific deployment context, with a specific set of declared permissions verified and a specific set of dependencies resolved. That binding is what the active descriptor captures at runtime: not just what the service is, but what was evaluated and approved before it ran.</p>
            <p>This matters when a service moves. If a capability that ran in the cloud is moved to an edge runtime, the trust evaluation does not transfer automatically. The edge runtime runs its own approval sequence — publisher verification, permission check, dependency resolution, communication policy — and produces its own provenance record. The two records can then be compared. If the edge runtime approved something the cloud runtime would have denied, that discrepancy is visible in the execution graph rather than hidden in runtime-specific logs.</p>
          </section>

          <section>
            <h2>What the runtime records</h2>
            <p>When a UMA runtime evaluates a service for execution, it records the outcome of each evaluation step as part of the active descriptor. That record includes: which publisher identity was verified, which permissions were declared and which were granted, which dependencies were resolved and whether their checksums matched expected values, and which communication paths were approved under the current trust policy.</p>
            <p>This record is not a log file appended after the fact. It is structural metadata that travels with the service's execution context. When the service emits an event or produces output, the provenance record is part of the execution context that a downstream runtime can inspect. A service that receives an event can ask not just "is this event schema compatible?" but "was the service that emitted this event approved by a runtime I recognize, under a trust policy that includes the permissions this event implies?"</p>
            <p>For audit and compliance use cases, this is the difference between reconstructing what happened from logs and having a signed execution graph that records what was approved, by whom, and when.</p>
          </section>

          <section>
            <h2>What the Chapter 9 example demonstrates</h2>
            <p>The Chapter 9 example is the proof layer for the trust model. It runs five scenarios in sequence: a service with a recognized publisher and declared permissions is allowed; a service that requests an undeclared permission is denied at the adapter binding stage; a service whose dependency resolves to a module with a mismatched checksum is held before execution; a service that attempts to communicate on a path forbidden by trust policy is blocked even though the event schema is compatible; and a service that was previously denied — because of a corrected permission declaration — is re-evaluated and approved, demonstrating that the trust model is not a static block list but a policy evaluation against the current state of the descriptor.</p>
            <p>Each scenario produces observable output that the smoke script validates. The proof is not that the system described these behaviors but that the running code enforced them.</p>
          </section>

          <section class="subpage-callout">
            <strong>Go deeper in the book</strong>
            <p>Chapter 9 expands the trust-boundary sequence into a complete governance model: how policies are composed, how trust decisions are recorded in active descriptors, and how the execution graph supports compliance workflows that span multiple runtimes.</p>
            <div class="subpage-inline-links">
              <a href="../trust-boundaries/">Trust boundaries</a>
              <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Universal Microservices Architecture (book)</a>
            </div>
          </section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
