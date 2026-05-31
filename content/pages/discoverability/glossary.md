---
ref: glossary
title: "UMA Glossary"
subtitle: "Definitions for core Universal Microservices Architecture terms. Each entry answers the definitional question directly so architects and engineers have a shared vocabulary."
macro_area: discoverability
content_type: reference
slug: glossary
canonical_url: "https://www.universalmicroservices.com/discoverability/glossary/"
left_nav_group: discoverability
chapter_ref: null
seo_description: "Glossary of Universal Microservices Architecture terms: active descriptor, portable service, runtime layer, adapter, contract, capability, workflow, and more."
breadcrumbs:
  - "Home"
  - "Discoverability"
  - "UMA Glossary"
related_refs:
  - faq
  - active-descriptors
  - what-belongs-in-the-runtime-layer
---

## intro

<section class="subpage-hero">
          <h1>UMA Glossary</h1>
          <p>
            Definitions for core Universal Microservices Architecture terms. Each entry answers the definitional
            question directly so architects and engineers have a shared vocabulary when reading the book, working
            through the examples, or applying the model to their own systems.
          </p>
        </section>

## main

<div class="subpage-body">
          <section>
            <h2>A</h2>
            <dl>
              <dt>Active descriptor</dt>
              <dd>
                Structured metadata the runtime reads before making an execution decision — not documentation beside
                a service. A descriptor declares input schema, output schema, emitted events, placement targets,
                version constraints, and evidence expectations. If any check fails, the runtime rejects the path
                with a specific reason. Because the descriptor constrains rather than informs, drift is detectable
                before production, not after an incident.
              </dd>

              <dt>Active runtime governance</dt>
              <dd>
                Per-execution enforcement of policy, trust, and compatibility constraints by the runtime layer.
                Governance is "active" because it applies at every invocation, not just at deployment.
                For any specific run, the runtime can explain why a capability was allowed, denied, or rerouted.
                That record exists as inspectable evidence after the fact.
              </dd>

              <dt>Adapter</dt>
              <dd>
                A runtime-provided binding that connects a portable service to a concrete host without changing
                the service's business logic. Adapters handle transport, protocol translation, and credential
                injection. Because adapters live in the runtime layer — not inside the service — the same
                service works across different hosts by swapping adapters, not by forking the code.
              </dd>

              <dt>Adapter binding</dt>
              <dd>
                Attaching an adapter to a portable service at runtime, not at build time. Because binding happens
                during the execution decision, one compiled artifact can run in different environments. The active
                descriptor declares what the service needs. The runtime resolves which adapter satisfies those
                needs in the current context.
              </dd>
            </dl>
          </section>

          <section>
            <h2>B</h2>
            <dl>
              <dt>Behavioral coherence</dt>
              <dd>
                The property of a system where the same business rule produces equivalent outcomes across all
                execution surfaces. A behaviorally coherent system can have many runtimes and deployment targets.
                What it does not have is multiple diverging copies of the same rule drifting apart over time.
                UMA treats behavioral coherence as an explicit architectural goal, not an assumption.
              </dd>

              <dt>Behavioral equivalence</dt>
              <dd>
                A narrower claim than behavioral coherence: identical inputs should produce identical outputs
                when the same portable service runs in two different runtimes. Behavioral equivalence is
                testable through parity proofs that run the service across environments and compare results.
                It is the concrete, measurable form of what "portable behavior" means in practice.
              </dd>
            </dl>
          </section>

          <section>
            <h2>C</h2>
            <dl>
              <dt>Capability</dt>
              <dd>
                The unit the runtime can discover, validate, and compose into a workflow. A capability is a
                named piece of behavior with a visible contract: input schema, output schema, version, placement
                rules, and events it emits or requires. The runtime selects, rejects, or chains capabilities
                based on declared contracts — not implicit knowledge of their internals.
              </dd>

              <dt>Contract</dt>
              <dd>
                The explicit declaration of what a service expects as input, produces as output, emits as
                events, requires as permissions, and leaves as execution evidence. In UMA, a contract is
                machine-readable and runtime-evaluated — not a human-readable agreement between teams.
                Contracts are what make portability verifiable rather than assumed.
              </dd>
            </dl>
          </section>

          <section>
            <h2>E</h2>
            <dl>
              <dt>Execution context</dt>
              <dd>
                The full set of runtime conditions for a single execution: host capabilities available,
                trust policy in effect, identity and permissions granted, and placement constraints applied.
                Two runs of the same service in different contexts may use different adapters, trust levels,
                and latency profiles — but the business behavior inside the portable boundary stays the same.
              </dd>
            </dl>
          </section>

          <section>
            <h2>F</h2>
            <dl>
              <dt>Feature flag evaluator</dt>
              <dd>
                A concrete portable service used throughout the UMA book and examples. It takes a flag key
                and evaluation context as inputs and returns a boolean or variant result. It is deterministic,
                stateless, and free of runtime-specific dependencies — which makes it an ideal first portable
                service. Chapter 4 runs it as a WASM module with an active descriptor, identically in native
                Rust and in a WASM host, without changing the compiled artifact.
              </dd>
            </dl>
          </section>

          <section>
            <h2>G</h2>
            <dl>
              <dt>Governance layer</dt>
              <dd>
                The part of the runtime responsible for policy, trust, versioning, and audit. It sits between
                the external request and the portable service. It evaluates active descriptors, enforces trust
                boundaries, records execution evidence, and rejects paths that violate declared constraints.
                Separating governance from business logic lets you change policy without touching the service
                and audit decisions without relying on the service to report them.
              </dd>
            </dl>
          </section>

          <section>
            <h2>H</h2>
            <dl>
              <dt>Host environment</dt>
              <dd>
                The concrete runtime that executes a portable service: a browser, an edge worker, a server
                running wasmtime, a Kubernetes sidecar, or an AI-adjacent workflow engine. The host supplies
                capabilities through adapters and WASI interfaces. UMA's portability goal is that the host
                can change without requiring the service to change — as long as the host satisfies the
                service's declared contract.
              </dd>
            </dl>
          </section>

          <section>
            <h2>P</h2>
            <dl>
              <dt>Parity proof</dt>
              <dd>
                Evidence that a portable service produces equivalent outputs across two or more execution
                environments. A parity proof runs the same service with the same inputs against different
                hosts — for example, native Rust and a WASM runtime — and compares outputs and observable
                side effects. Chapter 6 of the UMA examples is structured as a parity proof. Portability
                that is not proved is an assumption.
              </dd>

              <dt>Portable behavior</dt>
              <dd>
                Business logic that is expressed once, carries an explicit contract, and runs in more than
                one host environment without being reimplemented or forked. The test of portable behavior
                is not that the code compiles in multiple places. It is that the same artifact produces
                equivalent outcomes across environments and that a parity proof exists to confirm it.
              </dd>

              <dt>Portable service</dt>
              <dd>
                A service whose business behavior stays stable across execution environments. It carries
                an explicit contract, has no hidden runtime dependencies, and can be verified through
                parity proofs. A portable service is not tied to a specific framework, queue, or host
                assumption. In UMA, portable services are the durable center of the system; the runtime
                layer handles everything that varies by environment.
              </dd>
            </dl>
          </section>

          <section>
            <h2>R</h2>
            <dl>
              <dt>Runtime diversity</dt>
              <dd>
                The ability to run the same portable service across structurally different host environments —
                browser, edge, server, workflow engine — without splitting business behavior into separate
                implementations. Runtime diversity is an architectural goal in UMA. The system should be able
                to add or change execution targets without triggering behavioral drift.
              </dd>

              <dt>Runtime layer</dt>
              <dd>
                The governed infrastructure around a portable service that handles validation, adapter binding,
                placement decisions, trust enforcement, and execution evidence. The runtime layer owns
                everything that varies by environment; the service owns the durable rule. That separation
                lets you move a service to a new host without rewriting the business logic and audit execution
                decisions without instrumenting the service itself.
              </dd>

              <dt>Runtime placement</dt>
              <dd>
                The runtime's decision about where a portable service should execute for a specific request:
                browser, edge node, backend server, or workflow engine. The decision is governed by the
                service's declared placement constraints, the trust policy in effect, latency requirements,
                and host capabilities. Moving placement decisions into the runtime layer — rather than
                hardcoding them in services — is one of UMA's structural contributions.
              </dd>
            </dl>
          </section>

          <section>
            <h2>S</h2>
            <dl>
              <dt>Service boundary</dt>
              <dd>
                The declared edge of a portable service's responsibility: what it accepts as input, what it
                produces as output, and what it does not own. In UMA, a service boundary is expressed through
                the active descriptor — not through implicit code-level coupling. A well-drawn boundary lets
                the runtime validate compatibility, compose services into workflows, and detect when one
                service's change would break another.
              </dd>
            </dl>
          </section>

          <section>
            <h2>U</h2>
            <dl>
              <dt>Universal Microservices Architecture (UMA)</dt>
              <dd>
                An execution model for distributed systems that keeps business behavior portable across
                multiple environments — browser, edge, cloud, workflow, and AI-assisted paths — while
                runtime governance, contracts, trust, and placement stay explicit. UMA splits
                responsibility in two: the portable service owns the durable business rule, and the
                runtime layer owns everything that varies by environment. The goal is behavioral coherence
                across runtimes without duplicating logic across surfaces.
              </dd>
            </dl>
          </section>

          <section>
            <h2>W</h2>
            <dl>
              <dt>WASM component model</dt>
              <dd>
                A typed interface layer on top of core WebAssembly modules, stabilized alongside WASI 0.2.
                Components define imports and exports using WIT (WebAssembly Interface Types), which supports
                records, variants, options, lists, and streams. Two components that agree on a WIT interface
                compose at the binary level with no serialization boundary. For UMA, this means service
                contracts are machine-readable, version-aware, and embedded in the compiled artifact itself.
              </dd>

              <dt>WASI (WebAssembly System Interface)</dt>
              <dd>
                A standardized, capability-gated interface that gives WASM modules access to host resources —
                clocks, file descriptors, network sockets, and HTTP — without embedding host-specific code in
                the module. WASI 0.2, stable since February 2024, introduced the Component Model and added
                <code>wasi:http</code>, making server-side WASM practical for real workloads. The host decides
                which WASI capabilities to grant at startup; the module can only use what it explicitly receives.
              </dd>

              <dt>Workflow</dt>
              <dd>
                The execution path the runtime approves from one or more capabilities to satisfy a goal.
                A workflow in UMA is not handwritten orchestration glue. It emerges from declared events,
                subscriptions, capability contracts, and placement policy. The runtime can explain why that
                path was chosen, which capabilities participated, and what evidence the run produced.
              </dd>
            </dl>
          </section>

          <section class="subpage-callout">
            <strong>Want to go deeper?</strong>
            <p>
              This glossary captures the shared vocabulary. The pages below explain each concept
              in its full architectural context, with runnable examples where the idea matters most.
            </p>
            <div class="subpage-inline-links">
              <a href="../faq/">UMA FAQ</a>
              <a href="../active-descriptors/">Active Descriptors</a>
              <a href="../what-belongs-in-the-runtime-layer/">What belongs in the runtime layer?</a>
              <a href="../what-is-a-capability/">What is a capability?</a>
              <a href="../what-is-a-workflow/">What is a workflow?</a>
              <a href="../what-makes-a-service-portable/">What makes a service portable?</a>
              <a href="../how-to-prove-portability/">How to prove portability</a>
              <a href="../webassembly-microservices-architecture/">WebAssembly microservices architecture</a>
            </div>
          </section>
        </div>

        <section id="contacts" class="section contacts-band" data-shared-footer></section>
