import path from "path";
import { defineConfig } from "vitest/config";

const __dirname = import.meta.dirname;

// Tests are separated from vite.config.ts so Vitest's config typing does not
// conflict with the Vite plugin types used for the app build. The unit tests
// exercise store/route logic and polling hooks without importing styled UI
// components, so no Vite plugins are needed here.
export default defineConfig({
  resolve: {
    alias: {
      "@ds": path.resolve(__dirname, "./design-system/index.ts"),
      "@": path.resolve(__dirname, "./src"),
    },
  },
  test: {
    globals: true,
    environment: "jsdom",
    include: ["src/**/*.{test,spec}.{ts,tsx}", "design-system/showcase/*.test.tsx"],
    setupFiles: ["src/test/setupCssTokens.ts"],
  },
});
