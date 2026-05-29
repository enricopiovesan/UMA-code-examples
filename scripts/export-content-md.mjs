import fs from "node:fs/promises";
import path from "node:path";

const ROOT = process.cwd();
const BOOK_SITE = path.join(ROOT, "book-site");
const CONTENT_ROOT = path.join(ROOT, "content", "pages");

const WHY_UMA = new Set([
  "what-problem-does-uma-solve",
  "what-is-uma",
  "why-universal-microservices-exist",
  "what-is-a-universal-microservice",
  "from-stack-ownership-to-behavior-ownership",
  "why-software-architecture-keeps-fragmenting",
]);

const CORE_MODEL = new Set([
  "what-is-a-capability",
  "what-is-a-workflow",
  "what-is-a-uma-runtime",
  "what-belongs-in-the-runtime-layer",
  "active-descriptors",
  "late-bound-policy-enforcement",
  "what-makes-a-decision-discoverable",
  "what-is-wasm-mcp",
  "agent-vs-runtime",
]);

const PROOF = new Set([
  "what-makes-a-service-portable",
  "how-to-prove-portability",
  "benchmark-and-footprint",
]);

const LEARN = new Set([
  "learning-path",
  "book",
  "end-to-end-feature-flag-example",
]);

const EVOLVE = new Set([
  "contract-driven-orchestration",
  "service-graph-evolution",
  "how-systems-evolve-without-fragmentation",
  "what-makes-a-system-coherent",
  "trust-boundaries",
  "runtime-provenance-and-trust",
  "ai-native-runtime-governance",
]);

const DISCOVER = new Set([
  "faq",
  "diagrams",
  "about-enrico",
]);

const HOW_UMA_WORKS = new Set([
  "runtime-agnostic-architecture",
  "portable-business-logic",
  "architecture-drift-and-portable-business-logic",
  "webassembly-architecture",
  "migrating-to-uma-incrementally",
  "incremental-uma-adoption",
  "uma-production-readiness",
]);

const COMPARISONS = new Set([
  "common-criticisms-and-tradeoffs-of-uma",
  "uma-vs-serverless",
  "uma-vs-modular-monolith",
  "uma-vs-traditional-microservices",
  "why-software-architecture-keeps-fragmenting",
]);

const CHAPTERS = new Map([
  ["chapter-04-feature-flag-evaluator", ["Chapter 4", "Feature Flag Evaluator"]],
  ["chapter-05-post-fetcher-runtime", ["Chapter 5", "Post Fetcher Runtime"]],
  ["chapter-06-portability-lab", ["Chapter 6", "Portability Lab"]],
  ["chapter-07-metadata-orchestration", ["Chapter 7", "Metadata Orchestration"]],
  ["chapter-08-service-graph", ["Chapter 8", "Service Graph Evolution"]],
  ["chapter-09-trust-boundaries", ["Chapter 9", "Trust Boundaries"]],
  ["chapter-10-architectural-tradeoffs", ["Chapter 10", "Architectural Tradeoffs"]],
  ["chapter-11-evolution-without-fragmentation", ["Chapter 11", "Evolution Without Fragmentation"]],
  ["chapter-12-discoverable-decisions", ["Chapter 12", "Discoverable Decisions"]],
  ["chapter-13-portable-mcp-runtime", ["Chapter 13", "Portable MCP Runtime"]],
]);

