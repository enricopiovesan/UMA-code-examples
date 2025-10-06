## Makefile for building and running the UMA post fetcher example.

.PHONY: all build wasm clean test

all: build

# Build the Rust workspace
build:
	cargo build --workspace --release

# Build the WebAssembly module for the runtime
wasm:
	rustup target add wasm32-wasi || true
	cargo build -p uma_runtime --release --target wasm32-wasi

# Clean build artifacts
clean:
	cargo clean

# Run integration tests (cloud host only)
test:
	bash tests/integration/run_cloud.sh