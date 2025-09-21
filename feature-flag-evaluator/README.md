# Feature Flag Evaluator (ff‑eval)

This repository contains a minimal yet complete implementation of a deterministic feature flag evaluator.  The evaluator can be compiled to `wasm32‑wasi` and reused from the browser, edge workers, or cloud handlers via thin adapters.  The goal of this example is to show how to build a portable flag evaluation engine that takes JSON in and returns JSON out with no external dependencies in the core logic.

## Contract (v1)

The evaluator accepts an **input JSON** document with a flag definition and a context.  It produces an **output JSON** document describing whether the flag is enabled and which rule matched.

### Input

```json
{
  "flag": {
    "key": "paywall",
    "rules": [
      { "if": "country == 'CA'", "then": true },
      { "if": "rollout(0.20)", "then": true }
    ],
    "default": false
  },
  "context": {
    "userId": "u123",
    "country": "CA",
    "appVersion": "1.4.2"
  }
}
```

### Output

```json
{
  "key": "paywall",
  "enabled": true,
  "matchedRule": 0
}
```

### Rule language

* Supported operators: `==`, `!=`, `<`, `<=`, `>`, `>=`, `in`, `&&`, `||`.
* Identifiers resolve from the context (e.g. `country` resolves to `context.country`).
* Literals may be strings, numbers or booleans.
* A built‑in function `rollout(p)` (0 ≤ p ≤ 1) performs a deterministic hash of `flag.key` and `context.userId` and returns `true` if the resulting value is less than `p`.
* Evaluation is first‑match wins; if no rule matches, the evaluator returns the flag’s `default` value.

### Deterministic rollout

Rollouts are sticky: the same `flag.key` and `userId` will always produce the same bucket.

1. Concatenate `flag.key + ":" + context.userId`.
2. Hash the result using a small, deterministic 32‑bit FNV‑1a hash.
3. Divide the hash by `2^32` to obtain a value in the range [0, 1).
4. `rollout(p)` returns `true` if the value is strictly less than `p`.

## Repository structure

```
ff-eval/
  README.md              – this file
  contracts/
    input.schema.json    – minimal JSON Schema for the evaluator input
    output.schema.json   – minimal JSON Schema for the evaluator output
  core/
    Cargo.toml           – core library crate definition
    src/
      lib.rs             – pure evaluation logic
  wasi-app/
    Cargo.toml           – binary crate for the WASI executable
    src/
      main.rs            – reads JSON from stdin, calls the core, writes JSON to stdout
  adapters/
    browser/
      index.html         – example HTML page that illustrates the contract
      ff.js              – minimal browser adapter using a custom WASI polyfill
    edge/
      worker.ts          – Node‑based worker that runs the evaluator using Node’s built‑in WASI
    cloud/
      handler.ts         – Node‑based serverless handler invoking the evaluator using Node’s built‑in WASI
  tests/
    vectors/
      t1.json            – test vector: country CA; matches rule 0
      t2.json            – test vector: country US; matches rollout rule 1
      t3.json            – test vector: no match; uses default
```

The `contracts/` directory contains illustrative JSON Schemas to document the input and output structure.  These schemas are intentionally simple and do not cover all possible edge cases.

## Building and running

This example is split into two Rust crates: a core library (`ff_eval_core`) and a WASI executable (`ff_eval_wasi_app`).  The core contains all evaluation logic and has no dependencies beyond the standard library.  The WASI executable uses `serde` and `serde_json` to parse the input and serialize the output.

### Requirements

You will need a working Rust toolchain and (optionally) a Node.js runtime if you wish to run the provided adapters.  To execute the compiled WebAssembly module outside of Node you can install a dedicated WASI runtime such as Wasmtime or Wasmer.

