# SEO Full Audit Report — universalmicroservices.com

**Date:** 2026-05-30  
**Audited by:** Claude SEO (10 specialist agents in parallel)  
**Site:** https://www.universalmicroservices.com  
**Type:** Technical book / publisher site — "Universal Microservices Architecture" by Enrico Piovesan

---

## Overall SEO Health Score: 47 / 100

| Category | Weight | Score | Weighted |
|---|---|---|---|
| Technical SEO | 22% | 58 | 12.8 |
| Content Quality | 23% | 61 | 14.0 |
| On-Page SEO | 20% | 45 | 9.0 |
| Schema / Structured Data | 10% | 15 | 1.5 |
| Performance (CWV) | 10% | 40 | 4.0 |
| AI Search Readiness | 10% | 46 | 4.6 |
| Images | 5% | 30 | 1.5 |
| **Total** | | | **47 / 100** |

---

## Executive Summary

The site has solid information architecture and high-quality content depth on individual concept pages — particularly the FAQ, which is the strongest asset on the site. However, four compounding problems suppress the score severely:

1. **The refactor broke the sitemap** — 77% of sitemap URLs (41/53) return 404. These are old flat paths that were reorganised into subdirectories. Google and AI crawlers following the sitemap accumulate 404s. This is the single most urgent fix.

2. **Homepage has zero structured data** — no Schema.org JSON-LD whatsoever. The site's primary commercial asset (the book) and its author have no machine-readable identity. This blocks rich results, Knowledge Panel eligibility, and Google AI Overviews.

3. **Critical JS-rendering gaps** — the footer and blog section are rendered by `app.js`. Crawlers see empty elements. Trust signals, social links, and ongoing writing evidence are invisible to Google.

4. **No inbound link equity** — 1 confirmed referring domain (GitHub, all nofollow). Domain absent from Common Crawl. The site is not being discovered by crawlers from external sources.

### Top 5 Critical Issues
1. Sitemap has 41 dead URLs (77%) — old paths pre-refactor
2. Homepage missing all Schema.org JSON-LD (Book, WebSite, Person)
3. Footer and blog section JS-rendered — invisible to crawlers
4. Missing `og:image` on all pages — social shares render blank
5. No `llms.txt` — invisible to AI agent crawlers

### Top 5 Quick Wins
1. Fix sitemap.xml (remove 41 dead URLs, add 8 live hub pages)
2. Add `og:image` using book cover (1200×630px)
3. Add `width`/`height` to all 4 images — eliminates CLS
4. Add explicit AI crawler rules to robots.txt (5-minute change)
5. Create `/llms.txt` — low effort, high AI discoverability impact

---

## Technical SEO — Score: 58/100

**Hosted:** GitHub Pages via Fastly CDN. Static HTML, SSR, no JS rendering requirement for main content.

### Critical
- **Homepage missing core OG tags** — `og:title`, `og:description`, `og:type`, `og:image` all absent. Only `og:url` is set. Subpages have complete OG tags. Sharing the homepage on LinkedIn/Slack/X renders with no title, no description, no image.
- **Security headers completely absent** — no HSTS, CSP, X-Frame-Options, X-Content-Type-Options, Referrer-Policy. GitHub Pages limitation; requires Cloudflare or Vercel Edge to address.

### High
- **Title tag 103 characters** — truncated in SERPs at ~60 chars. Current: `Universal Microservices Architecture | Device-Independent Modeling for Modern Software Design with WASM`. Recommended: `Universal Microservices Architecture — Portable, WASM-Native Design`
- **Meta description 194 characters** — clipped at ~155. Trim while keeping key value proposition.
- **9 primary nav hub pages absent from sitemap** — `/why-uma/`, `/core-model/`, `/how-uma-works/`, `/learn-uma/`, `/proof/`, `/evolve-uma/`, `/discoverability/`, `/comparisons/`, `/reference-application/`
- **No structured data on homepage** — zero JSON-LD. Subpages have BreadcrumbList + WebPage; homepage has nothing.
- **No `og:image` on any page** — book cover at `/assets/cover.png` is the obvious candidate; needs 1200×630px version with absolute URL.

### Medium
- **All 4 images missing `width`/`height`** — browser cannot reserve space; all images cause CLS on load
- **Render-blocking Google Fonts** — synchronous `<link rel="stylesheet">` to external origin; `preconnect` hints present but don't eliminate the blocking round-trip
- **robots.txt has redundant explicit `Allow:` directives** for paths that are already covered by `Allow: /`, including paths that return 404
- **Sitemap missing `<lastmod>`** on all 53 entries — no freshness signaling to crawlers
- **Favicon uses relative path** (`./favicon.png`) — should be root-relative (`/favicon.png`) across all pages

