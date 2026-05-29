# UMA Content Source of Truth Spec

This spec defines how UMA website content should move from HTML-first authoring to Markdown-first
authoring without changing the homepage.

## Source of Truth

- `content/site-map.md` is the authoritative map for page order and macro areas.
- `content/pages/**.md` is the authoritative page content.
- The generated HTML is build output, not the source of truth.

## Recommended Folder Model

```text
content/
|- site-map.md
|- pages/
|  |- why-uma/
|  |- core-model/
|  |- how-uma-works/
|  |- examples/
|  |- evolve-uma/
|  |- discoverability/
|  |- _shared/
```

Recommended placement:
- page files are grouped by macro area
- each page file owns one page
- shared fragments live under `_shared/` if they are truly reusable

## Portable Fields

Each page Markdown file should carry frontmatter for:

- `ref`
- `title`
- `subtitle`
- `macro_area`
- `content_type`
- `slug`
- `canonical_url`
- `breadcrumbs`
- `left_nav_group`
- `chapter_ref`
- `seo_description`
- `related_refs`

## Portable Body Sections

Use a predictable section model so HTML generation stays efficient:

- `intro`
- `main`
- `book_ref`
- `related`
- `faq`

The generator should map these sections into the reusable page template spec.

## Reusable Components

These should be generated from shared data or shared templates, not duplicated per page:

- footer link groups
- breadcrumb generation
- left navigation / rail
- chapter previous / next navigation
- metadata schema
- global reference links
- book CTA references

## Maintenance Rules

- Update the site map once when page order changes.
- Update the shared footer once when a global link changes.
- Do not copy navigation blocks into page content.
- Keep page-specific content in the page file.
- Keep the homepage out of this workflow unless explicitly requested.

