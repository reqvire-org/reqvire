import path from "path";
import { defineConfig } from "vite";
import wyw from "@wyw-in-js/vite";
import react from "@vitejs/plugin-react";

const __dirname = import.meta.dirname;

export default defineConfig({
  plugins: [
    wyw({
      include: ["**/*.{ts,tsx}"],
      exclude: ["**/node_modules/**"],
    }),
    react(),
  ],
  build: {
    outDir: path.resolve(__dirname, "dist-kit"),
    emptyOutDir: true,
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
