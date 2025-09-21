// Browser adapter for the feature flag evaluator.
//
// This module exports a single asynchronous function `evaluateFlag` that
// accepts an object matching the evaluator’s input contract and returns
// the evaluation result.  It works by instantiating the compiled
// WebAssembly module (`ff_eval_wasi_app.wasm`) with a minimal WASI
// polyfill.  The polyfill implements only the system calls needed by
// this example: reading all input from standard input, writing all
// output to standard output, generating timestamps and random data,
// providing empty command‑line arguments and environment variables,
// and handling process exit.
//
// To use this adapter you must compile the evaluator for the
// `wasm32‑wasi` target (`cargo build --release --target wasm32-wasi -p
// ff_eval_wasi_app`) and copy the resulting
// `ff_eval_wasi_app.wasm` file into this directory.  When the
// browser loads `ff.js` it will fetch `ff_eval_wasi_app.wasm` from
// the same directory.
//
// The accompanying `index.html` demonstrates how to wire up a simple
// UI around this function.  See the README for details.

class WasmExit extends Error {
  constructor(code) {
    super(`WASM exited with code ${code}`);
    this.code = code;
  }
}

// Helper to create a minimal WASI import object.  The returned object
// implements only the syscalls that are needed for this program:
// fd_read (stdin), fd_write (stdout/stderr), random_get, clock_time_get,
// args_sizes_get/args_get, environ_sizes_get/environ_get, fd_close,
// fd_fdstat_get, fd_prestat_get, fd_prestat_dir_name and proc_exit.  All
// other calls throw an error.  The input to the evaluator is provided
// via stdinBytes and the output is collected into stdoutBytes.
function createWasiImports(memoryRef, stdinBytes, stdoutBytes) {
  // Pointers to the current memory, updated after instantiation.
  let memory = memoryRef;
  function updateMemory(mem) {
    memory = mem;
  }
  // TextDecoder for decoding UTF‑8 strings.
  const textDecoder = new TextDecoder('utf-8');
  return {
    wasi_snapshot_preview1: {
      fd_write: (fd, iovsPtr, iovsLen, nwrittenPtr) => {
        // Write from memory to stdout/stderr.  Only fd 1 (stdout) and 2
        // (stderr) are supported.  The iovs array is a list of
        // {pointer, length} pairs of 32‑bit values.
        if (fd !== 1 && fd !== 2) {
          // Unsupported file descriptor.
          return 52; // __wasi_errno_t::EBADF
        }
        const memU32 = new Uint32Array(memory.buffer);
        const memU8 = new Uint8Array(memory.buffer);
        let bytesWritten = 0;
        const start = iovsPtr >>> 2; // divide by 4 to get index in U32 array
        for (let i = 0; i < iovsLen; i++) {
          const offset = memU32[start + i * 2];
          const length = memU32[start + i * 2 + 1];
          const slice = memU8.slice(offset, offset + length);
          stdoutBytes.push(...slice);
          bytesWritten += length;
        }
        // Write the number of bytes written back to memory.
        memU32[nwrittenPtr >>> 2] = bytesWritten;
        return 0; // __wasi_errno_t::ESUCCESS
      },
      fd_read: (fd, iovsPtr, iovsLen, nreadPtr) => {
        // Read into memory from stdinBytes.  Only fd 0 (stdin) is supported.
        if (fd !== 0) {
          return 52; // EBADF
        }
        const memU32 = new Uint32Array(memory.buffer);
        const memU8 = new Uint8Array(memory.buffer);
        let bytesRead = 0;
        const start = iovsPtr >>> 2;
        for (let i = 0; i < iovsLen; i++) {
          const offset = memU32[start + i * 2];
          const length = memU32[start + i * 2 + 1];
          const chunk = stdinBytes.slice(0, length);
          memU8.set(chunk, offset);
          stdinBytes = stdinBytes.slice(chunk.length);
          bytesRead += chunk.length;
          if (stdinBytes.length === 0) {
            break;
          }
        }
        memU32[nreadPtr >>> 2] = bytesRead;
        return 0;
      },
      fd_close: () => 0,
      fd_fdstat_get: () => 0,
      fd_prestat_get: () => 0,
      fd_prestat_dir_name: () => 0,
      args_sizes_get: (argcPtr, argvBufSizePtr) => {
        const memU32 = new Uint32Array(memory.buffer);
        memU32[argcPtr >>> 2] = 0;
        memU32[argvBufSizePtr >>> 2] = 0;
        return 0;
      },
      args_get: () => 0,
      environ_sizes_get: (envCountPtr, envBufSizePtr) => {
        const memU32 = new Uint32Array(memory.buffer);
        memU32[envCountPtr >>> 2] = 0;
        memU32[envBufSizePtr >>> 2] = 0;
        return 0;
      },
      environ_get: () => 0,
      random_get: (bufPtr, bufLen) => {
        const memU8 = new Uint8Array(memory.buffer);
        // Fill with zeros.  Use crypto.getRandomValues if available.
        const view = memU8.subarray(bufPtr, bufPtr + bufLen);
        if (typeof crypto !== 'undefined' && typeof crypto.getRandomValues === 'function') {
          crypto.getRandomValues(view);
        } else {
          // fallback: deterministic pseudo‑random based on Math.random
          for (let i = 0; i < bufLen; i++) {
            view[i] = Math.floor(Math.random() * 256);
          }
        }
        return 0;
      },
      clock_time_get: (_id, _precision, timePtr) => {
        // Write the current time in nanoseconds as a 64‑bit little‑endian
        // integer.  We multiply milliseconds by 1_000_000 to convert to
        // nanoseconds.
        const nowNs = BigInt(Date.now()) * 1000000n;
        const memView = new DataView(memory.buffer);
        memView.setBigUint64(timePtr, nowNs, true);
        return 0;
      },
      proc_exit: (code) => {
        // Throw an exception to unwind the stack.  The caller catches
        // WasmExit to ignore a normal exit.
        throw new WasmExit(code);
      },
    },
    // Proxy handler to trap unimplemented calls and surface helpful errors.
    get: (target, prop) => {
      if (prop in target) {
        return target[prop];
      }
      // If an unknown import is requested, throw to aid debugging.
      throw new Error(`WASI function not implemented: ${prop.toString()}`);
    },
  };
}

