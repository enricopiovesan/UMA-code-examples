---
ref: chapter-07-webassembly-portability-wasm-runtimes
title: "Ch.7: WebAssembly and WASM Runtimes"
subtitle: "How WebAssembly provides a portable execution boundary, and what UMA adds on top of it."
macro_area: learn-uma
content_type: overview
slug: chapter-07-webassembly-portability-wasm-runtimes
canonical_url: "https://www.universalmicroservices.com/learn-uma/chapter-07-webassembly-portability-wasm-runtimes/"
left_nav_group: learn-uma
chapter_ref: null
seo_description: "Chapter 7: WebAssembly portability and WASM runtimes. How WASM modules and UMA descriptors make portable microservice execution inspectable and verifiable."
breadcrumbs:
  - "Home"
  - "Learn UMA"
  - "Chapter 7: Portability with WebAssembly and Native Runtimes"
related_refs:
  - book
  - learning-path
---

## intro

<section class="subpage-hero">
  <h1>Chapter 7: Portability with WebAssembly and Native Runtimes</h1>
  <p>How does WebAssembly provide a portable execution boundary, and what does UMA add on top of it?</p>
</section>

## main

<div class="subpage-body">
  <section>
    <h2>The question this chapter answers</h2>
    <p>WebAssembly has changed what "portable code" means in practice. A WASM binary runs in a browser, a server, an edge node, or a constrained device without recompilation. That is a meaningful guarantee. But it is not the same as a portable service. A portable service needs to carry its own contract. what it needs from the environment, what it produces, and what trace it leaves behind. Without that, portability is a runtime coincidence rather than an architectural property. Chapter 7 draws that line precisely.</p>
  </section>

  <section>
    <h2>The core concept</h2>
    <p>WebAssembly gives you a portable binary. UMA gives you a portable service. The difference is the contract.</p>
    <p>A WASM module runs in multiple environments. A Universal Microservice runs in multiple environments AND carries a descriptor that tells the runtime what it needs, what it produces, and what evidence it should leave behind. The active descriptor travels with the binary. The runtime reads it before execution begins, validates the current environment against it, and refuses to proceed if the requirements are not met.</p>
    <p>Chapter 7 shows how the WASM component model and WASI 0.2 provide the execution substrate, and how active descriptors sit on top of that substrate to make portability inspectable rather than assumed. The chapter builds a service that runs in both a browser WASM runtime and a server-side native runtime, using the same descriptor to govern both executions. The result is portability you can audit. not just portability you can claim.</p>
  </section>

  <section>
    <h2>Why WASM and not a shared library or container</h2>
    <p>Shared libraries solve a narrow portability problem: the same compiled code can be called from multiple programs on the same machine. But "the same machine" requires the same operating system, the same instruction set, and in practice the same runtime version. A shared library compiled against glibc 2.31 won't load in an environment with an older version. A shared library for x86-64 won't run on ARM without recompilation. Language runtime version mismatches are the most common cause of "works on my machine" failures in shared library deployments. The portability is real but limited: it extends across processes on compatible machines, not across structurally different execution environments.</p>
    <p>Containers solve a different problem: they package the application alongside its dependencies and make that package reproducible. The cost is that the host is inside the package. A container includes the OS layer, the language runtime, and the application. When you run the same container in a browser context, you're not running the container. browsers don't execute containers. When you run it at an edge node, you're typically running a stripped-down version with significant capability restrictions. Containers achieve deployment reproducibility. they don't achieve execution surface independence.</p>
    <p>WASM separates the portable logic from the host completely. The same .wasm binary runs in a browser's WASM runtime, in wasmtime on a server, in a Fastly Compute edge node, and in any other WASI-conforming runtime without recompilation. The host provides a sandboxed execution context. the binary provides the logic. There is no OS layer in the artifact, no language runtime version requirement, no assumption about the instruction set of the host. This is a structurally different portability guarantee from shared libraries or containers. it's host-independent, not just deployment-reproducible. That distinction is why WASM is the right compilation target for a service that needs to run on an execution surface the author has never seen.</p>
  </section>

  <section>
    <h2>What WASI 0.2 adds</h2>
    <p>WASI 0.1 established the principle that a WASM module could interact with system capabilities through a well-defined interface rather than directly. The interface covered files, environment variables, clocks, and random number generation: the POSIX-adjacent capabilities a typical server application needs. That was useful but insufficient for services that need to make network calls, handle HTTP requests, or interact with other services. More critically, WASI 0.1 didn't have a mechanism for encoding service contracts in the compiled artifact. The binary was portable, but its interface was implicit. you had to read the source code or documentation to know what it expected.</p>
    <p>WASI 0.2 became stable in February 2024 and addressed both limitations. The wasi:http interface added standardized HTTP request and response handling, enabling WASM services to participate in web-oriented workflows without a host-specific adapter for every runtime. More significant for UMA is the Component Model, which provides a type system and interface definition language (WIT. WASM Interface Types) that allows a WASM component to declare its imports and exports in the compiled artifact itself. When you compile a service to a WASM component under WASI 0.2, the binary contains a machine-readable declaration of its interface. Any WASI 0.2-conforming runtime can read that declaration before execution.</p>
    <p>The Component Model is what makes UMA's contract approach work at the binary level. An active descriptor declares what the service intends to guarantee. the Component Model's WIT interface declares what the binary actually provides. A conforming runtime can validate the two against each other before loading the service. if the descriptor claims an output the WIT interface doesn't produce, that's a contract violation detectable before any user traffic is affected. Chapter 7 uses WASI 0.2 throughout and builds the component model interface alongside the active descriptor, showing why the two artifacts are complementary rather than redundant. The descriptor governs execution. the Component Model makes the governance checkable at the binary level.</p>
  </section>

  <section>
    <h2>How it connects</h2>
    <p>Chapter 6 defined what the runtime layer owns. this chapter shows those responsibilities operating across two structurally different execution environments. The contract model introduced here becomes the basis for the event and orchestration work in Chapter 8.</p>
    <div class="subpage-inline-links">
      <a href="../chapter-06-uma-runtime-layer/">← Chapter 6: The UMA Runtime Layer</a>
      <a href="../chapter-08-service-contracts-events-orchestration/">Chapter 8: Contracts, Events, and Orchestration →</a>
      <a href="../../how-uma-works/webassembly-architecture/">WebAssembly architecture in UMA</a>
      <a href="../../core-model/active-descriptors/">Active descriptors</a>
      <a href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order the book on Amazon</a>
    </div>
  </section>
</div>
<section id="contacts" class="section contacts-band" data-shared-footer></section>
