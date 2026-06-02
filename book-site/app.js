const chapters = [
  {
    number: "Chapter 4",
    title: "Feature Flag Evaluator",
    question: "What is the smallest portable UMA service?",
    payoff: "Start with one contract, one service, and deterministic rule evaluation.",
    href: "examples/chapter-04-feature-flag-evaluator/",
  },
  {
    number: "Chapter 5",
    title: "Post Fetcher Runtime",
    question: "What belongs in the runtime layer?",
    payoff: "See validation, adapter binding, and lifecycle evidence around a pure service.",
    href: "examples/chapter-05-post-fetcher-runtime/",
  },
  {
    number: "Chapter 6",
    title: "Portability Lab",
    question: "How do you prove portability instead of assuming it?",
    payoff: "Compare the same service across native and WASI with observable parity.",
    href: "examples/chapter-06-portability-lab/",
  },
  {
    number: "Chapter 7",
    title: "Metadata Orchestration",
    question: "How does orchestration emerge from contracts and events?",
    payoff: "Watch policy, subscriptions, and telemetry shape a distributed flow.",
    href: "examples/chapter-07-metadata-orchestration/",
  },
  {
    number: "Chapter 8",
    title: "Service Graph Evolution",
    question: "How do systems grow without hidden rewiring?",
    payoff: "See a service graph emerge from compatibility and change over time.",
    href: "examples/chapter-08-service-graph/",
  },
  {
    number: "Chapter 9",
    title: "Trust Boundaries",
    question: "What governs portability when trust matters?",
    payoff: "Introduce identity, permissions, provenance, and runtime trust decisions.",
    href: "examples/chapter-09-trust-boundaries/",
  },
  {
    number: "Chapter 10",
    title: "Architectural Tradeoffs",
    question: "Which decisions preserve runtime coherence?",
    payoff: "Compare coherent and degraded designs through runtime-visible outcomes.",
    href: "examples/chapter-10-architectural-tradeoffs/",
  },
  {
    number: "Chapter 11",
    title: "Evolution Without Fragmentation",
    question: "How does a system keep changing without splitting apart?",
    payoff: "Follow drift, duplication, version sprawl, and runtime-governed recovery.",
    href: "examples/chapter-11-evolution-without-fragmentation/",
  },
  {
    number: "Chapter 12",
    title: "Discoverable Decisions",
    question: "How does a governed UMA system expose decisions as queryable artifacts?",
    payoff: "Follow capability projection, proposal, validation, revision, execution, and full traceability.",
    href: "examples/chapter-12-discoverable-decisions/",
  },
  {
    number: "Chapter 13",
    title: "Portable MCP Runtime",
    question: "How do MCP discovery, runtime validation, eventing, and agent proposals compose one portable experience?",
    payoff: "Generate a structured French report from distributed sources through authoritative runtime coordination.",
    href: "examples/chapter-13-portable-mcp-runtime/",
  },
];

const chapterCards = document.querySelector("#chapter-cards");
const blogCards = document.querySelector("#blog-cards");
const topbar = document.querySelector(".topbar");
const coverFrame = document.querySelector(".cover-frame");
const menuToggle = document.querySelector(".menu-toggle");
const mobileMenu = document.querySelector(".mobile-menu");
const mobileMenuClose = document.querySelector(".mobile-menu-close");
const sharedFooter = document.querySelector("[data-shared-footer]");
const subpageMain = document.querySelector("main.subpage-main");
const faviconHref = new URL("favicon.png", import.meta.url).href;
const blogHref = "https://medium.com/the-rise-of-device-independent-architecture";
const githubHref = "https://github.com/enricopiovesan/UMA-code-examples";
const whitePaperHref = "https://drive.google.com/file/d/1e8rvpXZ7Y89R5VxmAa1nihUDkKrG1TIj/view?pli=1";

const siteRoot = new URL(".", import.meta.url);
const currentPath = window.location.pathname.replace(/index\.html$/, "").replace(/\/+$/, "/");

function internalHref(path) {
  return new URL(path, siteRoot).href;
}

function externalLink(label, href) {
  return [label, href, true];
}

function internalLink(label, path) {
  return [label, path, false];
}

function normalizePath(path) {
  return path.replace(/index\.html$/, "").replace(/\/+$/, "/");
}

