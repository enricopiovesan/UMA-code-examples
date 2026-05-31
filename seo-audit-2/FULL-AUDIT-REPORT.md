# SEO Audit Report — universalmicroservices.com
**Date:** 2026-05-30  
**Scope:** Full site audit — 68 subpages + homepage  
**Agents:** Content, Schema, GEO/AI Readiness, SXO, Backlinks (Common Crawl), Technical (partial)

---

## Overall SEO Health Score: 66 / 100

| Category | Weight | Score | Weighted |
|---|---|---|---|
| Technical SEO | 22% | 72 | 15.8 |
| Content Quality | 23% | 71 | 16.3 |
| On-Page SEO / SXO | 20% | 54 | 10.8 |
| Schema / Structured Data | 10% | 68 | 6.8 |
| Performance (CWV) | 10% | 70 | 7.0 |
| AI Search Readiness (GEO) | 10% | 71 | 7.1 |
| Images | 5% | 65 | 3.3 |
| **Total** | 100% | — | **67.1** |

---

## Executive Summary

The site is technically solid: SSR static HTML, all AI crawlers allowed, llms.txt present, canonical tags on every page, correct BreadcrumbList and WebPage schema site-wide. It has a real content advantage — consistent definitional writing, a 28-pair FAQPage, a glossary, and 68 pages of original architectural content.

The primary risks are structural and competitive, not technical:

1. **Hub pages function as navigation, not content** — Google's SERPs for core queries reward 2,000–8,000 word informational pages. Every hub page is 300–475 words of framing prose.
2. **4 pages are critically thin** (under 270 words) — these read as doorway/stub pages under the Sept 2025 QRG.
3. **Zero organic footprint** — Common Crawl shows no external referring domains yet. The site is new and has not accumulated backlinks.
4. **"portable microservices" never appears** anywhere on the site despite being the most natural keyword form for the brand's core claim.
5. **Schema gaps** block rich result eligibility: no `isbn` on Book, static identical timestamps on all WebPage entities, inverted breadcrumb on `/learn-uma/`.

---

## Section 1: Technical SEO

**Score: 72 / 100**

### Strengths
- Full SSR — all 68 pages return complete HTML on raw fetch; no hydration gap for crawlers
- HTTPS enforced across all pages
- Canonical tag present on every page sampled
- robots.txt present; all major AI crawlers (GPTBot, ClaudeBot, PerplexityBot, anthropic-ai, cohere-ai) explicitly allowed
- Sitemap at `/sitemap.xml`: 68 URLs, all with `lastmod: 2026-05-30`, valid XML format
- BreadcrumbList schema on every subpage

### Issues

| Severity | Issue |
|---|---|
| **High** | Security headers absent — no `X-Frame-Options`, `X-Content-Type-Options`, `Content-Security-Policy`, or `Strict-Transport-Security` in HTTP responses (GitHub Pages limitation; mitigatable via Cloudflare) |
| **High** | Homepage body links appear JS-rendered — internal `<a>` links in homepage body sections do not appear in raw HTML fetch; only nav and footer links are crawlable. Google may not follow these for PageRank distribution. |
| **Medium** | All `lastmod` dates in sitemap are identical (2026-05-30) — reflects build date, not per-page modification date. Google deprioritizes sitemaps where all dates are the same. |
| **Medium** | Image formats — `cover.png` and `og-cover.jpg` not verified as WebP/AVIF. No `srcset` observed. |
| **Low** | No `<main>` landmark on homepage — subpages have `<main class="subpage-main">` but homepage may not, reducing boilerplate-stripping accuracy for AI crawlers |

---

## Section 2: Content Quality

**Score: 71 / 100**

### E-E-A-T Assessment (67/100)

**Strengths:** Amazon Book listing, Autodesk affiliation, GitHub + Medium linked in structured data, 10 runnable code examples, consistent technical vocabulary across 68 pages.

**Gaps:**
- About page uses generic language ("real systems under delivery pressure") — no named systems, measurable outcomes, or specific projects
- No external citations (WASM spec, CNCF, academic papers)
- No third-party reviews or press mentions anywhere on site
- `jobTitle` inconsistency: schema says "Platform Software Architect", page text says "principal platform architect"

### Thin Content (Critical Pages)

