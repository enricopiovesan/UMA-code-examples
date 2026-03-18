import { readFile } from "node:fs/promises";
import path from "node:path";

function thresholdsFromContract(contract) {
  const tagging = contract.parameters?.tagging ?? {};
  return {
    dark: typeof tagging.avg_dark_threshold === "number" ? tagging.avg_dark_threshold : 0.4,
    bright: typeof tagging.avg_bright_threshold === "number" ? tagging.avg_bright_threshold : 0.6,
  };
}

export async function loadContract(rootDir) {
  const source = await readFile(path.join(rootDir, "CONTRACT.json"), "utf8");
  return JSON.parse(source);
}

export function parsePgmAscii(source) {
  const lines = source
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.startsWith("#"));

  if (lines[0] !== "P2") {
    throw new Error("Only P2 PGM is supported");
  }

  const [widthRaw, heightRaw] = lines[1].split(/\s+/);
  const width = Number.parseInt(widthRaw, 10);
  const height = Number.parseInt(heightRaw, 10);
  const maxVal = Number.parseInt(lines[2], 10);
  const pixels = lines
    .slice(3)
    .flatMap((line) => line.split(/\s+/))
    .map((token) => Number.parseInt(token, 10));

  if (!Number.isInteger(width) || !Number.isInteger(height) || !Number.isInteger(maxVal)) {
    throw new Error("Invalid PGM header");
  }

  if (pixels.length !== width * height) {
    throw new Error("pixel count mismatch");
  }

  return { width, height, pixels, maxVal };
}

export function analyzeImageData(pgmSource, contract) {
  const { width, height, pixels, maxVal } = parsePgmAscii(pgmSource);
  const sum = pixels.reduce((total, value) => total + value, 0);
  const avg = sum / pixels.length;
  const avgNorm = maxVal > 0 ? avg / maxVal : 0;
  const min = Math.min(...pixels);
  const max = Math.max(...pixels);
  const contrast = maxVal > 0 ? (max - min) / maxVal : 0;
  const thresholds = thresholdsFromContract(contract);

  const tags = [];
  if (avgNorm < thresholds.dark) {
    tags.push("mostly_dark");
  }
  if (avgNorm > thresholds.bright) {
    tags.push("mostly_bright");
  }
  if (contrast > 0.8) {
    tags.push("high_contrast");
  }
  if (tags.length === 0) {
    tags.push("neutral");
  }

  return {
    tags,
    metrics: {
      width,
      height,
      avg: avgNorm,
      contrast,
    },
  };
}

export async function renderEvent(rootDir, imagePath, serviceName = "uma.image-analyzer:1.0.0") {
  const contract = await loadContract(rootDir);
  const pgmSource = await readFile(imagePath, "utf8");
  const result = analyzeImageData(pgmSource, contract);

  return {
    event: "image.analyzed",
    payload: {
      service: serviceName,
      path: imagePath,
      tags: result.tags,
      metrics: result.metrics,
    },
  };
}
