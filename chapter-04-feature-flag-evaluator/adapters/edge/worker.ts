// Illustrative edge adapter for the feature flag evaluator.
//
// This file demonstrates one possible worker-style host wrapper around the evaluator.
// It is not part of the validated default reader path and should be treated as example host integration code.
//
// This implementation spawns the `wasmtime` binary installed on the system
// to run the compiled WASI module.  It assumes that `wasmtime` is on
// the environment `PATH` and that `ff_eval_wasi_app.wasm` has been built
// into `target/wasm32-wasip1/release` relative to the project root.  The
// adapter reads the JSON body of the incoming request, writes it to
// the evaluator’s stdin and returns the JSON output.

import { closeSync, openSync } from 'fs';
import { mkdtemp, readFile, rm, writeFile } from 'fs/promises';
import { tmpdir } from 'os';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { WASI } from 'wasi';

// Compute the path to the wasm file relative to this script.  When compiled
// to JavaScript, __dirname resolves to the adapters/edge directory.
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const wasmPath = join(__dirname, '..', '..', 'target', 'wasm32-wasip1', 'release', 'ff_eval_wasi_app.wasm');

/**
 * Run the evaluator using Node’s built‑in WASI API.  This function
 * loads the compiled WebAssembly module into memory, feeds the JSON
 * input via a Readable stream, collects all output written to
 * stdout and resolves with the resulting string.  Any stderr
 * output is ignored.  Errors thrown by the evaluator are propagated.
 */
async function runEvaluator(input: any): Promise<string> {
  const wasmBinary = await readFile(wasmPath);
  const tmpDir = await mkdtemp(join(tmpdir(), 'ff-eval-edge-'));
  const stdinPath = join(tmpDir, 'stdin.json');
  const stdoutPath = join(tmpDir, 'stdout.json');
  const stderrPath = join(tmpDir, 'stderr.log');
  let stdinFd: number | undefined;
  let stdoutFd: number | undefined;
  let stderrFd: number | undefined;

  try {
    await writeFile(stdinPath, JSON.stringify(input), 'utf8');
    await writeFile(stdoutPath, '', 'utf8');
    await writeFile(stderrPath, '', 'utf8');

    stdinFd = openSync(stdinPath, 'r');
    stdoutFd = openSync(stdoutPath, 'w');
    stderrFd = openSync(stderrPath, 'w');

    const wasi = new WASI({
      version: 'preview1',
      args: [],
      env: {},
      stdin: stdinFd,
      stdout: stdoutFd,
      stderr: stderrFd,
    });
    const module = await WebAssembly.compile(wasmBinary);
    const instance = await WebAssembly.instantiate(module, {
      wasi_snapshot_preview1: wasi.wasiImport,
    });
    wasi.start(instance);
    return (await readFile(stdoutPath, 'utf8')).trim();
  } finally {
    if (stdinFd !== undefined) closeSync(stdinFd);
    if (stdoutFd !== undefined) closeSync(stdoutFd);
    if (stderrFd !== undefined) closeSync(stderrFd);
    await rm(tmpDir, { recursive: true, force: true });
  }
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
