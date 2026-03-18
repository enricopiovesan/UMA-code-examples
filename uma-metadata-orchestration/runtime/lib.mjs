import fs from "node:fs";
import path from "node:path";
import yaml from "js-yaml";

export function loadYaml(filePath) {
  return yaml.load(fs.readFileSync(filePath, "utf-8"));
}

export function matchPattern(pattern, eventName) {
  if (pattern.endsWith(".*")) {
    return eventName.startsWith(pattern.slice(0, -2));
  }
  return pattern === eventName;
}

export function bindContracts(publisher, subscriber) {
  const emits = publisher.events?.emits || [];
  const subscriptions = subscriber.events?.subscribes || [];
  const bindings = [];

  for (const emitted of emits) {
    for (const subscription of subscriptions) {
      if (matchPattern(subscription.pattern, emitted.name)) {
        bindings.push({ event: emitted.name, schema: emitted.schema });
      }
    }
  }

  return bindings;
}

export function findPolicyViolation(policy, evaluatorContract) {
  const placements = evaluatorContract.constraints?.placement || [];
  if (!placements.includes("browser")) {
    return null;
  }

  const denyRules = policy.deny || [];
  const rule = denyRules.find(
    (entry) =>
      entry.if &&
      entry.if.service === "ai.model.evaluator" &&
      entry.if.placement === "browser",
  );

  return rule ? `policy.deny ${rule.rule}` : null;
}

export function summarizeRunnerOutput(output) {
  const lines = output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);

  const bindingLines = lines
    .filter((line) => line.startsWith("[info] binding.created "))
    .sort();

  const parseJsonSuffix = (prefix) => {
    const line = lines.find((entry) => entry.startsWith(prefix));
    if (!line) {
      return null;
    }
    return JSON.parse(line.slice(prefix.length));
  };

  const telemetryJson = parseJsonSuffix("[info] telemetry.ok ");
  const cacheJson = parseJsonSuffix("[info] cache.ok ");
  const evaluatorJson = parseJsonSuffix("[info] evaluator.ok ");
  const policyLine = lines.find((line) => line.startsWith("[warn] policy.violation "));
  const validationLine = lines.find((line) => line.startsWith("[info] validation.passed "));

  return {
    bindings: bindingLines,
    policyLine: policyLine || null,
    validationLine: validationLine || null,
    telemetry: telemetryJson
      ? {
          source: telemetryJson.source,
          event: telemetryJson.event,
          status: telemetryJson.status,
        }
      : null,
    cache: cacheJson
      ? {
          source: cacheJson.source,
          event: cacheJson.event,
          status: cacheJson.status,
        }
      : null,
    evaluator: evaluatorJson
      ? {
          id: evaluatorJson.id,
          score: evaluatorJson.score,
        }
      : null,
  };
}

export function chapterRoot(currentDir = process.cwd()) {
  const direct = currentDir;
  if (fs.existsSync(path.join(direct, "contracts"))) {
    return direct;
  }

  const parent = path.resolve(currentDir, "..");
  if (fs.existsSync(path.join(parent, "contracts"))) {
    return parent;
  }

  throw new Error("unable to locate Chapter 7 project root");
}
