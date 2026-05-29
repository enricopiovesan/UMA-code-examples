# UMA Content Source of Truth

This directory is the authoring source for the UMA website content.
The HTML website is generated from these Markdown files at build time.

## Layout

- `site-map.md` - the canonical website structure and page ordering
- `pages/` - one Markdown file per page, grouped by macro area
- `pages/_shared/` - reusable authoring fragments or partials, if needed later

## Rules

- Do not publish these files directly as website pages.
- Keep the homepage separate unless explicitly instructed otherwise.
- Keep URLs stable even if page titles change.
- Use the semantic taxonomy and subpage template spec in `docs/site/`.

## Recommended Page Layout

```text
content/
|- site-map.md
|- pages/
|  |- why-uma/
|  |  |- what-problem-does-uma-solve.md
|  |  |- what-is-uma.md
|  |  |- why-universal-microservices-exist.md
|  |
|  |- core-model/
|  |  |- what-is-a-capability.md
|  |  |- what-is-a-workflow.md
|  |  |- what-is-a-uma-runtime.md
|  |
|  |- how-uma-works/
|  |  |- what-makes-a-service-portable.md
|  |  |- how-to-prove-portability.md
|  |  |- runtime-agnostic-architecture.md
|  |
|  |- examples/
|  |  |- chapter-04-feature-flag-evaluator.md
|  |  |- chapter-05-post-fetcher-runtime.md
|  |  |- chapter-06-portability-lab.md
|  |
|  |- evolve-uma/
|  |  |- service-graph-evolution.md
|  |  |- trust-boundaries.md
|  |  |- evolution-without-fragmentation.md
|  |
|  |- discoverability/
|  |  |- faq.md
|  |  |- diagrams.md
|  |  |- book.md
```

## Frontmatter Contract

Each page Markdown file should start with frontmatter containing:

- `ref`
- `title`
- `subtitle`
- `macro_area`
- `content_type`
- `slug`
- `canonical_url`
- `breadcrumbs`
- `left_nav_group`
- `chapter_ref` when relevant
- `seo_description`
- `related_refs`

## Body Contract

Each page body should use structured sections so the generator can assemble consistent HTML:

- `intro`
- `main`
- `book_ref`
- `related`
- `faq`

Use section markers or a small schema so content stays easy to generate and validate.

