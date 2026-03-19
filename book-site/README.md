# UMA Book Site

This folder contains the single-page GitHub Pages site for the *Universal Microservices Architecture* book.

## Local Preview

Because the site is plain static HTML, CSS, and JavaScript, you can preview it with any static file server.

Example:

```bash
cd book-site
python3 -m http.server 4173
```

Then open `http://localhost:4173`.

## Sync Blog Posts

The `UMA BLOG` section reads from [data/blog-posts.json](/Users/piovese/Documents/UMA-code-examples/book-site/data/blog-posts.json).

To regenerate that file from the Medium publication:

```bash
node book-site/scripts/sync_medium_blog.mjs
```

The script tries the Medium publication feed first and falls back to scraping the publication archive if needed. It writes normalized entries with:

- `title`
- `subtitle`
- `image`
- `url`
- `alt`

## Deployment

GitHub Pages deployment is handled by [`.github/workflows/book-site-pages.yml`](../.github/workflows/book-site-pages.yml).
The workflow publishes the contents of `book-site/` directly.

## Page Outline

1. Hero
   - headline: explain UMA in one sentence
   - supporting copy: explain why a developer should care
   - actions: explore the examples, follow the learning path, visit the book site

2. Why UMA
   - the problem: fragmented logic, runtime drift, hidden orchestration, trust gaps
   - the promise: one architectural model across client, edge, and cloud

3. What The Book Teaches
   - chapter progression from a single portable service to governed system evolution
   - emphasize that the examples are runnable, not decorative

4. Reader Learning Path
   - chapter-by-chapter tutorial ladder
   - one line per chapter describing the payoff

5. Hands-On Proof
   - explain that the repo includes Rust-first validated labs with TypeScript parity paths
   - point to smoke scripts and chapter examples as evidence

6. Why This Matters To Teams
   - tie the book back to real engineering concerns: portability, policy, trust, runtime clarity

7. Final Call To Action
   - visit the repo
   - start the learning path
   - visit the book homepage

## Copy Blocks

### Hero

**Headline**

Universal Microservices Architecture for systems that survive client, edge, and cloud.

**Supporting copy**

UMA is a practical architectural model for building portable services whose behavior stays coherent across runtimes. This book moves from first principles to runnable labs, showing how contracts, runtime policy, trust boundaries, and system evolution fit together without turning into ad hoc glue.

**Primary CTA**

Read the examples

**Secondary CTA**

Follow the learning path

### Why UMA

Modern systems do not fail because teams lack microservices. They fail because behavior fragments as services cross runtimes, hosts, policies, and trust boundaries.

The book and this repository focus on a harder question: how do you keep one architectural model intact while code moves between client, edge, and cloud?

UMA answers that by treating service logic, contracts, runtime policy, metadata, and observability as one system instead of separate concerns.

### What The Book Teaches

The learning path starts with a single portable service and expands chapter by chapter into runtime layers, portability proofs, orchestration, service graph evolution, trust enforcement, architectural tradeoffs, and long-term change without fragmentation.

Each stage is paired with hands-on examples so the reader can test the idea instead of taking it on faith.

### Hands-On Proof

The repository is not a slide deck in code form. It contains validated reader labs, Rust-first implementations, TypeScript parity paths, smoke checks, and chapter-by-chapter examples that map directly to the book.

If a reader wants to move from "what is UMA?" to "show me the runtime behavior," the path is already there.

### Final Call To Action

Start with the examples if you want proof.
Read the learning path if you want structure.
Visit the book homepage if you want the full UMA narrative.
