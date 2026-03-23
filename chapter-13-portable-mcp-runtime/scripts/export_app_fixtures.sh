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
  fixture_path="$FIXTURE_DIR/$scenario.json"
  cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- render "$scenario" json > "$fixture_path"
  summary="$(python3 - "$fixture_path" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
print(json.dumps({
    "id": data["scenario"],
    "title": data["title"],
    "summary": data["summary"],
    "status": data["status"],
}))
PY
)"
  if [[ "$first" -eq 0 ]]; then
    printf ',\n' >> "$tmp_index"
  fi
  printf '%s' "$summary" >> "$tmp_index"
  first=0
done < <(cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- list)
printf '\n]\n' >> "$tmp_index"

mv "$tmp_index" "$FIXTURE_DIR/index.json"
echo "Exported Chapter 13 app fixtures to app/fixtures/"