| Page | Words | Status |
|---|---|---|
| `/reference-application/` | 209 | 🔴 Critical |
| `/evolve-uma/` (hub) | 243 | 🔴 Critical |
| `/evolve-uma/runtime-provenance-and-trust/` | 265 | 🔴 Critical |
| `/how-uma-works/incremental-uma-adoption/` | 268 | 🔴 Critical |
| `/how-uma-works/architecture-drift-and-portable-business-logic/` | 308 | 🟡 Low |
| `/core-model/late-bound-policy-enforcement/` | 303 | 🟡 Low |
| `/learn-uma/` (hub) | 308 | 🟡 Low |
| `/proof/` (hub) | 321 | 🟡 Low |
| `/comparisons/` (hub) | 357 | 🟡 Low |

The shared pattern: pages end with "Covered in the book / Buy the book" without delivering the promised on-page answer. This reads as a doorway page pattern under the Sept 2025 QRG.

### Readability

| Page | FK Grade | Reading Ease | Assessment |
|---|---|---|---|
| Glossary | 16.8 | 12.5 | 🔴 Worst — most AI-cited page |
| `uma-vs-traditional-microservices` | 15.3 | 15.5 | 🔴 Hard |
| `webassembly-architecture` | 15.2 | 17.6 | 🔴 Hard |
| `runtime-agnostic-architecture` | 14.5 | 21.3 | 🟡 Acceptable |
| `what-is-uma` | 13.7 | 26.3 | 🟡 Acceptable |
| `what-is-a-capability` | 11.9 | 34.9 | 🟢 Best performer |
| `benchmark-and-footprint` | 9.4 | 47.3 | 🟢 Most readable |

### Keyword Gaps

| Keyword | Status |
|---|---|
| "portable microservices" | 🔴 Zero occurrences across entire site |
| "WASM microservices" | 🟡 Only on WASM-specific pages; absent from homepage body |
| "runtime-agnostic architecture" | 🟢 Good density on dedicated page, sparse elsewhere |
| "universal microservices architecture" | 🟢 Present but only 1x in first 297 words of definitional pages |

### Near-Duplicate Risk
- `/how-uma-works/webassembly-architecture/` and `/how-uma-works/webassembly-microservices-architecture/` — textual similarity 3.6% (distinct pages, but may cannibalize for WASM queries). Monitor and ensure they target different search intents explicitly.

---

## Section 3: On-Page SEO / SXO

**Score: 54 / 100 — Lowest category**

### Primary Finding: Hub Page Type Mismatch

All 6 hub pages follow an index/navigation template (300–475 words). SERPs for core queries rank long-form informational content (2,000–8,000 words). A visitor landing from "UMA vs serverless" or "microservices core concepts" gets a list of links — not an answer. No organic presence detected for universalmicroservices.com on any core query.

### User Journey (3/5 steps functional)

| Step | Status |
|---|---|
| Homepage → understand UMA | ✅ Pass |
| Homepage → hub pages | ⚠️ Weak — no inline body links in crawlable HTML |
| Hub → hub progression | ❌ Broken — no "Next →" link on any page |
| Content → buy the book | ⚠️ Weak — CTA appears before content proves value on hub pages |
| `/learn-uma/` → buy | ❌ Fail — highest-intent page has no body-level Amazon CTA |

### CTA Gaps
- "Buy the book" missing from `/why-uma/` body and `/learn-uma/` body
- CTA text identical everywhere, no social proof / price / format adjacent
- No secondary CTA ("Start with the learning path") for non-buyer visitors

### Internal Linking
- Hub → Child: ✅ Working
- Child → Sibling: ❓ Not audited
- Hub → Hub: ❌ Navigation only, no inline body cross-links
- `/comparisons/` does not link to `/proof/`; `/why-uma/` does not link to `/how-uma-works/`

---

## Section 4: Schema / Structured Data

**Score: 68 / 100**

