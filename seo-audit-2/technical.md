# Technical SEO Audit — universalmicroservices.com
**Date:** 2026-05-30  
**Scope:** 9 sampled URLs + robots.txt + sitemap.xml  
**Site type:** Static, GitHub Pages  
**Technical Score: 61 / 100**

---

## Summary Table

| Category | Status | Score |
|---|---|---|
| Crawlability | PASS | 9/10 |
| Indexability | WARN | 6/10 |
| Security Headers | FAIL | 1/10 |
| URL Structure | PASS | 9/10 |
| Mobile | PASS | 9/10 |
| Core Web Vitals (risk assessment) | WARN | 6/10 |
| Structured Data | PASS | 9/10 |
| JavaScript Rendering | PASS | 10/10 |
| IndexNow Protocol | FAIL | 0/10 |

---

## Issues by Severity

### Critical

#### C1 — Security headers entirely absent (all pages)
All 9 sampled pages return no security headers. GitHub Pages does not inject these by default; a CDN proxy (Cloudflare, Fastly) or a `_headers` file via Netlify/Vercel is required.

Missing on every page:
- `X-Frame-Options` — exposes site to clickjacking
- `X-Content-Type-Options` — allows MIME-sniffing attacks
- `Content-Security-Policy` — no XSS mitigation
- `Strict-Transport-Security` — HSTS not asserted at the HTTP layer (HTTPS works but browser preloading is impossible without this header)

**Fix:** Migrate CDN in front of GitHub Pages (Cloudflare free tier sufficient) and set:
```
X-Frame-Options: SAMEORIGIN
X-Content-Type-Options: nosniff
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
Content-Security-Policy: default-src 'self'; script-src 'self' https://www.googletagmanager.com https://fonts.googleapis.com 'unsafe-inline'; style-src 'self' https://fonts.googleapis.com 'unsafe-inline'; font-src https://fonts.gstatic.com; img-src 'self' data: https:
```

---

### High

#### H1 — Title tags: 6 of 7 pages outside 50–60 char target

| Page | Title | Length | Issue |
|---|---|---|---|
| `/` | Universal Microservices Architecture — Portable, WASM-Native Design | 67 | Too long |
| `/why-uma/` | Why UMA \| Universal Microservices Architecture | 46 | Too short |
| `/core-model/` | Core Model \| Universal Microservices Architecture | 49 | Too short |
| `/how-uma-works/` | How UMA Works \| Universal Microservices Architecture | 52 | OK |
| `/core-model/what-is-a-capability/` | What Is a Capability in UMA? \| Universal Microservices Architecture | 67 | Too long |
| `/comparisons/uma-vs-traditional-microservices/` | UMA vs Traditional Microservices \| Universal Microservices Architecture | 71 | Too long |
| `/learn-uma/book/` | Universal Microservices Architecture Book \| Universal Microservices Architecture | 80 | Too long — brand suffix redundant, title already contains full brand name |

**Fix:** Trim brand suffix where title already includes the brand ("Universal Microservices Architecture Book" needs no ` | Universal Microservices Architecture` appended). For short titles on section indexes, add a differentiating keyword phrase: e.g. "Why UMA — Portable Behavior for Distributed Systems".

#### H2 — Meta description: 2 of 7 pages outside 120–160 char target

| Page | Length | Issue |
|---|---|---|
| `/learn-uma/book/` | 177 | Too long — will be truncated in SERP |
| `/how-uma-works/` | 120 | Borderline (exactly at lower bound, acceptable) |

**Fix for `/learn-uma/book/`:** Shorten to ≤160 chars. Current: "Universal Microservices Architecture: the book for architects and senior engineers building portable, runtime-agnostic distributed systems with WebAssembly. Available on Amazon." — remove "Available on Amazon." and trim to ~150 chars.

#### H3 — `learn-uma/book/` breadcrumb parent points to `/learn-uma/learning-path/` not `/learn-uma/`

The BreadcrumbList schema on `/learn-uma/book/` sets position 2 to `https://www.universalmicroservices.com/learn-uma/learning-path/` instead of the section root `/learn-uma/`. This creates an inconsistency: the visible breadcrumb nav labels the parent "Learning Path" but the logical section root is `/learn-uma/`. Googlebot may resolve a different canonical path for this subtree.

