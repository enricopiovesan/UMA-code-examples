// Browser host shim for the UMA post fetcher.  This script illustrates how to
// load the compiled WebAssembly module and bind the browser's `fetch`
// function to the UMA `network.fetch` capability.  It is intentionally
// minimal and does not handle all edge cases.

// NOTE: At build time the Rust `uma_runtime` crate must be compiled to a
// WebAssembly module using `wasm-pack` or a similar tool.  The generated
// JS glue code should expose a `run_json` function matching the signature
// defined in `runtime/src/lib.rs`.

async function main() {
  // Dynamically import the WASM module.  Adjust the path to the generated
  // package as necessary (e.g. '../pkg/uma_runtime.js').
  const wasm = await import('../pkg/uma_runtime.js');
  // Construct the input document.
  const input = {
    request: {
      url: 'https://jsonplaceholder.typicode.com/posts/1',
      headers: { accept: 'application/json' },
    },
    runId: 'demo-001',
  };
  // Provide a network adapter that delegates to the browser's fetch.
  const adapter = {
    fetch: async (url: string, headers: { [key: string]: string }) => {
      const response = await fetch(url, { headers });
      const status = response.status;
      const respHeaders: { [key: string]: string } = {};
      response.headers.forEach((value, key) => {
        respHeaders[key] = value;
      });
      const body = await response.text();
      return { status, headers: respHeaders, body };
    },
  };
  // Run the service via the WASM module.  Provide the adapter as an
  // additional parameter if the generated binding supports it.  The exact
  // mechanism depends on the tooling used to generate bindings.  For
  // example, using wasm-bindgen you might expose a function that accepts
  // callback objects.  This is left as an exercise for the reader.
  const [outputJson, metadataJson] = await wasm.run_json(JSON.stringify(input), adapter);
  console.log('Service output:', outputJson);
  console.log('Lifecycle record:', metadataJson);
}

main().catch((err) => console.error(err));