### Low
- **No IndexNow implementation** — instant URL submission to Bing/Yandex on deploy; single key file + CI hook
- **No `<link rel="sitemap">` in HTML head** — belt-and-suspenders for crawlers
- **Redirects are clean** — all single-hop 301s; no chains ✓
- **H1 split across `<br>` tags** — minor accessibility concern; AI parsers read as three separate tokens

---

## Content Quality — Score: 61/100

**E-E-A-T composite: 56/100**

| Factor | Score | Notes |
|---|---|---|
| Experience (20%) | 13/20 | "20+ years" claim; no named employers/projects/case studies |
| Expertise (25%) | 17/25 | Precise vocabulary; no cited peers, no external publications |
| Authoritativeness (25%) | 10/25 | Amazon link is strongest signal; no press, no endorsements, no ISBN |
| Trustworthiness (30%) | 16/30 | Canonicals present; no contact, no privacy policy, footer JS-rendered |

### Critical
- **Hub pages critically thin** — `/proof/` is 130 words, `/comparisons/` 140, `/core-model/` 180, `/why-uma/` 200, `/how-uma-works/` 160. Boilerplate-to-content ratio ~3:1 on all hub pages. Google quality systems assess these as low-value pages.
- **Footer JS-rendered** — `data-shared-footer` injected by `app.js`. Contact info, social links, legal links all invisible to crawlers. Directly suppresses Trustworthiness score.

### High
- **Blog section JS-rendered** — `#blog-cards` populated client-side. The ongoing writing signal — strongest E-E-A-T proof for a technical author — invisible to Googlebot.
- **Author bio too generic** — "20+ years" with no named employer, project, conference talk, or third-party quote. Exactly the pattern flagged in Google's Sept 2025 QRG update on thin author bios.
- **No schema.org/Book** anywhere — the site's primary commercial asset has no machine-readable identity.
- **No schema.org/Person** on About page — author identity unverifiable by search engines.

### Medium
- **Duplicate boilerplate across hub pages** — identical H2s, identical card structure, identical footer pattern
- **Publication date signals absent** — no `datePublished`/`dateModified` on any page
- **FAQ has duplicate question** — "What belongs in the runtime layer?" appears twice; spec violation
- **Homepage OG tags incomplete**

### AI Citation Readiness by Page

| Page | Score | Reason |
|---|---|---|
| `/discoverability/faq/` | 90/100 | FAQPage schema, 28 Q&As, direct answers, quotable definitions |
| Homepage | 55/100 | Good comparison grid and problem cards; no schema to surface them |
| `/discoverability/about-enrico/` | 35/100 | One FAQ entry, no Person schema |
| Hub pages (5×) | 20/100 | Card titles only, no substantive passage content |

---

## On-Page SEO — Score: 45/100

- **Non-unique meta descriptions** — same description on every page tested
- **Title too long** — 103 chars, truncated in SERPs
- **No keyword targeting on hub pages** — hub pages target no specific query; they function as nav indexes only
- **Internal links anemic from homepage** — 66 anchor elements on homepage but only 2 point to internal pages (`/reference-application/`, ×2). All major sections are linked only via nav, not contextual in-body links.
- **GitHub and Medium in primary nav** — bleeds link equity off-site on every page load
- **Comparison pages not surfaced** on homepage — no signal for evaluator-stage visitors

---

## Schema / Structured Data — Score: 15/100

| Page | BreadcrumbList | WebPage | FAQPage | Book | Person | WebSite |
|---|---|---|---|---|---|---|
| / (homepage) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| /why-uma/ | ✅ | ✅ | ❌ | — | — | — |
| /discoverability/about-enrico/ | ✅ | ✅ | ❌ | — | ❌ | — |
| /learn-uma/book/ | ✅ | ✅ | ❌ | ❌ | — | — |
| /discoverability/faq/ | ✅ | ✅ | ✅ | — | — | — |

**Validation issues:**
- FAQPage duplicate question: "What belongs in the runtime layer?" — spec violation, one entry silently dropped by Google
- All existing BreadcrumbList and WebPage blocks are valid ✓

**Missing high-value schema (in priority order):**

### 1. WebSite — homepage (Critical)
```json
{
  "@context": "https://schema.org",
  "@type": "WebSite",
  "@id": "https://www.universalmicroservices.com/#website",
  "name": "Universal Microservices Architecture",
  "url": "https://www.universalmicroservices.com/",
  "description": "Technical reference, book, and runnable examples for Universal Microservices Architecture by Enrico Piovesan.",
  "inLanguage": "en",
  "publisher": {
    "@type": "Person",
    "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
    "name": "Enrico Piovesan"
  }
}
```

