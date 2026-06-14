import { spawnSync } from "node:child_process";
import { existsSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const root = import.meta.dirname + "/..";

const forbidden = [
  /^_ds_bundle\.js$/,
  /^design-system\/_ds_bundle\.js$/,
  /^design-system\/reqvire-explorer\.css$/,
  /^design-system\/_ds_manifest\.json$/,
  /^design-system\/_adherence\.oxlintrc\.json$/,
  /^design-system\/dist-kit\//,
  /^design-system\/dist-showcase\//,
  /^dist\//,
];

const result = spawnSync("git", ["ls-files", "-z", "--", "."], {
  cwd: root,
  encoding: "utf8",
});
if (result.status !== 0) {
  if (result.stderr) process.stderr.write(result.stderr);
  if (result.error) console.error(result.error.message);
  process.exit(result.status ?? 1);
}

const output = result.stdout ?? "";

const trackedForbidden = output
  .split("\0")
  .filter(Boolean)
  .filter((file) => existsSync(new URL(`../${file}`, import.meta.url)))
  .filter((file) => forbidden.some((pattern) => pattern.test(file)));

function listFiles(rootDir) {
  const absoluteRoot = join(root, rootDir);
  if (!existsSync(absoluteRoot)) return [];

  const files = [];
  function walk(dir, prefix = "") {
    for (const name of readdirSync(dir)) {
      const absolutePath = join(dir, name);
      const relativePath = prefix ? `${prefix}/${name}` : name;
      const stat = statSync(absolutePath);
      if (stat.isDirectory()) {
        walk(absolutePath, relativePath);
      } else if (stat.isFile()) {
        files.push(relativePath);
      }
    }
  }
  walk(absoluteRoot);
  return files.sort();
}

const assetContracts = [
  {
    label: "Explorer",
    dsRoot: "design-system/assets",
    publicRoot: "public/assets",
  },
  {
    label: "Design-system showcase",
    dsRoot: "design-system/assets",
    publicRoot: "design-system/showcase/public/assets",
  },
];

const assetFindings = [];
const generatedBrowserAssets = [
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
const generatedAssets = ["project-store.js", ...generatedBrowserAssets];

for (const contract of assetContracts) {
  const dsFiles = new Set(listFiles(contract.dsRoot));
  const publicFiles = new Set(listFiles(contract.publicRoot));
  for (const file of generatedAssets) {
    if (dsFiles.has(file)) {
      assetFindings.push(`${contract.dsRoot}/${file} is runtime-generated and must not be source.`);
    }
    if (publicFiles.has(file)) {
      assetFindings.push(`${contract.publicRoot}/${file} is runtime-generated and must not be source.`);
    }
  }
  for (const file of dsFiles) {
    if (publicFiles.has(file)) {
      assetFindings.push(`${contract.label} asset collision at assets/${file}. Keep one source owner.`);
    }
  }
}

if (trackedForbidden.length > 0 || assetFindings.length > 0) {
  for (const finding of assetFindings) {
    console.error(`- ${finding}`);
  }
  console.error("Generated artifacts must not be tracked as source:");
  for (const file of trackedForbidden) {
    console.error(`- ${file}`);
  }
  process.exit(1);
}

console.log("Generated artifact guard passed.");
