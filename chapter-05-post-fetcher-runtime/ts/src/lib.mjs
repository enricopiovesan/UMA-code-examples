import process from "node:process";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function normalizePost(json) {
  if (typeof json !== "object" || json === null || Array.isArray(json)) {
    return null;
  }

  if (
    typeof json.id !== "number" ||
    typeof json.userId !== "number" ||
    typeof json.title !== "string" ||
    typeof json.body !== "string"
  ) {
    return null;
  }

  return {
    id: json.id,
    user_id: json.userId,
    title: json.title,
    body: json.body,
  };
}

function errorMessage(status, parseError) {
  if (parseError) {
    return `parse error: ${parseError.message}`;
  }
  if (typeof status === "number") {
    return `status ${status}`;
  }
  return "unknown error";
}

class EventBus {
  constructor() {
    this.clock = 0;
    this.events = [];
  }

  emit(type, data) {
    this.events.push({
      t: String(this.clock),
      type,
      data,
    });
    this.clock += 1;
  }
}

class RetryAdapter {
  constructor(inner, retries = 3) {
    this.inner = inner;
    this.retries = retries;
  }

  async fetch(url, headers) {
    let lastError;
    for (let attempt = 0; attempt < this.retries; attempt += 1) {
      try {
        return await this.inner.fetch(url, headers);
      } catch (error) {
        lastError = error;
      }
    }

    throw lastError ?? new Error("retry adapter exhausted without an error");
  }
}

class CacheAdapter {
  constructor(inner) {
    this.inner = inner;
    this.cache = new Map();
  }

  async fetch(url, headers) {
    const cacheKey = JSON.stringify({ url, headers });
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey);
    }

    const response = await this.inner.fetch(url, headers);
    this.cache.set(cacheKey, response);
    return response;
  }
}

class HostFetchAdapter {
  async fetch(url, headers) {
    if (url === "uma-fixture://sample-post") {
      const fixturePath = path.resolve(__dirname, "../../tests/fixtures/sample_post.json");
      return {
        status: 200,
        headers: {
          "content-type": "application/json",
        },
        body: await fs.readFile(fixturePath, "utf8"),
      };
    }

    const response = await fetch(url, {
      method: "GET",
      headers,
    });
    return {
      status: response.status,
      headers: Object.fromEntries(response.headers.entries()),
      body: await response.text(),
    };
  }
}

function createAdapterManager(adapter) {
  let active = adapter ?? new HostFetchAdapter();
  let implName = adapter ? "custom" : "host-fetch";

  if (process.env.UMA_ENABLE_RETRY) {
    active = new RetryAdapter(active, 3);
    implName = `retry-${implName}`;
  }

  if (process.env.UMA_ENABLE_CACHE) {
    active = new CacheAdapter(active);
    implName = `cache-${implName}`;
  }

  return {
    adapter: active,
    binding: {
      impl: implName,
      host: "native",
    },
  };
}

export async function runJson(inputJson, adapter) {
  const input = JSON.parse(inputJson);
  const eventBus = new EventBus();
  eventBus.emit("start", { runId: input.runId });

  const allowedHeaders = new Set(["accept", "content-type", "authorization"]);
  let headerValidationFailed = false;

  for (const [key, value] of Object.entries(input.request.headers ?? {})) {
    const lower = key.toLowerCase();
    if (!allowedHeaders.has(lower)) {
      eventBus.emit("error", { error: `unexpected header ${key}` });
      headerValidationFailed = true;
    }
    if (String(value).length > 1024) {
      eventBus.emit("error", { error: `header ${key} too long` });
      headerValidationFailed = true;
    }
  }

  let normalizedPost = null;
  let finalState = "terminated";
  const adapterManager = createAdapterManager(adapter);

  if (!headerValidationFailed) {
    eventBus.emit("fetch_request", { url: input.request.url });

    try {
      const response = await adapterManager.adapter.fetch(input.request.url, input.request.headers ?? {});
      eventBus.emit("fetch_response", { status: response.status });

      try {
        const parsed = JSON.parse(response.body);
        normalizedPost = normalizePost(parsed);
        if (normalizedPost) {
          eventBus.emit("normalized", { id: normalizedPost.id });
        } else {
          eventBus.emit("error", { error: errorMessage(response.status, null) });
        }
      } catch (error) {
        eventBus.emit("error", { error: errorMessage(response.status, error) });
      }
    } catch (error) {
      eventBus.emit("fetch_response", { status: 0 });
      eventBus.emit("error", { error: error.message });
    }
  }

  if (eventBus.events.some((event) => event.type === "error")) {
    finalState = "failed";
    normalizedPost = normalizedPost ?? null;
  }

  eventBus.emit("end", {});

  return {
    output: {
      normalizedPost,
      events: eventBus.events,
    },
    lifecycle: {
      service: "uma-post-fetcher.service",
      service_version: "1.0.0",
      policy_ref: "default.runtime.policy",
      bindings: {
        "network.fetch": adapterManager.binding,
      },
      events: eventBus.events,
      state: finalState,
      logicalClock: eventBus.clock,
    },
  };
}

export function summarizeRun(report) {
  return {
    binding: report.lifecycle.bindings["network.fetch"],
    state: report.lifecycle.state,
    logicalClock: report.lifecycle.logicalClock,
    normalizedPost: report.output.normalizedPost,
    eventTypes: report.output.events.map((event) => event.type),
  };
}
