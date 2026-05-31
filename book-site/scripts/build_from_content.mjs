import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { execSync } from "node:child_process";

const ROOT = path.resolve(path.join(path.dirname(fileURLToPath(import.meta.url)), "..", ".."));
const CONTENT_ROOT = path.join(ROOT, "content", "pages");
const SITE_MAP_PATH = path.join(ROOT, "content", "site-map.md");
const BOOK_SITE = path.join(ROOT, "book-site");

const MACRO_NAV_LINKS = [
  ["Why", "why-uma/"],
  ["Core", "core-model/"],
  ["How it Works", "how-uma-works/"],
  ["Learn", "learn-uma/"],
  ["Proof", "proof/"],
  ["Evolution", "evolve-uma/"],
  ["Comparisons", "comparisons/"],
];

const HEADER_UTILITY_LINKS = [
  ["Ref App", "reference-application/"],
  ["GitHub", "https://github.com/enricopiovesan/UMA-code-examples"],
  ["Blog", "https://medium.com/the-rise-of-device-independent-architecture"],
];

function escapeHtml(text) {
  return String(text ?? "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

function stripQuotes(value) {
  const trimmed = value.trim();
  if (trimmed === "null") return null;
  if (trimmed.startsWith('"') && trimmed.endsWith('"')) {
    return trimmed.slice(1, -1).replace(/\\"/g, '"').replace(/\\\\/g, "\\");
  }
  return trimmed;
}

function parseFrontmatter(source) {
  const lines = source.split(/\r?\n/);
  if (lines[0] !== "---") {
    throw new Error("Missing frontmatter start");
  }

  const meta = {};
  let index = 1;

  while (index < lines.length && lines[index] !== "---") {
    const line = lines[index];

    if (line === "breadcrumbs:") {
      index += 1;
      meta.breadcrumbs = [];
      while (index < lines.length && lines[index].startsWith("  - ")) {
        meta.breadcrumbs.push(stripQuotes(lines[index].slice(4)));
        index += 1;
      }
      continue;
    }

    if (line === "related_refs:") {
      index += 1;
      meta.related_refs = [];
      while (index < lines.length && lines[index].startsWith("  - ")) {
        meta.related_refs.push(stripQuotes(lines[index].slice(4)));
        index += 1;
      }
      continue;
    }

    const match = line.match(/^([a-z_]+):\s*(.*)$/i);
    if (match) {
      const [, key, value] = match;
      meta[key] = stripQuotes(value);
    }

    index += 1;
  }

  return meta;
}

function splitSections(source) {
  const introMarker = "\n## intro\n";
  const mainMarker = "\n## main\n";
  const introStart = source.indexOf(introMarker);
  const mainStart = source.indexOf(mainMarker);

  if (introStart === -1 || mainStart === -1 || mainStart <= introStart) {
    throw new Error("Content markdown missing intro/main sections");
  }

  const intro = source.slice(introStart + introMarker.length, mainStart).trim();
  const main = source.slice(mainStart + mainMarker.length).trim();
  return { intro, main };
}

function slugToLabel(slug) {
  return String(slug || "")
    .replace(/[-_]+/g, " ")
    .replace(/\b\w/g, (char) => char.toUpperCase());
}

function resolveSiteMapGroupSlug(label) {
  const groupMap = {
    "Why UMA": "why-uma",
    "Core Model": "core-model",
    "How UMA Works": "how-uma-works",
    Proof: "proof",
    "Learning Path": "learn-uma",
    "Hands-On Examples": "examples",
    "System Evolution": "evolve-uma",
    "Discovery and References": "discoverability",
    "Comparisons and Tradeoffs": "comparisons",
  };

  return groupMap[label] || slugify(label);
}

function parseSiteMap(source) {
  const lines = source.split(/\r?\n/);
  const groups = [];
  let inMacroAreas = false;
  let current = null;

  for (const rawLine of lines) {
    const line = rawLine.trim();

    if (line === "## Macro Areas") {
      inMacroAreas = true;
      continue;
    }

    if (inMacroAreas && line.startsWith("## ")) {
      break;
    }

    if (!inMacroAreas || !line) {
      continue;
    }

    if (line.startsWith("### ")) {
      current = {
        label: line.slice(4).trim(),
        slug: resolveSiteMapGroupSlug(line.slice(4).trim()),
        slugs: [],
      };
      groups.push(current);
      continue;
    }

    if (current && line.startsWith("- ")) {
      current.slugs.push(line.slice(2).replace(/`/g, "").trim());
    }
  }

  return groups;
}

async function listMarkdownFiles(dir) {
  const entries = await fs.readdir(dir, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    if (entry.name.startsWith("_")) continue;
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...(await listMarkdownFiles(full)));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith(".md")) {
      files.push(full);
    }
  }

  return files;
}

function pagePathFromMeta(meta) {
  if (meta.ref === "examples") {
    return path.join(BOOK_SITE, "examples", "index.html");
  }

  if (meta.macro_area === "examples") {
    return path.join(BOOK_SITE, "examples", meta.slug, "index.html");
  }

  if (meta.macro_area && meta.ref === meta.macro_area) {
    return path.join(BOOK_SITE, meta.macro_area, "index.html");
  }

  if (meta.macro_area) {
    return path.join(BOOK_SITE, meta.macro_area, meta.slug, "index.html");
  }

  return path.join(BOOK_SITE, meta.slug, "index.html");
}

function legacyPagePathFromMeta(meta) {
  if (meta.ref === "examples" || meta.macro_area === "examples") {
    return pagePathFromMeta(meta);
  }

  if (meta.macro_area && meta.ref === meta.macro_area) {
    return pagePathFromMeta(meta);
  }

  if (meta.macro_area) {
    return path.join(BOOK_SITE, meta.slug, "index.html");
  }

  return pagePathFromMeta(meta);
}

function generatedCanonicalForOutPath(outPath) {
  const relative = path.relative(BOOK_SITE, outPath).replace(/\\/g, "/");
  const pathname = `/${relative.replace(/index\.html$/, "")}`;
  return new URL(pathname.endsWith("/") ? pathname : `${pathname}/`, "https://www.universalmicroservices.com/").href;
}

function relativePrefixFor(outPath) {
  const dir = path.dirname(outPath);
  const relative = path.relative(dir, BOOK_SITE).replace(/\\/g, "/");
  return relative ? `${relative}/` : "";
}

function homeAnchor(prefix, anchor) {
  return `${prefix}#${anchor}`;
}

function renderTopNav(prefix) {
  const macroLinks = MACRO_NAV_LINKS.map(([label, href]) => `<a href="${prefix}${href}">${label}</a>`).join("");
  const utilityLinks = HEADER_UTILITY_LINKS.map(
    ([label, href]) => `<a class="topnav-utility" href="${href}"${href.startsWith("http") ? ' target="_blank" rel="noreferrer noopener"' : ""}>${label}</a>`,
  ).join("");
  return `
        <div class="topbar-utility">
          ${utilityLinks}
          <a class="button button-primary header-cta" href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Pre-order</a>
          <button class="menu-toggle" type="button" aria-expanded="false" aria-controls="mobile-menu">Menu</button>
        </div>
      </div>
      <nav class="topbar-nav" aria-label="Primary">
        <div class="topbar-nav-inner">
          ${macroLinks}
        </div>
      </nav>
      <div class="topbar-row-placeholder">`;
}

function renderMobileNav(prefix) {
  const macroLinks = MACRO_NAV_LINKS.map(([label, href]) => `<a href="${prefix}${href}">${label}</a>`).join("");
  const utilityLinks = HEADER_UTILITY_LINKS.map(
    ([label, href]) => `<a href="${href}"${href.startsWith("http") ? ' target="_blank" rel="noreferrer noopener"' : ""}>${label}</a>`,
  ).join("");
  return `
      <aside class="mobile-menu" id="mobile-menu" aria-label="Mobile navigation" hidden>
        <div class="mobile-menu-panel">
          <button class="mobile-menu-close" type="button" aria-label="Close menu">Close</button>
          <nav class="mobile-menu-nav">
            ${macroLinks}
            ${utilityLinks}
          </nav>
        </div>
      </aside>`;
}

function stripTags(text) {
  return String(text ?? "").replace(/<[^>]+>/g, " ").replace(/\s+/g, " ").trim();
}

function slugify(text) {
  return String(text ?? "")
    .toLowerCase()
    .replace(/['’]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

function uniqueSlug(text, seen) {
  const base = slugify(text);
  let candidate = base;
  let index = 2;
  while (seen.has(candidate)) {
    candidate = `${base}-${index++}`;
  }
  seen.add(candidate);
  return candidate;
}

function decorateHeadings(main) {
  const seen = new Set();
  const outline = [];
  let currentSection = null;

  const html = main.replace(/<h([23])([^>]*)>([\s\S]*?)<\/h\1>/g, (match, level, attrs, inner) => {
    const text = stripTags(inner);
    const idMatch = attrs.match(/\sid="([^"]+)"/i);
    const id = idMatch ? idMatch[1] : uniqueSlug(text, seen);
    const updatedAttrs = idMatch ? attrs : `${attrs} id="${id}"`;

    if (level === "2") {
      currentSection = { label: text, href: `#${id}`, children: [] };
      outline.push(currentSection);
    } else if (currentSection) {
      currentSection.children.push({ label: text, href: `#${id}` });
    }

    return `<h${level}${updatedAttrs}>${inner}</h${level}>`;
  });

  return { html, outline };
}

function stripSharedFooterMarker(html) {
  return html.replace(/\s*<section id="contacts" class="section contacts-band" data-shared-footer><\/section>\s*$/s, "");
}

function getMacroGroup(meta, siteMapGroups) {
  return siteMapGroups.find((group) => group.slug === meta.macro_area) || null;
}

function buildBreadcrumbTrail(meta, currentOutPath, siteMapGroups, pagesBySlug) {
  const items = [];
  const currentUrl = generatedCanonicalForOutPath(currentOutPath);
  const homeUrl = makeAbsoluteUrl("/");
  const group = getMacroGroup(meta, siteMapGroups);
  const currentLabel = meta.title || "UMA";

  items.push({ label: "Home", href: homeUrl, current: false });

  if (group) {
    const hubSlug = group.slugs[0];
    const hubPage = hubSlug ? pagesBySlug.get(hubSlug) : null;
    const hubUrl = hubPage ? generatedCanonicalForOutPath(hubPage.outPath) : makeAbsoluteUrl(`/${group.slug}/`);
    const isHubPage = meta.ref === hubSlug;

    if (isHubPage) {
      items.push({ label: group.label, href: hubUrl, current: true });
      return items;
    }

    items.push({ label: group.label, href: hubUrl, current: false });
  }

  items.push({ label: currentLabel, href: currentUrl, current: true });
  return items;
}

function renderBreadcrumbs(meta, prefix, currentOutPath, siteMapGroups, pagesBySlug) {
  const items = buildBreadcrumbTrail(meta, currentOutPath, siteMapGroups, pagesBySlug);

  return `
        <nav class="page-breadcrumbs" aria-label="Breadcrumb">
          <ol>
            ${items
              .map((item) =>
                item.current
                  ? `<li><span aria-current="page">${escapeHtml(item.label)}</span></li>`
                  : `<li><a href="${escapeHtml(item.href)}">${escapeHtml(item.label)}</a></li>`,
              )
              .join("")}
          </ol>
        </nav>`;
}

function buildPageMaps(pages) {
  const bySlug = new Map();

  for (const page of pages) {
    const outPath = pagePathFromMeta(page.meta);
    const entry = { ...page, outPath };
    bySlug.set(page.meta.slug, entry);
  }

  return { bySlug };
}

function relativeLink(currentOutPath, targetOutPath) {
  const fromDir = path.dirname(currentOutPath);
  const toDir = path.dirname(targetOutPath);
  const relative = path.relative(fromDir, toDir).replace(/\\/g, "/");
  return relative ? `${relative}/` : "./";
}

function renderSectionNav(meta, siteMapGroups, pagesBySlug, currentOutPath) {
  const group = getMacroGroup(meta, siteMapGroups);
  if (!group) return "";

  const hubSlug = group.slugs[0];
  const hubPage = hubSlug ? pagesBySlug.get(hubSlug) : null;
  const hubHref = hubPage ? relativeLink(currentOutPath, hubPage.outPath) : null;

  // skip the hub page from the link list — it's represented by the label
  const links = group.slugs
    .slice(1)
    .map((slug) => {
      const page = pagesBySlug.get(slug);
      if (!page) return "";
      const href = relativeLink(currentOutPath, page.outPath);
      const isCurrent = page.meta.ref === meta.ref;
      const label = page.meta.title || slug;
      return `<li><a href="${href}" ${isCurrent ? 'class="section-nav-current" aria-current="page"' : ""}>${escapeHtml(label)}</a></li>`;
    })
    .filter(Boolean)
    .join("");

  const labelHtml = hubHref
    ? `<a class="section-nav-label" href="${hubHref}">${escapeHtml(group.label)}</a>`
    : `<div class="section-nav-label">${escapeHtml(group.label)}</div>`;

  return `
      <aside class="page-section-nav" aria-label="${escapeHtml(group.label)} navigation">
        ${labelHtml}
        ${links ? `<ul class="section-nav-links">${links}</ul>` : ""}
      </aside>`;
}

function renderMobileSectionNav(meta, siteMapGroups, pagesBySlug, currentOutPath) {
  const group = getMacroGroup(meta, siteMapGroups);
  if (!group || group.slugs.length <= 1) return "";

  const links = group.slugs
    .slice(1)
    .map((slug) => {
      const page = pagesBySlug.get(slug);
      if (!page) return "";
      const href = relativeLink(currentOutPath, page.outPath);
      const isCurrent = page.meta.ref === meta.ref;
      const label = page.meta.title || slug;
      return `<li><a href="${href}"${isCurrent ? ' class="section-nav-current" aria-current="page"' : ""}>${escapeHtml(label)}</a></li>`;
    })
    .filter(Boolean)
    .join("");

  if (!links) return "";

  return `
      <details class="mobile-section-nav">
        <summary>In this section: ${escapeHtml(group.label)}</summary>
        <ul>${links}</ul>
      </details>`;
}

function renderTocRail(outline) {
  if (!outline.length) {
    return "";
  }

  return `
      <aside class="page-toc-rail" aria-label="On this page">
        <div class="section-nav-label">On this page</div>
        ${renderOutlineList(outline, true)}
      </aside>`;
}

function renderOutlineList(items, ordered = true) {
  const tag = ordered ? "ol" : "ul";
  return `
    <${tag} class="${ordered ? "page-rail-outline" : "page-rail-links"}">
      ${items
        .map((item) => {
          const children = item.children?.length ? renderOutlineList(item.children, false) : "";
          return `<li><a href="${item.href}">${escapeHtml(item.label)}</a>${children}</li>`;
        })
        .join("")}
    </${tag}>`;
}

function makeAbsoluteUrl(pathname) {
  return new URL(pathname, "https://www.universalmicroservices.com/").href;
}

function escapeRegExp(value) {
  return String(value ?? "").replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function chapterDisplayName(meta) {
  const chapterRef = String(meta.chapter_ref || "").trim();
  const prefixPattern = chapterRef ? new RegExp(`^${escapeRegExp(chapterRef)}\\s+`, "i") : null;
  return String(meta.title || "")
    .replace(prefixPattern || /^/, "")
    .replace(/\s+Tutorial$/i, "")
    .trim();
}

function renderStructuredData(meta, rawMain, currentOutPath, siteMapGroups, pagesBySlug, dates) {
  const scripts = [];
  const canonical = generatedCanonicalForOutPath(currentOutPath);
  const breadcrumbs = buildBreadcrumbTrail(meta, currentOutPath, siteMapGroups, pagesBySlug).map((item) => ({
    name: item.label,
    item: item.current ? canonical : item.href,
  }));

  scripts.push({
    "@context": "https://schema.org",
    "@type": "BreadcrumbList",
    itemListElement: breadcrumbs.map((item, index) => ({
      "@type": "ListItem",
      position: index + 1,
      name: item.name,
      item: item.item,
    })),
  });

  const webPage = {
    "@context": "https://schema.org",
    "@type": "WebPage",
    "@id": `${canonical}#webpage`,
    name: meta.title || "UMA",
    description: meta.seo_description || meta.subtitle || "",
    url: canonical,
    inLanguage: "en",
    isPartOf: { "@id": "https://www.universalmicroservices.com/#website" },
    author: { "@type": "Person", "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan", "name": "Enrico Piovesan" },
  };
  if (dates?.published) webPage.datePublished = dates.published;
  if (dates?.modified) webPage.dateModified = dates.modified;
  scripts.push(webPage);

  // FAQ schema from data-faq sections (dt/dd pairs)
  if (rawMain.includes('data-faq="true"')) {
    const dtddPairs = [...rawMain.matchAll(/<dt>([\s\S]*?)<\/dt>\s*<dd>([\s\S]*?)<\/dd>/g)].map((match) => ({
      "@type": "Question",
      name: stripTags(match[1]).trim(),
      acceptedAnswer: { "@type": "Answer", text: stripTags(match[2]).trim() },
    }));
    if (dtddPairs.length) {
      scripts.push({ "@context": "https://schema.org", "@type": "FAQPage", mainEntity: dtddPairs });
    }
  }

  if (meta.ref === "faq") {
    const questions = [...rawMain.matchAll(/<h3[^>]*>([\s\S]*?)<\/h3>([\s\S]*?)(?=<h3[^>]*>|<\/section>|<\/div>)/g)].map((match) => ({
      "@type": "Question",
      name: stripTags(match[1]),
      acceptedAnswer: {
        "@type": "Answer",
        text: stripTags(match[2]),
      },
    }));

    if (questions.length) {
      scripts.push({
        "@context": "https://schema.org",
        "@type": "FAQPage",
        mainEntity: questions,
      });
    }
  }

  if (meta.chapter_ref) {
    const stepSection = rawMain.match(/<ol class="tutorial-steps">([\s\S]*?)<\/ol>/);
    const steps = stepSection
      ? [...stepSection[1].matchAll(/<li>\s*<strong>([\s\S]*?)<\/strong>[\s\S]*?<code>([\s\S]*?)<\/code>[\s\S]*?<\/li>/g)].map((match, index) => ({
          "@type": "HowToStep",
          position: index + 1,
          name: stripTags(match[1]),
          text: stripTags(match[2]),
        }))
      : [];

    if (steps.length) {
      scripts.push({
        "@context": "https://schema.org",
        "@type": "HowTo",
        name: `${meta.chapter_ref}: ${chapterDisplayName(meta)} UMA tutorial`,
        description: meta.seo_description || meta.subtitle || "",
        step: steps,
      });
    }

    scripts.push({
      "@context": "https://schema.org",
      "@type": "SoftwareSourceCode",
      name: `${meta.chapter_ref}: ${chapterDisplayName(meta)}`,
      codeRepository: `https://github.com/enricopiovesan/UMA-code-examples/tree/main/${meta.slug}`,
      programmingLanguage: ["Rust", "TypeScript"],
      runtimePlatform: "Rust, WASI, Node.js",
      description: meta.seo_description || meta.subtitle || "",
    });
  }

  // Chapter schema — for chapter landing pages under learn-uma
  if (meta.macro_area === "learn-uma" && meta.ref && meta.ref.startsWith("chapter-")) {
    const chapterNum = parseInt((meta.ref.match(/chapter-(\d+)/) || [])[1] || "0", 10);
    scripts.push({
      "@context": "https://schema.org",
      "@type": "Chapter",
      "@id": `${canonical}#chapter`,
      "name": meta.title || "",
      "description": meta.seo_description || "",
      "url": canonical,
      "position": chapterNum,
      "isPartOf": {
        "@type": "Book",
        "@id": "https://www.universalmicroservices.com/learn-uma/book/#book",
        "name": "Universal Microservices Architecture",
        "author": {
          "@type": "Person",
          "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
          "name": "Enrico Piovesan"
        }
      }
    });
  }

  // TechArticle schema — for concept, walkthrough, and comparison pages in key macro areas
  if (["why-uma","core-model","how-uma-works","proof","comparisons","evolve-uma"].includes(meta.macro_area) && meta.content_type !== "hub") {
    scripts.push({
      "@context": "https://schema.org",
      "@type": "TechArticle",
      "@id": `${canonical}#article`,
      "headline": meta.title || "",
      "description": meta.seo_description || meta.subtitle || "",
      "url": canonical,
      "inLanguage": "en",
      "author": {
        "@type": "Person",
        "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
        "name": "Enrico Piovesan"
      },
      "publisher": {
        "@type": "Organization",
        "@id": "https://www.universalmicroservices.com/#organization",
        "name": "Universal Microservices Architecture",
        "url": "https://www.universalmicroservices.com/"
      },
      "isPartOf": {
        "@type": "WebSite",
        "@id": "https://www.universalmicroservices.com/#website"
      }
    });
    // Backfill dates onto TechArticle from the already-built webPage
    const ta = scripts[scripts.length - 1];
    if (dates?.published) ta.datePublished = dates.published;
    if (dates?.modified) ta.dateModified = dates.modified;
  }

  // DefinedTermSet schema — for the glossary page
  if (meta.ref === "glossary") {
    const termMatches = [...rawMain.matchAll(/<h3[^>]*>([\s\S]*?)<\/h3>\s*<p>([\s\S]*?)<\/p>/g)];
    if (termMatches.length) {
      scripts.push({
        "@context": "https://schema.org",
        "@type": "DefinedTermSet",
        "@id": `${canonical}#glossary`,
        "name": "Universal Microservices Architecture Glossary",
        "url": canonical,
        "hasDefinedTerm": termMatches.map(m => ({
          "@type": "DefinedTerm",
          "name": stripTags(m[1]),
          "description": stripTags(m[2]),
          "inDefinedTermSet": `${canonical}#glossary`
        }))
      });
    }
  }

  // WebSite schema — on every page as the site identity anchor
  scripts.push({
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
  });

  // Book schema — on the /learn-uma/book/ page only
  if (meta.ref === "book" || meta.slug === "book") {
    scripts.push({
      "@context": "https://schema.org",
      "@type": "Book",
      "@id": "https://www.universalmicroservices.com/learn-uma/book/#book",
      "name": "Universal Microservices Architecture: Device-Independent Modeling for Distributed Systems Using WebAssembly",
      "url": "https://www.universalmicroservices.com/learn-uma/book/",
      "description": "A practical guide for architects and senior engineers on building portable, coherent distributed systems using Universal Microservices Architecture with WebAssembly and MCP. Published by Apress, August 2026.",
      "inLanguage": "en",
      "bookFormat": "https://schema.org/Paperback",
      "isbn": "979-8868827938",
      "datePublished": "2026-08-21",
      "author": {
        "@type": "Person",
        "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
        "name": "Enrico Piovesan"
      },
      "publisher": {
        "@type": "Organization",
        "name": "Apress",
        "url": "https://www.apress.com/"
      },
      "offers": {
        "@type": "Offer",
        "url": "https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4",
        "seller": { "@type": "Organization", "name": "Amazon" },
        "availability": "https://schema.org/PreOrder",
        "priceCurrency": "USD",
        "price": "56.99",
        "priceValidUntil": "2027-08-21"
      },
      "about": [
        { "@type": "Thing", "name": "Microservices Architecture" },
        { "@type": "Thing", "name": "WebAssembly" },
        { "@type": "Thing", "name": "Distributed Systems" }
      ],
      "hasPart": Array.from({length: 14}, (_, i) => {
        const n = String(i + 1).padStart(2, "0");
        return {
          "@type": "Chapter",
          "position": i + 1,
          "@id": `https://www.universalmicroservices.com/learn-uma/chapter-${n}-*/#chapter`
        };
      }).map((c, i) => {
        const slugMap = [
          "chapter-01-uma-introduction","chapter-02-device-independent-architecture",
          "chapter-03-what-is-universal-microservices-architecture","chapter-04-from-soa-to-metadata-driven-services",
          "chapter-05-building-portable-microservices","chapter-06-uma-runtime-layer",
          "chapter-07-webassembly-portability-wasm-runtimes","chapter-08-service-contracts-events-orchestration",
          "chapter-09-microservices-to-distributed-systems","chapter-10-security-trust-boundaries-microservices",
          "chapter-11-microservices-architecture-patterns","chapter-12-evolving-distributed-systems",
          "chapter-13-ai-agents-mcp-runtime","chapter-14-uma-reference-application"
        ];
        return { "@type": "Chapter", "position": i + 1, "@id": `https://www.universalmicroservices.com/learn-uma/${slugMap[i]}/#chapter` };
      })
    });
  }

  // Person schema — on the /discoverability/about-enrico/ page
  if (meta.ref === "about-enrico" || meta.slug === "about-enrico") {
    scripts.push({
      "@context": "https://schema.org",
      "@type": "Person",
      "@id": "https://www.universalmicroservices.com/discoverability/about-enrico/#enrico-piovesan",
      "name": "Enrico Piovesan",
      "url": "https://www.universalmicroservices.com/discoverability/about-enrico/",
      "image": "https://www.universalmicroservices.com/assets/enrico-piovesan.jpg",
      "jobTitle": "Principal Platform Architect",
      "description": "Platform software architect with more than two decades of experience building modular, cloud-native, and event-driven systems. Author of Universal Microservices Architecture.",
      "knowsAbout": ["Microservices Architecture", "WebAssembly", "Distributed Systems", "Cloud-Native Architecture"],
      "worksFor": {
        "@type": "Organization",
        "name": "Autodesk",
        "url": "https://www.autodesk.com/"
      },
      "sameAs": [
        "https://medium.com/@enrico.piovesan",
        "https://github.com/enricopiovesan",
        "https://www.linkedin.com/in/enricopiovesan/"
      ]
    });
  }

  return scripts
    .map((script) => `<script type="application/ld+json">${JSON.stringify(script)}</script>`)
    .join("\n    ");
}

function renderPage(meta, intro, main, outPath, outline, siteMapGroups, pagesBySlug, dates) {
  const prefix = relativePrefixFor(outPath);
  const title = escapeHtml(meta.title || "UMA Examples");
  const description = escapeHtml(meta.seo_description || meta.subtitle || "");
  const canonical = escapeHtml(generatedCanonicalForOutPath(outPath));
  const ogUrl = canonical;
  const ogType = meta.content_type === "hub" ? "website" : "article";
  const pageTitle = `${title} | Universal Microservices Architecture`;
  const structuredData = renderStructuredData(meta, main, outPath, siteMapGroups, pagesBySlug, dates);

  return `<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>${pageTitle}</title>
    <meta name="description" content="${description}" />
    <link rel="canonical" href="${canonical}" />
    <meta property="og:title" content="${title}" />
    <meta property="og:description" content="${description}" />
    <meta property="og:type" content="${ogType}" />
    <meta property="og:url" content="${ogUrl}" />
    <meta property="og:image" content="https://www.universalmicroservices.com/assets/og-cover.jpg" />
    <meta property="og:image:width" content="1200" />
    <meta property="og:image:height" content="630" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="${title}" />
    <meta name="twitter:description" content="${description}" />
    <meta name="twitter:image" content="https://www.universalmicroservices.com/assets/og-cover.jpg" />
    ${dates?.published ? `<meta name="article:published_time" content="${dates.published}" />` : ""}
    ${dates?.modified ? `<meta name="article:modified_time" content="${dates.modified}" />` : ""}
    ${structuredData}
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link rel="preload" as="style" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=Source+Serif+4:opsz,wght@8..60,500;8..60,700&family=Space+Grotesk:wght@400;500;700&display=swap" />
    <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=Source+Serif+4:opsz,wght@8..60,500;8..60,700&family=Space+Grotesk:wght@400;500;700&display=swap" rel="stylesheet" media="print" onload="this.media='all'" />
    <noscript><link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=Source+Serif+4:opsz,wght@8..60,500;8..60,700&family=Space+Grotesk:wght@400;500;700&display=swap" rel="stylesheet" /></noscript>
    <link rel="stylesheet" href="${prefix}styles.css" />
    <link rel="stylesheet" href="${prefix}subpages.css" />
    <link rel="icon" href="/favicon.png" type="image/png" sizes="64x64" />
    <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
    <!-- Google tag (gtag.js) -->
    <script async src="https://www.googletagmanager.com/gtag/js?id=G-J5ZJHZ3D5E"></script>
    <script>
      window.dataLayer = window.dataLayer || [];
      function gtag(){dataLayer.push(arguments);}
      gtag("js", new Date());
      gtag("config", "G-J5ZJHZ3D5E");
    </script>
  </head>
  <body>
    <div class="page-shell has-page-rail">
      <header class="topbar">
        <div class="topbar-row">
          <a class="brand" href="${prefix}">
            <span class="brand-mark">UMA</span>
            <span class="brand-name">Universal Microservices Architecture</span>
          </a>
${renderTopNav(prefix)}
        </div>
      </header>
${renderMobileNav(prefix)}

${renderSectionNav(meta, siteMapGroups, pagesBySlug, outPath)}

      <main class="subpage-main">
${renderBreadcrumbs(meta, prefix, outPath, siteMapGroups, pagesBySlug)}
${renderMobileSectionNav(meta, siteMapGroups, pagesBySlug, outPath)}

${intro}

${main}
      </main>

${renderTocRail(outline)}
    </div>
    <section id="contacts" class="section contacts-band" data-shared-footer></section>
    <script src="${prefix}app.js" type="module"></script>
${meta.ref === "diagrams" ? `    <script type="module">
      import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";
      mermaid.initialize({ startOnLoad: true, theme: "dark" });
    </script>` : ""}
  </body>
</html>
`;
}

function normalizeHtml(html) {
  return html.replace(/[ \t]+$/gm, "");
}

async function main() {
  const files = await listMarkdownFiles(CONTENT_ROOT);
  const pages = [];
  for (const file of files) {
    const source = await fs.readFile(file, "utf8");
    const meta = parseFrontmatter(source);
    const { intro, main: mainSection } = splitSections(source);
    pages.push({ meta, intro, main: mainSection, file });
  }

  const siteMapSource = await fs.readFile(SITE_MAP_PATH, "utf8");
  const siteMapGroups = parseSiteMap(siteMapSource);
  const { bySlug: pagesBySlug } = buildPageMaps(pages);

  let count = 0;
  for (const page of pages) {
    const outPath = pagePathFromMeta(page.meta);
    const legacyOutPath = legacyPagePathFromMeta(page.meta);
    const decorated = decorateHeadings(page.main);
    const renderedMain = stripSharedFooterMarker(decorated.html);
    if (legacyOutPath !== outPath) {
      await fs.rm(legacyOutPath, { force: true });
    }
    await fs.mkdir(path.dirname(outPath), { recursive: true });
    let dates;
    try {
      const log = execSync(`git log --follow --format="%aI" -- "${page.file}"`, { encoding: "utf8", cwd: ROOT }).trim().split("\n").filter(Boolean);
      if (log.length > 0) dates = { published: log[log.length - 1], modified: log[0] };
    } catch { /* git unavailable or file untracked — skip dates */ }
    await fs.writeFile(outPath, normalizeHtml(renderPage(page.meta, page.intro, renderedMain, outPath, decorated.outline, siteMapGroups, pagesBySlug, dates)));
    count += 1;
  }

  console.log(`Generated ${count} book-site HTML pages from Markdown content.`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
