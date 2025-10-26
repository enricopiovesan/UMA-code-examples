#!/usr/bin/env bash
set -euo pipefail
# Compile runner TypeScript on the fly using node --loader if available or ts-node.
# For simplicity, transpile with a local tsc pass if present, otherwise run via ts-node/register.
if ! command -v node >/dev/null 2>&1; then
  echo "Node.js is required"
  exit 1
fi
if ! command -v wasmtime >/dev/null 2>&1; then
  echo "Wasmtime is required to execute WASI modules"
  exit 1
fi
# Compile runner with a lightweight local tsc if available, else use ts-node in ESM mode
# Here we rely on node to run an ESM TypeScript file using ts-node/register, but to keep the example self contained
# we ship a tiny JS launcher that imports the TS via ts-node if available. For portability we also ship a precompiled JS version.
# To simplify, we will ship a precompiled JS file next to the TS file.

node runtime/runner.js
