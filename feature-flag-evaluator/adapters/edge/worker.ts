// Edge adapter for the feature flag evaluator.
//
// This implementation spawns the `wasmtime` binary installed on the system
// to run the compiled WASI module.  It assumes that `wasmtime` is on
// the environment `PATH` and that `ff_eval_wasi_app.wasm` has been built
// into `target/wasm32-wasi/release` relative to the project root.  The
// adapter reads the JSON body of the incoming request, writes it to
// the evaluator’s stdin and returns the JSON output.

import { readFile } from 'fs/promises';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { Readable, Writable } from 'stream';
import { WASI } from 'wasi';

// Compute the path to the wasm file relative to this script.  When compiled
// to JavaScript, __dirname resolves to the adapters/edge directory.
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const wasmPath = join(__dirname, '..', '..', 'target', 'wasm32-wasi', 'release', 'ff_eval_wasi_app.wasm');

/**
 * Run the evaluator using Node’s built‑in WASI API.  This function
 * loads the compiled WebAssembly module into memory, feeds the JSON
 * input via a Readable stream, collects all output written to
 * stdout and resolves with the resulting string.  Any stderr
 * output is ignored.  Errors thrown by the evaluator are propagated.
 */
async function runEvaluator(input: any): Promise<string> {
  // Load the WebAssembly binary.
  const wasmBinary = await readFile(wasmPath);
  // Prepare the input stream.
  const inputStr = JSON.stringify(input);
  const stdin = new Readable({
    read() {
      this.push(Buffer.from(inputStr, 'utf8'));
      this.push(null);
    },
  });
  // Collect stdout.
  let output = '';
  const stdout = new Writable({
    write(chunk, _enc, cb) {
      output += chunk.toString();
      cb();
    },
  });
  // Discard stderr.
  const stderr = new Writable({
    write(_chunk, _enc, cb) {
      cb();
    },
  });
  // Instantiate WASI with our custom streams.
  const wasi = new WASI({ args: [], env: {}, stdin, stdout, stderr });
  const module = await WebAssembly.compile(wasmBinary);
  const instance = await WebAssembly.instantiate(module, {
    wasi_snapshot_preview1: wasi.wasiImport,
  });
  // Run the module.  The return value is the exit code.
  wasi.start(instance);
  return output.trim();
}

export default {
  async fetch(request: Request): Promise<Response> {
    try {
      const input = await request.json();
      const outputString = await runEvaluator(input);
      return new Response(outputString, {
        headers: { 'content-type': 'application/json' },
      });
    } catch (err) {
      // If parsing fails or the evaluator fails, return an error response.
      return new Response('Bad Request', { status: 400 });
    }
  },
};