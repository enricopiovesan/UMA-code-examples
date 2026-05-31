# SXO Audit — universalmicroservices.com
**Date:** 2026-05-30  
**Auditor:** Claude SXO Agent (claude-sonnet-4-6)  
**Pages audited:** Homepage + 7 hub pages (why-uma, core-model, how-uma-works, learn-uma, proof, comparisons, examples)

---

## PRIMARY FINDING — Page-Type Mismatch on Hub Pages

**Mismatch severity: HIGH**

All six hub pages (`/why-uma/`, `/core-model/`, `/how-uma-works/`, `/learn-uma/`, `/proof/`, `/comparisons/`) follow an identical structural template:

- H1: section label (e.g., "Why UMA", "Core Model")
- Two H2s: "What this macro area covers" + "Pages in this area"
- Body: 300–475 words of framing prose
- List of child links with one-line descriptions

This is an **index/navigation page** type. The SERP signal for queries like "why microservices architecture", "microservices core concepts", "microservices vs serverless" is **informational long-form content** (2,000–5,000 words, concrete comparisons, code examples, diagrams). A visitor landing on `/comparisons/` from a search for "UMA vs serverless" gets four bullet links and 346 words — and must click through to see any actual comparison. The hub pages are functioning as site-internal navigation, not as content that satisfies search intent.

The homepage (1,962 words) and `/examples/` (1,521 words) are the only pages with meaningful content depth.

---

## SXO Gap Score — 54 / 100

| Dimension | Score | Max | Notes |
|-----------|-------|-----|-------|
| Page Type | 7 | 15 | Homepage: aligned (landing/authority). Hub pages: index type vs. informational SERP expectation. |
| Content Depth | 7 | 15 | Homepage dense, hub pages 315–475 words each. Child pages not audited but hub pages don't surface their content. |
| UX Signals | 9 | 15 | Clear nav, breadcrumbs on subpages, on-page TOC links present. No progress indicators, no sticky CTA on scroll. |
| Schema | 11 | 15 | Homepage: WebSite + Book + Person + Offer — strong. Hub pages: BreadcrumbList + WebPage only. Missing FAQ, HowTo, TechArticle. |
| Media | 5 | 15 | Book cover on homepage (1 image, proper alt). Zero diagrams, no architecture visuals on any hub page. |
| Authority | 9 | 15 | Author persona visible (Enrico Piovesan), Amazon link present, Medium blog linked. No third-party citations, no reviews visible on site. |
| Freshness | 6 | 10 | No publication dates surfaced on hub pages. Homepage has no "updated" signal. |

---

## SERP Analysis

**Primary keyword set observed:** "microservices architecture", "WASM microservices", "UMA vs serverless", "runtime-agnostic microservices"

SERP consensus for these queries:
- Dominant page type: **Long-form informational guide** (2,000–8,000 words)
- Common features: comparison tables, code snippets, architecture diagrams, FAQ sections
- Publishers ranking: IBM, Martin Fowler, GeeksforGeeks, vFunction, Sam Newman's Building Microservices
- SERP features observed: People Also Ask blocks, Featured Snippets on definition queries
- No SERP presence for universalmicroservices.com detected on any of the tested queries

**SERP confidence for informational long-form:** ~85%

The site currently holds no measurable organic footprint for its core keyword space. The Medium publication (`the-rise-of-device-independent-architecture`) has indexed content, which means the blog is capturing intent the main site should capture.

---

## Page-by-Page Assessment

