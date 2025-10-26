#!/usr/bin/env node
// Generates a Mermaid graph from contracts emits/subscribes.
import fs from "node:fs";
import path from "node:path";
import yaml from "js-yaml";

const contractsDir = path.join(process.cwd(), "contracts");
const outDir = path.join(process.cwd(), "docs", "diagrams");
fs.mkdirSync(outDir, { recursive: true });

function loadYaml(p) { return yaml.load(fs.readFileSync(p, "utf-8")); }
function match(pattern, name) { return pattern.endsWith(".*") ? name.startsWith(pattern.slice(0,-2)) : pattern === name; }

const files = fs.readdirSync(contractsDir).filter(f => f.endsWith(".contract.yaml"));
const contracts = files.map(f => loadYaml(path.join(contractsDir, f)));

let edges = [];
for (const pub of contracts) {
  const emits = (pub.events && pub.events.emits) || [];
  for (const e of emits) {
    for (const sub of contracts) {
      const subs = (sub.events && sub.events.subscribes) || [];
      for (const s of subs) {
        if (match(s.pattern, e.name)) edges.push([pub.name, sub.name]);
      }
    }
  }
}

const lines = ["graph TD"];
for (const [a,b] of edges) lines.push(`  ${a.replace(/\./g,'_')} --> ${b.replace(/\./g,'_')}`);

const mermaid = lines.join("\n") + "\n";
const outPath = path.join(outDir, "orchestration_graph.mmd");
fs.writeFileSync(outPath, mermaid);
console.log("[info] graph.written", outPath);
