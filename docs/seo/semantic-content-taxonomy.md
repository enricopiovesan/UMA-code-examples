# UMA Semantic Content Taxonomy

This file defines the durable SEO and information-architecture structure for the UMA website.
Page titles, slugs, and subpage names are intentionally left open. The purpose of this file is to
keep the semantic role of each page stable over time so the site can grow without losing structure.

## Rules

- Each page should map to one primary `ref`.
- A page may have one secondary `ref` only when it genuinely supports discovery or learning.
- A `ref` may expand into sub content and sub sub content, but the semantic role should stay the same.
- Page names can change. The category and content type should remain stable.
- SEO should follow the taxonomy, not the other way around.

## Content Map

| Ref | Main category | Content type | Sub content | Sub sub content |
|---|---|---|---|---|
| `why-uma` | Positioning | overview | problem statement | comparison, differentiation, motivation |
| `core-model` | Core Model | explainer | capability, workflow, runtime, decision, trust | glossary, term relationships, model boundaries |
| `how-uma-works` | How It Works | walkthrough | contracts, events, policies, placement, runtime selection | execution flow, validation flow, approval flow |
| `proof` | Proof | evidence | portability, parity, benchmarks, footprint | measurements, smoke gates, reproducibility |
| `learn-uma` | Learning Path | onboarding | book flow, guided path, progression | chapter sequence, prerequisite order, next-step guidance |
| `examples` | Hands-On Examples | tutorial hub | chapter labs, runnable demos, reference paths | chapter sections, validation commands, subpage routes |
| `evolve-uma` | System Evolution | architecture progression | orchestration, service graphs, trust boundaries, compatibility | incremental adoption, fragmentation avoidance, governed change |
| `discoverability` | Discovery and References | resource hub | FAQ, diagrams, blog, book, about | canonical links, supporting references, external proof |

## Suggested Page Roles

### Positioning
- The user should understand what UMA is, why it exists, and what problem it solves.
- Content should answer the “why this model?” question before introducing implementation detail.

### Core Model
- The user should learn the vocabulary of UMA.
- Pages here should define the main semantic building blocks and their relationships.

### How It Works
- The user should see how the model becomes executable.
- Pages here should explain contracts, runtime decisions, and the mechanics of placement.

### Proof
- The user should be able to validate UMA with observable behavior.
- Pages here should prioritize benchmarks, parity, smoke gates, and repeatable output.

### Learning Path
- The user should be guided from concept to runnable examples in a predictable order.
- Pages here should reduce friction for first-time readers.

### Hands-On Examples
- The user should find the chapter-aligned labs and tutorial entry points.
- Pages here should make the repository feel navigable by chapter, not just by topic.

### System Evolution
- The user should see how UMA handles growth, compatibility, and change.
- Pages here should cover orchestration, versioning, trust, and long-term architecture.

### Discovery and References
- The user should find supporting pages and canonical references quickly.
- Pages here should act as the support layer for the rest of the site.

## SEO Notes

- Use the main category as the primary search-intent cluster.
- Use content type to describe the intent of the page, not the title.
- Use sub content and sub sub content to extend topical depth without fragmenting the category.
- Keep future pages inside the existing refs unless a genuinely new semantic area appears.
- Prefer one strong page per ref, then add subpages only when the content needs a deeper sequence.
