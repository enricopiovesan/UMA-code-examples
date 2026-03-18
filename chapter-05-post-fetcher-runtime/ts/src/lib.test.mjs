import test from "node:test";
import assert from "node:assert/strict";
import { runJson, summarizeRun } from "./lib.mjs";

test("normalizes a successful fetch deterministically", async () => {
  const report = await runJson(
    JSON.stringify({
      request: { url: "https://example.com", headers: {} },
      runId: "ts-1",
    }),
    {
      async fetch() {
        return {
          status: 200,
          headers: {},
          body: JSON.stringify({ id: 1, userId: 2, title: "t", body: "b" }),
        };
      },
    },
  );

  assert.deepEqual(summarizeRun(report), {
    binding: { impl: "custom", host: "native" },
    state: "terminated",
    logicalClock: 5,
    normalizedPost: { id: 1, user_id: 2, title: "t", body: "b" },
    eventTypes: ["start", "fetch_request", "fetch_response", "normalized", "end"],
  });
});

test("header validation fails before fetch", async () => {
  let called = false;
  const report = await runJson(
    JSON.stringify({
      request: { url: "https://example.com", headers: { "x-foo": "bar" } },
      runId: "ts-2",
    }),
    {
      async fetch() {
        called = true;
        throw new Error("should not run");
      },
    },
  );

  assert.equal(called, false);
  assert.equal(report.output.normalizedPost, null);
  assert.equal(report.lifecycle.state, "failed");
  assert.deepEqual(report.output.events.map((event) => event.type), ["start", "error", "end"]);
});

test("parse errors are explicit", async () => {
  const report = await runJson(
    JSON.stringify({
      request: { url: "https://example.com", headers: {} },
      runId: "ts-3",
    }),
    {
      async fetch() {
        return {
          status: 200,
          headers: {},
          body: "not-json",
        };
      },
    },
  );

  assert.equal(report.lifecycle.state, "failed");
  assert.match(report.output.events.find((event) => event.type === "error").data.error, /^parse error:/);
});

test("wrapper binding order is cache then retry then base", async () => {
  process.env.UMA_ENABLE_RETRY = "1";
  process.env.UMA_ENABLE_CACHE = "1";

  try {
    const report = await runJson(
      JSON.stringify({
        request: { url: "https://example.com", headers: {} },
        runId: "ts-4",
      }),
      {
        async fetch() {
          return {
            status: 200,
            headers: {},
            body: JSON.stringify({ id: 1, userId: 2, title: "t", body: "b" }),
          };
        },
      },
    );
    assert.equal(report.lifecycle.bindings["network.fetch"].impl, "cache-retry-custom");
  } finally {
    delete process.env.UMA_ENABLE_RETRY;
    delete process.env.UMA_ENABLE_CACHE;
  }
});

test("fixture URL is resolved hermetically", async () => {
  const report = await runJson(
    JSON.stringify({
      request: { url: "uma-fixture://sample-post", headers: { accept: "application/json" } },
      runId: "ts-fixture",
    }),
  );

  assert.equal(report.lifecycle.state, "terminated");
  assert.equal(report.lifecycle.bindings["network.fetch"].impl, "host-fetch");
  assert.equal(report.output.normalizedPost.id, 1);
  assert.deepEqual(
    report.output.events.map((event) => event.type),
    ["start", "fetch_request", "fetch_response", "normalized", "end"],
  );
});
