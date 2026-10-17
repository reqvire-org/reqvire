import path from "path";
import { defineConfig } from "vitest/config";
import { preactAliases } from "./preact.config";

const __dirname = import.meta.dirname;

// Tests are separated from vite.config.ts so Vitest's config typing does not
// conflict with the Vite plugin types used for the app build. The unit tests
// load the real foundation CSS and exercise UI, store, route and polling logic.
// Browser/build checks verify Linaria extraction; these DOM tests need no plugin.
export default defineConfig({
  esbuild: { jsx: "automatic", jsxImportSource: "preact" },
  resolve: {
    dedupe: ["preact"],
    alias: [
      ...preactAliases,
      { find: "@ds/review", replacement: path.resolve(__dirname, "./design-system/review/index.ts") },
      { find: "@ds", replacement: path.resolve(__dirname, "./design-system/index.ts") },
      { find: "@", replacement: path.resolve(__dirname, "./src") },
    ],
  },
  test: {
    globals: true,
    environment: "jsdom",
    css: true,
    include: ["src/**/*.{test,spec}.{ts,tsx}", "design-system/*.test.ts", "design-system/patterns/**/*.{test,spec}.{ts,tsx}", "design-system/review/**/*.{test,spec}.{ts,tsx}"],
    setupFiles: ["src/test/setupCssTokens.ts"],
  },
});
