#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FIXTURE_DIR="$ROOT_DIR/app/fixtures"
MANIFEST_PATH="$ROOT_DIR/rust/Cargo.toml"

if ! command -v python3 >/dev/null 2>&1; then
  echo "Missing required command: python3" >&2
  exit 1
fi

mkdir -p "$FIXTURE_DIR"

tmp_index="$(mktemp)"
printf '[\n' > "$tmp_index"

first=1
while IFS= read -r scenario; do
  cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- render "$scenario" json > "$FIXTURE_DIR/$scenario.json"
  summary="$(cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- render "$scenario" json | python3 -c 'import json,sys; data=json.load(sys.stdin); print(json.dumps({"id": data["scenario"], "title": data["title"], "summary": data["summary"], "status": data["status"]}))')"
  if [[ "$first" -eq 0 ]]; then
    printf ',\n' >> "$tmp_index"
  fi
  printf '%s' "$summary" >> "$tmp_index"
  first=0
done < <(cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- list)
printf '\n]\n' >> "$tmp_index"

mv "$tmp_index" "$FIXTURE_DIR/index.json"
echo "Exported Chapter 13 app fixtures to app/fixtures/"
