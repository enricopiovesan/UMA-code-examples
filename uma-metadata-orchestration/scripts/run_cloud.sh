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
# The quick-start path is fail-open so readers can see the full orchestration flow.
POLICY_FAIL_MODE="${POLICY_FAIL_MODE:-open}" node runtime/runner.js
