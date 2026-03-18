#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_DIR="$(mktemp -d)"
ARCHIVE_PATH="$TMP_DIR/repo.tar"
CHECKOUT_DIR="$TMP_DIR/checkout"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd git
require_cmd tar

mkdir -p "$CHECKOUT_DIR"

git -C "$ROOT_DIR" archive --format=tar HEAD -o "$ARCHIVE_PATH"
tar -xf "$ARCHIVE_PATH" -C "$CHECKOUT_DIR"

# Reuse a local wasmtime install if the machine does not provide one globally.
if ! command -v wasmtime >/dev/null 2>&1; then
  for candidate in "$ROOT_DIR"/.bin/wasmtime-*; do
    if [[ -x "$candidate/wasmtime" ]]; then
      export PATH="$candidate:$PATH"
      break
    fi
    if [[ -x "$candidate/bin/wasmtime" ]]; then
      export PATH="$candidate/bin:$PATH"
      break
    fi
  done
fi

echo "Running reader docs check from clean checkout: $CHECKOUT_DIR"
(cd "$CHECKOUT_DIR" && ./scripts/check_reader_docs.sh)

echo "Running reader smoke from clean checkout: $CHECKOUT_DIR"
(cd "$CHECKOUT_DIR" && ./scripts/smoke_reader_paths.sh)

echo "Fresh reader checkout simulation passed."
