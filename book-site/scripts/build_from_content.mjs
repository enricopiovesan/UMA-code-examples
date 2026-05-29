import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = path.resolve(path.join(path.dirname(fileURLToPath(import.meta.url)), "..", ".."));
const CONTENT_ROOT = path.join(ROOT, "content", "pages");
const BOOK_SITE = path.join(ROOT, "book-site");

const HOME_LINKS = [
  ["Why", "why-uma"],
  ["What", "book-arc"],
  ["How", "learning-path"],
  ["Hands-on", "hands-on"],
  ["Who", "team-value"],
  ["Contacts", "contacts"],
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

  return path.join(BOOK_SITE, meta.slug, "index.html");
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
  const links = HOME_LINKS.map(([label, anchor]) => `<a href="${homeAnchor(prefix, anchor)}">${label}</a>`).join("");
  return `
        <button class="menu-toggle" type="button" aria-expanded="false" aria-controls="mobile-menu">
          Menu
        </button>
        <nav class="topnav" aria-label="Primary">
          ${links}
          <a class="topnav-github" href="https://github.com/enricopiovesan/UMA-code-examples">GitHub</a>
        </nav>
        <a class="button button-primary header-cta" href="https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4">Buy the book</a>`;
}

function renderMobileNav(prefix) {
  const links = HOME_LINKS.map(([label, anchor]) => `<a href="${homeAnchor(prefix, anchor)}">${label}</a>`).join("");
  return `
      <aside class="mobile-menu" id="mobile-menu" aria-label="Mobile navigation" hidden>
        <div class="mobile-menu-panel">
          <button class="mobile-menu-close" type="button" aria-label="Close menu">Close</button>
          <nav class="mobile-menu-nav">
            ${links}
            <a href="https://github.com/enricopiovesan/UMA-code-examples">GitHub</a>
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

function renderBreadcrumbs(meta, prefix) {
  const currentLabel = meta.title || "UMA";
  const items = [{ label: "Home", href: prefix }];

  if (meta.macro_area === "examples") {
    if (meta.ref !== "examples") {
      items.push({ label: "Examples", href: "../" });
    }
  }

  items.push({ label: currentLabel, current: true });

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

function chapterNumber(ref) {
  const match = String(ref || "").match(/(\d+)/);
  return match ? Number(match[1]) : Number.POSITIVE_INFINITY;
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

function renderStructuredData(meta, rawMain) {
  const scripts = [];
  const canonical = meta.canonical_url || makeAbsoluteUrl("/");
  const breadcrumbs = [{ name: "Home", item: makeAbsoluteUrl("/") }];

  if (meta.macro_area === "examples") {
    if (meta.ref !== "examples") {
      breadcrumbs.push({ name: "Examples", item: makeAbsoluteUrl("/examples/") });
    }
  }

  breadcrumbs.push({ name: meta.title || "UMA", item: canonical });

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

  scripts.push({
    "@context": "https://schema.org",
    "@type": "WebPage",
    name: meta.title || "UMA",
    description: meta.seo_description || meta.subtitle || "",
    url: canonical,
    inLanguage: "en",
  });

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

  return scripts
    .map((script) => `<script type="application/ld+json">${JSON.stringify(script)}</script>`)
    .join("\n    ");
}

function renderPageRail(meta, outline, chapterEntries, prefix) {
  const exploreLinks = [
    ["home", "/"],
    ["book", "book/"],
    ["examples", "examples/"],
    ["learning path", "learning-path/"],
    ["faq", "faq/"],
    ["blog", "https://medium.com/the-rise-of-device-independent-architecture", true],
    ["reference application", "https://www.universalmicroservices.com/reference-application/", true],
    ["github", "https://github.com/enricopiovesan/UMA-code-examples", true],
  ];

  const exploreMarkup = exploreLinks
    .map(([label, href, external = false]) => {
      const attrs = external ? ' target="_blank" rel="noreferrer noopener"' : "";
      const linkHref = external ? href : href.startsWith("/") ? href : `${prefix}${href}`;
      return `<li><a href="${linkHref}"${attrs}>${escapeHtml(label)}</a></li>`;
    })
    .join("");

  const chapter = meta.chapter_ref ? chapterEntries.find((entry) => entry.meta.ref === meta.ref) : null;
  const chapterMarkup = meta.chapter_ref
    ? (() => {
        const index = chapterEntries.findIndex((entry) => entry.meta.ref === meta.ref);
        const links = [
          `<li><a href="../">examples hub</a></li>`,
          index > 0 ? `<li><a href="../${chapterEntries[index - 1].meta.slug}/">previous: ${escapeHtml(chapterEntries[index - 1].meta.chapter_ref)}</a></li>` : "",
          index < chapterEntries.length - 1 ? `<li><a href="../${chapterEntries[index + 1].meta.slug}/">next: ${escapeHtml(chapterEntries[index + 1].meta.chapter_ref)}</a></li>` : "",
          `<li><a href="https://github.com/enricopiovesan/UMA-code-examples/tree/main/${meta.slug}" target="_blank" rel="noreferrer noopener">source folder</a></li>`,
        ]
          .filter(Boolean)
          .join("");

        return `
    <nav class="page-rail-block">
      <h2>In the examples path</h2>
      <ul class="page-rail-links">
        ${links}
      </ul>
    </nav>`;
      })()
    : "";

  return `
      <aside class="page-rail" aria-label="Page navigation">
    <nav class="page-rail-block">
      <h2>On this page</h2>
      ${renderOutlineList(outline, true)}
    </nav>
    <nav class="page-rail-block">
      <h2>Explore UMA</h2>
      <ul class="page-rail-links">
        ${exploreMarkup}
      </ul>
    </nav>
    ${chapterMarkup}
      </aside>`;
}

function renderPage(meta, intro, main, outPath, outline, chapterEntries) {
  const prefix = relativePrefixFor(outPath);
  const title = escapeHtml(meta.title || "UMA Examples");
  const description = escapeHtml(meta.seo_description || meta.subtitle || "");
  const canonical = escapeHtml(meta.canonical_url || "https://www.universalmicroservices.com/");
  const ogUrl = canonical;
  const ogType = meta.ref === "faq" || meta.content_type === "hub" ? "website" : "article";
  const pageTitle = `${title} | Universal Microservices Architecture`;
  const structuredData = renderStructuredData(meta, main);

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
    <meta name="twitter:card" content="summary_large_image" />
    ${structuredData}
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=Source+Serif+4:opsz,wght@8..60,500;8..60,700&family=Space+Grotesk:wght@400;500;700&display=swap" rel="stylesheet" />
    <link rel="stylesheet" href="${prefix}styles.css" />
    <link rel="stylesheet" href="${prefix}subpages.css" />
    <link rel="icon" href="${prefix}favicon.png" type="image/png" sizes="64x64" />
    <link rel="icon" href="${prefix}favicon.svg" type="image/svg+xml" />
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
        <a class="brand" href="${prefix}#top">
          <span class="brand-mark">UMA</span>
          <span class="brand-copy">
            <strong>Universal Microservices Architecture</strong>
          </span>
        </a>
${renderTopNav(prefix)}
      </header>
${renderMobileNav(prefix)}

${renderPageRail(meta, outline, chapterEntries, prefix)}

      <main class="subpage-main">
${renderBreadcrumbs(meta, prefix)}

${intro}

${main}
      </main>
      <section id="contacts" class="section contacts-band" data-shared-footer></section>
    </div>
    <script src="${prefix}app.js" type="module"></script>
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
    pages.push({ meta, intro, main: mainSection });
  }

  const chapterEntries = pages
    .filter((page) => page.meta.chapter_ref)
    .sort((a, b) => chapterNumber(a.meta.chapter_ref) - chapterNumber(b.meta.chapter_ref));

  let count = 0;
  for (const page of pages) {
    const outPath = pagePathFromMeta(page.meta);
    const decorated = decorateHeadings(page.main);
    const renderedMain = stripSharedFooterMarker(decorated.html);
    await fs.mkdir(path.dirname(outPath), { recursive: true });
    await fs.writeFile(outPath, normalizeHtml(renderPage(page.meta, page.intro, renderedMain, outPath, decorated.outline, chapterEntries)));
    count += 1;
  }

  console.log(`Generated ${count} book-site HTML pages from Markdown content.`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
