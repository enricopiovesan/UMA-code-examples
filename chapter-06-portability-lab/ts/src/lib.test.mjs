import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { mkdtemp, writeFile } from "node:fs/promises";
import os from "node:os";
import { analyzeImageData, parsePgmAscii, renderEvent } from "./lib.mjs";

const rootDir = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");

test("checkerboard image is high contrast", async () => {
  const event = await renderEvent(rootDir, path.join(rootDir, "sample-data", "sample.pgm"));
  assert.deepEqual(event.payload.tags, ["high_contrast"]);
  assert.equal(event.payload.metrics.width, 8);
  assert.equal(event.payload.metrics.height, 8);
});

test("bright image is mostly bright", async () => {
  const event = await renderEvent(rootDir, path.join(rootDir, "sample-data", "bright.pgm"));
  assert.deepEqual(event.payload.tags, ["mostly_bright"]);
});

test("invalid pgm magic is rejected", () => {
  assert.throws(() => parsePgmAscii("P5\n1 1\n255\n0\n"), /Only P2 PGM is supported/);
});

test("contract thresholds change tags", () => {
  const result = analyzeImageData("P2\n2 2\n10\n3 3 3 3\n", {
    parameters: {
      tagging: {
        avg_dark_threshold: 0.35,
        avg_bright_threshold: 0.9,
      },
    },
  });
  assert.deepEqual(result.tags, ["mostly_dark"]);
});

test("pixel count mismatch is rejected", async () => {
  const tmpDir = await mkdtemp(path.join(os.tmpdir(), "uma-ch6-ts-"));
  const imagePath = path.join(tmpDir, "bad.pgm");
  await writeFile(imagePath, "P2\n2 2\n255\n1 2\n", "utf8");
  await assert.rejects(renderEvent(rootDir, imagePath), /pixel count mismatch/);
});
