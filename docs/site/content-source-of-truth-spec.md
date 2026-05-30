# UMA Content Source of Truth Spec

This spec defines how UMA website content should move from HTML-first authoring to Markdown-first
authoring for the generated subpages.

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

## Recommended Page Layout

Each generated subpage should follow this semantic order so SEO, navigation, and reuse stay consistent:

1. Metadata
- title, description, canonical URL, Open Graph tags, structured data

2. Global header
- brand
- primary navigation
- mobile navigation
- purchase CTA

3. Breadcrumbs
- Home
- macro area or hub
- current page

4. Page intro
- hero title
- short lead paragraph
- optional tutorial kicker or category label

5. Main content
- core explanation, grouped into semantic sections
- headings that describe the topic clearly
- lists, cards, tables, or callouts when they improve scanability

6. Page-specific guidance
- tutorial route block
- source folder link
- previous / next chapter links when relevant
- validation command or smoke path when relevant

7. Page rail
- on-this-page outline
- explore links
- chapter navigation when relevant

8. Shared footer
- grouped footer links
- book CTA
- reference links

Recommended reusable blocks:
- `intro`
- `main`
- `book_ref`
- `related`
- `faq`

Recommended authoring rule:
- keep reusable navigation and footer content out of page Markdown
- keep page-specific copy in the page file
- let the generator assemble the shared shell, breadcrumbs, and page rail from metadata

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