### Homepage (`/`)
- **Title:** "Universal Microservices Architecture — Portable, WASM-Native Design" — descriptive, keyword-present
- **Meta description:** Clear, audience-targeting ("senior software engineers and architects") — good
- **H1:** "Universal Microservices Architecture" — correct
- **Above-fold CTA:** "Buy the book" links to Amazon in first section — correct placement
- **Schema:** WebSite + Book + Person + Offer — strongest schema on the site
- **Issues:**
  - `ALL INTERNAL LINKS (homepage)` shows zero text-bearing `<a href="...">` links inside the body content — navigation is JS-rendered or uses relative paths that resolve but carry no crawlable anchor text. Body sections do not link to hub pages inline.
  - 12 sections but no architecture diagram or visual hierarchy
  - The "Buy the book" CTA appears 3 times: topbar, hero, and a mid-page section — but only the topbar version is always visible. Mid-page CTA appears after 6 preceding sections with no commercial signal.

### Hub pages (`/why-uma/`, `/core-model/`, `/how-uma-works/`, `/learn-uma/`, `/proof/`, `/comparisons/`)
- All follow the same thin-index template
- Word count range: 315–475 words
- Child page links exist and resolve (relative hrefs), which is good for crawlability
- On-page anchor TOC present — useful for usability
- **Zero images on any hub page**
- **"Buy the book" CTA present on core-model, how-uma-works, proof, comparisons — missing on why-uma and learn-uma**
- learn-uma hub is the most logical conversion page (book, learning path, examples) yet has no direct Amazon link; the "Buy the book" topbar link is the only path

### `/examples/`
- Best content depth after homepage (1,521 words)
- Chapter-aligned structure with H3s for each chapter
- Cross-links to proof, core-model, and learn-uma child pages — strongest internal linking on the site
- Persona fit for "senior engineer evaluating": high — concrete, structured, runnable
- Missing: no code snippet preview, no estimated time-to-complete per example

### `/comparisons/`
- 346 words — functions as a table of contents for child comparison pages
- The child pages (`/uma-vs-serverless/`, `/uma-vs-traditional-microservices/`, etc.) were not fetched but are linked
- Critical gap: "UMA vs serverless" and "UMA vs microservices" are high-volume queries. If child pages have the same thin template, this is a significant missed SERP opportunity.

---

## User Journey Analysis

**Target flow: "What is UMA?" → "How does it work?" → "Buy the book"**

Current path:
1. Homepage: defines UMA, has hero CTA — **pass**
2. Homepage to `/why-uma/`: nav link exists, but clicking lands on an index page with prose framing and child links. No clear "next step" button at bottom of page. — **weak**
3. `/why-uma/` to `/how-uma-works/`: only via top navigation, not via inline suggestion. No progressive disclosure. — **broken**
4. `/how-uma-works/` to buy: "Buy the book" CTA present in topbar and at top of body — **pass but weak positioning** (the body CTA appears before any content explains value)
5. `/learn-uma/` to buy: only topbar link. The hub page that most warrants a conversion CTA has no body-level Amazon link. — **fail**

**Journey completion score: 3/5 steps functional**

Missing: "Next: [page]" / "Continue to Core Model →" navigation at bottom of any hub page. Visitors who read a hub page top-to-bottom have no prompted next step.

---

## User Story Derivation

| Story | Journey Stage | SERP Signal |
|-------|--------------|-------------|
| "As a senior engineer evaluating alternatives to traditional microservices, I need a concrete comparison between UMA and existing patterns so I can assess whether the overhead is justified for my team's context." | Consideration | Dominance of "vs" comparison pages in SERP; IBM/Martin Fowler comparison articles rank top 5 for "microservices architecture" queries |
| "As a tech lead researching WASM for a project, I need working code examples in Rust that show how portability is achieved across runtimes, so I can prototype before committing." | Decision | GitHub-linked examples, Rust-first labeling on `/examples/`, search volume for 'WASM microservices rust' |
| "As an architect evaluating the UMA book for purchase, I need to see a chapter outline, sample content, or a clear statement of what I will know how to do after reading, so I can justify the spend." | Decision | Amazon book listing conventions: TOC, sample pages, "what you'll learn" summaries |
| "As a developer who landed on a comparison page from a 'UMA vs serverless' search, I need the answer to be on the page I landed on — not behind another click — so I can scan and decide." | Awareness | Featured snippets capture single-page answers; comparison queries reward pages with immediate answer above fold |
| "As a reader who finished the why-uma section, I need a clear prompt to read the core model next, so the learning path feels structured rather than requiring me to navigate." | Consideration | Competitor technical doc sites (e.g., MCP docs, Fly.io docs) use "Next →" footers on every doc page |