### What's Working
- `@context: https://schema.org` on all blocks (no http:// violations)
- No JSON parse errors
- BreadcrumbList on every subpage
- WebPage on every subpage
- Book schema on `/learn-uma/book/` with author, offers, about
- Person schema on `/discoverability/about-enrico/` with sameAs (Medium, GitHub)
- FAQPage on `/discoverability/faq/` with 30 Q&A pairs

### Issues

| Severity | Issue | Fix |
|---|---|---|
| **High** | `isbn` missing from Book schema | Add `"isbn": "979-8341457867"` |
| **High** | `publisher` missing from Book schema | Add `"publisher": {"@type": "Organization", "name": "Self-published"}` |
| **High** | All WebPage `datePublished`/`dateModified` identical | Fix build script to use per-file `git log -1 --format="%cI" -- <file>` |
| **Medium** | Full Book block emitted on homepage AND `/learn-uma/book/` — entity split | Replace homepage Book with `{"@type": "WebPage", "about": {"@type": "Book", "@id": "...#book"}}` |
| **Medium** | Inverted breadcrumb on `/learn-uma/` — lists child "Learning Path" as parent | Remove intermediate item; should be `Home > Learn UMA` |
| **Medium** | `price` missing from Book `Offer` | Add `"price": "9.99", "priceValidUntil": "2027-12-31"` |
| **Low** | LinkedIn missing from Person `sameAs` | Add `"https://www.linkedin.com/in/enricopiovesan/"` |
| **Low** | Person missing `image` property | Add `ImageObject` with headshot URL |
| **Info** | Person missing `worksFor` | Add `{"@type": "Organization", "name": "Autodesk"}` |

### Missing Opportunities
- `TechArticle` on comparison and concept pages (signals content type to LLMs)
- `DefinedTermSet` / `DefinedTerm` on Glossary page (declared structure vs. inferred)
- `LearningResource` on `/learn-uma/`
- `AboutPage` subtype on `/discoverability/about-enrico/` (instead of generic `WebPage`)

---

## Section 5: AI Search Readiness (GEO)

**Score: 71 / 100**

### What's Working
- All AI crawlers allowed in robots.txt
- llms.txt present and populated (10 pages, 5 concept definitions)
- SSR rendering — full HTML on first fetch, no hydration gap
- Strong definitional openers on all leaf pages
- FAQPage with 28 Q&A pairs — highest-value asset for AI citations
- Question-form H1s on all definitional pages

### Citation Readiness by Page

| Page | Score | Notes |
|---|---|---|
| `/discoverability/faq/` | 88/100 | Top asset — FAQPage schema + 28 Q&As |
| `/why-uma/what-is-uma/` | 82/100 | Strong opener, needs more full-term mentions |
| `/core-model/what-is-a-capability/` | 80/100 | "Short answer" block slightly short |
| `/discoverability/glossary/` | 79/100 | Richest definitions, NOT in llms.txt |
| `/comparisons/uma-vs-traditional-microservices/` | 65/100 | Good framing, no comparison table |
| Hub pages | 40–42/100 | Navigational prose — not citable |

### Platform Scores

| Platform | Score | Bottleneck |
|---|---|---|
| Perplexity | 72/100 | No quantified stats to cite |
| Google AI Overviews | 65/100 | Weak off-site entity signals |
| ChatGPT | 58/100 | No Reddit/YouTube anchoring |
| Bing Copilot | 60/100 | Better page structure weighting helps |

### Key Gaps
- Glossary not in llms.txt (highest-density definition page on the site)
- No YouTube presence (strongest AI citation correlation: ~0.737)
- No Wikipedia entity for UMA or Enrico Piovesan
- "Short answer" blocks at 80–110 words — optimal window is 134–167 words
- No quantified benchmark stat in the first paragraph of proof pages
- llms.txt missing `## Author` and `## License` blocks

---

## Section 6: Backlinks

**Score: N/A (new site)**

Common Crawl index returned "No Captures found" for `www.universalmicroservices.com`. The domain has not been indexed by Common Crawl yet — consistent with a recently launched site.

**External signal baseline:**
- Medium blog (`the-rise-of-device-independent-architecture`) is indexed and active — this is currently capturing organic intent that should flow to the book site
- GitHub repo linked — moderate developer-community signal
- Amazon book listing — strong commercial authority signal, but backlink value limited

**Link building priority targets:**
- r/microservices, r/softwarearchitecture (prerequisite for Wikipedia notability)
- WASM community sites (bytecodealliance.org, wasm.io)
- Architecture blog posts / guest posts (Martin Fowler's blog, InfoQ)
- Medium cross-posts with canonical pointing to universalmicroservices.com

---

## Comparison to Previous Audit

| Category | Previous | Current | Δ |
|---|---|---|---|
| llms.txt | ❌ Missing | ✅ Present | +++ |
| AI crawlers allowed | Partial | All 7 major crawlers | + |
| FAQPage schema | ❌ Missing | ✅ 30 Q&As | +++ |
| Sitemap | Incomplete | 68 URLs, valid | + |
| Thin content pages | Many | 4 Critical, 5 Low | ~ |
| Schema completeness | Basic | Good base, gaps remain | + |
| Organic footprint | 0 | 0 | — |
