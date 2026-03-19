const chapters = [
  {
    number: "Chapter 4",
    title: "Feature Flag Evaluator",
    question: "What is the smallest portable UMA service?",
    payoff: "Start with one contract, one service, and deterministic rule evaluation.",
    href: "https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-04-feature-flag-evaluator",
  },
  {
    number: "Chapter 5",
    title: "Post Fetcher Runtime",
    question: "What belongs in the runtime layer?",
    payoff: "See validation, adapter binding, and lifecycle evidence around a pure service.",
    href: "https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-05-post-fetcher-runtime",
  },
  {
    number: "Chapter 6",
    title: "Portability Lab",
    question: "How do you prove portability instead of assuming it?",
    payoff: "Compare the same service across native and WASI with observable parity.",
    href: "https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-06-portability-lab",
  },
  {
    number: "Chapter 7",
    title: "Metadata Orchestration",
    question: "How does orchestration emerge from contracts and events?",
    payoff: "Watch policy, subscriptions, and telemetry shape a distributed flow.",
    href: "https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-07-metadata-orchestration",
  },
  {
    number: "Chapter 8",
    title: "Service Graph Evolution",
    question: "How do systems grow without hidden rewiring?",
    payoff: "See a service graph emerge from compatibility and change over time.",
    href: "https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-08-service-graph",
  },
  {
    number: "Chapter 9",
    title: "Trust Boundaries",
    question: "What governs portability when trust matters?",
    payoff: "Introduce identity, permissions, provenance, and runtime trust decisions.",
    href: "https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-09-trust-boundaries",
  },
  {
    number: "Chapter 10",
    title: "Architectural Tradeoffs",
    question: "Which decisions preserve runtime coherence?",
    payoff: "Compare coherent and degraded designs through runtime-visible outcomes.",
    href: "https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-10-architectural-tradeoffs",
  },
  {
    number: "Chapter 11",
    title: "Evolution Without Fragmentation",
    question: "How does a system keep changing without splitting apart?",
    payoff: "Follow drift, duplication, version sprawl, and runtime-governed recovery.",
    href: "https://github.com/enricopiovesan/UMA-code-examples/tree/main/chapter-11-evolution-without-fragmentation",
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

const siteRoot = new URL(".", import.meta.url);
const footerPages = [
  ["book", "book/"],
  ["what is uma", "what-is-uma/"],
  ["learning path", "learning-path/"],
  ["examples", "examples/"],
  ["webassembly architecture", "webassembly-architecture/"],
  ["runtime-agnostic architecture", "runtime-agnostic-architecture/"],
  ["portable business logic", "portable-business-logic/"],
  ["trust boundaries", "trust-boundaries/"],
  ["service graph evolution", "service-graph-evolution/"],
  ["diagrams", "diagrams/"],
  ["faq", "faq/"],
  ["about enrico", "about-enrico/"],
];

if (sharedFooter) {
  const footerLinks = footerPages
    .map(([label, path]) => `<a href="${new URL(path, siteRoot)}">${label}</a>`)
    .join("");

  sharedFooter.innerHTML = `
    <div class="contacts-heading">
      <h2>contacts</h2>
      <div class="contacts-meta">
        <nav class="contacts-nav" aria-label="Contacts">
          <a href="https://enricopiovesan.com">enricopiovesan.com</a>
          <a href="https://www.instagram.com/enricopiovesan/">instagram</a>
          <a href="https://www.linkedin.com/in/enricopiovesan/">linkedin</a>
          <a href="https://x.com/enricopiovesan">x</a>
        </nav>
      </div>
    </div>
    <nav class="footer-links" aria-label="Site links">
      ${footerLinks}
    </nav>
  `;
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
  fetch("./data/blog-posts.json")
    .then((response) => response.json())
    .then((posts) => {
      for (const post of posts) {
        const article = document.createElement("article");
        article.className = "lab-card";
        article.innerHTML = `
          <a class="lab-image-link" href="${post.url}" target="_blank" rel="noreferrer">
            <img class="lab-thumb" src="${post.image}" alt="${post.alt || post.title}" />
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
}

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
