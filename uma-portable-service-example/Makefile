
        .PHONY: build test wasm run-native run-wasm fmt lint

        build:
	cd runtime && cargo build --all && cargo build -p runner_wasm --target wasm32-wasi

        test:
	cd runtime && cargo test --all

        wasm:
	cd runtime && rustup target add wasm32-wasi && cargo build -p runner_wasm --target wasm32-wasi

        run-native:
	cd runtime && cargo run -p runner_native -- ../sample-data/sample.pgm

        run-wasm:
	wasmtime runtime/target/wasm32-wasi/debug/runner_wasm.wasm -- sample-data/sample.pgm

        fmt:
	cd runtime && cargo fmt --all

        lint:
	cd runtime && cargo clippy --all-targets -- -D warnings