### 2. Book — /learn-uma/book/ (Critical)
```json
{
  "@context": "https://schema.org",
  "@type": "Book",
  "@id": "https://www.universalmicroservices.com/learn-uma/book/#book",
  "name": "Universal Microservices Architecture",
  "url": "https://www.universalmicroservices.com/learn-uma/book/",
  "description": "A practical guide for architects and senior engineers on building portable, coherent distributed systems using Universal Microservices Architecture with WebAssembly and MCP.",
  "inLanguage": "en",
  "bookFormat": "https://schema.org/EBook",
  "author": {
    "@type": "Person",
    "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
    "name": "Enrico Piovesan"
  },
  "offers": {
    "@type": "Offer",
    "url": "https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4",
    "seller": { "@type": "Organization", "name": "Amazon" },
    "availability": "https://schema.org/InStock",
    "priceCurrency": "USD"
  },
  "about": [
    { "@type": "Thing", "name": "Microservices Architecture" },
    { "@type": "Thing", "name": "WebAssembly" },
    { "@type": "Thing", "name": "Distributed Systems" }
  ]
}
```

### 3. Person — /discoverability/about-enrico/ (Critical)
```json
{
  "@context": "https://schema.org",
  "@type": "Person",
  "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
  "name": "Enrico Piovesan",
  "url": "https://www.universalmicroservices.com/discoverability/about-enrico/",
  "jobTitle": "Platform Software Architect",
  "description": "Platform software architect with more than two decades of experience building modular, cloud-native, and event-driven systems. Author of Universal Microservices Architecture.",
  "knowsAbout": ["Microservices Architecture", "WebAssembly", "Distributed Systems", "Cloud-Native Architecture"],
  "sameAs": [
    "https://www.linkedin.com/in/enricopiovesan",
    "https://medium.com/@enricopiovesan"
  ]
}
```

---

## Performance (Core Web Vitals) — Score: 40/100

**Hosted on:** GitHub Pages / Fastly CDN | **TTFB:** ~70ms ✓ | **Cache-Control:** `max-age=600` (⚠ too short)

| Signal | Estimated Status | Primary Driver |
|---|---|---|
| LCP | Needs Improvement / Poor | No preload on `cover.png`, render-blocking Fonts CSS, PNG format |
| CLS | Poor (likely > 0.1) | All 4 images missing `width`/`height`; FOUT despite `display=swap` |
| INP | Likely Good | Minimal JS, module-deferred `app.js`, async GA4 |

**Images audit:**

| Image | Format | Size | `width`/`height` | `loading` | `fetchpriority` |
|---|---|---|---|---|---|
| `cover.png` | PNG | 92.8 KB | ❌ | ❌ | ❌ |
| `home-quote-1.jpg` | JPEG | 130.8 KB | ❌ | ❌ | — |
| `home-quote-2.jpg` | JPEG | 190.4 KB | ❌ | ❌ | — |
| `home-quote-3.jpg` | JPEG | 107.1 KB | ❌ | ❌ | — |

**Fix priority:**
1. Add `width`/`height` to all 4 images — eliminates CLS (zero performance cost)
2. Add `fetchpriority="high"` + `<link rel="preload">` for `cover.png` — expected LCP improvement 300–800ms
3. Add `loading="lazy"` to quote images — removes ~433 KB from initial load path
4. Convert to WebP/AVIF — estimated 40–60% size reduction
5. Load Google Fonts non-blocking — `media="print" onload="this.media='all'"` pattern
6. Increase `Cache-Control` to `max-age=86400` for static assets

---

## AI Search Readiness — Score: 46/100

**GEO breakdown:**

| Dimension | Score |
|---|---|
| Citability | 52/100 |
| Structural Readability | 68/100 |
| Multi-Modal Content | 20/100 |
| Authority & Brand Signals | 38/100 |
| Technical Accessibility | 46/100 |

**Platform-specific readiness:**

| Platform | Score | Blocking Issues |
|---|---|---|
| Google AI Overviews | 28/100 | No Schema.org; dead concept pages; no `dateModified` |
| ChatGPT / GPTBot | 35/100 | No llms.txt; no explicit GPTBot Allow |
| Perplexity | 42/100 | Sitemap 404s reduce crawl confidence |
| Bing Copilot | 31/100 | No Schema.org; incomplete OG; no author entity |

**Critical gaps:**
- No `/llms.txt` — invisible to AI agent crawlers
- No explicit AI crawler rules (GPTBot, ClaudeBot, PerplexityBot, anthropic-ai)
- H1 split across `<br>` tags — AI parsers read as 3 separate tokens
- Homepage passages 20–40 words — well below 134–167 word citation-optimal range
- FAQ is strongest AI-citable asset (90/100) but only reachable via nested URL `/discoverability/faq/`

