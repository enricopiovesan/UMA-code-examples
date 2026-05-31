# SEO Action Plan — universalmicroservices.com
**Date:** 2026-05-30 | **Overall Score: 66/100**

Items ordered: Critical → High → Medium → Low. Effort = 1 person working alone.

---

## 🔴 CRITICAL — Fix immediately

### C1. Expand 4 thin pages to ≥ 600 words
**Impact: Content quality / QRG risk**  
Pages: `/reference-application/`, `/evolve-uma/`, `/evolve-uma/runtime-provenance-and-trust/`, `/how-uma-works/incremental-uma-adoption/`  
All four end with "Covered in the book" without answering the stated topic. Under Sept 2025 QRG these read as doorway pages.  
**Action:** Expand each to ≥ 600 words of substantive on-page content. Remove or demote the book CTA to a "go deeper" reference at the end rather than the primary content.  
**Effort:** 3–5 hrs

### C2. Fix Book schema: add isbn, publisher, price
**Impact: Rich result eligibility — Book cards in Google Search**  
**Action:** In `build_from_content.mjs`, update the Book JSON-LD emitted on `/learn-uma/book/`:
```json
"isbn": "979-8341457867",
"publisher": {"@type": "Organization", "name": "Self-published"},
"offers": {
  "@type": "Offer",
  "price": "9.99",
  "priceCurrency": "USD",
  "priceValidUntil": "2027-12-31",
  "availability": "https://schema.org/InStock",
  "seller": {"@type": "Organization", "name": "Amazon"},
  "url": "https://www.amazon.com/..."
}
```
Verify actual ISBN against Amazon listing before committing.  
**Effort:** 30 min

### C3. Fix WebPage timestamps (per-file git log)
**Impact: Crawl freshness signals — all pages currently show identical dateModified**  
The build script is resolving timestamps to HEAD rather than per source file. This means Google sees every page as modified at the same instant.  
**Action:** In `build_from_content.mjs`, confirm `git log --follow --format="%aI" -- "${page.file}"` is used (not `git log -1`). If the issue persists, debug the git call — the lastmod in sitemap.xml also needs to reflect per-file dates.  
**Effort:** 1 hr

---

## 🟠 HIGH — Fix within 1 week

### H1. Add "portable microservices" to homepage body and key definitional pages
**Impact: Keyword coverage — phrase has zero occurrences site-wide**  
The brand promise is portable microservices but the phrase never appears.  
**Action:** Add to `/content/pages/why-uma/what-is-a-universal-microservice.md` intro and to `index.html` homepage hero section (1 natural mention each).  
**Effort:** 30 min

### H2. Fix inverted breadcrumb on `/learn-uma/`
**Impact: Schema accuracy, breadcrumb display in SERPs**  
Current: `Home > Learning Path > Learn UMA` (child listed as parent)  
Should be: `Home > Learn UMA`  
**Action:** Fix breadcrumb data in build script for the learn-uma hub page.  
**Effort:** 30 min

### H3. Remove duplicate Book schema from homepage
**Impact: Entity disambiguation — two Book entities compete for canonical ownership**  
**Action:** Replace the full Book block on the homepage with a reference stub:
```json
{
  "@type": "WebPage",
  "@id": "https://www.universalmicroservices.com/#webpage",
  "about": {"@type": "Book", "@id": "https://www.universalmicroservices.com/learn-uma/book/#book"}
}
```
**Effort:** 30 min

### H4. Add LinkedIn to Person sameAs + worksFor Autodesk
**Impact: Author entity authority, Knowledge Panel eligibility, GEO signals**  
**Action:** In `build_from_content.mjs`, update Person JSON-LD:
```json
"sameAs": [
  "https://medium.com/@enrico.piovesan",
  "https://github.com/enricopiovesan",
  "https://www.linkedin.com/in/enricopiovesan/"
],
"worksFor": {"@type": "Organization", "name": "Autodesk", "url": "https://www.autodesk.com/"}
```
**Effort:** 20 min

