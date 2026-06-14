import path from "path";
import { defineConfig } from "vite";
import wyw from "@wyw-in-js/vite";
import react from "@vitejs/plugin-react";
import { assetMergePlugin } from "./vite.assetMerge";

const __dirname = import.meta.dirname;

export default defineConfig({
  root: path.resolve(__dirname, "showcase"),
  base: "./",
  publicDir: path.resolve(__dirname, "showcase/public"),
  plugins: [
    assetMergePlugin({
      dsAssetsDir: path.resolve(__dirname, "assets"),
      publicAssetsDir: path.resolve(__dirname, "showcase/public/assets"),
      generatedAssetsDir: path.resolve(__dirname, "../.vite/generated-assets"),
      label: "Design-system showcase",
    }),
    wyw({
      include: ["**/*.{ts,tsx}"],
      exclude: ["**/node_modules/**"],
    }),
    react(),
  ],
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