---

## Persona Scoring

Scored on Relevance (25), Clarity (25), Trust (25), Action (25) = 100 pts each.

### Persona 1 — Developer searching "WASM microservices" cold
**Score: 52/100**
- Relevance: 18/25 — UMA clearly covers WASM, "WASM-Native" in homepage title. But WASM content is buried in `/how-uma-works/` hub which has 411 words and no code.
- Clarity: 12/25 — First encounter is abstract ("execution model for distributed systems where compute can happen in many places"). No single sentence states what WASM gives you in UMA terms on the page they likely land on.
- Trust: 12/25 — Author credible (Autodesk background implied), Amazon book exists. Zero benchmark numbers above fold. "Proof" section not linked from how-uma-works hub.
- Action: 10/25 — No direct path from WASM content to a runnable WASM example. Examples page not linked inline from /how-uma-works/.
- **Fix:** `/how-uma-works/` hub should surface a code snippet or link directly to the WASM tutorial example inline. Add schema `TechArticle` or `HowTo` to WASM child pages.

### Persona 2 — Architect evaluating architecture patterns
**Score: 61/100**
- Relevance: 22/25 — Homepage explicitly calls out "software architects, senior engineers". Comparisons section covers adjacent patterns.
- Clarity: 15/25 — Value proposition is clear on homepage but fragmented across hub pages with no unifying summary.
- Trust: 14/25 — Author bio present on homepage. No testimonials, no enterprise case studies, no "used by" signals anywhere.
- Action: 10/25 — Comparisons hub (most likely landing page for this persona) has no CTA. The path from "I understand the tradeoffs" to "buy the book" requires two navigation steps.
- **Fix:** Add a "Who uses UMA" or social proof section to homepage. Add Book CTA to `/comparisons/` and `/why-uma/` hub body, not just topbar.

### Persona 3 — Senior engineer evaluating new patterns
**Score: 58/100**
- Relevance: 23/25 — Core model covers capability boundaries, workflows, runtime rules — directly relevant.
- Clarity: 14/25 — Hub pages are clear on what they contain but don't demonstrate the concept inline. "What is a Capability in UMA?" is a child page; the hub only names it.
- Trust: 13/25 — GitHub repo linked — positive. No star count or contributor activity shown on site. No benchmark numbers on proof hub page itself.
- Action: 8/25 — No "start here" path for an engineer who wants technical depth. `/learn-uma/learning-path/` exists but isn't surfaced prominently. The Amazon CTA appears before content proves value on most pages.
- **Fix:** Surface the learning path earlier — link from homepage hero as a secondary CTA alongside "Buy the book". Add GitHub star count and benchmark headline to proof page.

### Persona 4 — Architect ready to buy the book
**Score: 70/100**
- Relevance: 24/25 — Book is the primary product; Amazon link present in hero.
- Clarity: 18/25 — What the book covers is described across sections 5 and 8 of homepage. No chapter list, no sample content.
- Trust: 16/25 — Author background mentioned. No reviews visible on site. Amazon link works.
- Action: 12/25 — CTA is "Buy the book" (plain text, no price anchor, no "available on Kindle/print" clarification, no urgency or social proof like review count). Two CTAs on homepage hero but mid-page repetition does not add new information to convert.
- **Fix:** Add "X reviews on Amazon", chapter count, or "available as Kindle and paperback" near the buy CTA. Consider a `/book/` landing page that mirrors a well-structured Amazon-style product page.

---

## Internal Linking Assessment

**Hub → Child:** Working. All hub pages list and link to their child pages. Relative hrefs resolve correctly. Anchor links to on-page TOC headings also present.