### H5. Add "Buy the book" CTA to `/why-uma/` and `/learn-uma/` body
**Impact: Conversion — highest-intent hub pages have no body-level Amazon link**  
**Action:** Add to both hub page markdown files a book CTA section after the main content.  
**Effort:** 30 min

### H6. Fix title tags that are too short or too long
**Impact: Technical SEO — click-through rate in SERPs**  
Affected: `/learn-uma/book/` (80 chars, truncated), `/why-uma/` (46 chars), `/core-model/` (49 chars).  
**Action:** Update `seo_description` and `title` frontmatter in affected markdown files. Target 55–60 chars.  
**Effort:** 30 min

### H7. Add Glossary and key pages to llms.txt
**Impact: AI search readiness — glossary is richest definitional source, not listed**  
**Action:** Update `/book-site/llms.txt`:
- Add Glossary as first entry under `## Key pages`
- Add `/why-uma/what-is-uma/` and `/core-model/what-is-a-capability/`
- Add `## Author` block: `Enrico Piovesan — principal platform architect at Autodesk with 20+ years in distributed systems. Author of Universal Microservices Architecture (Amazon, 2025).`
- Add `## License` block: `Content on this site may be cited with attribution to Enrico Piovesan and universalmicroservices.com.`
- Expand concept definitions from single-line to 130–160 words each  
**Effort:** 1 hr

---

## 🟡 MEDIUM — Fix within 1 month

### M1. Expand hub pages with 400+ words of inline content
**Impact: SXO — hub pages are the most likely URLs to rank; they must answer the query**  
Current: 300–475 words of framing prose + child page list  
Target: 700–900 words total, with actual definitions/explanations above the child page list  
Priority order: `/comparisons/`, `/why-uma/`, `/core-model/`, `/how-uma-works/`  
**Effort:** 4–6 hrs

### M2. Add "Next →" navigation footer to all hub pages
**Impact: UX / user journey — 3/5 journey steps currently broken**  
**Action:** Add a "Continue reading" link at the bottom of each hub page pointing to the logical next section. E.g., `/why-uma/` → `/core-model/`, `/core-model/` → `/how-uma-works/`.  
**Effort:** 1 hr

### M3. Add inline cross-links between related hubs
**Impact: Internal PageRank flow + crawlable anchor text**  
- `/comparisons/` → `/proof/` (natural reading progression)
- `/why-uma/` → `/how-uma-works/`
- `/how-uma-works/` → `/learn-uma/`  
**Effort:** 1 hr

### M4. Expand "short answer" blocks to 134–167 words
**Impact: GEO — optimal AI citation window**  
Target pages: `what-is-uma`, `what-is-a-capability`, `what-is-a-workflow`, `what-is-a-universal-microservice`, `proof/what-makes-a-service-portable`  
Each needs 30–50 more words — one elaborating sentence about context/implication.  
**Effort:** 1.5 hrs

### M5. Add a quantified benchmark headline to `/proof/benchmark-and-footprint/`
**Impact: GEO citability — specific stats are the most cited technical claims**  
**Action:** Add a clear headline stat in the first paragraph: e.g., "The WASM feature flag evaluator runs identically across Wasmtime, Node.js, and browser environments in under 2ms with a binary footprint under 150KB."  
Also: restructure benchmark data into an HTML table with labeled columns (runtime, startup, memory, footprint).  
**Effort:** 1 hr

### M6. Add TechArticle schema to comparison and concept pages
**Impact: Content-type signal for LLMs and Google**  
**Action:** Emit `TechArticle` JSON-LD on all comparison pages and key definitional leaf pages. Include `headline`, `author`, `datePublished`, `dateModified`, `isPartOf`.  
**Effort:** 2 hrs (in build script)

### M7. Add DefinedTermSet / DefinedTerm schema to Glossary
**Impact: AI citation — declared structure vs. inferred**  
Schema.org has explicit types for glossaries. Each term becomes a `DefinedTerm` within a `DefinedTermSet`.  
**Effort:** 2 hrs

### M8. Fix Google Fonts render-blocking
**Impact: LCP / Core Web Vitals**  
**Action:** Add `rel="preload"` for font CSS + `font-display: swap`. Consider self-hosting IBM Plex Mono to eliminate external network dependency.  
**Effort:** 1 hr