**Fix:** Decide on one canonical parent (`/learn-uma/` or `/learn-uma/learning-path/`) and apply consistently across both the HTML breadcrumb `<nav>` and the JSON-LD `BreadcrumbList`.

---

### Medium

#### M1 — No IndexNow protocol implementation

No IndexNow key file found at any well-known path (`/indexnow.txt`, `/<key>.txt`). With 69 URLs and frequent `lastmod` updates (all set to today's date), IndexNow would enable near-instant indexing on Bing and Yandex without waiting for recrawl.

**Fix:** Generate a key at [bing.com/indexnow](https://www.bing.com/indexnow), place `<key>.txt` at site root, then POST URL submissions on each deploy. Can be automated in a GitHub Actions workflow.

#### M2 — Sitemap `lastmod` dates are build-time timestamps, not content-change dates

All 69 entries in `/sitemap.xml` carry `<lastmod>2026-05-30</lastmod>` — today's build date. If the site is rebuilt frequently without content changes, crawlers learn to ignore `lastmod` as unreliable, reducing its signal value for prioritizing recrawls.

**Fix:** Track actual content modification dates per page (e.g., via git log `--follow` on each source file) and emit per-page `lastmod` from the build pipeline.

#### M3 — Google Fonts loaded via render-blocking `<link>` without `font-display: swap`

All pages load three Google Font families via a standard `<link href="https://fonts.googleapis.com/css2?...&display=swap">`. The `display=swap` parameter is present in the URL (correct), but the `<link>` is not marked `media="print" onload="this.media='all'"` for non-render-blocking loading, nor is a `<link rel="preload">` present for the CSS itself. On slow connections the fonts block paint.

**Fix:** Add `rel="preload"` for the Google Fonts CSS or self-host fonts to avoid a cross-origin render-blocking request:
```html
<link rel="preload" as="style" href="https://fonts.googleapis.com/css2?family=...&display=swap" onload="this.rel='stylesheet'">
<noscript><link rel="stylesheet" href="https://fonts.googleapis.com/css2?..."></noscript>
```

#### M4 — `og:type` inconsistency: section index pages use `"website"` instead of `"article"`

`/why-uma/`, `/core-model/`, `/how-uma-works/` all set `og:type: website`. Only leaf pages use `"article"`. Section index pages that have substantive content should use `"article"` for richer social previews. Conversely, the homepage correctly uses `"book"`.

**Fix:** Set `og:type: article` on all content pages (section indexes and leaves), reserving `"website"` solely for the homepage.

#### M5 — Subpage navigation omits "Examples", "Discoverability", "Reference App" links

The top-navigation on subpages (`/why-uma/`, `/core-model/`, etc.) renders fewer links than the homepage nav. The homepage includes: Why, Core, How it Works, Learn, Proof, Examples, Evolution, Discoverability, Comparisons, Ref App. Subpage `topbar-nav` contains only: Why, Core, How it Works, Learn, Proof, Evolution, Comparisons. Three sections (Examples, Discoverability, Ref App) are absent.

This reduces PageRank flow to those sections from deep pages and degrades UX.

**Fix:** Align subpage nav HTML to match homepage nav structure.

---

### Low

#### L1 — H1 capitalisation mismatch vs. title tag on two pages

- `/core-model/what-is-a-capability/`: Title tag is "What Is a Capability in UMA?" (title case); H1 is "What is a capability in UMA?" (sentence case). Google may display either in SERPs.
- `/comparisons/uma-vs-traditional-microservices/`: Title tag is "UMA vs Traditional Microservices"; H1 is "UMA vs traditional microservices".

Consistent casing between title and H1 is a best practice signal. Pick one convention and apply it.

#### L2 — No `twitter:title` or `twitter:description` meta tags

All pages include `twitter:card` and `twitter:image` but omit `twitter:title` and `twitter:description`. Twitter/X falls back to `og:title`/`og:description` but explicit tags ensure correct rendering if OG tags are absent or truncated.

**Fix:** Add to `<head>` template:
```html
<meta name="twitter:title" content="[page title]" />
<meta name="twitter:description" content="[page description]" />
```

#### L3 — `<link rel="preload">` for cover image only on homepage

The homepage has `<link rel="preload" as="image" href="./assets/cover.png">` for the LCP candidate (book cover image). Subpages with hero images or above-the-fold images lack equivalent preloads.

#### L4 — No `robots` meta tag (acceptable but worth noting)

No page uses `<meta name="robots">`. With all pages set to `Allow: /` in robots.txt this is fine. If any page needs `noindex` in future (e.g., `/examples/` chapter pages with thin content), the infrastructure for per-page robot directives should be added to the build template.

---

## Per-Page Detail

### robots.txt — PASS

```
User-agent: GPTBot        Allow: /
User-agent: OAI-SearchBot Allow: /
User-agent: ClaudeBot     Allow: /
User-agent: PerplexityBot Allow: /
User-agent: anthropic-ai  Allow: /
User-agent: Googlebot-Extended Allow: /
User-agent: cohere-ai     Allow: /
User-agent: *             Allow: /
Sitemap: https://www.universalmicroservices.com/sitemap.xml
```

- AI crawler management: explicit Allow directives for 7 major AI crawlers. Correct approach.
- No Disallow directives — entire site is crawlable.
- Sitemap reference is correct absolute URL.
- Missing: `Crawl-delay` directive (not required; GitHub Pages has no rate-limiting concern).

### sitemap.xml — PASS with caveats

- 69 URLs total (matches reported 68 subpages + homepage).
- Valid `xmlns` namespace.
- All entries have `<lastmod>` (but see M2 above — dates are build timestamps).
- No `<priority>` or `<changefreq>` tags (omitting these is correct; Google ignores them).
- No image sitemap (`<image:image>`) — not required but could improve image indexing for book cover.

### Homepage `/` — WARN

| Check | Result |
|---|---|
| HTTP status | 200 |
| Redirect chain | None (HTTPS redirect from HTTP verified: HTTP → HTTPS, single hop) |
| Title length | 67 chars — WARN (see H1) |
| Meta description | 137 chars — OK |
| Canonical | Present, matches URL |
| H1 count | 1 — OK |
| Viewport | `width=device-width, initial-scale=1.0` — OK |
| Security headers | All missing — FAIL |
| Images | 4 found, all have alt text — OK |
| Structured data | WebSite + Book schemas — OK |
| JS rendering | SSR (static HTML, full content present without JS) — OK |

### `/why-uma/` — WARN

| Check | Result |
|---|---|
| HTTP status | 200 |
| Title length | 46 chars — WARN |
| Meta description | 128 chars — OK |
| Canonical | Present, matches URL |
| H1 | "Why UMA" — OK (1 H1) |
| Viewport | Present — OK |
| Security headers | All missing — FAIL |
| Structured data | BreadcrumbList + WebPage + WebSite — OK |

### `/core-model/` — WARN

| Check | Result |
|---|---|
| HTTP status | 200 |
| Title length | 49 chars — WARN |
| Meta description | 124 chars — OK |
| Canonical | Present, matches URL |
| H1 | "Core Model" — OK |
| Structured data | BreadcrumbList + WebPage + WebSite — OK |

### `/how-uma-works/` — PASS

| Check | Result |
|---|---|
| HTTP status | 200 |
| Title length | 52 chars — OK |
| Meta description | 120 chars — OK |
| Canonical | Present, matches URL |
| H1 | "How UMA Works" — OK |
| Structured data | BreadcrumbList + WebPage + WebSite — OK |

### `/core-model/what-is-a-capability/` — WARN

| Check | Result |
|---|---|
| HTTP status | 200 |
| Title length | 67 chars — WARN |
| Meta description | 158 chars — OK |
| Canonical | Present, matches URL |
| H1 | "What is a capability in UMA?" (sentence case vs. title case in tag) — WARN (see L1) |
| Structured data | BreadcrumbList (3-level) + WebPage + WebSite — OK |

### `/comparisons/uma-vs-traditional-microservices/` — WARN

| Check | Result |
|---|---|
| HTTP status | 200 |
| Title length | 71 chars — WARN |
| Meta description | 140 chars — OK |
| Canonical | Present, matches URL |
| H1 | "UMA vs traditional microservices" (case mismatch) — WARN (see L1) |
| Structured data | BreadcrumbList (3-level) + WebPage + WebSite — OK |

### `/learn-uma/book/` — WARN

| Check | Result |
|---|---|
| HTTP status | 200 |
| Title length | 80 chars — FAIL |
| Meta description | 177 chars — FAIL (truncated in SERP) |
| Canonical | Present, matches URL |
| H1 | "Universal Microservices Architecture Book" — OK (1 H1) |
| Breadcrumb schema parent | Points to `/learn-uma/learning-path/` not `/learn-uma/` — WARN (see H3) |
| Structured data | BreadcrumbList + WebPage + Book + WebSite — OK (Book schema with ISBN-less offer is valid) |

---

## Core Web Vitals Risk Assessment (Static Analysis)

No lab or field data fetched; assessment is based on source inspection.

### LCP — Low risk
- Homepage preloads hero cover image (`<link rel="preload" as="image">`).
- All pages are SSR static HTML — no JS waterfall blocking LCP candidate.
- Google Fonts use `display=swap` parameter — text LCP candidates paint with fallback font immediately.
- Risk: Cross-origin Google Fonts CSS request may add ~100–200ms on cold load.

### INP — Low risk
- No client-side SPA framework. Interactivity is limited to mobile menu toggle (vanilla JS).
- No heavy event listeners on scroll or input.

### CLS — Low risk
- Viewport meta tag present on all pages.
- Images on homepage (`cover.png`, quote JPGs) — no `width`/`height` attributes visible in fetched HTML, which can cause layout shift if CSS does not constrain dimensions. **Recommend:** add explicit `width` and `height` attributes to all `<img>` tags to allow browser to reserve space before image load.
- Google Fonts with `display=swap` can cause FOUT (Flash of Unstyled Text) which may contribute minor CLS if font metrics differ significantly from fallback.

---

## JavaScript Rendering

- All pages return complete content in raw HTML without JavaScript execution.
- No SPA shell pattern detected.
- Googlebot can index all content without JS rendering.
- Google Tag Manager / gtag loaded async — does not block rendering.

---

## Structured Data Summary

| Schema type | Pages | Valid |
|---|---|---|
| `WebSite` | All pages | Yes |
| `Book` | `/`, `/learn-uma/book/` | Yes |
| `WebPage` | All subpages | Yes |
| `BreadcrumbList` | All subpages | Yes (3-level where applicable) |

No `Organization`, `Person`, or `FAQPage` schemas detected on pages where they would be natural fits (`/discoverability/about-enrico/`, `/discoverability/faq/` — not sampled but noted from sitemap).

---

## Prioritised Action List

| Priority | Action | Effort | Impact |
|---|---|---|---|
| 1 | Add security headers via CDN (Cloudflare) | Medium | Critical |
| 2 | Fix `/learn-uma/book/` title (80→≤60) and description (177→≤160) | Low | High |
| 3 | Shorten long titles on `/`, `/core-model/what-is-a-capability/`, `/comparisons/uma-vs-traditional-microservices/` | Low | High |
| 4 | Lengthen short titles on `/why-uma/` (46), `/core-model/` (49) | Low | High |
| 5 | Fix breadcrumb schema parent on `/learn-uma/book/` | Low | High |
| 6 | Implement IndexNow key file + GitHub Actions deployment hook | Low | Medium |
| 7 | Add `width`/`height` to all `<img>` tags to prevent CLS | Low | Medium |
| 8 | Fix per-page `lastmod` in sitemap to reflect actual content dates | Medium | Medium |
| 9 | Non-render-blocking Google Fonts loading | Low | Medium |
| 10 | Add `twitter:title` and `twitter:description` to all pages | Low | Low |
| 11 | Unify nav links across homepage and subpage templates | Low | Low |
| 12 | Standardise H1 capitalisation to match title tags | Low | Low |
| 13 | Add `og:type: article` to section index pages | Low | Low |
