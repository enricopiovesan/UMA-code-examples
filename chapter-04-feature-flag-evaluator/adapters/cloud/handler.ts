// Illustrative cloud adapter for the feature flag evaluator.
//
// This file shows one possible serverless-style host wrapper around the evaluator.
// It is not part of the validated default reader path and should be treated as example host integration code.
//
// This handler is designed for Node.js serverless environments (for example
// AWS Lambda).  It spawns the `wasmtime` binary installed on the system to
// run the compiled WASI module.  The compiled evaluator must be present at
// `target/wasm32-wasip1/release/ff_eval_wasi_app.wasm` relative to the
// project root.  The handler reads the JSON body of the incoming event,
// writes it to the evaluator’s stdin and returns the JSON output.

import { closeSync, openSync } from 'fs';
import { mkdtemp, readFile, rm, writeFile } from 'fs/promises';
import { tmpdir } from 'os';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { WASI } from 'wasi';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const wasmPath = join(__dirname, '..', '..', 'target', 'wasm32-wasip1', 'release', 'ff_eval_wasi_app.wasm');

async function runEvaluator(input: any): Promise<string> {
  const wasmBinary = await readFile(wasmPath);
  const tmpDir = await mkdtemp(join(tmpdir(), 'ff-eval-cloud-'));
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
