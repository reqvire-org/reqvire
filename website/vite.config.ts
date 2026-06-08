import path from "path"
const __dirname = import.meta.dirname
import { copyFile } from "node:fs/promises"
import react from "@vitejs/plugin-react"
import tailwindcss from "@tailwindcss/vite"
import { defineConfig } from "vite"

export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  build: {
    outDir: path.resolve(__dirname, "dist"),
    emptyOutDir: true,
  },
  ssgOptions: {
    dirStyle: "nested",
    async onFinished(dir: string) {
      await copyFile(path.join(dir, "index.html"), path.join(dir, "404.html"))
    },
  },
})
