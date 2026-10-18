import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync, existsSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import type { ResolvedConfig } from "vite";
import { afterEach, expect, it } from "vitest";
import { assetMergePlugin } from "./vite.assetMerge";

const roots: string[] = [];
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

it("in-memory preview builds leave existing production assets untouched", () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "reqvire-preview-assets-")); roots.push(root);
  const assets = path.join(root, "source"); mkdirSync(assets);
  writeFileSync(path.join(assets, "brand.svg"), "new artwork");
  const output = path.join(root, "dist/assets"); mkdirSync(output, { recursive: true });
  writeFileSync(path.join(output, "brand.svg"), "production artwork");
  const plugin = assetMergePlugin({ dsAssetsDir: assets, publicAssetsDir: path.join(root, "public"), label: "Preview" });
  const resolved = plugin.configResolved as (config: ResolvedConfig) => void;
  const close = plugin.closeBundle as () => void;
  resolved({ command: "build", root, build: { write: false, outDir: "dist" } } as ResolvedConfig);
  close();
  expect(readFileSync(path.join(output, "brand.svg"), "utf8")).toBe("production artwork");
  rmSync(output, { recursive: true });
  close();
  expect(existsSync(output)).toBe(false);
  resolved({ command: "build", root, build: { write: true, outDir: "dist" } } as ResolvedConfig);
  close();
  expect(readFileSync(path.join(output, "brand.svg"), "utf8")).toBe("new artwork");
  expect(close).toThrow("Asset output collision");
});

it("resource previews validate source assets without requiring web-app icon output", () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "reqvire-preview-icons-")); roots.push(root);
  const assets = path.join(root, "source"); mkdirSync(assets);
  writeFileSync(path.join(assets, "brand.svg"), "source artwork");
  const publicAssets = path.join(root, "public"); mkdirSync(publicAssets);
  const plugin = assetMergePlugin({ dsAssetsDir: assets, publicAssetsDir: publicAssets, generatedAssetsDir: path.join(root, ".vite/generated-assets"), label: "Preview" });
  const resolved = plugin.configResolved as (config: ResolvedConfig) => void;
  expect(() => resolved({ command: "build", root, build: { write: false, outDir: "dist" } } as ResolvedConfig)).not.toThrow();
  expect(() => resolved({ command: "build", root, build: { write: true, outDir: "dist" } } as ResolvedConfig)).toThrow("Generated browser assets are missing");
  writeFileSync(path.join(publicAssets, "brand.svg"), "conflicting artwork");
  expect(() => resolved({ command: "build", root, build: { write: false, outDir: "dist" } } as ResolvedConfig)).toThrow("same output path");
});