function slugify(text) {
  return text
    .toLowerCase()
    .replace(/['’]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

function escapeYaml(text) {
  return String(text ?? "")
    .replace(/\\/g, "\\\\")
    .replace(/"/g, '\\"')
    .replace(/\r?\n/g, " ");
}

function stripTags(html) {
  return html
    .replace(/<[^>]+>/g, " ")
    .replace(/&nbsp;/g, " ")
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/\s+/g, " ")
    .trim();
}

function extractMatch(html, regex) {
  const match = html.match(regex);
  return match ? match[1] : "";
}

function extractMainSections(html) {
  const start = html.indexOf("<main");
  if (start < 0) return { main: "", hero: "", body: "" };
  const openEnd = html.indexOf(">", start);
  const end = html.lastIndexOf("</main>");
  const main = html.slice(openEnd + 1, end >= 0 ? end : html.length);
  const heroStart = main.indexOf('<section class="subpage-hero');
  if (heroStart < 0) {
    return { main, hero: "", body: main.trim() };
  }
  const heroEnd = main.indexOf("</section>", heroStart);
  if (heroEnd < 0) {
    return { main, hero: main.slice(heroStart).trim(), body: "" };
  }
  const hero = main.slice(heroStart, heroEnd + "</section>".length).trim();
  const body = main.slice(heroEnd + "</section>".length).trim();
  return { main, hero, body };
}

function buildFrontmatter(meta) {
  const lines = [];
  lines.push("---");
  lines.push(`ref: ${meta.ref}`);
  lines.push(`title: "${escapeYaml(meta.title)}"`);
  if (meta.subtitle) lines.push(`subtitle: "${escapeYaml(meta.subtitle)}"`);
  lines.push(`macro_area: ${meta.macro_area}`);
  lines.push(`content_type: ${meta.content_type}`);
  lines.push(`slug: ${meta.slug}`);
  if (meta.canonical_url) lines.push(`canonical_url: "${escapeYaml(meta.canonical_url)}"`);
  lines.push(`left_nav_group: ${meta.left_nav_group}`);
  lines.push(`chapter_ref: ${meta.chapter_ref ? `"${escapeYaml(meta.chapter_ref)}"` : "null"}`);
  if (meta.seo_description) lines.push(`seo_description: "${escapeYaml(meta.seo_description)}"`);
  lines.push("breadcrumbs:");
  for (const crumb of meta.breadcrumbs) {
    lines.push(`  - "${escapeYaml(crumb)}"`);
  }
  lines.push("related_refs:");
  for (const related of meta.related_refs) {
    lines.push(`  - ${related}`);
  }
  lines.push("---");
  return lines.join("\n");
}

function classify(slug) {
  if (slug === "examples") {
    return { macro_area: "examples", content_type: "hub", chapter_ref: null };
  }
  if (CHAPTERS.has(slug)) {
    return { macro_area: "examples", content_type: "tutorial", chapter_ref: CHAPTERS.get(slug)[0] };
  }
  if (slug === "book" || slug === "learning-path" || slug === "end-to-end-feature-flag-example") {
    return { macro_area: "learn-uma", content_type: "onboarding", chapter_ref: null };
  }
  if (WHY_UMA.has(slug)) return { macro_area: "why-uma", content_type: "overview", chapter_ref: null };
  if (CORE_MODEL.has(slug)) return { macro_area: "core-model", content_type: "explainer", chapter_ref: null };
  if (PROOF.has(slug)) return { macro_area: "proof", content_type: "proof", chapter_ref: null };
  if (LEARN.has(slug)) return { macro_area: "learn-uma", content_type: "onboarding", chapter_ref: null };
  if (EVOLVE.has(slug)) return { macro_area: "evolve-uma", content_type: "walkthrough", chapter_ref: null };
  if (DISCOVER.has(slug)) return { macro_area: "discoverability", content_type: "resource", chapter_ref: null };
  if (HOW_UMA_WORKS.has(slug)) return { macro_area: "how-uma-works", content_type: "walkthrough", chapter_ref: null };
  if (COMPARISONS.has(slug)) return { macro_area: "comparisons", content_type: "comparison", chapter_ref: null };
  return { macro_area: "discoverability", content_type: "resource", chapter_ref: null };
}

async function listSourcePages() {
  const sources = [];

  async function walk(dir, prefix = "") {
    const entries = await fs.readdir(dir, { withFileTypes: true });
    for (const entry of entries) {
      if (!entry.isDirectory()) continue;
      if (["assets", "data", "node_modules", "scripts", "target"].includes(entry.name)) continue;
      const full = path.join(dir, entry.name);
      const index = path.join(full, "index.html");
      const subentries = await fs.readdir(full, { withFileTypes: true }).catch(() => []);
      const hasIndex = subentries.some((e) => e.isFile() && e.name === "index.html");
      if (hasIndex) {
        sources.push({ slug: prefix ? `${prefix}/${entry.name}` : entry.name, file: index });
      }
      if (entry.name === "examples") {
        await walk(full, "examples");
      }
    }
  }

  await walk(BOOK_SITE);
  return sources.filter((item) => item.slug !== "index");
}

function macroAreaLabel(area) {
  return area
    .replace(/-/g, " ")
    .replace(/\b\w/g, (m) => m.toUpperCase());
}

function relatedRefsFor(slug, area, allByArea) {
  const siblings = allByArea.get(area) || [];
  return siblings.filter((entry) => entry.slug !== slug).slice(0, 4).map((entry) => entry.slug);
}

async function main() {
  const pages = await listSourcePages();
  const grouped = new Map();
  for (const page of pages) {
    const sourceHtml = await fs.readFile(page.file, "utf8");
    const { hero, body } = extractMainSections(sourceHtml);
    const title = extractMatch(sourceHtml, /<title>(.*?)<\/title>/i).split("|")[0].trim();
    const seoDescription = extractMatch(sourceHtml, /<meta\s+name="description"\s+content="([^"]+)"/i);
    const canonical = extractMatch(sourceHtml, /<link\s+rel="canonical"\s+href="([^"]+)"/i);
    const heroText = stripTags(hero);
    const slug = page.slug.replace(/^examples\//, "");
    const { macro_area, content_type, chapter_ref } = classify(slug);
    const groupedLabel = macroAreaLabel(macro_area);
    if (!grouped.has(macro_area)) grouped.set(macro_area, []);
    grouped.get(macro_area).push({ slug, title });
    const related_refs = []; // filled later
    const meta = {
      ref: slug,
      title,
      subtitle: heroText,
      macro_area,
      content_type,
      slug,
      canonical_url: canonical,
      left_nav_group: macro_area,
      chapter_ref,
      seo_description: seoDescription,
      breadcrumbs: page.slug.startsWith("examples/")
        ? ["Home", "Examples", chapter_ref ? `${chapter_ref}: ${title}` : title]
        : ["Home", groupedLabel, title],
      related_refs,
    };
    page.meta = meta;
    page.hero = hero;
    page.body = body;
  }

  for (const page of pages) {
    const siblings = relatedRefsFor(page.meta.slug, page.meta.macro_area, grouped);
    page.meta.related_refs = siblings;
  }

  await fs.mkdir(CONTENT_ROOT, { recursive: true });
  for (const page of pages) {
    const outDir = path.join(CONTENT_ROOT, page.meta.macro_area);
    await fs.mkdir(outDir, { recursive: true });
    const outPath = path.join(outDir, `${page.meta.slug}.md`);
    const md = [
      buildFrontmatter(page.meta),
      "",
      "## intro",
      "",
      page.hero || "",
      "",
      "## main",
      "",
      page.body || "",
      "",
    ].join("\n");
    await fs.writeFile(outPath, md);
  }

  console.log(`Generated ${pages.length} content markdown files.`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
