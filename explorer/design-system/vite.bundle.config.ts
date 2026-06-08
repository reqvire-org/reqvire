import path from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

const __dirname = import.meta.dirname;

export default defineConfig({
  root: __dirname,
  plugins: [react(), tailwindcss()],
  build: {
    outDir: __dirname,
    emptyOutDir: false,
    minify: false,
    sourcemap: false,
    lib: {
      entry: path.resolve(__dirname, "index.ts"),
      name: "ReqvireExplorerDesignSystem_48409e",
      formats: ["iife"],
      fileName: () => "_ds_bundle.js",
    },
    rollupOptions: {
      external: ["react", "react-dom"],
      output: {
        globals: {
          react: "React",
          "react-dom": "ReactDOM",
        },
      },
    },
  },
});
