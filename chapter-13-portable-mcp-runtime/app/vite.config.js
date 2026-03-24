import { defineConfig } from "vite";

export default defineConfig(({ command }) => ({
  base: command === "build" ? "/reference-application/" : "/",
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          "g6-vendor": ["@antv/g6"],
        },
      },
    },
  },
}));
