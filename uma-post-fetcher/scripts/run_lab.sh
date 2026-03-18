#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'EOF'
Run one of the guided Chapter 5 reader labs.

Usage:
  ./scripts/run_lab.sh <lab-name>

Available labs:
  lab1-cloud-golden-path
  lab2-header-validation-fail-fast
  lab3-adapter-binding-and-wrappers
EOF
}

run_header_lab() {
  local output
  output="$(
    cargo run --quiet -p uma_runtime <<'EOF'
{"request":{"url":"https://example.com","headers":{"x-foo":"bar"}},"runId":"reader-lab-2"}
EOF
  )"

  echo "$output"

  if ! printf '%s\n' "$output" | jq -e '.output.normalizedPost == null' >/dev/null; then
    echo "Expected header validation lab to skip normalization output." >&2
    exit 1
  fi

  if printf '%s\n' "$output" | jq -e '.output.events[] | select(.type=="fetch_request")' >/dev/null; then
    echo "Header validation lab should not emit fetch_request when validation fails." >&2
    exit 1
  fi

  if ! printf '%s\n' "$output" | jq -e '.output.events[] | select(.type=="error" and .data.error=="unexpected header x-foo")' >/dev/null; then
    echo "Expected an explicit header validation error event." >&2
    exit 1
  fi

  if ! printf '%s\n' "$output" | jq -e '.lifecycle.state == "failed"' >/dev/null; then
    echo "Expected lifecycle state to be failed after header validation." >&2
    exit 1
  fi
}

run_wrapper_lab() {
  local output
  output="$(UMA_ENABLE_RETRY=1 UMA_ENABLE_CACHE=1 bash "$ROOT_DIR/hosts/cloud/run.sh")"

  echo "$output"

  if ! printf '%s\n' "$output" | sed -n '/^{/,$p' | jq -e '.lifecycle.bindings["network.fetch"].impl == "cache-retry-host-fetch"' >/dev/null; then
    echo "Expected lifecycle binding to record cache-retry-host-fetch." >&2
    exit 1
  fi
}

run_lab() {
  case "$1" in
    lab1-cloud-golden-path)
      bash "$ROOT_DIR/tests/integration/run_cloud.sh"
      ;;
    lab2-header-validation-fail-fast)
      run_header_lab
      ;;
    lab3-adapter-binding-and-wrappers)
      run_wrapper_lab
      ;;
    *)
      echo "Unknown lab: $1" >&2
      echo >&2
      usage >&2
      exit 1
      ;;
  esac
}

if [[ "${1:-}" == "--help" || $# -eq 0 ]]; then
  usage
  [[ $# -eq 0 ]] && exit 1
  exit 0
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "Missing required command: jq" >&2
  exit 1
fi

pushd "$ROOT_DIR" >/dev/null
run_lab "$1"
popd >/dev/null