**Minimum `llms.txt` to create at site root:**
```
# Universal Microservices Architecture
> Book and reference site on portable microservices, WebAssembly, MCP, and runtime-agnostic architecture. Author: Enrico Piovesan.

## Key pages
- /discoverability/faq/: Full FAQ — definitions, comparisons, adoption guidance
- /core-model/: Vocabulary for capabilities, workflows, and runtime
- /why-uma/: Architectural motivation and problem framing
- /examples/: Chapter-aligned runnable labs
- /learn-uma/book/: The book — available on Amazon
```

---

## Visual SEO

- **Missing `og:image` site-wide** — social shares on LinkedIn, X, Slack render blank
- **Desktop hero CTA below fold** — primary "Buy the book" hero button not visible at 1280×800; only small nav-level link visible above fold
- **Mobile "On This Page" rail hides H1** — entire first viewport on subpages is the nav list; H1 only visible after scroll
- **Page title truncated in SERPs** — 103 chars, Google displays ~60

---

## SXO / Search Experience — Score: 43/100

**Business finding:** Author's Medium articles outrank his own site for informational UMA queries. The site captures only branded navigational traffic; it misses the entire commercial and informational funnel.

**SERP mismatches by intent:**

| Query intent | SERP dominant type | Homepage match |
|---|---|---|
| "buy microservices book" (commercial) | Amazon product pages + listicles | Partial — CTA present but no price/reviews |
| "what is UMA" (informational) | Long-form blog posts | Partial — lede too abstract, no diagram |
| "UMA vs serverless" (comparison) | Comparison articles | None |
| "universalmicroservices.com" (navigational) | Own site | Strong ✓ |

**Conversion gaps (severity-ordered):**
- Zero social proof — no testimonials, star rating, review count, named endorsements (Critical)
- CTA mismatch — "Buy the book" front-loaded for cold awareness traffic (High)
- No format/price/page-count for buyer decision (Medium)
- GitHub and Medium in primary nav bleed equity off-site (Medium)
- No sample chapter download — no "try before you buy" bridge (Medium)

---

## Semantic Cluster Architecture — Score: 3.4/10

**Current state:** Hub-and-hub architecture — 9 macro sections, no designated pillar page. Authority distributed without concentration.

**Top 3 missing keyword opportunities:**

| Rank | Keyword | Volume | Gap |
|---|---|---|---|
| 1 | `WebAssembly microservices` | Medium-High | No dedicated page; Medium articles partially fill |
| 2 | `MCP model context protocol microservices` | Breakout 2026 | Concept mentioned, zero pages |
| 3 | `portable microservices architecture` | Low-Med | Intent split across 4+ pages |

**Cannibalization risks:**
- "portable microservices" intent split across `/proof/`, `/how-uma-works/`, `/core-model/`
- "runtime agnostic architecture" used as primary concept on 3+ pages with no single owner
- Medium publication competes with site for "device independent architecture"

**Tutorial gap (severe):** Rust+WASM and TypeScript+WASM tutorials exist only as GitHub repos — not indexed content. Bottom-funnel "WASM microservices tutorial" traffic goes entirely to Fermyon, DEV Community, and Medium.

---

## Backlinks

**Referring domains:** 1 confirmed (github.com, all `nofollow` — passes no PageRank)  
**Common Crawl:** Domain absent from Jan–Mar 2026 crawl  
**Homepage internal:external link ratio:** 1:11 (2 internal links vs 22 external)

**Critical:** 8 pages linked from GitHub README return 404 — wasted inbound equity.

**Link-building opportunities:**
1. Fix 404s linked from GitHub README (recover inbound links — Low effort, High value)
2. Amazon Author Central profile with website URL (High-authority follow link from DA 96)
3. CNCF landscape / Bytecode Alliance / wapm.io directory submissions
4. Guest posts on The New Stack, InfoQ, DZone
5. Add UMA site URL to Medium publication "About" section

---

## Data Limitations

- No Google Search Console → actual ranking positions, CTR, crawl errors not available
- No Moz API → DA/PA, spam scores not available (add free Moz key for Tier 1 data)
- Performance CWV are estimated from source signals — not field data (CrUX). Real data requires PageSpeed Insights API key (free).
- Amazon page bot-blocked — review count, ASIN confirmation, follow link status not verified
- Backlink data limited to Common Crawl (not in index) + direct verification

---

*Generated by Claude SEO — 10 specialist agents: technical, content, schema, sitemap, performance, visual, geo, backlinks, sxo, cluster*
