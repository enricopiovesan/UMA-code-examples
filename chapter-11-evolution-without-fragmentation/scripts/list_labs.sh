#!/usr/bin/env bash
set -euo pipefail

cargo run --locked --quiet --manifest-path rust/Cargo.toml -- list
