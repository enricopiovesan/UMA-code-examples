// Edge host runner.  This script can be executed with Deno or Node (with
// appropriate flags) to run the UMA post fetcher WebAssembly module.  It
// assumes that the wasm module has been compiled using wasm-bindgen to
// generate a JS wrapper exposing a `run_json` function.

// Adjust the import path depending on your build pipeline.  For example,
// using wasm-pack with the --target deno flag will generate a `.ts` file
// directly consumable by Deno.
import init, { run_json } from '../target/wasm32-wasi/release/uma_runtime.js';

async function main() {
  await init();
  const input = {
    request: {
      url: 'https://jsonplaceholder.typicode.com/posts/1',
      headers: { accept: 'application/json' },
    },
    runId: 'demo-001',
  };
  const [outputJson, metadataJson] = run_json(JSON.stringify(input));
  console.log('Service output:', outputJson);
  console.log('Lifecycle record:', metadataJson);
}

main().catch((err) => console.error(err));