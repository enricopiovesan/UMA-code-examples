## Browser Integration Test

To run the UMA post fetcher in the browser, follow these steps:

1. Build the WebAssembly component using `wasm-pack` targeting the browser.  From the project root:

   ```sh
   wasm-pack build --target web --out-dir adapters/network/ts-host/src/pkg --out-name uma_runtime runtime
   ```

   This command will compile the `uma_runtime` crate to WebAssembly and generate the corresponding JavaScript glue code in `adapters/network/ts-host/src/pkg`.

2. Install dependencies and start the dev server:

   ```sh
   cd adapters/network/ts-host
   npm install
   npm run dev
   ```

   The application will be served at `http://localhost:5173` (or another port as reported by Vite).

3. Open the application in your browser.  The console will show the service output and lifecycle record.  You should see deterministic event ordering and a logical clock equal to the number of events.

For a production build, run `npm run build` and deploy the contents of the `dist` directory.  The Wasm module and JS glue will be bundled automatically by Vite.