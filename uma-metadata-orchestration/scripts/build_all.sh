#!/usr/bin/env bash
set -euo pipefail
echo "Building image.tagger (WASI)"
pushd services/image.tagger >/dev/null
  ./build.sh
popd >/dev/null

echo "Building edge.cache (WASI)"
pushd services/edge.cache >/dev/null
  ./build.sh
popd >/dev/null

echo "Building telemetry.logger (TypeScript)"
pushd services/telemetry.logger >/dev/null
  npm install
  npm run build
popd >/dev/null

echo "All builds completed"


echo "Building ai.model.evaluator (TypeScript)"
pushd services/ai.model.evaluator >/dev/null
  npm install
  npm run build
popd >/dev/null
