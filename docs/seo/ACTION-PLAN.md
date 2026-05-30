# SEO Action Plan — universalmicroservices.com

**Date:** 2026-05-30 | **Overall Score:** 47/100  
**Priority:** Critical → High → Medium → Low

---

## 🔴 CRITICAL — Fix immediately (blocks indexing / crawlers)

### C1. Fix sitemap.xml — remove 41 dead URLs, add 8 live hub pages
**Impact:** Crawl budget, Google + AI crawler trust  
**Effort:** Low (script update)

- Remove all 41 URLs that return 404 (old flat-path URLs pre-refactor)
- Add the 8 live hub pages missing from sitemap:
  - `/why-uma/`, `/core-model/`, `/how-uma-works/`, `/proof/`
  - `/evolve-uma/`, `/discoverability/`, `/comparisons/`, `/reference-application/`
- Add `<lastmod>` with real per-page dates to all entries
- Remove dead-path `Allow:` directives from robots.txt

### C2. Add Schema.org JSON-LD — homepage (WebSite + Book + Person)
**Impact:** Rich results, Google AI Overviews, Knowledge Panel eligibility  
**Effort:** Low (paste JSON-LD into `<head>`)

Three ready-to-paste blocks are in `FULL-AUDIT-REPORT.md` (Schema section).
- `WebSite` block → every page `<head>`
- `Book` block → `/learn-uma/book/` page
- `Person` block → `/discoverability/about-enrico/` page

### C3. Server-side render the footer and blog section
**Impact:** E-E-A-T Trustworthiness score; blog writing signal visible to crawlers  
**Effort:** Medium

- Footer (`data-shared-footer`): inject as static HTML at build time instead of via `app.js`
- Blog cards (`#blog-cards`): either SSR at build or add a static fallback list of 3–5 article titles+links in a `<noscript>` block

### C4. Add `og:image` to all pages
**Impact:** Social share previews (LinkedIn, Slack, X, iMessage)  
**Effort:** Low

- Create a 1200×630px version of the book cover
- Add as absolute URL: `<meta property="og:image" content="https://www.universalmicroservices.com/assets/og-cover.jpg">`
- Also add `twitter:image` and `twitter:card: summary_large_image`

### C5. Complete Open Graph tags on homepage
**Impact:** Homepage social sharing — currently renders blank  
**Effort:** Low (3 meta tags)

```html
<meta property="og:title" content="Universal Microservices Architecture" />
<meta property="og:description" content="The architecture book for portable, runtime-agnostic distributed systems with WASM and MCP. For senior engineers and architects." />
<meta property="og:type" content="book" />
```

---

## 🟠 HIGH — Fix within 1 week

### H1. Create `/llms.txt`
**Impact:** AI agent crawlers (agentic pipelines, scoped indexing)  
**Effort:** Low (one file, 20 lines)

Content template in `FULL-AUDIT-REPORT.md` (AI Search Readiness section).

### H2. Add explicit AI crawler rules to robots.txt
**Impact:** GPTBot, ClaudeBot, PerplexityBot, Bing Copilot indexing priority  
**Effort:** Low (5-minute change)

Add before the existing `User-agent: *` block:
```
User-agent: GPTBot
Allow: /

User-agent: OAI-SearchBot
Allow: /

User-agent: ClaudeBot
Allow: /

User-agent: PerplexityBot
Allow: /

User-agent: anthropic-ai
Allow: /

User-agent: Googlebot-Extended
Allow: /
```

### H3. Fix all image attributes
**Impact:** CLS (likely Poor → Good), LCP improvement  
**Effort:** Low (HTML changes)

For every `<img>` on homepage:
- Add `width` and `height` (intrinsic dimensions)
- Add `loading="lazy"` to `home-quote-1/2/3.jpg`
- Add `fetchpriority="high"` to `cover.png`
- Add `<link rel="preload" as="image" href="/assets/cover.png">` in `<head>`

### H4. Shorten title tag and meta description
**Impact:** SERP click-through rate  
**Effort:** Low

- Title: max 60 chars — `Universal Microservices Architecture — Portable, WASM-Native Design`
- Description: max 155 chars, unique per page, differentiated from competitors

### H5. Fix FAQPage duplicate question
**Impact:** Schema validation, Google rich results  
**Effort:** Low

Remove the duplicate "What belongs in the runtime layer?" entry from `/discoverability/faq/` JSON-LD `mainEntity` array. Keep the more detailed second occurrence.

### H6. Add hub page content (300–500 words per hub)
**Impact:** Content quality score, thin content risk  
**Effort:** Medium

All 5 macro hub pages (`/why-uma/`, `/core-model/`, `/how-uma-works/`, `/proof/`, `/comparisons/`) need substantive explanatory prose above the card grid. Currently 130–200 words each with ~3:1 boilerplate-to-content ratio.

### H7. Move GitHub and Medium from primary nav to footer
**Impact:** Internal link equity, nav clarity  
**Effort:** Low

Primary nav should be internal-only. GitHub and Medium links moved to footer reduce equity bleed on every page load and reduce nav density (currently 10+ items).

### H8. Fix 8 dead URLs in GitHub README
**Impact:** Recover inbound link equity from GitHub  
**Effort:** Low

Update `README.md` links from flat paths (e.g. `/what-is-uma/`) to current nested paths (e.g. `/why-uma/what-is-uma/`), or remove links to pages that don't exist yet.

---

## 🟡 MEDIUM — Fix within 1 month

### M1. Strengthen author bio (About page)
**Impact:** E-E-A-T Authoritativeness  
**Effort:** Low (content edit)

Add at minimum one verifiable specific:
- Named current employer (Autodesk, if public-facing is intentional)
- A named production project
- A conference talk or podcast appearance
- Add author photo
- Add LinkedIn + Medium profile links with `rel="me"`
- Add `Person` JSON-LD (block in FULL-AUDIT-REPORT.md)

