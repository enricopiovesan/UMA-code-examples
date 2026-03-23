#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODEL_DIR="$ROOT_DIR/models"
MANIFEST="$MODEL_DIR/manifest.json"

if ! command -v python3 >/dev/null 2>&1; then
  echo "Missing required command: python3" >&2
  exit 1
fi

if ! command -v curl >/dev/null 2>&1; then
  echo "Missing required command: curl" >&2
  exit 1
fi

if ! command -v shasum >/dev/null 2>&1; then
  echo "Missing required command: shasum" >&2
  exit 1
fi

if [[ ! -f "$MANIFEST" ]]; then
  echo "Missing model manifest: $MANIFEST" >&2
  exit 1
fi

mkdir -p "$MODEL_DIR"

python3 - "$MANIFEST" "$MODEL_DIR" <<'PY'
import json
import pathlib
import subprocess
import sys
import urllib.request

manifest_path = pathlib.Path(sys.argv[1])
model_dir = pathlib.Path(sys.argv[2])
manifest = json.loads(manifest_path.read_text())

model_id = manifest["modelId"]
revision = manifest["revision"]

for file_info in manifest["files"]:
    name = file_info["name"]
    rel_path = file_info["path"]
    expected = file_info["sha256"]
    target = model_dir / name
    url = f"https://huggingface.co/{model_id}/resolve/{revision}/{rel_path}"

    if target.exists():
        actual = subprocess.check_output(["shasum", "-a", "256", str(target)], text=True).split()[0]
        if actual == expected:
            print(f"Verified {name}")
            continue
        print(f"Checksum mismatch for {name}; re-downloading", file=sys.stderr)
        target.unlink()

    print(f"Downloading {name}")
    with urllib.request.urlopen(url) as response, target.open("wb") as handle:
        handle.write(response.read())

    actual = subprocess.check_output(["shasum", "-a", "256", str(target)], text=True).split()[0]
    if actual != expected:
        target.unlink(missing_ok=True)
        raise SystemExit(
            f"Checksum mismatch for {name}: expected {expected}, got {actual}"
        )

    print(f"Verified {name}")
PY

echo "Chapter 13 models are ready in $MODEL_DIR"
