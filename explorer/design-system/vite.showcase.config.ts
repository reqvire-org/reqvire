import path from "path";
import { defineConfig } from "vite";
import wyw from "@wyw-in-js/vite";
import { preactAliases, preactRuntimeGuard } from "../preact.config";
import { assetMergePlugin } from "./vite.assetMerge";

const __dirname = import.meta.dirname;
const explorerRoot = path.resolve(__dirname, "..");

const watchIgnored = [
  "**/.git/**",
  "**/node_modules/**",
  "**/.vite/**",
  "**/dist/**",
  "**/dist-kit/**",
  "**/dist-showcase/**",
  "**/target/**",
];

export default defineConfig({
  root: path.resolve(__dirname, "showcase"),
  base: "./",
  esbuild: { jsx: "automatic", jsxImportSource: "preact" },
  publicDir: path.resolve(__dirname, "showcase/public"),
  cacheDir: path.resolve(explorerRoot, ".vite/showcase-cache"),
  plugins: [
    preactRuntimeGuard(),
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
  ],
  resolve: {
    dedupe: ["preact"],
    alias: [
      ...preactAliases,
      { find: "@ds", replacement: path.resolve(__dirname, "index.ts") },
    ],
  },
  server: {
    fs: {
      allow: [
        path.resolve(__dirname),
        path.resolve(explorerRoot, "src"),
        path.resolve(explorerRoot, ".vite/generated-assets"),
      ],
    },
    watch: {
      ignored: watchIgnored,
      usePolling: true,
      interval: 500,
    },
  },
  build: {
    outDir: path.resolve(__dirname, "dist-showcase"),
    emptyOutDir: true,
  },
});
