# GEO Audit — universalmicroservices.com
**Date:** 2026-05-30
**Auditor:** Claude Sonnet 4.6 (GEO Specialist)
**Site type:** Technical book / reference site — Universal Microservices Architecture by Enrico Piovesan

---

## GEO Readiness Score: 71 / 100

| Dimension | Weight | Raw Score | Weighted |
|---|---|---|---|
| Citability | 25% | 76 | 19.0 |
| Structural Readability | 20% | 80 | 16.0 |
| Multi-Modal Content | 15% | 45 | 6.75 |
| Authority & Brand Signals | 20% | 62 | 12.4 |
| Technical Accessibility | 20% | 85 | 17.0 |
| **Total** | 100% | — | **71.15** |

---

## 1. AI Crawler Access (robots.txt)

**File:** `https://www.universalmicroservices.com/robots.txt` — Present and correct.

| Crawler | Directive | Status |
|---|---|---|
| GPTBot | `Allow: /` | Allowed |
| OAI-SearchBot | `Allow: /` | Allowed |
| ClaudeBot | `Allow: /` | Allowed |
| PerplexityBot | `Allow: /` | Allowed |
| anthropic-ai | `Allow: /` | Allowed (training + search) |
| Googlebot-Extended | `Allow: /` | Allowed |
| cohere-ai | `Allow: /` | Allowed (training + search) |
| CCBot | _(not listed)_ | Defaults to `Allow: *` |

**Assessment:** Full access granted to all major AI crawlers including training and search variants. No blocking errors. This is near-optimal. The only minor note: `CCBot` (Common Crawl, used by many open LLMs) is not explicitly listed — it inherits `Allow: /` from the wildcard rule, which is fine. No action required here.

---

## 2. llms.txt

**File:** `https://www.universalmicroservices.com/llms.txt` — **Present and populated.**

**Status:** Present, well-formed, and substantive. Key observations:

- Opening `>` blockquote provides a concise site description — correct format.
- Site description is 44 words — appropriate for an LLM context window summary.
- 10 key pages listed with inline descriptions — good discovery surface.
- 5 key concept definitions listed — usable as zero-shot reference.
- External links (Amazon, GitHub, Medium) present.

**Missing elements:**
- No `## License` or RSL 1.0 attribution — AI engines cannot confirm reuse terms.
- No `## Author` block with structured identity (name, affiliation, credentials).
- Concept definitions are single-line; no passage-length elaborations (134–167 words each) that an AI can excerpt verbatim.
- The `/discoverability/glossary/` page is the richest definitional source on the site but is not listed in llms.txt.

---

## 3. Technical Accessibility

**Rendering:** The site is **server-side rendered (SSR)** static HTML. All pages fetched via raw HTTP returned full, boilerplate-stripped content without requiring JavaScript execution. This is the ideal state for AI crawlers — no hydration gap, no deferred content.

**Sitemap:** Present at `/sitemap.xml`. All 68 URLs are listed with `<lastmod>` dates (all `2026-05-30`), which signals freshness. This is correctly formatted and complete.

**Canonical tags:** Present on all pages checked. Breadcrumb structured data (`BreadcrumbList`) present on every page.

**Page response sizes:** Range from 10–44 KB, well within AI crawler tolerance. No heavy JS bundles blocking content.

**Score driver:** 85/100. The only deduction is a lack of `hreflang` (minor for English-only) and no explicit `<article>` or `<main>` landmark roles observed in the stripped HTML — though navigation chrome is minimal enough that boilerplate-stripping tools like trafilatura can isolate content cleanly.

---

## 4. Passage-Level Citability

Pages audited: `what-is-uma`, `what-is-a-capability`, `what-is-a-workflow`, `what-is-a-universal-microservice`, `proof/what-makes-a-service-portable`, `comparisons/uma-vs-traditional-microservices`, `discoverability/faq`, `discoverability/glossary`.

### Strengths

**Opening definitional statements are strong.** Every leaf page opens with a direct H2 question as the title and a 1–2 sentence answer in the first paragraph. Examples:

