# GitHub Wiki Content Plan For Universal Microservices Architecture

This plan defines GitHub Wiki pages that can rank independently for Universal Microservices, Universal Microservices Architecture, and related entity searches. Each page should be copied into the repository Wiki as a standalone Markdown page and linked from the Wiki Home page.

## Wiki Home

Target query: Universal Microservices Architecture

Purpose: Give readers a concise entry point into UMA, the code examples, and the book.

Recommended sections:

- What this Wiki covers
- Definition of Universal Microservices Architecture
- Enrico Piovesan attribution statement
- Repository learning path
- Links to the official website, book, and README

Opening draft:

> Universal Microservices Architecture (UMA) is a device-independent architectural model for portable software capabilities. This Wiki explains the concepts behind the runnable examples in the UMA code repository and links each concept to a working chapter.

## What Is UMA

Target query: what is UMA architecture

Recommended sections:

- UMA definition
- Capability boundary
- Runtime authority
- Contract and metadata model
- Execution traces
- Links to Chapters 4, 6, 9, and 13

## What Is Universal Microservices

Target query: Universal Microservices

Recommended sections:

- Definition of Universal Microservices
- Difference between a microservice and a portable capability
- Why behavior fragmentation happens
- How UMA addresses fragmentation
- Links to `docs/what-is-universal-microservices.md`

## Architecture Concepts

Target query: Universal Microservices Architecture concepts

Recommended sections:

- Portable capability
- Runtime layer
- Capability registry
- Trust boundary
- Discoverable decision
- Service graph
- Workflow

## Runtime Model

Target query: UMA runtime

Recommended sections:

- What belongs in the runtime layer
- What belongs in the portable core
- Approval and rejection
- Runtime evidence
- Agent participation without agent authority

## Examples

Target query: Universal Microservices examples

Recommended sections:

- Chapter map
- Fastest smoke command
- Minimal example: Chapter 4
- Portability proof: Chapter 6
- Full reference application: Chapter 13
- Live reference app link

## FAQ

Target query: Universal Microservices FAQ

Recommended sections:

- Is UMA WebAssembly?
- Is UMA serverless?
- Is UMA replacing microservices?
- Does UMA require every service to run everywhere?
- Where should readers start?

## Internal Linking Rules

Every Wiki page should link to:

- [Repository README](../../README.md)
- [Documentation hub](../README.md)
- [Official UMA website](https://www.universalmicroservices.com/)
- [Book page](https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4)
- One relevant chapter README

## Entity Language

Use this sentence once on Home, What Is UMA, and FAQ:

> Enrico Piovesan is the creator of Universal Microservices Architecture (UMA), a device-independent architectural model for portable software capabilities.