/**
 * Evaluate a feature flag using the WASI‑compiled WebAssembly module.
 *
 * This function accepts an input object conforming to the evaluator
 * contract and returns a Promise that resolves to the evaluation
 * result.  It fetches the compiled module (`ff_eval_wasi_app.wasm`)
 * relative to the current script’s directory, instantiates it with
 * a minimal WASI polyfill, writes the JSON input to stdin, executes
 * the module and reads the JSON output from stdout.
 *
 * @param {Object} input The feature flag and context to evaluate.
 * @returns {Promise<Object>} A Promise that resolves to the output JSON.
 */
export async function evaluateFlag(input) {
  // Serialize the input to a UTF‑8 encoded Uint8Array.
  const inputStr = JSON.stringify(input);
  let stdinBytes = new TextEncoder().encode(inputStr);
  const stdoutBytes = [];
  // Placeholder for the WebAssembly.Memory instance; will be set after
  // instantiation.  The wasiImports object captures a reference to
  // this variable and updates it when the module is instantiated.
  let memory;
  // Create the import object with stub WASI functions.  The
  // `memory` reference is updated later.
  const wasiImports = {
    wasi_snapshot_preview1: {},
  };
  // Generate the actual imports, capturing our stdin/stdout buffers.
  const imports = createWasiImports({
    get buffer() {
      return memory.buffer;
    },
    get bufferView() {
      return new Uint8Array(memory.buffer);
    },
  }, stdinBytes, stdoutBytes);
  // Fetch and compile the WebAssembly module.  The file must reside
  // alongside this script.
  const response = await fetch('ff_eval_wasi_app.wasm');
  if (!response.ok) {
    throw new Error(`Failed to fetch ff_eval_wasi_app.wasm: ${response.status} ${response.statusText}`);
  }
  const wasmBytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(wasmBytes, imports);
  // Save the memory after instantiation.
  memory = instance.exports.memory;
  // Execute the WebAssembly module.  The _start function runs
  // the evaluator.  Catch a WasmExit to handle normal process exit.
  try {
    instance.exports._start();
  } catch (err) {
    if (err instanceof WasmExit) {
      // Ignore normal exit.
    } else {
      throw err;
    }
  }
  // Decode the collected stdout bytes into a string and parse as JSON.
  const outputStr = new TextDecoder('utf-8').decode(new Uint8Array(stdoutBytes));
  return JSON.parse(outputStr);
}