import path from "path";
import { defineConfig } from "vite";
import wyw from "@wyw-in-js/vite";
import { linariaTagResolver } from "@reqvire-org/ads/linaria";
import { preactAliases, preactRuntimeGuard } from "../preact.config";

const __dirname = import.meta.dirname;

export default defineConfig({
  esbuild: { jsx: "automatic", jsxImportSource: "preact" },
  resolve: { alias: preactAliases, dedupe: ["preact"] },
  plugins: [
    preactRuntimeGuard(),
    wyw({
      tagResolver: linariaTagResolver,
      include: ["**/*.{ts,tsx}"],
      exclude: ["**/node_modules/**"],
    }),
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
      external: [/^preact(?:\/.*)?$/],
      output: {
        globals: {
          preact: "preact",
          "preact/hooks": "preactHooks",
          "preact/compat": "preactCompat",
          "preact/jsx-runtime": "preactJsxRuntime",
        },
      },
    },
  },
});
