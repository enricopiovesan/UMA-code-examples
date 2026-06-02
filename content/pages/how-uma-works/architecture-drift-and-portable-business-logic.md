---
ref: architecture-drift-and-portable-business-logic
title: "Architecture Drift and Portable Business Logic"
subtitle: "Architecture drift and portable business logic Architecture drift often begins when the same business behavior is copied into browser code, mobile code, backend code, edge functions, workflow glue, and AI-adjacent automation. UMA attacks that drift by making the durable behavior portable and governed."
macro_area: how-uma-works
content_type: walkthrough
slug: architecture-drift-and-portable-business-logic
canonical_url: "https://www.universalmicroservices.com/architecture-drift-and-portable-business-logic/"
left_nav_group: how-uma-works
chapter_ref: null
seo_description: "Learn how UMA reduces architecture drift by keeping business behavior portable, contract-shaped, and governed across browser, edge, backend, and AI-assisted runtime paths."
breadcrumbs:
  - "Home"
  - "How Uma Works"
  - "Architecture Drift and Portable Business Logic"
related_refs:
  - incremental-uma-adoption
  - migrating-to-uma-incrementally
  - portable-business-logic
  - runtime-agnostic-architecture
---

## intro

<section class="subpage-hero"><h1>Architecture drift and portable business logic</h1><p>Architecture drift often begins when the same business behavior is copied into browser code, mobile code, backend code, edge functions, workflow glue, and AI-adjacent automation. UMA attacks that drift by making the durable behavior portable and governed.</p></section>

## main

