import path from "path";
import { defineConfig } from "vitest/config";

const __dirname = import.meta.dirname;

// Tests are separated from vite.config.ts so Vitest's config typing does not
// conflict with the Vite plugin types used for the app build. The unit tests
// (route parsing, store loading) exercise pure logic and jsdom globals and do
// not render React components, so no Vite plugins are needed here.
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
    include: ["src/**/*.{test,spec}.{ts,tsx}"],
    setupFiles: ["src/test/setupCssTokens.ts"],
  },
});
