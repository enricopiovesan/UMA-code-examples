#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pushd "$ROOT_DIR" >/dev/null
./scripts/validate_graph_contracts.sh
./scripts/run_graph_demo.sh lab1-upload-only
./scripts/run_graph_demo.sh lab2-image-tagger
./scripts/run_graph_demo.sh lab3-indexer
./scripts/run_graph_demo.sh lab4-broken-compat
./scripts/run_graph_demo.sh lab5-fixed-compat
./scripts/graph_diff.sh lab1-upload-only lab2-image-tagger
./scripts/graph_diff.sh lab3-indexer lab4-broken-compat
./scripts/graph_diff.sh lab4-broken-compat lab5-fixed-compat
popd >/dev/null

echo "Chapter 8 smoke run completed successfully."
