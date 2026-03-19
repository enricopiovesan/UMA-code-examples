#!/usr/bin/env node

import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const defaultPublication = "https://medium.com/the-rise-of-device-independent-architecture";
const defaultOutput = path.resolve(__dirname, "../data/blog-posts.json");
const targetPostCount = 25;

const args = process.argv.slice(2);

if (args.includes("--help")) {
  console.log(`Usage:
  node book-site/scripts/sync_medium_blog.mjs [publication-url] [output-file]

Defaults:
  publication-url: ${defaultPublication}
  output-file: ${defaultOutput}`);
  process.exit(0);
}

const publicationUrl = sanitizePublicationUrl(args[0] ?? defaultPublication);
const outputFile = path.resolve(args[1] ?? defaultOutput);

const posts = await syncPublication(publicationUrl);

await mkdir(path.dirname(outputFile), { recursive: true });
await writeFile(outputFile, `${JSON.stringify(posts, null, 2)}\n`, "utf8");

console.log(`Wrote ${posts.length} blog posts to ${outputFile}`);

async function syncPublication(url) {
  const slug = url.split("/").filter(Boolean).at(-1);
  const feedCandidates = [
    new URL(`/feed/${slug}`, url).toString(),
    new URL(`/feed/publication/${slug}`, url).toString(),
  ];

  for (const feedUrl of dedupe(feedCandidates)) {
    try {
      const xml = await fetchText(feedUrl);
      const items = parseRssItems(xml, url);
      if (items.length > 0) {
        return await expandPublicationPosts(items, url, targetPostCount);
      }
    } catch {
      // Fall through to the next candidate and then the archive fallback.
    }
  }

  return await scrapeArchive(url);
}

async function scrapeArchive(publicationUrl) {
  const archiveUrls = [
    `${publicationUrl.replace(/\/$/, "")}/archive`,
    ...buildMonthlyArchiveUrls(publicationUrl, 48),
  ];
  const discoveredLinks = new Set();

  for (const archiveUrl of archiveUrls) {
    try {
      const archiveHtml = await fetchText(archiveUrl);
      for (const link of extractArticleLinks(archiveHtml, publicationUrl)) {
        discoveredLinks.add(link);
      }
    } catch {
      // Ignore blocked or empty archive pages and keep probing the rest.
    }
  }

  const links = [...discoveredLinks];
  const posts = [];

  for (const link of links) {
    try {
      const html = await fetchText(link);
      const post = parseArticlePage(html, link);
      if (post) posts.push(post);
    } catch {
      // Ignore individual failures and keep going.
    }
  }

  if (posts.length === 0) {
    throw new Error(`Unable to extract Medium posts from ${publicationUrl}`);
  }

  return normalizePosts(posts);
}

async function expandPublicationPosts(seedPosts, publicationUrl, minimumCount) {
  const posts = new Map();
  const queue = [];

  for (const post of seedPosts) {
    posts.set(post.url, post);
    for (const link of post.relatedLinks ?? []) {
      if (!posts.has(link)) queue.push(link);
    }
  }

  while (queue.length > 0 && posts.size < minimumCount) {
    const link = queue.shift();
    if (posts.has(link)) continue;

    try {
      const html = await fetchText(link);
      const post = parseArticlePage(html, link, publicationUrl);
      if (!post) continue;
      posts.set(post.url, post);
      for (const related of post.relatedLinks ?? []) {
        if (!posts.has(related)) queue.push(related);
      }
    } catch {
      // Ignore individual failures and continue expanding from the links that work.
    }
  }

  return normalizePosts([...posts.values()]);
}

async function fetchText(url) {
  const response = await fetch(url, {
    headers: {
      "user-agent": "UMA Book Site Sync/1.0",
      accept: "text/html,application/rss+xml,application/xml;q=0.9,*/*;q=0.8",
    },
  });

  if (!response.ok) {
    throw new Error(`Request failed for ${url}: ${response.status}`);
  }

  return await response.text();
}

function parseRssItems(xml, publicationUrl) {
  const items = xml.match(/<item\b[\s\S]*?<\/item>/gi) ?? [];
  return dedupeBy(
    items
      .map((item) => {
        const title = cleanText(readTag(item, "title"));
        const link = normalizeArticleUrl(cleanText(readTag(item, "link")));
        const encoded = readTag(item, "content:encoded");
        const description = readTag(item, "description");
        const content = encoded || description;
        const subtitle = excerptFromHtml(content);
        const image = imageFromHtml(content);
        const publishedAt = cleanText(readTag(item, "pubDate"));
        const relatedLinks = extractPublicationArticleLinks(content, publicationUrl);
        if (!title || !link) return null;
        return {
          title,
          subtitle,
          image: image || "",
          url: link,
          alt: title,
          publishedAt,
          relatedLinks,
        };
      })
      .filter(Boolean),
    (post) => post.url,
  );
}

function parseArticlePage(html, url, publicationUrl) {
  const title = cleanText(readMeta(html, "property", "og:title") || readMeta(html, "name", "title"));
  const subtitle = cleanText(readMeta(html, "property", "og:description") || readMeta(html, "name", "description"));
  const image = cleanText(readMeta(html, "property", "og:image"));
  const publishedAt = cleanText(readMeta(html, "property", "article:published_time"));
  const relatedLinks = extractPublicationArticleLinks(html, publicationUrl);

  if (!title) return null;

  return {
    title,
    subtitle,
    image,
    url: normalizeArticleUrl(url),
    alt: title,
    publishedAt,
    relatedLinks,
  };
}

