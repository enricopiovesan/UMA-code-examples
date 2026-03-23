#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST_PATH="$ROOT_DIR/rust/Cargo.toml"
EXAMPLES_DIR="$ROOT_DIR/examples"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/list_labs.sh [--ids-only]" >&2
  exit 0
fi

if [[ "${1:-}" == "--ids-only" ]]; then
  cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- list
  exit 0
fi

while IFS= read -r lab; do
  scenario_path="$(find "$EXAMPLES_DIR" -mindepth 2 -maxdepth 2 -name scenario.json | while IFS= read -r path; do
    if python3 - "$path" "$lab" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
lab = sys.argv[2]
data = json.loads(path.read_text())
raise SystemExit(0 if data.get("id") == lab else 1)
PY
    then
      printf '%s\n' "$path"
      break
    fi
  done)"

  if [[ -z "$scenario_path" ]]; then
    printf '%s - %s\n' "$lab" "Missing scenario metadata"
    printf '  %s\n' "Unable to locate scenario.json for this lab."
    continue
  fi

  title="$(python3 - "$scenario_path" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
print(data["title"])
PY
)"
  summary="$(python3 - "$scenario_path" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
print(data["summary"])
PY
)"
  printf '%s - %s\n' "$lab" "$title"
  printf '  %s\n' "$summary"
done < <(cargo run --locked --quiet --manifest-path "$MANIFEST_PATH" -- list)
