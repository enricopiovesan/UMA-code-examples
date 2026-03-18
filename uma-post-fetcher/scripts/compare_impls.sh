#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'EOF'
Compare the Chapter 5 Rust runtime and TypeScript reference implementation.

Usage:
  ./scripts/compare_impls.sh [lab-name]

Available labs:
  lab1-cloud-golden-path
  lab2-header-validation-fail-fast
  lab3-adapter-binding-and-wrappers

If no lab name is provided, all Chapter 5 parity checks run.
EOF
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd cargo
require_cmd node
require_cmd python3

run_single() {
  local lab="$1"
  local rust_json
  local ts_json
  local input_json
  local tmp_dir
  local server_pid=""
  local port="${UMA_DEMO_PORT:-18080}"

  tmp_dir="$(mktemp -d)"
  trap 'if [[ -n "$server_pid" ]]; then kill "$server_pid" >/dev/null 2>&1 || true; wait "$server_pid" 2>/dev/null || true; fi; rm -rf "$tmp_dir"' RETURN

  case "$lab" in
    lab1-cloud-golden-path|lab3-adapter-binding-and-wrappers)
      mkdir -p "$tmp_dir/posts"
      cp "$ROOT_DIR/tests/fixtures/sample_post.json" "$tmp_dir/posts/1"
      python3 -m http.server "$port" --bind 127.0.0.1 --directory "$tmp_dir" >/dev/null 2>&1 &
      server_pid=$!
      sleep 1
      input_json='{"request":{"url":"http://127.0.0.1:'"$port"'/posts/1","headers":{"accept":"application/json"}},"runId":"demo-001"}'
      ;;
    lab2-header-validation-fail-fast)
      input_json='{"request":{"url":"https://example.com","headers":{"x-foo":"bar"}},"runId":"demo-001"}'
      ;;
    *)
      echo "Unknown lab: $lab" >&2
      usage >&2
      exit 1
      ;;
  esac

  if [[ "$lab" == "lab3-adapter-binding-and-wrappers" ]]; then
    rust_json="$(cd "$ROOT_DIR" && UMA_ENABLE_RETRY=1 UMA_ENABLE_CACHE=1 printf '%s' "$input_json" | cargo run --quiet -p uma_runtime)"
    ts_json="$(cd "$ROOT_DIR" && UMA_ENABLE_RETRY=1 UMA_ENABLE_CACHE=1 printf '%s' "$input_json" | node ts/src/main.mjs)"
  else
    rust_json="$(cd "$ROOT_DIR" && printf '%s' "$input_json" | cargo run --quiet -p uma_runtime)"
    ts_json="$(cd "$ROOT_DIR" && printf '%s' "$input_json" | node ts/src/main.mjs)"
  fi

  node - "$lab" "$rust_json" "$ts_json" <<'EOF'
const assert = require("node:assert/strict");
const [, , lab, rustRaw, tsRaw] = process.argv;
const rust = JSON.parse(rustRaw);
const ts = JSON.parse(tsRaw);

const summarize = (report) => ({
  binding: report.lifecycle.bindings["network.fetch"],
  state: report.lifecycle.state,
  logicalClock: report.lifecycle.logicalClock,
  normalizedPost: report.output.normalizedPost,
  eventTypes: report.output.events.map((event) => event.type),
});

const left = summarize(rust);
const right = summarize(ts);

try {
  assert.deepStrictEqual(left, right);
} catch {
  console.error(`Implementation mismatch for ${lab}`);
  console.error(JSON.stringify({ rust: left, ts: right }, null, 2));
  process.exit(1);
}

console.log(`Implementation parity verified for ${lab}`);
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

if [[ $# -eq 0 ]]; then
  for lab in \
    lab1-cloud-golden-path \
    lab2-header-validation-fail-fast \
    lab3-adapter-binding-and-wrappers
  do
    run_single "$lab"
  done
else
  run_single "$1"
fi