* **Rust toolchain:** Install Rust via [rustup](https://rustup.rs/).  On macOS, Linux and Windows this single installer sets up `cargo`, `rustc` and related tools.  After installation open a new shell and verify with `cargo --version`.
* **Node.js (optional):** The edge and cloud adapters are written for Node.js and use Node’s built‑in [`wasi`](https://nodejs.org/api/wasi.html) module to execute the evaluator.  Install Node.js from [nodejs.org](https://nodejs.org/) or via your system’s package manager and verify installation with `node --version`.  Node 16 or later is recommended.
* **WASI runtime (optional):** To run the compiled WebAssembly module outside of Node—for example from a shell—you can install [Wasmtime](https://github.com/bytecodealliance/wasmtime) or [Wasmer](https://github.com/wasmerio/wasmer).  On macOS you can install Wasmtime via `brew install wasmtime`; on Linux download a release from the project’s GitHub page and extract it somewhere on your `PATH`; on Windows download the Wasmtime zip archive and add the extracted directory to your `PATH`.  Verify installation with `wasmtime --version`.

### Environment setup

Once you have Rust installed you need to add the `wasm32-wasi` compilation target and install a WASI runtime.  The exact steps differ slightly between operating systems.

**macOS**

1. Add the WASI target:

   ```sh
   rustup target add wasm32-wasi
   ```

2. (Optional) Install a standalone WASI runtime via Homebrew if you wish to run the `.wasm` outside of Node:

   ```sh
   brew install wasmtime
   ```

3. Install Node.js if you plan to use the Node adapters:

   ```sh
   brew install node
   ```

4. Verify that `node --version` succeeds.  If you installed Wasmtime, verify `wasmtime --version` as well.

**Linux**

1. Add the WASI compilation target:

   ```sh
   rustup target add wasm32-wasi
   ```

2. (Optional) Download a [Wasmtime release](https://github.com/bytecodealliance/wasmtime/releases) for Linux, extract the archive into a directory (for example `~/bin/wasmtime`) and ensure that directory is in your `PATH`.  You can also install Wasmtime via your package manager if available.

3. Install Node.js via your package manager.  For example on Ubuntu: `sudo apt-get install nodejs`.

4. Verify `node --version`.  If you installed Wasmtime, verify `wasmtime --version`.

**Windows**

1. Add the WASI compilation target in PowerShell:

   ```powershell
   rustup target add wasm32-wasi
   ```

2. (Optional) Download the Windows Wasmtime zip from the [Wasmtime releases page](https://github.com/bytecodealliance/wasmtime/releases).  Extract it to a directory (for example `C:\wasmtime`) and add this directory to your `PATH` if you plan to run the `.wasm` outside of Node.

3. Install Node.js via the installer from [nodejs.org](https://nodejs.org/) if you plan to run the Node adapters.

4. Verify that `node --version` works from a new terminal.  If you installed Wasmtime, verify `wasmtime --version`.

After completing the above steps you can build the project and run the examples as described below.

To build the WASI binary:

```sh
rustup target add wasm32-wasi
cargo build --release --target wasm32-wasi -p ff_eval_wasi_app
```

The compiled WebAssembly module will be written to
`target/wasm32-wasi/release/ff_eval_wasi_app.wasm`.

### Running unit tests

The core library includes a suite of unit tests that exercise equality, rollout, membership, numeric comparisons and logical operators.  To run them use:

```sh
cargo test -p ff_eval_core
```

This command will compile the library and run all tests.  You should see output indicating that all tests have passed.

### Executing test vectors

Three JSON files under `tests/vectors/` demonstrate typical inputs and expected outcomes.  A convenience script is provided under `scripts/run_vectors.sh` to pipe each vector into the evaluator via `wasmtime`.

```sh
./scripts/run_vectors.sh
```

Ensure you have built the WASI module first (`cargo build --release --target wasm32-wasi -p ff_eval_wasi_app`) and that `wasmtime` is installed and on your `PATH`.

### Running locally with wasmtime or wasmer

Assuming you have [wasmtime](https://github.com/bytecodealliance/wasmtime) or [wasmer](https://github.com/wasmerio/wasmer) installed, you can run the evaluator on a JSON file.  For example:

```sh
echo '{"flag":{"key":"paywall","rules":[{"if":"country == \'CA\'","then":true},{"if":"rollout(0.20)","then":true}],"default":false},"context":{"userId":"u123","country":"CA"}}' \
| wasmtime target/wasm32-wasi/release/ff_eval_wasi_app.wasm
```

This should print `{"key":"paywall","enabled":true,"matchedRule":0}`.  If the input cannot be parsed, the process exits with status 1.

### Browser and other environments
Running a WASI module in a browser requires a JavaScript polyfill that implements the WASI system interface.  This repository now includes a minimal browser adapter at `adapters/browser/ff.js` and a sample HTML page at `adapters/browser/index.html`.  The adapter defines an `evaluateFlag` function that:

1. Fetches the compiled WebAssembly module (`ff_eval_wasi_app.wasm`) relative to the HTML page.
2. Provides a minimal WASI implementation in JavaScript to supply stdin, stdout, environment variables and clocks.
3. Writes the input JSON to stdin, invokes the module’s `_start` function and collects the JSON output from stdout.

To run the browser demo:

1. Build the WASI module:

   ```sh
   cargo build --release --target wasm32-wasi -p ff_eval_wasi_app
   ```

2. Copy the compiled module into the `adapters/browser` directory (for example using `cp target/wasm32-wasi/release/ff_eval_wasi_app.wasm ff-eval/adapters/browser/`).

3. Open `adapters/browser/index.html` in a web server that supports ES modules (for example `python3 -m http.server`) and click “Evaluate” to run the evaluator in your browser.

Two Node‑based adapters are provided:

* **Edge worker (`adapters/edge/worker.ts`)**: This file exports a `fetch` function suitable for use in a Cloudflare Worker or similar environment.  It uses Node’s built‑in `wasi` module to instantiate and run the compiled WebAssembly module in memory.  The worker reads the incoming request body as JSON, writes it to the evaluator’s stdin and returns the evaluator’s stdout as the response.  Because it uses the standard `wasi` API there is no dependency on an external runtime like `wasmtime`.

* **Cloud handler (`adapters/cloud/handler.ts`)**: This file defines an AWS Lambda–style handler that uses Node’s `wasi` API.  Like the edge worker it loads the compiled module into memory, writes the JSON input to stdin and returns the JSON output.  It expects the compiled `.wasm` file to reside in `target/wasm32-wasi/release/ff_eval_wasi_app.wasm`.  If the input is invalid JSON or the module fails, it returns a 400 or 500 status accordingly.

For environments that support Deno or other runtimes, you can adapt these examples by replacing the Node‑specific APIs with appropriate equivalents and ensuring that a WASI implementation (either built‑in or via a polyfill) is available.

## Extending the evaluator

The provided implementation is intentionally small.  For a more complete feature set, you might consider adding:

* Additional operators (`!=`, `<`, `>`, `in`, `&&`, `||`).
* A hand‑written parser or combinator parser to build an AST rather than using ad‑hoc string matching.
* Weighted variants and multiple buckets.
* Time windows and scheduling of flags.
* Segments loaded by the adapter rather than bundled into the flag.
* Remote configuration fetched by the adapter.
* Audit logging in the cloud adapter (never inside the WASM module).

These are left as exercises to the reader and future work.

This repository now includes support for inequality and numeric comparison operators, the `in` operator and logical `&&` and `||` operators.  The implementation lives in `core/src/lib.rs`, and there are unit tests demonstrating how these operators work.  To extend the evaluator further—for example to support arrays in the context, nested parentheses or more complex operators—you can modify `eval_expr`, `parse_term_as_value` and the helper functions defined at the bottom of the core module.  Remember to add corresponding tests.
