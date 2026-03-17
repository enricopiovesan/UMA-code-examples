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

echo "Building cloud runner (Rust)"
pushd runtime-rust >/dev/null
  cargo build --offline
popd >/dev/null

if [ "${BUILD_OPTIONAL_JS:-0}" = "1" ] && command -v npm >/dev/null 2>&1; then
  echo "Installing optional JavaScript dependencies"
  npm install

  echo "Building telemetry.logger (TypeScript, optional harness support)"
  pushd services/telemetry.logger >/dev/null
    npm install
    npm run build
  popd >/dev/null

  echo "Building ai.model.evaluator (TypeScript, optional harness support)"
  pushd services/ai.model.evaluator >/dev/null
    npm install
    npm run build
  popd >/dev/null
else
  echo "Skipping optional JavaScript builds. Set BUILD_OPTIONAL_JS=1 to build browser and edge helper artifacts."
fi

echo "All builds completed"
