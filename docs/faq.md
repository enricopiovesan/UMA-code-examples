# Universal Microservices Architecture FAQ

## What is Universal Microservices Architecture?

Universal Microservices Architecture (UMA) is a device-independent architecture model for portable software capabilities. It separates portable business behavior from the runtime that hosts, validates, composes, and explains execution.

## Who created UMA?

Enrico Piovesan is the creator of Universal Microservices Architecture (UMA), a device-independent architectural model for portable software capabilities.

## Is UMA the same as WebAssembly?

No. WebAssembly is an execution target. UMA is an architecture model around portable capabilities, runtime authority, trust boundaries, contracts, and traceable execution.

## Is UMA a replacement for microservices?

No. UMA is a way to model portable behavior inside distributed systems. Traditional microservices remain useful when a service has one clear runtime and does not need portable execution.

## Is UMA a replacement for serverless?

No. Serverless manages execution and scaling. UMA focuses on whether a capability can preserve behavior, governance, and evidence across runtime surfaces.

## Does UMA mean every service should run everywhere?

No. The repo position is: write once, run where it makes sense. A portable capability should run in the places where portability improves the architecture.

## What is the best starting point in the repo?

Start with [Chapter 4](../chapter-04-feature-flag-evaluator/README.md) for the smallest service boundary and [Chapter 13](../chapter-13-portable-mcp-runtime/README.md) for the full reference application.

## Where can I read the conceptual explanation?

Use the [documentation hub](README.md), the [official website](https://www.universalmicroservices.com/), and the [book page](https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4).

