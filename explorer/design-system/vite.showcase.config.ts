import path from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const __dirname = import.meta.dirname;

export default defineConfig({
  root: path.resolve(__dirname, "showcase"),
  base: "./",
  plugins: [react()],
  resolve: {
    alias: {
      "@ds": path.resolve(__dirname, "index.ts"),
    },
  },
  build: {
    outDir: path.resolve(__dirname, "dist-showcase"),
    emptyOutDir: true,
  },
});
