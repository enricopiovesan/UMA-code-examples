#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "Usage: ./scripts/run_cloud.sh" >&2
  echo "Runs the validated Rust cloud runner. Set POLICY_FAIL_MODE=closed to stop on policy violations." >&2
  exit 0
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust and cargo are required"
  exit 1
fi
if ! command -v wasmtime >/dev/null 2>&1; then
  os="$(uname -s | tr '[:upper:]' '[:lower:]')"
  arch="$(uname -m)"
  case "$arch" in
    arm64) arch="aarch64" ;;
    x86_64) arch="x86_64" ;;
  esac
  case "$os" in
    darwin) os="macos" ;;
    linux) os="linux" ;;
  esac
  for base in ".bin" "../.bin"; do
    found=$(find "$base" -maxdepth 2 -type f -path "*${arch}-${os}*/wasmtime" 2>/dev/null | head -n 1 || true)
    if [ -n "${found:-}" ]; then
      PATH="$(dirname "$found"):$PATH"
      export PATH
      break
    fi
  done
fi
if ! command -v wasmtime >/dev/null 2>&1; then
  echo "Wasmtime is required to execute WASI modules"
  exit 1
fi
if [ ! -f services/image.tagger/target/wasm32-wasip1/release/image_tagger.wasm ] || \
   [ ! -f services/edge.cache/target/wasm32-wasip1/release/edge_cache.wasm ]; then
  echo "Built WASI artifacts are missing."
  echo "Run ./scripts/build_all.sh first to compile the services."
  exit 1
fi
if [ ! -f runtime-rust/Cargo.toml ]; then
  echo "Rust cloud runner is missing."
  exit 1
fi
# The quick-start path is fail-open so readers can see the full orchestration flow.
POLICY_FAIL_MODE="${POLICY_FAIL_MODE:-open}" cargo run --locked --quiet --manifest-path runtime-rust/Cargo.toml