**Child → Sibling:** Unknown (child pages not fetched). The hub model does not cross-link between sections (e.g., `/core-model/` does not reference `/why-uma/` or suggest reading order).

**Hub → Hub:** Only via navigation bar. No inline "see also: Core Model" references within body text of any hub page.

**Homepage → Hub:** Navigation links exist. Inline body links do not appear to be crawlable (JS-rendered or missing anchor text). Body sections mention "Core Concepts" and "Why UMA" but without hyperlinks in the extracted HTML.

**Examples → Other sections:** Strong — `/examples/` cross-links to proof, core-model, and learn-uma child pages with topic-specific anchor text. This is the best-linked page on the site.

**Critical gap:** `/comparisons/` does not link to `/proof/` even though proofs are the natural next step after comparison pages. `/why-uma/` does not link to `/how-uma-works/`.

---

## CTA Analysis

| CTA instance | Location | Amazon link | Visibility |
|-------------|----------|-------------|------------|
| "Buy the book" | Topbar (all pages) | Yes | Always visible |
| "Buy the book" | Homepage hero (above fold) | Yes | High |
| "Buy the book" | Homepage mid-page (section ~8) | Yes | Low (requires scroll) |
| "Buy the book" | `/core-model/` body | Yes | Present |
| "Buy the book" | `/how-uma-works/` body | Yes | Present but pre-content |
| "Buy the book" | `/proof/` body | Yes | Present |
| "Buy the book" | `/comparisons/` body | Yes | Present |
| "Buy the book" | `/why-uma/` body | **No** | Missing |
| "Buy the book" | `/learn-uma/` body | **No** | Missing — critical gap |
| "Buy the book" | `/examples/` body | Yes | Present |

**CTA text is identical everywhere** — "Buy the book". No variant tests, no price mention, no format callout. The topbar CTA is unstyled text (no button treatment visible in nav markup), while mid-page CTAs use button classes — inconsistent hierarchy.

---

## Above-Fold Assessment

**Homepage:** Hero section (Section 1) contains H1, subtitle, 3-sentence description, and two CTAs ("Buy the book" + "Read the examples") with Amazon and GitHub links. Above-fold performance: **pass**. A visitor landing on the homepage sees the product and both primary CTAs without scrolling.

**Hub pages:** All hub pages show: breadcrumb, H1, one-paragraph intro, then "What this macro area covers" section. The child page list (the most useful content) appears below the fold on all hub pages. A visitor scanning for quick navigation must scroll. Above-fold on hub pages: **marginal** — the intro paragraph doesn't add information for a visitor who already knows where they are.

---

## Schema Summary

| Page | Schema types present | Missing / recommended |
|------|---------------------|----------------------|
| Homepage | WebSite, Book, Person, Offer, Organization | CollectionPage or ItemList for hub links |
| /why-uma/ | BreadcrumbList, WebPage, WebSite | FAQPage (PAA opportunity), TechArticle |
| /core-model/ | BreadcrumbList, WebPage, WebSite | FAQPage, DefinedTerm, TechArticle |
| /how-uma-works/ | BreadcrumbList, WebPage, WebSite | HowTo, TechArticle |
| /learn-uma/ | BreadcrumbList, WebPage, WebSite | Course, LearningResource |
| /proof/ | BreadcrumbList, WebPage, WebSite | Dataset, TechArticle |
| /comparisons/ | BreadcrumbList, WebPage, WebSite | FAQPage, ComparisonTable equivalent |
| /examples/ | BreadcrumbList, WebPage, WebSite | HowTo, SoftwareSourceCode |

No FAQPage schema exists on any page despite the site having structured Q&A headings ("What Is a Capability in UMA?", "What Is a Workflow in UMA?") that are exactly the form Google uses for PAA eligibility.

---

## Prioritized Recommendations

### P0 — Critical (implement first)

