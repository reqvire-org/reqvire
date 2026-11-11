import path from "path";
import { defineConfig, type Plugin } from "vite";
import wyw from "@wyw-in-js/vite";
import { linariaTagResolver } from "@reqvire-org/ads/linaria";
import { developmentImportGuard } from "@reqvire-org/ads/scenarios/vite";
import { preactAliases, preactRuntimeGuard } from "../preact.config";

const __dirname = import.meta.dirname;

export default defineConfig({
  esbuild: { jsx: "automatic", jsxImportSource: "preact" },
  resolve: { alias: preactAliases, dedupe: ["preact"] },
  plugins: [
    // ADS types this plugin with its own private Vite copy; the plugin API is compatible.
    developmentImportGuard() as Plugin,
    preactRuntimeGuard(),
    wyw({
      tagResolver: linariaTagResolver,
      include: ["**/*.{ts,tsx}"],
      exclude: ["**/node_modules/**"],
    }),
  ],
  build: {
    // Outside design-system/: ADS 0.4.5 scans every script there, generated output included, and its check fails on it.
    outDir: path.resolve(__dirname, "../dist-kit"),
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
