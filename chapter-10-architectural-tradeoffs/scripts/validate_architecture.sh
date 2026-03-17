#!/usr/bin/env bash
set -euo pipefail

LAB="${1:-}"
if [[ -n "$LAB" ]]; then
  cargo run --locked --quiet --manifest-path rust/Cargo.toml -- validate "$LAB"
else
  cargo run --locked --quiet --manifest-path rust/Cargo.toml -- validate
fi