function slugify(text) {
  return text
    .toLowerCase()
    .replace(/['’]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

const chapterEntries = chapters.map((chapter) => ({
  ...chapter,
  path: normalizePath(chapter.href),
}));
const chapterByPath = new Map(chapterEntries.map((chapter) => [chapter.path, chapter]));
const footerColumns = [
  [
    {
      title: "Why UMA",
      links: [
        internalLink("why UMA", "why-uma/"),
        internalLink("what problem does UMA solve?", "why-uma/what-problem-does-uma-solve/"),
        internalLink("what is UMA?", "why-uma/what-is-uma/"),
        internalLink("why universal microservices exist", "why-uma/why-universal-microservices-exist/"),
        internalLink("what is a universal microservice?", "why-uma/what-is-a-universal-microservice/"),
        internalLink("from stack ownership to behavior ownership", "why-uma/from-stack-ownership-to-behavior-ownership/"),
      ],
    },
    {
      title: "Core Model",
      links: [
        internalLink("core model", "core-model/"),
        internalLink("what is a capability?", "core-model/what-is-a-capability/"),
        internalLink("what is a workflow?", "core-model/what-is-a-workflow/"),
        internalLink("what is a uma runtime?", "core-model/what-is-a-uma-runtime/"),
        internalLink("what belongs in the runtime layer?", "core-model/what-belongs-in-the-runtime-layer/"),
        internalLink("active descriptors", "core-model/active-descriptors/"),
        internalLink("late-bound policy enforcement", "core-model/late-bound-policy-enforcement/"),
        internalLink("what makes a decision discoverable?", "core-model/what-makes-a-decision-discoverable/"),
        internalLink("what is wasm mcp?", "core-model/what-is-wasm-mcp/"),
        internalLink("agent vs runtime", "core-model/agent-vs-runtime/"),
      ],
    },
  ],
  [
    {
      title: "How UMA Works",
      links: [
        internalLink("how UMA works", "how-uma-works/"),
        internalLink("runtime-agnostic architecture", "how-uma-works/runtime-agnostic-architecture/"),
        internalLink("portable business logic", "how-uma-works/portable-business-logic/"),
        internalLink("architecture drift and portable business logic", "how-uma-works/architecture-drift-and-portable-business-logic/"),
        internalLink("webassembly architecture", "how-uma-works/webassembly-architecture/"),
        internalLink("migrating to uma incrementally", "how-uma-works/migrating-to-uma-incrementally/"),
        internalLink("incremental uma adoption", "how-uma-works/incremental-uma-adoption/"),
        internalLink("uma production readiness", "how-uma-works/uma-production-readiness/"),
      ],
    },
    {
      title: "Proof",
      links: [
        internalLink("proof", "proof/"),
        internalLink("what makes a service portable?", "proof/what-makes-a-service-portable/"),
        internalLink("how to prove portability", "proof/how-to-prove-portability/"),
        internalLink("benchmark and footprint", "proof/benchmark-and-footprint/"),
      ],
    },
  ],
  [
    {
      title: "Learn UMA",
      links: [
        internalLink("learning path", "learn-uma/learning-path/"),
        internalLink("book", "learn-uma/book/"),
        internalLink("end-to-end feature flag example", "learn-uma/end-to-end-feature-flag-example/"),
      ],
    },
    {
      title: "Hands-on Examples",
      links: [
        internalLink("examples", "examples/"),
        internalLink("chapter 4-6 tutorials", "examples/#foundations"),
        internalLink("chapter 7-9 tutorials", "examples/#orchestration-and-trust"),
        internalLink("chapter 10-13 tutorials", "examples/#evolution-and-discoverability"),
        externalLink("source repository", githubHref),
      ],
    },
  ],
  [
    {
      title: "System Evolution",
      links: [
        internalLink("evolve UMA", "evolve-uma/"),
        internalLink("contract-driven orchestration", "evolve-uma/contract-driven-orchestration/"),
        internalLink("service graph evolution", "evolve-uma/service-graph-evolution/"),
        internalLink("how systems evolve without fragmentation", "evolve-uma/how-systems-evolve-without-fragmentation/"),
        internalLink("what makes a system coherent?", "evolve-uma/what-makes-a-system-coherent/"),
        internalLink("trust boundaries", "evolve-uma/trust-boundaries/"),
        internalLink("runtime provenance and trust", "evolve-uma/runtime-provenance-and-trust/"),
        internalLink("ai-native runtime governance", "evolve-uma/ai-native-runtime-governance/"),
      ],
    },
    {
      title: "Discoverability and References",
      links: [
        internalLink("discoverability", "discoverability/"),
        internalLink("diagrams", "discoverability/diagrams/"),
        internalLink("faq", "discoverability/faq/"),
        internalLink("about enrico", "discoverability/about-enrico/"),
        externalLink("blog", blogHref),
        externalLink("github", githubHref),
        externalLink("reference application", "https://www.universalmicroservices.com/reference-application/"),
        externalLink("read the white paper", whitePaperHref),
      ],
    },
    {
      title: "Comparisons and Tradeoffs",
      links: [
        internalLink("comparisons", "comparisons/"),
        internalLink("uma vs serverless", "comparisons/uma-vs-serverless/"),
        internalLink("uma vs modular monolith", "comparisons/uma-vs-modular-monolith/"),
        internalLink("uma vs traditional microservices", "comparisons/uma-vs-traditional-microservices/"),
        internalLink("common criticisms and tradeoffs of uma", "comparisons/common-criticisms-and-tradeoffs-of-uma/"),
      ],
    },
  ],
];

if (sharedFooter) {
  const amazonHref = "https://www.amazon.com/Universal-Microservices-Architecture-Device-Independent-Modelling/dp/B0GTTTTQH4";

  function renderCluster(cluster) {
    const links = cluster.links
      .map(([label, path, external = false]) => {
        const href = external ? path : new URL(path, siteRoot).href;
        const attrs = external ? ' target="_blank" rel="noreferrer noopener"' : "";
        return `<a href="${href}"${attrs}>${label}</a>`;
      })
      .join("");
    return `<div class="footer-cluster"><h3>${cluster.title}</h3><nav class="footer-links">${links}</nav></div>`;
  }

  const footerColumnsMarkup = footerColumns
    .map((column, index) => `
      <section class="footer-column" aria-label="Footer links column ${index + 1}">
        ${column.map(renderCluster).join("")}
      </section>`)
    .join("");

  sharedFooter.innerHTML = `
    <div class="footer-inner">
      <div class="footer-book-featured">
        <a class="footer-book-cover-large" href="${amazonHref}" aria-label="Pre-order Universal Microservices Architecture on Amazon">
          <img src="${new URL("./assets/cover.png", siteRoot)}" alt="Universal Microservices Architecture book cover" />
        </a>
        <div class="footer-book-featured-copy">
          <p class="footer-book-kicker">The book — Apress, August 2026</p>
          <h2 class="footer-book-title">Universal Microservices Architecture</h2>
          <p class="footer-book-desc">The complete guide to portable, runtime-agnostic service design — with working examples and architecture diagrams. Published by Apress.</p>
          <div class="footer-book-actions">
            <a class="button button-dark footer-book-button" href="${amazonHref}" target="_blank" rel="noreferrer noopener">Pre-order on Amazon</a>
            <a class="footer-book-wp-link" href="${whitePaperHref}" target="_blank" rel="noreferrer noopener">Read the white paper →</a>
          </div>
        </div>
      </div>

      <div class="footer-divider"></div>

      <div class="footer-bottom">
        <div class="footer-grid">
          ${footerColumnsMarkup}
        </div>
        <div class="footer-meta">
          <h2 class="footer-contacts-heading">contacts</h2>
          <nav class="contacts-nav" aria-label="Contacts">
            <a href="https://enricopiovesan.com">enricopiovesan.com</a>
            <a href="https://www.instagram.com/enricopiovesan/">instagram</a>
            <a href="https://www.linkedin.com/in/enricopiovesan/">linkedin</a>
            <a href="https://x.com/enricopiovesan">x</a>
          </nav>
        </div>
      </div>
    </div>
  `;
}

function createLinkList(items, ordered = false, className = "") {
  const list = document.createElement(ordered ? "ol" : "ul");
  list.className = className;

  for (const [label, path, external = false] of items) {
    const item = document.createElement("li");
    const link = document.createElement("a");
    link.href = external ? path : internalHref(path);
    link.textContent = label;
    if (external) {
      link.target = "_blank";
      link.rel = "noreferrer noopener";
    }
    item.appendChild(link);
    list.appendChild(item);
  }

  return list;
}

function getCurrentChapter() {
  const normalizedPath = normalizePath(currentPath);
  const chapter = chapterEntries.find((entry) => normalizedPath.endsWith(entry.path));
  if (chapter) {
    return chapter;
  }

  const examplesMatch = normalizedPath.match(/\/examples\/(chapter-[^/]+)\//);
  if (examplesMatch) {
    return chapterByPath.get(normalizePath(`examples/${examplesMatch[1]}/`)) || null;
  }

  return null;
}

function getPageLabel() {
  const heading = document.querySelector("main.subpage-main h1");
  if (heading) {
    return heading.textContent.replace(/\s+/g, " ").trim();
  }

  return document.title.split("|")[0].trim();
}

function buildBreadcrumbs() {
  const breadcrumbs = document.createElement("nav");
  breadcrumbs.className = "page-breadcrumbs";
  breadcrumbs.setAttribute("aria-label", "Breadcrumb");

  const list = document.createElement("ol");
  const items = [{ label: "Home", href: internalHref("/") }];
  const chapter = getCurrentChapter();

  if (currentPath.includes("/examples/")) {
    items.push({ label: "Examples", href: internalHref("examples/") });
    if (chapter) {
      items.push({ label: `${chapter.number}: ${chapter.title}`, href: internalHref(chapter.href) });
    } else if (!currentPath.endsWith("/examples/")) {
      items.push({ label: getPageLabel() });
    }
  } else if (currentPath !== "/") {
    items.push({ label: getPageLabel() });
  }

  for (const [index, item] of items.entries()) {
    const li = document.createElement("li");
    if (index === items.length - 1 || !item.href) {
      const current = document.createElement("span");
      current.textContent = item.label;
      current.setAttribute("aria-current", "page");
      li.appendChild(current);
    } else {
      const link = document.createElement("a");
      link.href = item.href;
      link.textContent = item.label;
      li.appendChild(link);
    }
    list.appendChild(li);
  }

  breadcrumbs.appendChild(list);
  return breadcrumbs;
}

const headerMacroLinks = [
  ["Why UMA", "why-uma/"],
  ["Core Model", "core-model/"],
  ["How UMA Works", "how-uma-works/"],
  ["Learn UMA", "learn-uma/"],
  ["Proof", "proof/"],
  ["Examples", "examples/"],
  ["System Evolution", "evolve-uma/"],
  ["Discoverability", "discoverability/"],
  ["Comparisons", "comparisons/"],
];

const headerUtilityLinks = [
  ["Ref App", "https://www.universalmicroservices.com/reference-application/"],
  ["GitHub", githubHref],
  ["Blog", blogHref],
];

function buildHeadingOutline(main) {
  const headings = [...main.querySelectorAll("h2, h3")].filter((heading) => !heading.closest(".contacts-band"));
  const outline = [];
  const ids = new Set([...document.querySelectorAll("[id]")].map((element) => element.id));
  let currentSection = null;

  for (const heading of headings) {
    if (!heading.id) {
      let base = slugify(heading.textContent);
      let suffix = 2;
      while (ids.has(base)) {
        base = `${base}-${suffix++}`;
      }
      heading.id = base;
      ids.add(base);
    }

    if (heading.tagName === "H2") {
      currentSection = { label: heading.textContent.trim(), href: `#${heading.id}`, children: [] };
      outline.push(currentSection);
      continue;
    }

    if (currentSection) {
      currentSection.children.push({ label: heading.textContent.trim(), href: `#${heading.id}` });
    }
  }

  return outline;
}

function renderOutlineList(items, ordered = true) {
  const list = document.createElement(ordered ? "ol" : "ul");
  list.className = ordered ? "page-rail-outline" : "page-rail-links";

  for (const item of items) {
    const li = document.createElement("li");
    const link = document.createElement("a");
    link.href = item.href;
    link.textContent = item.label;
    li.appendChild(link);

    if (item.children?.length) {
      li.appendChild(renderOutlineList(item.children, false));
    }

    list.appendChild(li);
  }

  return list;
}

function buildPageRail(main) {
  const rail = document.createElement("aside");
  rail.className = "page-rail";
  rail.setAttribute("aria-label", "Page navigation");

  const chapter = getCurrentChapter();
  const outline = buildHeadingOutline(main);
  const exploreLinks = [
    ["home", "/"],
    ["book", "learn-uma/book/"],
    ["examples", "examples/"],
    ["learning path", "learn-uma/learning-path/"],
    ["faq", "discoverability/faq/"],
    ["blog", blogHref, true],
    ["reference application", "https://www.universalmicroservices.com/reference-application/", true],
    ["github", githubHref, true],
  ];

  rail.innerHTML = `
    <nav class="page-rail-block">
      <h2>On this page</h2>
    </nav>
    <nav class="page-rail-block">
      <h2>Explore UMA</h2>
    </nav>
  `;

  const [outlineBlock, exploreBlock] = rail.querySelectorAll(".page-rail-block");
  outlineBlock.appendChild(renderOutlineList(outline, true));
  exploreBlock.appendChild(createLinkList(exploreLinks, false, "page-rail-links"));

  if (chapter) {
    const currentIndex = chapterEntries.findIndex((entry) => entry.path === chapter.path);
    const chapterLinks = [
      ["examples hub", "examples/"],
      currentIndex > 0 ? [`previous: ${chapterEntries[currentIndex - 1].number}`, chapterEntries[currentIndex - 1].href] : null,
      currentIndex < chapterEntries.length - 1 ? [`next: ${chapterEntries[currentIndex + 1].number}`, chapterEntries[currentIndex + 1].href] : null,
      ["source folder", `https://github.com/enricopiovesan/UMA-code-examples/tree/main/${chapter.href}`, true],
    ].filter(Boolean);

    const chapterBlock = document.createElement("nav");
    chapterBlock.className = "page-rail-block";
    chapterBlock.innerHTML = "<h2>In the examples path</h2>";
    chapterBlock.appendChild(createLinkList(chapterLinks, false, "page-rail-links"));
    rail.appendChild(chapterBlock);
  }

  return rail;
}

function enhanceSubpageLayout() {
  if (!subpageMain) return;

  const pageShell = document.querySelector(".page-shell");
  if (!pageShell || pageShell.classList.contains("has-page-rail")) return;

  pageShell.classList.add("has-page-rail");
  subpageMain.prepend(buildBreadcrumbs());
  pageShell.insertBefore(buildPageRail(subpageMain), subpageMain);
}

function ensureFavicon() {
  if (document.querySelector('link[rel="icon"]')) return;

  const link = document.createElement("link");
  link.rel = "icon";
  link.type = "image/png";
  link.href = faviconHref;
  document.head.appendChild(link);
}

function ensureHeaderNavigation() {
  const desktopNav = document.querySelector(".topnav");
  const mobileNav = document.querySelector(".mobile-menu-nav");
  const fillNav = (nav, mobile = false) => {
    if (!nav) return;

    nav.innerHTML = "";

    for (const [label, href] of headerMacroLinks) {
      const link = document.createElement("a");
      link.href = internalHref(href);
      link.textContent = label;
      nav.appendChild(link);
    }

    headerUtilityLinks.forEach(([label, href], index) => {
      const link = document.createElement("a");
      link.href = href;
      link.textContent = label;
      if (href.startsWith("http")) {
        link.target = "_blank";
        link.rel = "noreferrer noopener";
      }
      if (!mobile && index === 0) {
        link.className = "topnav-github";
      }
      nav.appendChild(link);
    });
  };

  fillNav(desktopNav, false);
  fillNav(mobileNav, true);
}

if (chapterCards) {
  for (const chapter of chapters) {
    const article = document.createElement("article");
    article.className = "chapter-card reveal";
    article.innerHTML = `
      <header>
        <div>
          <span class="chapter-number">${chapter.number}</span>
          <strong>${chapter.title}</strong>
        </div>
        <span class="chapter-meta">${chapter.question}</span>
      </header>
      <p>${chapter.payoff}</p>
      <a class="chapter-link" href="${chapter.href}">Open the example</a>
    `;
    chapterCards.appendChild(article);
  }
}

if (blogCards) {
  const loadBlog = () => {
    fetch("./data/blog-posts.json")
      .then((response) => response.json())
      .then((posts) => {
        for (const post of posts) {
          const article = document.createElement("article");
          article.className = "lab-card";
          article.innerHTML = `
            <a class="lab-image-link" href="${post.url}" target="_blank" rel="noreferrer">
              <img class="lab-thumb" src="${post.image}" alt="${post.alt || post.title}" loading="lazy" />
            </a>
            <div class="lab-copy">
              <div class="lab-title">
                <strong>${post.title}</strong>
              </div>
              <div class="lab-meta">
                <p>${post.subtitle}</p>
              </div>
            </div>
            <a class="plus-mark" href="${post.url}" target="_blank" rel="noreferrer" aria-label="Open ${post.title}">+</a>
          `;
          blogCards.appendChild(article);
        }
      })
      .catch(() => {
        blogCards.innerHTML = `<p class="blog-load-error">Blog posts will appear here once the manifest is available.</p>`;
      });
  };
  if ("requestIdleCallback" in window) {
    requestIdleCallback(loadBlog, { timeout: 3000 });
  } else {
    setTimeout(loadBlog, 2000);
  }
}

document.querySelectorAll(`a[href="${githubHref}"], a[href="${blogHref}"]`).forEach((link) => {
  link.setAttribute("target", "_blank");
  link.setAttribute("rel", "noreferrer noopener");
});

if (topbar) {
  const syncTopbar = () => {
    topbar.classList.toggle("is-at-top", window.scrollY <= 8);
  };

  syncTopbar();
  window.addEventListener("scroll", syncTopbar, { passive: true });
}

if (menuToggle && mobileMenu && mobileMenuClose) {
  const openMenu = () => {
    mobileMenu.hidden = false;
    mobileMenu.classList.add("is-open");
    menuToggle.setAttribute("aria-expanded", "true");
  };

  const closeMenu = () => {
    mobileMenu.classList.remove("is-open");
    mobileMenu.hidden = true;
    menuToggle.setAttribute("aria-expanded", "false");
  };

  menuToggle.addEventListener("click", openMenu);
  mobileMenuClose.addEventListener("click", closeMenu);
  mobileMenu.addEventListener("click", (event) => {
    if (event.target === mobileMenu) {
      closeMenu();
    }
  });
  for (const link of mobileMenu.querySelectorAll("a")) {
    link.addEventListener("click", closeMenu);
  }
}

if (coverFrame) {
  const resetCoverShadow = () => {
    coverFrame.style.setProperty("--cover-shadow-x", "16px");
    coverFrame.style.setProperty("--cover-shadow-y", "-20px");
    coverFrame.style.setProperty("--cover-shadow-dark-x", "-4px");
    coverFrame.style.setProperty("--cover-shadow-dark-y", "-11px");
  };

  resetCoverShadow();

  const updateCoverShadow = (event) => {
    const rect = coverFrame.getBoundingClientRect();
    const x = (event.clientX - rect.left) / rect.width - 0.5;
    const y = (event.clientY - rect.top) / rect.height - 0.5;

    const accentX = clamp(Math.round(-x * 30), -15, 15);
    const accentY = clamp(Math.round(-y * 30), -15, 15);
    const darkX = clamp(Math.round(x * 20), -15, 15);
    const darkY = clamp(Math.round(y * 20), -15, 15);

    coverFrame.style.setProperty("--cover-shadow-x", `${accentX}px`);
    coverFrame.style.setProperty("--cover-shadow-y", `${accentY}px`);
    coverFrame.style.setProperty("--cover-shadow-dark-x", `${darkX}px`);
    coverFrame.style.setProperty("--cover-shadow-dark-y", `${darkY}px`);
  };

  window.addEventListener("pointermove", updateCoverShadow, { passive: true });
}

enhanceSubpageLayout();
ensureFavicon();
ensureHeaderNavigation();

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

const observer = new IntersectionObserver(
  (entries) => {
    for (const entry of entries) {
      if (entry.isIntersecting) {
        entry.target.classList.add("is-visible");
        observer.unobserve(entry.target);
      }
    }
  },
  {
    threshold: 0.2,
  },
);

for (const element of document.querySelectorAll(".reveal")) {
  observer.observe(element);
}

// ── Code tabs ──────────────────────────────────────────────────
document.querySelectorAll(".code-tab-bar").forEach((bar) => {
  bar.addEventListener("click", (event) => {
    const tab = event.target.closest(".code-tab");
    if (!tab) return;
    const group = tab.closest(".code-tabs");
    if (!group) return;
    const target = tab.dataset.tab;
    group.querySelectorAll(".code-tab").forEach((t) => t.classList.toggle("active", t === tab));
    group.querySelectorAll(".code-tab-panel").forEach((p) => p.classList.toggle("active", p.dataset.panel === target));
  });
});