<div class="subpage-body">
          <section><h2>The problem</h2><p>A duplicated rule can start as a harmless optimization. Over time, each copy adapts to local pressures. The web client handles one edge case, the backend handles another, the edge function adds a shortcut, and nobody can point to one authoritative behavior anymore.</p><p>UMA does not promise magic elimination of drift. It gives teams a model for reducing drift by keeping business behavior behind explicit contracts and visible runtime authority.</p></section>
          <section class="subpage-grid"><article class="subpage-card"><h3>Portable core</h3><p>Keep the durable behavior testable outside one host.</p></article><article class="subpage-card"><h3>Runtime wrapper</h3><p>Let validation, adapters, and policy live around the core.</p></article><article class="subpage-card"><h3>Observable parity</h3><p>Compare behavior from outputs and events, not wishful code similarity.</p></article><article class="subpage-card"><h3>Governed evolution</h3><p>Use contracts and runtime decisions to manage version growth.</p></article></section>
          <section><h2>The conversion point</h2><p>This is the pressure that makes the book worth reading. The website can name the pattern, but the book walks through how a system moves from one portable behavior to runtime governance, trust, discoverability, and long-term evolution.</p></section>

          <section>
            <h2>What architecture drift looks like in practice</h2>
            <p>Drift is not dramatic. It does not announce itself as a failure. It arrives as a sequence of small, individually reasonable decisions that accumulate into a system where no one can confidently state what the rule is without checking every copy.</p>
            <p>A pricing rule gets an edge-case fix on the backend service after a QA report. The person who makes the fix does not update the browser copy, because the browser copy is in a different repository owned by a different team. Three weeks later, the mobile client ships with neither fix, because the mobile version was forked from an older state of the backend. The three implementations now handle one edge case differently each. None of the differences surface as errors. they produce different numbers in low-frequency paths, and the divergence is only discovered when a customer reports inconsistent totals.</p>
            <p>A validation rule accumulates different error messages across the mobile client and the server because two teams localized the error text independently and made different choices about phrasing. The rules themselves are still identical. But the observable behavior (what the user sees when validation fails) now differs by surface.</p>
            <p>A feature flag evaluator handles null inputs differently in the TypeScript version than in the Rust version, because the TypeScript version was written first, shipped with a defensive null check, and the Rust version was written six months later by someone who did not know about the null edge case because it was not in the spec. Both implementations pass their tests, because both test suites were written against the existing implementation, not against a shared behavioral contract.</p>
            <p>Each of these drift events is small. None is a crisis. The aggregate is a system where the authoritative version of any given rule is undefined: it exists in multiple places, in multiple states, and "what does the pricing rule do?" is a research project rather than a question with a clear answer.</p>
          </section>

          <section>
            <h2>Why drift is structural, not behavioral</h2>
            <p>Teams do not intend to drift. The engineers who copy a pricing rule into the browser are not planning to maintain a divergent version. They copy it because the architecture requires a copy: the browser cannot call the backend in that context, so the logic must travel. The act of copying is the correct response to an architectural constraint. The divergence that follows is the automatic, predictable output of a system where the same logic lives in multiple codebases.</p>
            <p>This distinction matters because it points away from behavioral solutions. Better documentation does not fix drift (it records the intended state, which is already documented somewhere, which is why the implementations were supposed to be equivalent in the first place. Stricter code review does not fix drift) review can catch a specific divergence when it is introduced, but it cannot systematically ensure that a fix in one repository propagates to every other repository that holds a copy. Cross-team communication processes do not fix drift. they create coordination overhead that makes the next copy more expensive but does not eliminate the structural need for copies.</p>
            <p>The structural fix is not a process. It is a model where the same compiled artifact runs in all environments, so there is only one thing to review and one place where the rule lives. When the pricing rule is a portable service with a stable contract, there is no browser copy and no server copy. there is one binary, one contract, and a runtime that selects the appropriate adapter for each execution surface. The artifact that runs in the browser is the same artifact that runs on the server. There is no mechanism by which they can diverge, because they are not separate things.</p>
            <p>That is what makes the fix structural: it removes the condition that makes drift possible, rather than adding governance over a condition that remains.</p>
          </section>

          <section>
            <h2>How portable business logic eliminates drift by construction</h2>
            <p>A portable service has one compiled artifact. When the pricing rule is expressed as a portable WASM module with a declared contract, that module is the rule. Not a copy of the rule. Not a representation of the rule. The rule.</p>
            <p>The contract defines what the rule accepts, what it returns, and what it guarantees. The contract is the source of truth for the rule's behavior. not the documentation, not the comments, not the test suite, but a machine-readable descriptor that the runtime enforces at every invocation. When the contract says the pricing rule returns a price as a decimal with two-place precision, every runtime that calls the rule gets that guarantee.</p>
            <p>A parity proof verifies that the portable artifact produces equivalent outputs across all execution surfaces where it is deployed. This is not a claim about code similarity: it is an empirical verification from emitted events and return values. The proof runs in CI. If the behavior on the edge runtime diverges from the behavior on the server runtime, the proof fails, the build fails, and the divergence is caught before it ships rather than discovered in a customer report six weeks later.</p>
            <p>Drift cannot happen between copies that do not exist. When the browser, the server, the edge node, and the mobile offline context all execute the same WASM binary through runtime-selected adapters, there are no copies to diverge. When a rule changes, there is one change: one commit, one review, one compiled artifact, one CI run that proves the new behavior is equivalent across all execution surfaces. The rule's history is a single Git log, not a merge reconciliation across three repositories. The rule's current state is a single binary hash, not a comparison exercise across four codebases.</p>
            <p>That is what "eliminates drift by construction" means in practice: the architectural model makes the maintenance of multiple diverging copies structurally unnecessary, rather than trying to enforce consistency across copies that must exist.</p>
          </section>

          <section class="subpage-callout"><strong>Covered in the book</strong><p>Chapters 4, 6, 10, and 11 show the drift problem from different angles: portable behavior, proof of parity, coherence tradeoffs, and evolution without fragmentation.</p><div class="subpage-inline-links"><a href="../portable-business-logic/">Portable business logic</a><a href="../examples/chapter-04-feature-flag-evaluator/">Chapter 4 example</a><a href="../examples/chapter-10-architectural-tradeoffs/">Chapter 10 example</a><a href="../examples/chapter-11-evolution-without-fragmentation/">Chapter 11 example</a><a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order the book</a></div></section>
        </div>
        <section id="contacts" class="section contacts-band" data-shared-footer></section>
