// Edge host sketch.  This file is not part of the validated quick-start
// path; it expects a generated JS/Wasm binding package to exist under
// `hosts/edge/pkg`.  If you generate such a package, update the import
// below to match your chosen toolchain.

async function main() {
  let init: () => Promise<unknown>;
  let run_json: (input: string) => [string, string];
  try {
    ({ default: init, run_json } = await import('./pkg/uma_runtime.js'));
  } catch (err) {
    throw new Error(
      'Missing hosts/edge/pkg/uma_runtime.js. Generate a compatible JS/Wasm package before using the edge host sketch.'
    );
  }

  await init();
  const input = {
    request: {
      url: 'http://127.0.0.1:18080/posts/1',
      headers: { accept: 'application/json' },
    },
    runId: 'demo-001',
  };
  const [outputJson, metadataJson] = run_json(JSON.stringify(input));
  console.log('Service output:', outputJson);
  console.log('Lifecycle record:', metadataJson);
}

main().catch((err) => console.error(err));