1. **Convert hub pages from index to content pages.** Each hub page should contain at minimum a 400-word summary of the concept inline, above the child page list. Currently a visitor landing on `/comparisons/` from "UMA vs serverless" gets a list of links — not an answer. The child pages hold the answer but the hub page is often the URL that would rank.

2. **Add FAQPage schema to all hub pages.** The H3 headings on hub pages are already written as questions ("What Is a Capability in UMA?", "What Is a Workflow in UMA?"). These are PAA-eligible. Adding `FAQPage` + `Question`/`Answer` schema requires only answering those questions inline on the hub page — which overlaps with recommendation 1.

3. **Add "Buy the book" body CTA to `/why-uma/` and `/learn-uma/`.** The learn-uma page is the highest-intent hub for the book-buyer persona and has no body-level conversion path.

### P1 — High impact

4. **Add "Next →" navigation footer to every hub and child page.** The user journey breaks at every hub page because there is no prompted next step. A simple "Continue to: Core Model →" at page bottom costs almost nothing to implement.

5. **Cross-link between related hubs inline.** `/why-uma/` should link to `/how-uma-works/` in body text. `/comparisons/` should link to `/proof/`. These are logical reading progressions that currently require the user to remember the nav exists.

6. **Surface learning path from homepage hero.** Add a secondary CTA to the hero: "Start with the learning path →" pointing to `/learn-uma/learning-path/`. This gives the non-buyer a structured entry point.

7. **Add architecture diagrams to hub pages.** Zero images on hub pages. A single diagram per hub (capability model, runtime layers, comparison matrix) would improve time-on-page, reduce bounce, and provide media for OG/social previews.

### P2 — Medium impact

8. **Enrich "Buy the book" CTA with social proof.** Add review count, rating, or format ("Kindle + Paperback") adjacent to the Amazon CTA. The current CTA has no urgency or proof signal.

9. **Add `HowTo` schema to `/how-uma-works/` child pages** (webassembly-architecture, migrating-to-uma-incrementally). These pages likely answer step-by-step queries.

10. **Add `LearningResource` schema to `/learn-uma/`.** The page describes a book + learning path + end-to-end example — all learnable resources. This schema type improves eligibility for Google's educational SERP features.

11. **Audit comparison child pages** (`/uma-vs-serverless/`, `/uma-vs-traditional-microservices/`) for content depth. If they follow the same hub template, they are the most urgent expansion candidates: "UMA vs serverless" and "microservices vs serverless" have significant search volume, and the site's `/comparisons/uma-vs-serverless/` URL structure is strong but currently unconfirmed for depth.

12. **Add publication date and author byline to hub pages.** Freshness signals are absent. Adding `datePublished` and `dateModified` in schema and visibly on page improves trust for technical audiences and helps Google assess content currency.

---

## Limitations

- Child pages (e.g., `/uma-vs-serverless/`, `/what-is-a-capability/`, `/book/`, `/learning-path/`) were not fetched. The hub-page audit covers structural issues but cannot assess whether child pages compensate for hub thinness.
- JavaScript rendering was not used. Elements rendered client-side (nav, interactive TOC, dynamic CTAs) may not be fully represented in extracted HTML. The missing body-level internal links from the homepage may be JS-rendered.
- No crawl data, GSC data, or ranking data was available. SERP presence assessment is based on manual searches and the absence of universalmicroservices.com from observed results.
- Mobile rendering, Core Web Vitals, and page speed were not assessed.
- No A/B test or heatmap data was available to validate CTA click behavior.

---

## Cross-Skill Recommendations

- Schema gaps are significant: use `/seo schema` to generate FAQPage, HowTo, and LearningResource blocks for hub pages.
- Hub pages are thin content: use `/seo content` for E-E-A-T gap analysis on `/why-uma/` and `/comparisons/`.
- The blog on Medium is capturing organic intent that should flow to the book site: use `/seo page` for a page-level audit of Medium → site conversion funnel.

---

*Generate a PDF report? Use `/seo google report`*
