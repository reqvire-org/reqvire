import { defineConfig } from "@playwright/test";

const executablePath = process.env.REQVIRE_TEST_BROWSER || process.env.CHROME_BIN;

// ADS supplies the isolated preview host, selected files/tags and its reporter.
// Local checks share the Chrome requirement of MCP captures; CI installs Chromium.
export default defineConfig({
  testDir: "design-system",
  testMatch: "**/*.ads.behavior.spec.ts",
  workers: 2,
  fullyParallel: true,
  timeout: 90_000,
  use: {
    browserName: "chromium",
    ...(executablePath ? { launchOptions: { executablePath } } : process.env.CI ? {} : { channel: "chrome" }),
    viewport: { width: 1280, height: 1000 },
  },
});