function extractPublicationArticleLinks(input, publicationUrl) {
  const base = sanitizePublicationUrl(publicationUrl);
  const publicationPath = new URL(base).pathname.replace(/\/$/, "");
  const host = new URL(base).host;
  const regex = /https?:\/\/[^"' )<>]+/g;
  const matches = input.match(regex) ?? [];

  return dedupe(
    matches
      .map((match) => decodeHtmlEntities(match))
      .map(normalizeArticleUrl)
      .filter((link) => {
        try {
          const parsed = new URL(link);
          return (
            parsed.host === host &&
            parsed.pathname.startsWith(`${publicationPath}/`) &&
            !parsed.pathname.endsWith("/archive") &&
            !parsed.pathname.endsWith("/about") &&
            !parsed.pathname.endsWith("/latest")
          );
        } catch {
          return false;
        }
      }),
  );
}

function extractArticleLinks(html, publicationUrl) {
  const publicationHost = new URL(publicationUrl).host.replace(/\./g, "\\.");
  const publicationPath = new URL(publicationUrl).pathname.replace(/\/$/, "").replace(/\//g, "\\/");
  const regex = new RegExp(`href="(https?:\\\\/\\\\/${publicationHost}${publicationPath}[^"#?]+)"`, "g");
  const links = [];
  let match;

  while ((match = regex.exec(html))) {
    links.push(normalizeArticleUrl(decodeHtmlEntities(match[1].replace(/\\\//g, "/"))));
  }

  return dedupe(
    links.filter(
      (link) =>
        !link.endsWith("/archive") &&
        !link.endsWith("/about") &&
        !link.endsWith("/latest"),
    ),
  );
}

function readTag(input, tag) {
  const escaped = tag.replace(/[-/\\^$*+?.()|[\]{}]/g, "\\$&");
  const match = input.match(new RegExp(`<${escaped}[^>]*>([\\s\\S]*?)<\\/${escaped}>`, "i"));
  return match ? stripCdata(match[1]) : "";
}

function readMeta(input, attr, value) {
  const escaped = value.replace(/[-/\\^$*+?.()|[\]{}]/g, "\\$&");
  const match = input.match(new RegExp(`<meta[^>]+${attr}=["']${escaped}["'][^>]+content=["']([^"']+)["']`, "i"));
  return match ? decodeHtmlEntities(match[1]) : "";
}

function imageFromHtml(html) {
  const match = html.match(/<img[^>]+src=["']([^"']+)["']/i);
  return match ? decodeHtmlEntities(match[1]) : "";
}

function excerptFromHtml(html) {
  const withoutImages = html.replace(/<img[\s\S]*?>/gi, " ");
  const text = cleanText(withoutImages.replace(/<[^>]+>/g, " "));
  const cleaned = cleanSubtitle(text);
  return cleaned.length > 180 ? `${cleaned.slice(0, 177).trimEnd()}...` : cleaned;
}

function cleanText(value) {
  return decodeHtmlEntities(stripCdata(value))
    .replace(/\s+/g, " ")
    .trim();
}

function cleanSubtitle(value) {
  return value
    .replace(/\s+The Rise of Device-Independent Architecture[\s|–-].*$/i, "")
    .replace(/\s+\|\s+Part\s+\d+.*$/i, "")
    .replace(/\s+Part\s+\d+.*$/i, "")
    .replace(/\s{2,}/g, " ")
    .trim();
}

function stripCdata(value) {
  return value.replace(/^<!\[CDATA\[/, "").replace(/\]\]>$/, "");
}

function decodeHtmlEntities(value) {
  return value
    .replace(/&amp;/g, "&")
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&apos;/g, "'")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">");
}

function normalizeArticleUrl(url) {
  try {
    const parsed = new URL(url);
    parsed.search = "";
    parsed.hash = "";
    return parsed.toString().replace(/\/$/, "");
  } catch {
    return url;
  }
}

function sanitizePublicationUrl(url) {
  return url.replace(/\/$/, "");
}

function buildMonthlyArchiveUrls(publicationUrl, monthsBack) {
  const base = `${sanitizePublicationUrl(publicationUrl)}/archive`;
  const urls = [];
  const cursor = new Date();
  cursor.setUTCDate(1);

  for (let i = 0; i < monthsBack; i += 1) {
    const year = cursor.getUTCFullYear();
    const month = String(cursor.getUTCMonth() + 1).padStart(2, "0");
    urls.push(`${base}/${year}/${month}`);
    cursor.setUTCMonth(cursor.getUTCMonth() - 1);
  }

  return urls;
}

function normalizePosts(posts) {
  return posts
    .sort((a, b) => {
      const aDate = Date.parse(a.publishedAt || "") || 0;
      const bDate = Date.parse(b.publishedAt || "") || 0;
      return bDate - aDate;
    })
    .map(({ relatedLinks, publishedAt, ...post }) => post);
}

function dedupe(values) {
  return [...new Set(values)];
}

function dedupeBy(values, keyFn) {
  const seen = new Set();
  const result = [];
  for (const value of values) {
    const key = keyFn(value);
    if (seen.has(key)) continue;
    seen.add(key);
    result.push(value);
  }
  return result;
}
