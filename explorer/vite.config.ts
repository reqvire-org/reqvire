import path from "path";
import { defineConfig } from "vite";
import wyw from "@wyw-in-js/vite";
import react from "@vitejs/plugin-react";
import { assetMergePlugin } from "./design-system/vite.assetMerge";

const __dirname = import.meta.dirname;

// The served Explorer must run from local files and simple static servers, so
// assets are emitted with relative paths and no CDN-loaded framework or
// stylesheet. core/build.rs embeds the whole dist/ tree, which means lazy
// chunks are safe and heavy renderers can stay off the initial route path.
//
// Test configuration lives in vitest.config.ts so this file uses the plain
// Vite `defineConfig` typing and avoids Vite/Vitest plugin type conflicts.
export default defineConfig({
  base: "./",
  plugins: [
    assetMergePlugin({
      dsAssetsDir: path.resolve(__dirname, "design-system/assets"),
      publicAssetsDir: path.resolve(__dirname, "public/assets"),
      generatedAssetsDir: path.resolve(__dirname, ".vite/generated-assets"),
      label: "Explorer",
    }),
    wyw({
      include: ["**/*.{ts,tsx}"],
      exclude: ["**/node_modules/**"],
    }),
    react(),
  ],
  resolve: {
    alias: {
      "@ds": path.resolve(__dirname, "./design-system/index.ts"),
      "@": path.resolve(__dirname, "./src"),
    },
  },
  build: {
    outDir: "dist",
    assetsDir: "assets",
    sourcemap: false,
    rollupOptions: {
      output: {
        entryFileNames: "assets/explorer.js",
        chunkFileNames: "assets/explorer-[name].js",
        assetFileNames: (info) =>
          info.names?.[0]?.endsWith(".woff2")
            ? "fonts/[name][extname]"
            : "assets/explorer.[ext]",
      },
    },
  },
});
