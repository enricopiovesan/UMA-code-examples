#!/usr/bin/env bash
set -euo pipefail

cargo run --offline --quiet --manifest-path rust/Cargo.toml -- list
