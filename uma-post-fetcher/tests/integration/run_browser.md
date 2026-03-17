## Browser Host Sketch

The browser files in this repository are an illustrative host sketch, not a validated quick-start.  They show how a browser-facing shim could call into a generated JS/Wasm package for `uma_runtime`, but this sample does not currently ship that generated package.

If you want to continue from the sketch, the rough shape is:

1. Generate a browser-consumable JS/Wasm package for the runtime crate using your preferred binding toolchain.

   ```sh
   cd adapters/network/ts-host
   npm install
   npm run dev
   ```

2. Update `src/host.ts` so its import path and call signature match the generated package you chose.

3. Serve the Vite app and verify the runtime output in the browser console.

Until a concrete JS binding layer is added to this repository, treat the browser host as reference scaffolding rather than a turnkey integration test.