### M9. Add `twitter:title` and `twitter:description` meta tags
**Impact: Social sharing CTR**  
Currently absent on all pages.  
**Effort:** 30 min (in build template)

### M10. Add `width`/`height` attributes to all `<img>` tags
**Impact: CLS — cumulative layout shift**  
**Effort:** 1 hr

### M11. Surface learning path from homepage hero as secondary CTA
**Impact: SXO — non-buyer visitors have no clear entry point**  
**Action:** Add `<a href="/learn-uma/learning-path/">Start with the learning path →</a>` below the hero CTA pair.  
**Effort:** 20 min

### M12. Add /privacy/ page
**Impact: Trust infrastructure — basic QRG requirement**  
**Effort:** 30 min

---

## 🟢 LOW — Backlog

### L1. Implement Cloudflare for security headers
GitHub Pages cannot serve `X-Frame-Options`, `CSP`, `HSTS`, or `X-Content-Type-Options`. Cloudflare proxy resolves all four with a `_headers` file or Transform Rules.  
**Effort:** 2 hrs setup

### L2. Implement IndexNow
Submit URL change notifications to Bing/Yandex immediately on deploy. One webhook call per build.  
**Effort:** 1 hr

### L3. Enrich About page with specific outcomes
Add named systems, measurable results, or specific architectural decisions to the about-enrico page. "Real systems under delivery pressure" should become "designed the event-driven ingestion layer at Autodesk handling X events/day."  
**Effort:** 1 hr (content)

### L4. Add Person `image` property to schema
Add a headshot ImageObject to the Person JSON-LD. Prerequisite: upload a headshot to `/assets/enrico-piovesan.jpg`.  
**Effort:** 20 min

### L5. Add comparison: UMA vs service mesh (Istio/Linkerd)
Architects evaluating UMA will expect this comparison. No page exists.  
**Effort:** 2–3 hrs (content)

### L6. Add dedicated MCP integration page
MCP search demand rising in 2025–2026. Coverage is currently scattered. A `/how-uma-works/mcp-integration/` or `/core-model/mcp-runtime/` page would consolidate this.  
**Effort:** 2 hrs (content)

### L7. Rewrite Glossary definitions to FK Grade ≤ 12
Glossary has the worst readability (FK 16.8) but is the most AI-cited page type on technical reference sites. Each definition should be 2–3 standalone sentences.  
**Effort:** 3 hrs

### L8. Create YouTube explainer: "What is Universal Microservices Architecture?"
YouTube presence has ~0.737 correlation with AI citation — the strongest known GEO signal. A 5–8 min video using the verbatim phrasing from `/what-is-uma/` would create a cross-domain entity anchor.  
**Effort:** 4–8 hrs (production)

### L9. Participate in Reddit (r/microservices, r/softwarearchitecture)
Prerequisite for Wikipedia notability. Share insights, answer architecture questions, reference the site naturally.  
**Effort:** Ongoing

### L10. Add og:type = "article" on non-homepage pages
Currently all pages emit `og:type: website`. Hub and leaf pages should be `article`.  
**Effort:** 30 min

---

## Quick Wins Summary (ordered by ROI)

| Action | Effort | Impact |
|---|---|---|
| H1 — Add "portable microservices" to homepage + key page | 30 min | High |
| C2 — Add isbn/publisher/price to Book schema | 30 min | High |
| H2 — Fix inverted breadcrumb on /learn-uma/ | 30 min | High |
| H3 — Remove duplicate Book block from homepage | 30 min | Medium-High |
| H4 — Add LinkedIn + worksFor to Person schema | 20 min | Medium-High |
| H5 — Add book CTA to /why-uma/ and /learn-uma/ | 30 min | Medium |
| H7 — Update llms.txt (Glossary, Author, License) | 1 hr | High |
| M2 — Add "Next →" nav to hub pages | 1 hr | Medium |
| M9 — Add twitter:title/description meta | 30 min | Low-Medium |
| M11 — Add learning path CTA to homepage hero | 20 min | Medium |
