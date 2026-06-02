---
ref: faq
title: "UMA FAQ"
subtitle: "Universal Microservices Architecture FAQ This FAQ is meant to be maintained over time as the site, examples, and book evolve. It collects short, direct answers to the questions that come up most often when people first encounter UMA and want to understand how the model behaves in practice. The shortest framing to keep in mind is this: UMA is an execution model for distributed systems where compute can happen in many places and the system decides where logic runs."
macro_area: discoverability
content_type: resource
slug: faq
canonical_url: "https://www.universalmicroservices.com/faq/"
left_nav_group: discoverability
chapter_ref: null
seo_description: "Frequently asked questions about Universal Microservices Architecture, including portability, WebAssembly, runtime governance, trust boundaries, and examples."
breadcrumbs:
  - "Home"
  - "Discoverability"
  - "UMA FAQ"
related_refs:
  - about-enrico
  - diagrams
---

## intro

<section class="subpage-hero">
          <h1>Universal Microservices Architecture FAQ</h1>
          <p>
            This FAQ is meant to be maintained over time as the site, examples, and book evolve. It collects short, direct answers to the
            questions that come up most often when people first encounter UMA and want to understand how the model behaves in practice.
          </p>
          <p>
            The shortest framing to keep in mind is this: UMA is an execution model for distributed systems where compute can happen in
            many places and the system decides where logic runs.
          </p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>Foundations</h2>
            <h3>What is Universal Microservices Architecture?</h3>
            <p>
              Universal Microservices Architecture is an execution model for distributed systems where compute can happen in many places
              and the system decides where logic runs. It keeps service behavior portable while making contracts, runtime policy, trust,
              and execution boundaries explicit.
            </p>
            <h3>Why does UMA exist?</h3>
            <p>
              UMA exists because modern systems increasingly duplicate the same business logic across many runtime surfaces. That duplication
              creates drift, inconsistent outcomes, and hidden architecture. UMA addresses that by preserving one portable expression of the
              behavior and surrounding it with explicit runtime governance.
            </p>
            <h3>What problem does UMA solve that good platform tooling still leaves behind?</h3>
            <p>
              UMA solves a different problem from deployment tooling alone. Platforms can schedule, route, and observe workloads well, but
              they do not automatically keep one business behavior coherent across browser, edge, backend, workflow, and AI-assisted
              execution surfaces. UMA is meant to close that architectural gap.
            </p>
            <p>
              The dedicated <a href="../what-problem-does-uma-solve/">what problem does UMA solve?</a> page is the fastest entry point if
              you want the direct version of that argument before the wider concept cluster.
            </p>
            <h3>What is the difference between stack ownership and behavior ownership?</h3>
            <p>
              Stack ownership organizes software around runtime or team boundaries. Behavior ownership organizes it around the durable rule
              or domain meaning that must survive across those boundaries. UMA pushes toward the second view.
            </p>
            <h3>What does runtime-agnostic architecture actually mean?</h3>
            <p>
              It means the service behavior can stay semantically stable while the runtime around it changes. Validation, transport,
              placement, identity, permissions, and trust still matter, but they stop being the place where the business meaning of the
              service is quietly rewritten.
            </p>
            <p>
              The useful shorthand is not “write once, run everywhere.” It is “write once, run where it makes sense.”
            </p>
            <p>
              The dedicated <a href="../runtime-agnostic-architecture/">runtime-agnostic architecture</a> page goes deeper into that
              distinction and why it matters for modern systems that span browser, edge, backend, and workflow execution surfaces.
            </p>
            <h3>How is UMA different from traditional microservices?</h3>
            <p>
              Traditional microservices usually focus on deployable units, service ownership, and networked boundaries. UMA focuses on a
              different question: can the same business behavior remain stable while the execution context changes? That leads to a stronger
              emphasis on portability, contracts, runtime visibility, and long-term system coherence.
            </p>
            <p>
              If you want the longer version of that comparison, the dedicated
              <a href="../uma-vs-traditional-microservices/">UMA vs traditional microservices</a> page goes deeper without flattening the
              distinction into a slogan.
            </p>
            <h3>Does UMA replace frontend and backend architecture?</h3>
            <p>
              No. UMA does not erase frontend, backend, edge, or cloud concerns. It gives teams a different way to place business behavior
              inside those environments. The important shift is that service meaning is no longer tightly bound to one stack.
            </p>
          </section>

          <section>
            <h2>Runtime and execution</h2>
            <h3>Is UMA only about WebAssembly?</h3>
            <p>
              No. WebAssembly is a strong fit because it gives portable behavior a practical execution boundary, but the larger subject is
              architectural coherence across runtimes. UMA is about service meaning, contracts, runtime visibility, and system evolution.
            </p>
            <h3>Why does WebAssembly matter here?</h3>
            <p>
              WebAssembly matters because it gives portable service behavior a compact, sandboxed execution target that is easier to move
              between environments. It does not define the whole model by itself, but it makes runtime portability more practical.
            </p>
            <h3>How do I prove portability instead of assuming it?</h3>
            <p>
              Portability should be validated with evidence. Run the same service behavior in more than one runtime target, compare the
              outputs, and inspect the surrounding runtime behavior that shapes the result. The examples on this site are designed to make
              that proof visible instead of leaving it implicit.
            </p>
            <p>
              The <a href="../how-to-prove-portability/">how to prove portability</a> page turns that answer into a more concrete checklist
              based on observable parity and explicit runtime differences.
            </p>
            <h3>Do the UMA examples publish any benchmark or footprint data?</h3>
            <p>
              Yes. The site now includes benchmark and footprint notes for selected early chapters so readers can inspect artifact size,
              repeated timing data, and the exact local environment used to generate them. The point is not to claim one universal winner.
              It is to show the tradeoff honestly and make the portability claim inspectable.
            </p>
            <p>
              The <a href="../benchmark-and-footprint/">benchmark and footprint notes</a> page is the best place to start if you want
              measured proof before reading deeper into the model.
            </p>
            <h3>What should happen when compatibility breaks in a UMA service graph?</h3>
            <p>
              The break should remain visible. Missing edges, waiting consumers, or failed compatibility checks are healthier than a system
              that quietly hides the problem, because they make graph drift inspectable and easier to repair.
            </p>
            <p>
              The <a href="../service-graph-evolution/">service graph evolution</a> page now covers that failure-and-recovery angle more
              directly.
            </p>
            <h3>Can a UMA system work and still be architecturally incoherent?</h3>
            <p>
              Yes. A system can still produce the expected result while degrading through over-granular boundaries, vague event semantics,
              runtime ambiguity, or orchestration that makes the architecture harder to explain than the output suggests.
            </p>
            <p>
              The <a href="../what-makes-a-system-coherent/">what makes a system coherent?</a> page isolates that distinction directly.
            </p>
            <h3>How can a system evolve without fragmentation?</h3>
            <p>
              By keeping change anchored to explicit behavior, visible compatibility, and governed coexistence. Drift does not always mean
              a rewrite is next, but it does mean the system needs a clear authority for deciding which behavior is still valid.
            </p>
            <p>
              The <a href="../how-systems-evolve-without-fragmentation/">how systems evolve without fragmentation</a> page follows that
              progression from contract anchor through drift, duplication, version sprawl, and runtime-governed recovery.
            </p>
          </section>

          <section>
            <h2>Governance and trust</h2>
            <h3>Where do trust boundaries live in UMA?</h3>
            <p>
              Trust boundaries live in the runtime and governance layer around the portable service. Identity resolution, permissions,
              provenance, policy evaluation, and audit evidence should stay visible there rather than being hidden inside business logic.
            </p>
            <h3>Can a service be valid and still be denied in UMA?</h3>
            <p>
              Yes. A service can be syntactically valid and still be denied because it requested undeclared permissions, carries untrusted
              dependency provenance, or attempts communication the runtime trust policy does not allow.
            </p>
            <p>
              The <a href="../trust-boundaries/">trust boundaries</a> page now covers that distinction more directly, including why
              compatibility alone is not the same thing as authorization.
            </p>
            <h3>What is the difference between an agent and the runtime in UMA?</h3>
            <p>
              The agent is the reasoning side of the system. It can interpret a goal, inspect available capabilities, and propose a path.
              The runtime is the authority side of the system. It validates contracts, enforces constraints, and decides what is actually
              allowed to execute.
            </p>
            <h3>What is a capability in UMA?</h3>
            <p>
              A capability is the unit the runtime can discover and reason about. It represents a named piece of behavior with a visible
              contract and enough meaning for the runtime to validate, select, reject, or compose into a workflow.
            </p>
            <h3>What makes a service portable in UMA?</h3>
            <p>
              A service becomes portable when its business behavior stays stable, its contract remains explicit, its outputs stay
              comparable across runtimes, and the runtime-specific concerns around it do not quietly redefine the rule itself.
            </p>
            <p>
              The <a href="../what-makes-a-service-portable/">what makes a service portable?</a> page turns that into a practical design
              test instead of leaving portability as a vague promise.
            </p>
            <h3>What is a workflow in UMA?</h3>
            <p>
              A workflow is the path the runtime approves from one or more capabilities in order to satisfy a goal. It is visible enough
              for the system to explain why that path was chosen.
            </p>
            <h3>What is contract-driven orchestration?</h3>
            <p>
              Contract-driven orchestration means the runtime creates the execution path from declared events, subscriptions, metadata, and
              policy instead of burying the flow inside handwritten workflow glue. That makes orchestration much easier to govern and audit.
            </p>
            <p>
              The <a href="../contract-driven-orchestration/">contract-driven orchestration</a> page goes deeper into why that distinction
              matters once systems start emitting events and binding subscribers dynamically.
            </p>
            <h3>What is a UMA runtime?</h3>
            <p>
              A UMA runtime is the governed layer around portable behavior. It discovers capabilities, checks compatibility and
              constraints, approves the valid execution path, and keeps enough evidence for the result to remain explainable afterward.
            </p>
            <h3>What makes a decision discoverable in UMA?</h3>
            <p>
              A decision becomes discoverable when proposal, validation, revision, approved execution, and trace remain inspectable as
              artifacts instead of collapsing into one opaque runtime result.
            </p>
            <p>
              The <a href="../what-makes-a-decision-discoverable/">what makes a decision discoverable?</a> page turns that into a concrete
              lifecycle.
            </p>
            <h3>What belongs in the runtime layer?</h3>
            <p>
              The runtime layer should own validation, adapter binding, policy, trust, lifecycle evidence, and the conditions around
              execution that vary by environment. The service should keep the durable rule. the runtime should keep the governed execution
              context around it.
            </p>
            <p>
              The dedicated <a href="../what-belongs-in-the-runtime-layer/">what belongs in the runtime layer?</a> page turns that split
              into a practical design question instead of leaving it as a slogan.
            </p>
            <h3>What is WASM MCP in UMA?</h3>
            <p>
              WASM MCP is the discovery and invocation surface the runtime can use to understand which capabilities are available. It makes
              capability availability more explicit instead of leaving it buried inside implementation code.
            </p>
            <h3>What does governed execution mean?</h3>
            <p>
              Governed execution means that the runtime can explain why a capability was allowed, denied, routed, or adapted in a specific
              way. The point is not only to run code, but to keep the decisions around that execution understandable and reviewable.
            </p>
            <h3>Can UMA support enterprise systems?</h3>
            <p>
              Yes. In fact, UMA is most relevant when systems have to survive scale, multiple runtimes, organizational boundaries, and long
              product lifecycles. That is where hidden drift and runtime ambiguity become expensive, and where clearer contracts and
              boundaries matter most.
            </p>
          </section>

          <section>
            <h2>Adoption and learning path</h2>
            <h3>Do I need to adopt UMA all at once?</h3>
            <p>
              No. The model can start with a single portable service and then expand as runtime, orchestration, trust, and governance
              concerns become more important. That is also how the book and examples are structured.
            </p>
            <h3>Who is UMA for?</h3>
            <p>
              UMA is most useful for architects, senior engineers, platform engineers, and technical leads who need a more durable way to
              reason about systems that cross runtime boundaries and long product lifecycles.
            </p>
            <h3>Where should I start?</h3>
            <p>
              Start with the <a href="../what-is-uma/">What is UMA?</a> page if you want the conceptual definition. Start with the
              <a href="../learning-path/">Learning Path</a> if you want the chapter sequence. Start with the
              <a href="../examples/">Examples</a> page if you want runnable proof immediately.
            </p>
            <h3>Do I need to move away from Medium to use this site?</h3>
            <p>
              No. Medium can remain the publishing channel while this site acts as the authority hub for the book, examples, topic pages,
              and the reader journey. The two properties can support each other instead of competing.
            </p>
          </section>

          <section class="subpage-callout">
            <strong>This page will keep growing</strong>
            <p>
              This FAQ is meant to be maintained over time. As the site grows and new chapter pages are added, the recurring questions from
              readers, Medium posts, and the repository should feed back into this page so it stays useful as a quick-reference layer.
            </p>
            <div class="subpage-inline-links">
              <a href="../what-is-uma/">What is UMA?</a>
              <a href="../what-problem-does-uma-solve/">What problem does UMA solve?</a>
              <a href="../from-stack-ownership-to-behavior-ownership/">From stack ownership to behavior ownership</a>
              <a href="../uma-vs-traditional-microservices/">UMA vs traditional microservices</a>
              <a href="../what-makes-a-service-portable/">What makes a service portable?</a>
              <a href="../what-is-a-capability/">What is a capability?</a>
              <a href="../what-is-a-workflow/">What is a workflow?</a>
              <a href="../contract-driven-orchestration/">Contract-driven orchestration</a>
              <a href="../what-is-a-uma-runtime/">What is a UMA runtime?</a>
              <a href="../what-belongs-in-the-runtime-layer/">What belongs in the runtime layer?</a>
              <a href="../what-is-wasm-mcp/">What is WASM MCP?</a>
              <a href="../learning-path/">Learning Path</a>
              <a href="../examples/">Examples</a>
              <a href="../agent-vs-runtime/">Agent vs runtime</a>
              <a href="../diagrams/">Diagrams</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
