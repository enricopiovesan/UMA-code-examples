#!/usr/bin/env bash
set -euo pipefail

LAB="${1:-}"
if [[ -n "$LAB" ]]; then
  cargo run --offline --quiet --manifest-path rust/Cargo.toml -- validate "$LAB"
else
  cargo run --offline --quiet --manifest-path rust/Cargo.toml -- validate
fi
