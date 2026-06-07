import { defineConfig } from "vitest/config";

// Tests are separated from vite.config.ts so Vitest's config typing does not
// conflict with the Vite plugin types used for the app build. The unit tests
// (route parsing, store loading) exercise pure logic and jsdom globals and do
// not render React components, so no Vite plugins are needed here.
export default defineConfig({
  test: {
    globals: true,
    environment: "jsdom",
    include: ["src/**/*.{test,spec}.{ts,tsx}"],
  },
});
