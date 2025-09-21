// Cloud adapter for the feature flag evaluator.
//
// This handler is designed for Node.js serverless environments (for example
// AWS Lambda).  It spawns the `wasmtime` binary installed on the system to
// run the compiled WASI module.  The compiled evaluator must be present at
// `target/wasm32-wasi/release/ff_eval_wasi_app.wasm` relative to the
// project root.  The handler reads the JSON body of the incoming event,
// writes it to the evaluator’s stdin and returns the JSON output.

import { readFile } from 'fs/promises';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { Readable, Writable } from 'stream';
import { WASI } from 'wasi';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const wasmPath = join(__dirname, '..', '..', 'target', 'wasm32-wasi', 'release', 'ff_eval_wasi_app.wasm');

async function runEvaluator(input: any): Promise<string> {
  // Read the compiled WebAssembly binary.
  const wasmBinary = await readFile(wasmPath);
  // Prepare stdin as a stream containing the JSON input.
  const inputStr = JSON.stringify(input);
  const stdin = new Readable({
    read() {
      this.push(Buffer.from(inputStr, 'utf8'));
      this.push(null);
    },
  });
  // Capture stdout.
  let output = '';
  const stdout = new Writable({
    write(chunk, _enc, cb) {
      output += chunk.toString();
      cb();
    },
  });
  // Ignore stderr.
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
  wasi.start(instance);
  return output.trim();
}

export async function handler(event: any) {
  try {
    const input = JSON.parse(event.body);
    const output = await runEvaluator(input);
    return {
      statusCode: 200,
      headers: { 'content-type': 'application/json' },
      body: output,
    };
  } catch (err) {
    return {
      statusCode: 400,
      body: 'Bad Request',
    };
  }
}