import path from "node:path";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { build } from "vite";
import { runScenarioCompiler } from "@reqvire-org/ads/scenarios/compiler";
import { createVitePreviewBuilder, previewEntriesPlugin } from "@reqvire-org/ads/scenarios/vite";

const root = path.resolve(import.meta.dirname, "..");
const runScript = promisify(execFile);
const config = path.join(root, "vite.config.ts");
await runScenarioCompiler(root, {
  watched: process.argv.includes("--watch"),
  compilerFiles: [import.meta.filename, config, path.join(root, "preact.config.ts"), path.join(root, "design-system/vite.assetMerge.ts")],
  async validate() {
    try { await runScript("npm", ["run", "check"], { cwd: root }); }
    catch (error) { throw new Error([error.stdout, error.stderr].filter(Boolean).join("\n") || error.message); }
  },
  buildGraph: createVitePreviewBuilder({
    build,
    configure(entries, options) {
      const mount = path.join(options.dsRoot, "showcase/mcp/main.tsx");
      // Reuse production extraction, framework aliases, assets and runtime guards.
      return { configFile: config, root, base: "./", publicDir: false,
        plugins: [previewEntriesPlugin(entries.map(entry => ({ ...entry, root })), mount)],
        worker: { format: "iife" },
        build: { write: false, cssCodeSplit: true, assetsInlineLimit: 0, modulePreload: false,
          rollupOptions: { preserveEntrySignatures: "strict", input: Object.fromEntries(entries.map((_entry, index) => [`scenario-${index}`, `ads-preview-entry:${index}`])),
            output: { entryFileNames: "assets/preview-[name]-[hash].js", chunkFileNames: "assets/[name]-[hash].js", assetFileNames: "assets/[name]-[hash][extname]" } } },
      };
    },
  }),
});
