import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  statSync,
} from "node:fs";
import path from "node:path";
import type { Plugin, ResolvedConfig } from "vite";

const GENERATED_BROWSER_ASSETS = [
  "android-chrome-192x192.png",
  "apple-touch-icon.png",
  "browserconfig.xml",
  "favicon-16x16.png",
  "favicon-32x32.png",
  "favicon.ico",
  "icon-192.png",
  "icon-512.png",
  "logo.png",
  "mstile-150x150.png",
  "site.webmanifest",
];
const DEFAULT_FORBIDDEN_ASSETS = ["project-store.js", ...GENERATED_BROWSER_ASSETS];

export interface AssetMergePluginOptions {
  dsAssetsDir: string;
  publicAssetsDir: string;
  generatedAssetsDir?: string;
  label: string;
  forbiddenAssets?: string[];
}

function listFiles(root: string): string[] {
  if (!existsSync(root)) return [];

  const files: string[] = [];
  for (const name of readdirSync(root)) {
    const file = path.join(root, name);
    const stat = statSync(file);
    if (stat.isDirectory()) {
      for (const child of listFiles(file)) {
        files.push(path.join(name, child).replaceAll(path.sep, "/"));
      }
    } else if (stat.isFile()) {
      files.push(name);
    }
  }
  return files.sort();
}

function assertAssetContract({
  dsAssetsDir,
  publicAssetsDir,
  generatedAssetsDir,
  label,
  forbiddenAssets = DEFAULT_FORBIDDEN_ASSETS,
}: AssetMergePluginOptions) {
  const roots = [
    { label: "DS", files: new Set(listFiles(dsAssetsDir)) },
    { label: "public", files: new Set(listFiles(publicAssetsDir)) },
    ...(generatedAssetsDir ? [{ label: "generated", files: new Set(listFiles(generatedAssetsDir)) }] : []),
  ];
  const generatedFiles = generatedAssetsDir ? new Set(listFiles(generatedAssetsDir)) : null;
  const forbidden = forbiddenAssets.filter((file) =>
    roots.some((root) => root.label !== "generated" && root.files.has(file)),
  );
  const missingGeneratedBrowserAssets = generatedFiles
    ? GENERATED_BROWSER_ASSETS.filter((file) => !generatedFiles.has(file))
    : [];
  const collisions: string[] = [];

  for (let left = 0; left < roots.length; left += 1) {
    for (let right = left + 1; right < roots.length; right += 1) {
      for (const file of roots[left].files) {
        if (roots[right].files.has(file)) {
          collisions.push(`${file} (${roots[left].label} + ${roots[right].label})`);
        }
      }
    }
  }

  if (forbidden.length || missingGeneratedBrowserAssets.length || collisions.length) {
    const lines = [`${label} asset contract failed.`];
    if (forbidden.length) {
      lines.push("Runtime-generated assets must not be checked into DS/public asset roots:");
      for (const file of forbidden) lines.push(`- assets/${file}`);
    }
    if (missingGeneratedBrowserAssets.length) {
      lines.push("Generated browser assets are missing. Run `npm run generate:icons` first:");
      for (const file of missingGeneratedBrowserAssets) lines.push(`- assets/${file}`);
    }
    if (collisions.length) {
      lines.push("DS and public asset roots must not define the same output path:");
      for (const file of collisions) lines.push(`- assets/${file}`);
    }
    throw new Error(lines.join("\n"));
  }
}

function contentTypeFor(file: string) {
  switch (path.extname(file).toLowerCase()) {
    case ".svg":
      return "image/svg+xml";
    case ".png":
      return "image/png";
    case ".ico":
      return "image/x-icon";
    case ".webmanifest":
      return "application/manifest+json";
    case ".xml":
      return "application/xml";
    case ".js":
      return "application/javascript";
    case ".json":
      return "application/json";
    case ".woff2":
      return "font/woff2";
    default:
      return "application/octet-stream";
  }
}

function resolveAssetPath(root: string, urlPath: string) {
  const relativePath = decodeURIComponent(urlPath.replace(/^\/+assets\//, ""));
  const absolutePath = path.resolve(root, relativePath);
  const relativeToRoot = path.relative(root, absolutePath);
  if (relativeToRoot.startsWith("..") || path.isAbsolute(relativeToRoot)) {
    return null;
  }
  return absolutePath;
}

function copyAssetsToDist(assetDir: string, outDir: string) {
  for (const file of listFiles(assetDir)) {
    const source = path.join(assetDir, file);
    const target = path.join(outDir, "assets", file);
    if (existsSync(target)) {
      throw new Error(`Asset output collision: dist/assets/${file}`);
    }
    mkdirSync(path.dirname(target), { recursive: true });
    copyFileSync(source, target);
  }
}

export function assetMergePlugin(options: AssetMergePluginOptions): Plugin {
  let config: ResolvedConfig;

  return {
    name: "reqvire-asset-merge",
    configResolved(resolved) {
      config = resolved;
      assertAssetContract(options);
    },
    configureServer(server) {
      server.middlewares.use((request, response, next) => {
        const urlPath = request.url?.split("?")[0] ?? "";
        if (!urlPath.startsWith("/assets/")) {
          next();
          return;
        }

        const assetRoots = [options.generatedAssetsDir, options.dsAssetsDir].filter(
          (root): root is string => Boolean(root),
        );
        for (const root of assetRoots) {
          const assetPath = resolveAssetPath(root, urlPath);
          if (!assetPath || !existsSync(assetPath) || !statSync(assetPath).isFile()) {
            continue;
          }
          response.statusCode = 200;
          response.setHeader("Content-Type", contentTypeFor(assetPath));
          response.end(readFileSync(assetPath));
          return;
        }

        next();
      });
    },
    closeBundle() {
      if (config.command !== "build") return;
      const outDir = path.resolve(config.root, config.build.outDir);
      copyAssetsToDist(options.dsAssetsDir, outDir);
      if (options.generatedAssetsDir) {
        copyAssetsToDist(options.generatedAssetsDir, outDir);
      }
    },
  };
}
