# UMA Subpage Template Spec

This document defines the reusable structure for all non-home pages on the UMA website.
It is intentionally homepage-agnostic. The homepage has its own composition rules and should not
be modified from this spec unless explicitly requested.

The goal is to keep subpages semantically consistent, SEO-friendly, and easy to maintain as the
site grows. Page titles, slugs, and exact copy can change. The template contract should not.

## Core Principle

Each subpage should be assembled from the same portable building blocks:

- shared metadata
- shared breadcrumbs
- shared left navigation
- page-specific hero and intro
- page-specific body content
- optional book or chapter reference block
- shared footer

The reusable parts should live in one shared source of truth so navigation or footer updates do not
require editing every page by hand.

## Template Contract

### 1. Metadata

Purpose:
- define search intent and canonical identity
- help search engines classify the page

Reusable components:
- canonical URL pattern
- Open Graph title and description pattern
- Twitter card pattern
- JSON-LD schema shape

Page-specific fields:
- `title`
- `description`
- canonical path
- social image selection, if needed
- schema type and page name

### 2. Breadcrumbs

Purpose:
- show the current page location in the site hierarchy
- reinforce topical relationships for readers and search engines

Reusable components:
- `Home`
- main content area label
- current section label
- optional chapter or subpage label

Page-specific fields:
- the current page label
- any parent section labels needed for that route

### 3. Left-Side Menu / Rail

Purpose:
- keep navigation close to the content
- expose sibling and related pages without pushing readers to the footer

Reusable components:
- `On this page`
- `Explore UMA`
- `In the examples path` when the page belongs to a chapter example

Page-specific fields:
- the page outline
- chapter previous / next links when relevant
- source folder link when relevant

### 4. Hero / Intro

Purpose:
- explain the page in one glance
- capture search intent and reader intent quickly

Reusable components:
- hero layout
- heading hierarchy
- intro paragraph rhythm
- optional kicker or label

Page-specific fields:
- the page title
- the page subtitle or intro copy
- any chapter or series label

### 5. Main Content Body

Purpose:
- deliver the substance of the page
- support SEO depth without flattening the page into a single generic block

Reusable components:
- section heading pattern
- body text rhythm
- callout block pattern
- inline link pattern
- list and grid patterns

Page-specific fields:
- the section order
- the content blocks
- the examples, proof points, or explanations

### 6. Book / Chapter Reference Block

Purpose:
- connect the page to the book and to the implementation path
- give readers a clear next step

Reusable components:
- book CTA language
- chapter source link pattern
- previous / next chapter pattern
- examples hub link pattern

Page-specific fields:
- chapter number
- chapter title
- source repository path
- recommended next step

### 7. Footer

Purpose:
- provide durable site-wide navigation
- support discovery, conversion, and topic clustering

Reusable components:
- footer group headings
- link clusters
- book CTA card
- external links to GitHub, blog, white paper, and reference application

Page-specific fields:
- none, ideally
- the footer should be shared across all non-home pages

## Portable Component Registry

These are the shared components that should be treated as reusable modules rather than page-local copy.

| Component | What it does | What changes often | What should stay stable |
|---|---|---|---|
| Metadata schema | Defines page identity for SEO and social sharing | title, description, canonical path | field structure and semantic purpose |
| Breadcrumb builder | Shows page position in the site tree | current page label, parent labels | breadcrumb pattern and ordering |
| Left rail navigation | Exposes page outline and sibling paths | page outline, chapter links | rail layout and section headings |
| Hero block | Introduces the page | intro copy, kicker, page title | structure and hierarchy |
| Body section blocks | Organizes the main content | section content and order | section styling and spacing rules |
| Book/chapter reference | Connects content to the book and examples | chapter-specific links | CTA structure and placement |
| Footer cluster set | Provides global navigation and resource links | link lists and labels | grouping model and update mechanism |

## Recommended Data Ownership

To avoid editing every page when a site-wide link changes, the following should be centralized:

- footer link groups
- global resource links
- chapter list and chapter ordering
- source repository path generation
- external reference links
- blog and white paper links

To keep SEO consistent, the following should be page-owned:

- title
- description
- canonical path
- hero intro
- body content
- section-level headings
- any page-specific CTA text

## SEO Structure

The site should be organized around semantic content areas rather than arbitrary page names.

Each subpage should answer one primary intent:

- what UMA is
- why UMA exists
- how UMA works
- how to prove UMA
- how to learn UMA
- how UMA evolves
- where to find supporting references
- where to run the examples

For SEO, each page should keep:

- one main category
- one primary content type
- one clear intro
- one clear body topic
- one clear next step

That gives search engines a stable signal while leaving room for the page name and slug to evolve.

## Maintenance Rules

- Update the shared footer once, not page by page.
- Update the breadcrumb/rail logic once, not page by page.
- Keep page-specific content in the page document, not in shared navigation data.
- Add new subpages by fitting them into an existing category first.
- Create a new category only when a genuinely new semantic area appears.

## Explicit Exclusion

Do not use this spec to redesign the homepage.
The homepage has its own narrative role and should be handled separately.