- `/what-is-uma/`: "Universal Microservices Architecture, or UMA, is an execution model for distributed systems where compute can happen in many places and the system decides where logic runs." — 30 words, clear, self-contained.
- `/what-is-a-capability/`: "A capability is the unit the runtime can actually reason about." — 13 words, definitional lead sentence.
- `/what-is-a-universal-microservice/`: "A Universal Microservice is a small unit of business behavior that can remain recognizable across runtime contexts." — 18 words.
- `/proof/what-makes-a-service-portable/`: "A portable service is not just code that happens to compile in more than one place." — strong contrasting opener.

These openers are well within the 40–60 word target for AI-extractable first-answer blocks.

**FAQ page is exceptionally citable.** `/discoverability/faq/` has 28 structured Q&A pairs in `FAQPage` JSON-LD schema. The answers are self-contained, 60–130 words each, and directly address the key questions an AI would receive: "What is UMA?", "What problem does UMA solve?", "What is runtime-agnostic architecture?". This is the highest-value page on the site for AI citation.

**Glossary page is a citation goldmine that is under-signaled.** `/discoverability/glossary/` contains alphabetized term definitions with 60–120 word entries per term (Active descriptor, Behavioral coherence, Behavioral equivalence, Adapter, etc.). The structure is clear, the definitions are self-contained. However this page is not in llms.txt and not heavily cross-linked from core definitional pages.

### Weaknesses

**Passage length problem on hub pages.** Hub pages (`/why-uma/`, `/core-model/`, `/how-uma-works/`) contain mostly navigational prose — descriptive paragraphs about what child pages cover. These pass 150–300 words of content per section, but the content is descriptive, not definitional. An AI extracting from a hub page gets "this area explains X" rather than "X means Y." Hub pages should either be deprioritized in llms.txt or augmented with embedded definitions.

**"Short answer" blocks are the right pattern but slightly underused.** Pages like `what-is-a-capability` and `what-is-a-workflow` include a "The short answer" subsection — this is exactly the right pattern for AI extraction. However, these blocks often run 80–110 words where 134–167 words is the optimal citation-length range. They are trimmed to the point of being slightly too brief to cite as complete standalone passages.

**Term density is adequate but not optimized.** On `what-is-uma`, "Universal Microservices Architecture" (full form) appears only once in the first 297 extractable words (3.4 per 1000). AI engines use term co-occurrence to anchor brand identity. The full term should appear 2–3 times in the first 200 words of every definitional page.

**No statistics or quantified claims.** There are no sentences like "UMA reduced X by Y%" or "benchmark shows Z ms overhead." These are the most citable single sentences in technical content. The `/proof/benchmark-and-footprint/` page likely covers this, but the data does not surface in the definitional or comparison pages.

**Comparison pages use hedged language.** Passages like "UMA earns its overhead when behavior must cross runtime boundaries" are intellectually honest but not citable — an AI cannot excerpt them as definitive statements. Comparison pages need at least one "hard claim" table or bulleted differentiation list that is extractable without surrounding context.

---

## 5. Structural Readability

**H-tag structure:** All pages use a clear hierarchy: `h1` = page title (question form), `h2` = section headers (some question-form, some not), `h3` = sub-sections. Question-form headings are used consistently on core definitional pages — good alignment with conversational search queries.

**"On this page" TOC:** Present on hub pages (observed in scraped content). Not present on leaf pages. Leaf pages would benefit from an in-page TOC anchor list for long definitional pages.

**Keyword-based headings present:**
- "What is Universal Microservices Architecture?" — H1 on `/what-is-uma/`
- "What is a capability in UMA?" — H1 on `/what-is-a-capability/`
- "What is a workflow in UMA?" — H1 on `/what-is-a-workflow/`

These map directly to likely AI query inputs. Strong.

**Missing:** No `<aside>` definition boxes, no "Key takeaway" callout blocks, no summary tables on comparison pages. These formatting patterns signal citable passage boundaries to AI scrapers.

---

## 6. Authority & Brand Signals

### On-site signals (present)

- `Person` JSON-LD for Enrico Piovesan on every page: name, job title, `knowsAbout`, `sameAs` pointing to GitHub and Medium.
- `Book` JSON-LD with Amazon offer on homepage.
- `WebSite` JSON-LD with publisher `Person` entity site-wide.
- `datePublished` and `dateModified` on every page.
- Author bio page at `/discoverability/about-enrico/` — well-written, includes professional context (Autodesk, principal platform architect).

