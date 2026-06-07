import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// The exported Explorer must run from local files and simple static servers
// (`index.html` + asset bundle), so we use relative asset paths and no
// CDN-loaded framework or stylesheet (Tailwind is compiled at build time).
//
// Asset filenames are deterministic and unhashed (`assets/explorer.js` +
// `assets/explorer.css`) so the Rust HTML export (core/build.rs) can embed the
// built bundle at compile time and emit it as the exported `index.html`. The
// SPA has no code-splitting, so a single JS entry and single CSS asset emit.
//
// Test configuration lives in vitest.config.ts so this file uses the plain
// Vite `defineConfig` typing and avoids Vite/Vitest plugin type conflicts.
export default defineConfig({
  base: "./",
  plugins: [react(), tailwindcss()],
  build: {
    outDir: "dist",
    assetsDir: "assets",
    sourcemap: false,
    rollupOptions: {
      output: {
        entryFileNames: "assets/explorer.js",
        chunkFileNames: "assets/explorer-[name].js",
        assetFileNames: "assets/explorer.[ext]",
      },
    },
  },
});
