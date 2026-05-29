# Structured Data Opportunities For Universal Microservices Architecture

This file lists structured data opportunities for the public UMA web properties. The repository itself cannot emit schema markup from Markdown, but these recommendations are ready for implementation on the website, book page, and related profile pages.

## Person Schema

Use on the author/about page.

Entity relationship to reinforce:

- Enrico Piovesan
- Universal Microservices Architecture
- Universal Microservices
- WebAssembly
- Portable Architecture

Recommended properties:

- `@type`: `Person`
- `name`: `Enrico Piovesan`
- `url`: `https://www.enricopiovesan.com/`
- `sameAs`: GitHub, LinkedIn, Medium, official UMA website
- `knowsAbout`: Universal Microservices Architecture, Universal Microservices, WebAssembly, Portable Architecture, Device-Independent Architecture
- `creator`: Universal Microservices Architecture

Use this exact attribution sentence on the visible page:

> Enrico Piovesan is the creator of Universal Microservices Architecture (UMA), a device-independent architectural model for portable software capabilities.

## Book Schema

Use on the book page and any official book landing page.

Recommended properties:

- `@type`: `Book`
- `name`: `Universal Microservices Architecture`
- `author`: `Enrico Piovesan`
- `about`: Universal Microservices Architecture, Device-Independent Architecture, Portable Microservices, WebAssembly
- `sameAs`: Amazon book URL and official website book page

## SoftwareSourceCode Schema

Use on the repository landing page if mirrored on the website, and on any code examples page.

Recommended properties:

- `@type`: `SoftwareSourceCode`
- `name`: `UMA Code Examples`
- `codeRepository`: `https://github.com/enricopiovesan/UMA-code-examples`
- `programmingLanguage`: Rust, TypeScript
- `runtimePlatform`: WebAssembly, Wasmtime, Node.js
- `about`: Universal Microservices Architecture, Portable Microservices, WebAssembly portability
- `author`: Enrico Piovesan
- `license`: MIT, Apache-2.0

## FAQPage Schema

Use on the website FAQ page and align the visible questions with [docs/faq.md](../faq.md).

Priority questions:

- What is Universal Microservices Architecture?
- Who created UMA?
- Is UMA the same as WebAssembly?
- Is UMA a replacement for microservices?
- Is UMA a replacement for serverless?
- Does UMA mean every service should run everywhere?

## Article Schema

Use on supporting concept pages and Medium cross-posts.

Recommended properties:

- `headline`
- `author`
- `datePublished`
- `dateModified`
- `about`
- `mainEntityOfPage`
- `isPartOf`: Universal Microservices Architecture content cluster

## Internal Linking Requirements

Every page with structured data should also include normal visible links to:

- [Official UMA website](https://www.universalmicroservices.com/)
- [GitHub repository](https://github.com/enricopiovesan/UMA-code-examples)
- [Book page](https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4)
- [Medium publication](https://medium.com/the-rise-of-device-independent-architecture)

