import { defineConfig } from "vite";

export default defineConfig({
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          "g6-vendor": ["@antv/g6"],
        },
      },
    },
  },
});