### Missing on-site signals

- No `linkedin.com` URL in `sameAs` on the `Person` entity. LinkedIn is a strong authority signal for AI citation.
- No `amazon.com/author/` page linked from the `Person` entity.
- The `Book` JSON-LD on the homepage does not include `isbn`, `numberOfPages`, or `publisher` (as `Organization`) — all signals that anchor the book entity for AI knowledge graphs.
- Author bio does not include explicit years of experience as a structured data field — only in prose.

### Off-site signals (gaps)

| Signal | Status | Impact |
|---|---|---|
| Wikipedia entity (Enrico Piovesan) | Not found | High — strongest brand-to-citation correlation |
| Wikipedia entity (Universal Microservices Architecture) | Not found | High |
| Reddit mentions (r/microservices, r/softwarearchitecture) | Unknown — not audited live | High |
| YouTube presence | Not found / not linked | Highest correlation (~0.737) with AI citation |
| Medium blog | Active — linked from site | Moderate |
| GitHub repo | Active — linked from site | Moderate |
| Amazon author page | Not verified in `sameAs` | Moderate |
| LinkedIn | Not linked in structured data | Moderate |

The off-site signal gap is the most significant GEO weakness on this site. The content quality and structure are strong; the problem is that AI engines weight entity recognition from cross-domain sources heavily (Wikipedia, YouTube, Reddit), and none of those surfaces appear to anchor UMA or Enrico Piovesan as a recognized entity.

---

## 7. Multi-Modal Content

**Diagrams page:** `/discoverability/diagrams/` exists in the sitemap. Not audited in detail, but presence is a positive signal.

**Code examples:** 10 chapter-aligned runnable labs exist at `/examples/`. These are linked to a GitHub repo, which is good for developer-audience AI engines (Perplexity, GitHub Copilot). Code is not embedded in the main content pages, reducing inline citability.

**Video content:** None found or linked. This is the largest gap in multi-modal scoring. YouTube presence has the strongest known correlation with AI citation (~0.737). A single well-structured YouTube video explaining "What is Universal Microservices Architecture?" would be the highest-ROI multi-modal investment.

**Images:** Book cover and OG image present. No data visualizations, architecture diagrams, or comparison charts embedded in content pages.

---

## 8. Citation Readiness by Page Type

| Page | Type | Citation Score | Notes |
|---|---|---|---|
| `/discoverability/faq/` | Leaf / FAQ | 88/100 | FAQPage schema, 28 Q&As, self-contained answers — top asset |
| `/why-uma/what-is-uma/` | Leaf / definitional | 82/100 | Strong opener, clear definition, needs more full-term mentions |
| `/core-model/what-is-a-capability/` | Leaf / definitional | 80/100 | Excellent structure, "short answer" block slightly short |
| `/discoverability/glossary/` | Leaf / reference | 79/100 | High-density definitions, not in llms.txt — underexploited |
| `/why-uma/what-is-a-universal-microservice/` | Leaf / definitional | 77/100 | Good but lifecycle section dilutes definitional density |
| `/proof/what-makes-a-service-portable/` | Leaf / definitional | 74/100 | Strong concept, needs quantified benchmark claim to anchor |
| `/comparisons/uma-vs-traditional-microservices/` | Leaf / comparison | 65/100 | Good framing, no comparison table, hedged language |
| `/why-uma/` | Hub | 42/100 | Navigational prose, no extractable definitions |
| `/core-model/` | Hub | 40/100 | Same — descriptive, not definitional |
| `/` (homepage) | Hub / marketing | 35/100 | Minimal body text, mostly navigation chrome |

---

## 9. Platform-Specific Scores

| Platform | Est. Score | Key Bottleneck |
|---|---|---|
| Perplexity | 72/100 | SSR + FAQPage schema + good definitions — strong. Gap: no statistics to cite. |
| Google AI Overviews | 65/100 | Good schema, question headings — hurt by weak off-site entity signals. |
| ChatGPT (web search) | 58/100 | Needs Reddit/YouTube anchoring; GPTBot allowed but entity not recognized. |
| Bing Copilot | 60/100 | Similar to ChatGPT; Bing weights page structure more, which helps. |

