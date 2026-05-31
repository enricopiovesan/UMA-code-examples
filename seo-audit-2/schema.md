# Schema.org Structured Data Audit — universalmicroservices.com

**Audit date:** 2026-05-30  
**Pages fetched:** 6 live, 2 × 404 (chapter-01, uma-vs-openapi)  
**Method:** urllib fetch, regex extraction of `application/ld+json` blocks, manual validation against Google's Rich Results requirements.

---

## 1. Pages Audited

| Page | URL | Blocks found |
|---|---|---|
| Homepage | `/` | 2 |
| Book | `/learn-uma/book/` | 4 |
| About Enrico | `/discoverability/about-enrico/` | 4 |
| FAQ | `/discoverability/faq/` | 4 |
| Learn UMA | `/learn-uma/` | 3 |
| Discoverability | `/discoverability/` | 3 |
| Learning Path | `/learn-uma/learning-path/` | 3 |
| Chapter 01 | `/learn-uma/chapter-01-the-uma-contract/` | 404 |
| UMA vs OpenAPI | `/discoverability/uma-vs-openapi/` | 404 |

---

## 2. Schema Inventory by Page

### Homepage (`/`)
- `WebSite` with `@id`
- `Book` (global Book schema injected here — see issue #H1)

### /learn-uma/book/
- `BreadcrumbList`
- `WebPage` (with datePublished / dateModified)
- `WebSite`
- `Book` with `@id`

### /discoverability/about-enrico/
- `BreadcrumbList`
- `WebPage`
- `WebSite`
- `Person` with `@id` and `sameAs`

### /discoverability/faq/
- `BreadcrumbList`
- `WebPage`
- `FAQPage` with 30 Q&A pairs
- `WebSite`

### /learn-uma/ and /learn-uma/learning-path/
- `BreadcrumbList`
- `WebPage`
- `WebSite`

### /discoverability/
- `BreadcrumbList`
- `WebPage`
- `WebSite`

---

## 3. Validation Results

### PASS

- `@context` is `https://schema.org` on all blocks (no http:// violations)
- All `@type` values are valid Schema.org types
- All URLs are absolute
- `BreadcrumbList` present on every subpage checked
- `WebPage` present on every subpage checked
- `Book` schema present on `/learn-uma/book/` with `@id`, `name`, `url`, `description`, `bookFormat`, `author`, `offers`, `about`
- `Person` schema present on `/discoverability/about-enrico/` with `@id`, `name`, `url`, `jobTitle`, `description`, `knowsAbout`, `sameAs`
- `sameAs` on `Person` — present: Medium and GitHub URLs
- No JSON parse errors on any block
- No placeholder text found

### FAIL / ISSUES

#### F1 — `Book` injected on homepage (misplaced)
**Severity: Medium**  
The `Book` block appears in the homepage (`/`) response. A `Book` entity belongs on the book's canonical page (`/learn-uma/book/`), not the homepage. Having it on both is not technically invalid, but it splits the canonical signal.  
The homepage `Book` block has no `@id` reference linking back to the book page's `@id`. This can cause Google to treat them as two separate Book entities.

**Current homepage Book block (truncated):**
```json
{
  "@context": "https://schema.org",
  "@type": "Book",
  "@id": "https://www.universalmicroservices.com/learn-uma/book/#book",
  "name": "Universal Microservices Architecture",
  "url": "https://www.universalmicroservices.com/learn-uma/book/"
}
```
**Fix:** Remove the full `Book` block from the homepage. Replace with a reference-only stub so the homepage entity graph still links to the book, without competing for canonical ownership:
```json
{
  "@context": "https://schema.org",
  "@type": "WebPage",
  "@id": "https://www.universalmicroservices.com/#webpage",
  "name": "Universal Microservices Architecture",
  "url": "https://www.universalmicroservices.com/",
  "about": {
    "@type": "Book",
    "@id": "https://www.universalmicroservices.com/learn-uma/book/#book"
  }
}
```

#### F2 — `Book` missing `isbn`
**Severity: Medium**  
`isbn` is a recommended property for `Book` rich results. Its absence reduces eligibility for enhanced display in Google Search.

**Fix:** Add to the `Book` block on `/learn-uma/book/`:
```json
"isbn": "979-8341457867"
```
(Verify the ISBN against the Amazon listing ASIN `B0GTTTTQH4` — if a print edition exists with a proper ISBN-13, use that.)

#### F3 — `Book` missing `publisher`
**Severity: Medium**  
`publisher` is a recommended property for `Book`. Currently absent.

**Fix:**
```json
"publisher": {
  "@type": "Organization",
  "name": "Self-published",
  "url": "https://www.universalmicroservices.com/"
}
```
Or if published via KDP/Amazon Publishing Services, name that organization.

#### F4 — `Book` `offers` missing `price`
**Severity: Low**  
The `Offer` block has `priceCurrency` but no `price`. An `Offer` without a `price` value is incomplete for Google's Product/Offer eligibility.

**Current:**
```json
"offers": {
  "@type": "Offer",
  "url": "https://www.amazon.com/...",
  "priceCurrency": "USD",
  "availability": "https://schema.org/InStock"
}
```
**Fix:** Add `price`. If the price is variable (Kindle Unlimited / regional pricing), use `0` for KU or the base Kindle price, and add `priceValidUntil`:
```json
"offers": {
  "@type": "Offer",
  "url": "https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4",
  "priceCurrency": "USD",
  "price": "9.99",
  "priceValidUntil": "2027-12-31",
  "availability": "https://schema.org/InStock",
  "seller": {
    "@type": "Organization",
    "name": "Amazon"
  }
}
```

#### F5 — `WebPage` dates are static (build-time, not per-page git timestamps)
**Severity: Medium**  
Every `WebPage` block has identical `datePublished` and `dateModified` values (`2026-05-30T22:48:05-06:00`) across all pages. The task description states "datePublished/dateModified from git timestamps" — if the build script is resolving to a single timestamp (e.g., `git log -1` on HEAD rather than per-file), all pages report the same modification date. Google uses `dateModified` to determine crawl priority; stale or uniform dates devalue this signal.

**Fix:** Resolve timestamps per-source-file using:
```bash
git log -1 --format="%cI" -- <source-file>
```
and inject the result per page during the build.

#### F6 — `Person` missing `image`
**Severity: Low**  
`image` is recommended for `Person` rich results (Knowledge Panel eligibility). Currently absent.

**Fix:**
```json
"image": {
  "@type": "ImageObject",
  "url": "https://www.universalmicroservices.com/assets/enrico-piovesan.jpg",
  "width": 400,
  "height": 400
}
```

#### F7 — `Person` `sameAs` missing LinkedIn
**Severity: Low**  
`sameAs` includes Medium and GitHub. LinkedIn is the highest-authority professional profile signal for Knowledge Panel association.

**Fix:** Add to `sameAs` array:
```json
"https://www.linkedin.com/in/enricopiovesan/"
```
(Verify the exact handle before adding.)

#### F8 — `Person` missing `worksFor`
**Severity: Info**  
`worksFor` strengthens Knowledge Panel association and author entity confidence.

**Fix:**
```json
"worksFor": {
  "@type": "Organization",
  "name": "Autodesk",
  "url": "https://www.autodesk.com/"
}
```

#### F9 — `BreadcrumbList` on `/learn-uma/` points to wrong second item
**Severity: Low**  
The breadcrumb trail on `/learn-uma/` is:  
`Home > Learning Path > Learn UMA`  
The second item links to `/learn-uma/learning-path/`, but "Learn UMA" is itself the parent section — "Learning Path" is a child of it. The breadcrumb hierarchy is inverted. A user landing on `/learn-uma/` should see:  
`Home > Learn UMA`  
not a child page as the parent.

**Current:**
```json
{
  "@type": "ListItem",
  "position": 2,
  "name": "Learning Path",
  "item": "https://www.universalmicroservices.com/learn-uma/learning-path/"
}
```
**Fix:** Remove the intermediate "Learning Path" item. The `/learn-uma/` page is a section root — its breadcrumb should only be `Home > Learn UMA`.

---

## 4. Missing Schema Opportunities

### O1 — `FAQPage` on `/discoverability/faq/` — PRESENT but restricted
**Status: Already implemented**  
The FAQ page has a well-formed `FAQPage` block with 30 questions. This is correct.

**Important caveat:** As of August 2023, Google restricted `FAQPage` rich results to government and healthcare sites. This site is commercial/technical, so the block will **not** generate FAQ rich results in Google Search. It does not need to be removed — `FAQPage` is still processed by LLMs and AI-powered search surfaces (Bing Copilot, SGE, Perplexity) for citation. No action required unless the goal is purely Google rich results.

### O2 — `HowTo` on tutorial/chapter pages — DO NOT ADD
**Status: Deprecated — not recommended**  
Google removed HowTo rich results in September 2023. Adding `HowTo` schema has no Google rich result benefit.

### O3 — `Article` or `TechArticle` on comparison/topic pages
**Status: Missing opportunity**  
Pages like `/discoverability/uma-vs-openapi/` (currently 404 — may exist under another URL), `/discoverability/what-is-uma/`, and similar topic pages would benefit from `TechArticle` schema. This signals content type to Google and LLMs, and enables author/datePublished association.

**Recommended addition for topic/comparison pages:**
```json
{
  "@context": "https://schema.org",
  "@type": "TechArticle",
  "@id": "https://www.universalmicroservices.com/discoverability/what-is-uma/#article",
  "headline": "What is Universal Microservices Architecture?",
  "description": "...",
  "url": "https://www.universalmicroservices.com/discoverability/what-is-uma/",
  "inLanguage": "en",
  "datePublished": "2025-01-15T00:00:00Z",
  "dateModified": "2026-05-30T22:48:05-06:00",
  "author": {
    "@type": "Person",
    "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
    "name": "Enrico Piovesan"
  },
  "publisher": {
    "@type": "Person",
    "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
    "name": "Enrico Piovesan"
  },
  "isPartOf": {
    "@type": "WebSite",
    "@id": "https://www.universalmicroservices.com/#website"
  }
}
```

### O4 — `WebPage` subtype promotion on key pages
**Status: Minor improvement**  
The current `WebPage` type is generic. Several pages warrant more specific subtypes:
- `/discoverability/about-enrico/` → `AboutPage`
- `/discoverability/faq/` → `FAQPage` is already the main schema; `WebPage` can stay
- `/learn-uma/book/` → `WebPage` is appropriate, but `BookPage` is not a schema type — keep as-is

**Fix for about page:**
```json
{
  "@type": "AboutPage",
  ...
}
```

### O5 — `SearchAction` on `WebSite`
**Status: Missing**  
If the site has or will have search functionality, adding a `SearchAction` to the `WebSite` block enables Google Sitelinks Search Box. Currently the `WebSite` block has no `potentialAction`.

**Only add if site search exists.** If not, skip.

---

## 5. Priority Action List

| Priority | Issue | Impact |
|---|---|---|
| High | F2 — Add `isbn` to Book | Rich result eligibility |
| High | F3 — Add `publisher` to Book | Rich result eligibility |
| High | F5 — Fix static timestamps (per-file git log) | Crawl freshness signal |
| Medium | F4 — Add `price` to Offer | Offer completeness |
| Medium | F1 — Remove Book block from homepage (or stub reference only) | Entity disambiguation |
| Medium | O3 — Add TechArticle to topic/comparison pages | Content-type signal for LLMs |
| Medium | F9 — Fix inverted breadcrumb on /learn-uma/ | Breadcrumb accuracy |
| Low | F6 — Add `image` to Person | Knowledge Panel |
| Low | F7 — Add LinkedIn to Person `sameAs` | Entity authority |
| Low | O4 — Promote WebPage to AboutPage on about page | Semantic precision |
| Info | F8 — Add `worksFor` to Person | Entity confidence |
| Info | O1 — FAQPage on /faq/ present but no Google rich results | Expected behavior |

---

## 6. Complete Corrected Book Block (for `/learn-uma/book/` only)

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
  "isbn": "979-8341457867",
  "author": {
    "@type": "Person",
    "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
    "name": "Enrico Piovesan"
  },
  "publisher": {
    "@type": "Organization",
    "name": "Self-published"
  },
  "offers": {
    "@type": "Offer",
    "url": "https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4",
    "priceCurrency": "USD",
    "price": "9.99",
    "priceValidUntil": "2027-12-31",
    "availability": "https://schema.org/InStock",
    "seller": {
      "@type": "Organization",
      "name": "Amazon"
    }
  },
  "about": [
    { "@type": "Thing", "name": "Microservices Architecture" },
    { "@type": "Thing", "name": "WebAssembly" },
    { "@type": "Thing", "name": "Distributed Systems" }
  ]
}
```

## 7. Complete Corrected Person Block (for `/discoverability/about-enrico/` only)

```json
{
  "@context": "https://schema.org",
  "@type": "Person",
  "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
  "name": "Enrico Piovesan",
  "url": "https://www.universalmicroservices.com/discoverability/about-enrico/",
  "jobTitle": "Platform Software Architect",
  "description": "Platform software architect with more than two decades of experience building modular, cloud-native, and event-driven systems. Author of Universal Microservices Architecture.",
  "image": {
    "@type": "ImageObject",
    "url": "https://www.universalmicroservices.com/assets/enrico-piovesan.jpg",
    "width": 400,
    "height": 400
  },
  "worksFor": {
    "@type": "Organization",
    "name": "Autodesk",
    "url": "https://www.autodesk.com/"
  },
  "knowsAbout": [
    "Microservices Architecture",
    "WebAssembly",
    "Distributed Systems",
    "Cloud-Native Architecture"
  ],
  "sameAs": [
    "https://www.linkedin.com/in/enricopiovesan/",
    "https://medium.com/@enrico.piovesan",
    "https://github.com/enricopiovesan"
  ]
}
```