### M2. Desktop hero CTA above the fold
**Impact:** Conversion rate, bounce rate  
**Effort:** Low-Medium (CSS/layout)

"Buy the book" hero CTA button is below fold at 1280×800. Reposition or duplicate it so it's visible in the first 800px.

### M3. Collapse "On This Page" rail on mobile
**Impact:** Mobile user experience, H1 visibility for crawlers  
**Effort:** Low-Medium (CSS)

On mobile (`< 768px`), wrap the "On This Page" rail in a `<details>` disclosure widget or move it below the H1+intro paragraph. Currently hides the H1 entirely on mobile subpages.

### M4. Unify H1 tag (remove `<br>` splits)
**Impact:** AI entity recognition, screen readers  
**Effort:** Low

Replace `Universal<br>Microservices<br>Architecture` with a single unbroken string. Use CSS for line breaking if the visual design requires it.

### M5. Add homepage definition block (100–150 words)
**Impact:** Google AI Overviews citability, Perplexity citation  
**Effort:** Low (content addition)

Add a direct, self-contained definition of UMA immediately below the H1 that answers "What is Universal Microservices Architecture?" — currently the lede is a framing sentence, not citable by AI systems. Aim for 134–167 words.

### M6. Add `datePublished`/`dateModified` to all pages
**Impact:** Freshness signals, Google AIO eligibility, Perplexity ranking  
**Effort:** Low (build-time injection)

Generate from Git commit timestamps in the build script.

### M7. Convert images to WebP/AVIF
**Impact:** Page weight, LCP  
**Effort:** Medium

- `cover.png` → WebP (est. 55–65 KB vs 92.8 KB)
- `home-quote-*.jpg` → WebP (est. 40–60% reduction)
- Serve via `<picture>` with JPEG fallback

### M8. Increase Cache-Control TTL
**Impact:** CDN efficiency, performance  
**Effort:** Low (if on Cloudflare; not configurable on raw GitHub Pages)

Change `max-age=600` (10 min) to `max-age=86400` (1 day) with fingerprinted asset filenames. Requires CDN layer (Cloudflare free tier works).

### M9. Add social proof to homepage
**Impact:** Conversion rate, E-E-A-T Trustworthiness  
**Effort:** Medium

- 2–3 named testimonials with title and company
- Amazon star rating / review count displayed
- Consider ISBN/ASIN visible on the page

### M10. Add a "free sample chapter" path
**Impact:** Conversion, email capture, trust bridge for cold traffic  
**Effort:** Medium

Create `/sample-chapter/` with gated or ungated PDF download. Bridges awareness → decision gap for architects evaluating the book.

---

## 🟢 LOW — Backlog

### L1. Implement IndexNow
Place a key file at domain root + fire API call in CI/CD on deploy. Accelerates Bing/Yandex indexation of new pages.

### L2. Set up Cloudflare or Vercel Edge
Enables security headers (HSTS, CSP, X-Frame-Options, nosniff) that GitHub Pages cannot serve natively. Also enables `Cache-Control` customisation.

### L3. Add favicon as root-relative path
All pages: `/favicon.png`, `/favicon.svg` instead of `./` or `../` relative paths.

### L4. Add privacy policy and terms pages
Required for full QRG Trustworthiness compliance. Even a minimal 1-page policy counts.

### L5. Create UMA Glossary page
Captures long-tail definitional queries ("what is an Active Descriptor", "what is a UMA Runtime") and builds topical authority. Low effort, high topical signal.

### L6. Add `TechArticle` schema to concept pages
Adds `proficiencyLevel`, `dependencies`, `about`, `datePublished` signals on `/core-model/` subpages.

### L7. Add `<link rel="sitemap">` to HTML head
Belt-and-suspenders sitemap discovery for non-Google crawlers.

### L8. Set up Amazon Author Central
Add website URL to author profile. High-authority follow link from DA 96 domain. Also enables claiming the author page on the book's Amazon listing.

---

## Content Strategy — New Pages to Create (Priority Ranked)

| Priority | Page | Target Keyword | Cluster |
|---|---|---|---|
| P1 | "WebAssembly Microservices Architecture" | `WebAssembly microservices` | A |
| P1 | "MCP + WebAssembly: AI-Native Portable Microservices" | `MCP model context protocol microservices` | D |
| P1 | "WASM vs Docker and Kubernetes: Portability Comparison" | `WASM vs container performance` | B+E |
| P2 | Rust+WASM tutorial (from GitHub example) | `WASM microservices tutorial Rust` | C |
| P2 | TypeScript+WASM tutorial (from GitHub example) | `WASM microservices tutorial TypeScript` | C |
| P2 | "Microservices without Kubernetes" | `microservices without Kubernetes` | A |
| P3 | Reference Application Walkthrough | `microservices reference architecture` | C |
| P3 | "What is WASI?" — WASM foundational explainer | `what is WASI` | A |
| P3 | UMA Glossary | `active descriptor microservices` | Standalone |
| P4 | "Microservices Architecture Book" landing page | `microservices architecture book 2026` | Standalone |

---

## Score Impact Projection

If Critical + High items above are completed:

| Category | Current | Projected |
|---|---|---|
| Technical SEO | 58 | 78 |
| Content Quality | 61 | 72 |
| On-Page SEO | 45 | 65 |
| Schema | 15 | 65 |
| Performance (CWV) | 40 | 68 |
| AI Search Readiness | 46 | 72 |
| Images | 30 | 70 |
| **Overall** | **47** | **~71** |

---

*Full findings in `FULL-AUDIT-REPORT.md` | Generated 2026-05-30*