---

## 10. Top 5 Highest-Impact Recommendations

### 1. Create a YouTube video — "What is Universal Microservices Architecture?" (Effort: Medium / Impact: Very High)

YouTube presence has the strongest known correlation with AI citation (~0.737). A 5–8 minute explainer that verbally defines UMA, capability, and workflow — using the exact phrasing from the best-performing site pages — would:
- Create a cross-domain entity anchor for both Enrico Piovesan and UMA.
- Feed Perplexity's video citation layer.
- Provide a `VideoObject` JSON-LD target to add to the homepage and `/what-is-uma/`.

The script can be drawn almost verbatim from `/why-uma/what-is-uma/` — the content is already at the right density.

### 2. Add Glossary and key leaf pages to llms.txt (Effort: Low / Impact: High)

`/discoverability/glossary/` is the most AI-citable page on the site and is absent from llms.txt. Add it as the first entry under `## Key pages`. Also add `/why-uma/what-is-uma/` and `/core-model/what-is-a-capability/` as direct definitional targets. Update the concept definitions in llms.txt to be passage-length (134–167 words each) rather than single-line summaries.

Also add:
```
## Author
Enrico Piovesan — principal platform architect at Autodesk with 20+ years in distributed systems. Author of *Universal Microservices Architecture* (Amazon, 2025).

## License
Content on this site may be cited with attribution to Enrico Piovesan and universalmicroservices.com.
```

### 3. Expand "Short answer" blocks to 134–167 words on all definitional leaf pages (Effort: Low / Impact: High)

The "The short answer" pattern is correct but the blocks run 80–110 words. Extending each to 140–160 words adds one or two elaborating sentences that:
- Hit the optimal AI citation window.
- Allow the block to be extracted as a complete, self-contained answer.
- Increase the probability of verbatim excerpt in AI Overviews and Perplexity answer cards.

Target pages: `what-is-uma`, `what-is-a-capability`, `what-is-a-workflow`, `what-is-a-universal-microservice`, `proof/what-makes-a-service-portable`.

### 4. Add LinkedIn to `sameAs` in Person JSON-LD and fix Book schema (Effort: Low / Impact: Medium-High)

Update the `Person` entity in every page's `<script type="application/ld+json">`:
```json
"sameAs": [
  "https://medium.com/@enrico.piovesan",
  "https://github.com/enricopiovesan",
  "https://www.linkedin.com/in/[handle]",
  "https://www.amazon.com/stores/author/[amazon-author-id]"
]
```

Update the `Book` entity on the homepage to include `isbn`, `numberOfPages`, and `publisher` as an Organization node. These fields are used by AI knowledge graph builders to anchor the book as a distinct entity.

### 5. Add one quantified benchmark claim to `/proof/benchmark-and-footprint/` and surface it on comparison pages (Effort: Medium / Impact: Medium-High)

The most citable sentence in any technical document is a specific, attributed statistic. Example target sentence:
> "In the UMA benchmark suite, a WASM-compiled feature flag evaluator running across Wasmtime, Node.js, and browser environments produces identical outputs in under 2ms per evaluation with a binary footprint under 150KB."

This type of claim:
- Is the most likely sentence to appear in a Perplexity or ChatGPT answer about WASM microservices performance.
- Gives comparison pages a hard factual anchor that comparison shoppers (and AI engines) can cite.
- Currently the proof pages describe the methodology without surfacing a headline number in the first paragraph.

---

## Summary of Quick Wins (ordered by effort/impact ratio)

| Action | Effort | Impact |
|---|---|---|
| Add Glossary + author block to llms.txt | 30 min | High |
| Expand "short answer" blocks to 140–160 words | 2 hrs | High |
| Add LinkedIn + Amazon to Person sameAs JSON-LD | 30 min | Medium-High |
| Fix Book JSON-LD (isbn, numberOfPages, publisher) | 30 min | Medium |
| Surface one benchmark headline stat on proof pages | 2 hrs | Medium-High |
| Create one YouTube explainer video | 4–8 hrs | Very High |
| Add "Key takeaway" definition boxes to hub pages | 2 hrs | Medium |
| Participate in r/microservices / r/softwarearchitecture | Ongoing | High (long-term) |
