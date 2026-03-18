import { defineConfig } from 'vite';

// Vite configuration for the UMA post fetcher browser host.  Adjust as
// necessary to support WASM loading.  This config assumes that the
// `uma_runtime` package built by wasm-pack will output JS and WASM files
// into `src/pkg` and that Vite should serve them directly.

export default defineConfig({
  resolve: {
    alias: {},
  },
  optimizeDeps: {
    exclude: ["uma_runtime"],
  },
});