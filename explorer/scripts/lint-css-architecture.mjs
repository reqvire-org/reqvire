/*
 * CSS/namespace architecture guard.
 *
 * Enforces:
 *   - rq-* classes/selectors are DS-owned only.
 *   - ex-* classes/selectors are Explorer app-owned only.
 *   - App code imports DS TypeScript through @ds only.
 *   - design-system/styles.css remains the import-only public CSS entry.
 *   - HTML entry inline styles remain mount-bootstrap only.
 *
 * Apps may set documented --rq-* CSS variables. That is the customization API
 * and is intentionally not treated as rq-* class ownership.
 */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const srcRoot = resolve(root, "src");
const dsRoot = resolve(root, "design-system");
const dsStylesEntryPath = resolve(dsRoot, "styles.css");
const htmlMountStyleFiles = [
  resolve(root, "index.html"),
  resolve(dsRoot, "showcase/index.html"),
];

const excludedPathParts = [
  "/node_modules/",
  "/dist/",
  "/dist-kit/",
  "/dist-showcase/",
  "/showcase/",
  "_ds_bundle.js",
  "_ds_manifest.json",
  "_adherence.oxlintrc.json",
];

function walk(dir) {
  const files = [];
  for (const name of readdirSync(dir)) {
    const path = resolve(dir, name);
    const normalized = path.replaceAll("\\", "/");
    if (excludedPathParts.some((part) => normalized.includes(part))) continue;
    const stat = statSync(path);
    if (stat.isDirectory()) {
      files.push(...walk(path));
    } else if (stat.isFile() && /\.(css|tsx?|jsx?)$/.test(path)) {
      files.push(path);
    }
  }
  return files;
}

function stripComments(src) {
  return src.replace(/\/\*[\s\S]*?\*\//g, (m) => {
    const lines = m.split("\n");
    return lines.map((line, index) => (index === 0 || index === lines.length - 1 ? " ".repeat(line.length) : "")).join("\n");
  });
}

function lineInfo(source, index) {
  const before = source.slice(0, index);
  const line = before.split("\n").length;
  const column = index - before.lastIndexOf("\n");
  const lineText = source.split("\n")[line - 1]?.trim() ?? "";
  return { line, column, lineText };
}

function recordPatternFindings({ file, source, regex, label, message, findings }) {
  const rel = relative(root, file).replaceAll("\\", "/");
  const clean = stripComments(source);
  for (const match of clean.matchAll(regex)) {
    const index = match.index ?? 0;
    const { line, column, lineText } = lineInfo(source, index);
    findings.push({ file: rel, line, column, label, message, lineText });
  }
}

function assertSrcDesignSystemImports(findings) {
  const allowedDirectImports = new Map([
    ["src/main.tsx", new Set(["../design-system/styles.css"])],
  ]);
  const importRegex =
    /\bfrom\s+["']([^"']+)["']|\bimport\s+["']([^"']+)["']|\bimport\s*\(\s*["']([^"']+)["']\s*\)/g;

  for (const file of walk(srcRoot)) {
    const source = readFileSync(file, "utf8");
    const rel = relative(root, file).replaceAll("\\", "/");
    const clean = stripComments(source);

    for (const match of clean.matchAll(importRegex)) {
      const specifier = match[1] ?? match[2] ?? match[3] ?? "";
      const isForbidden =
        specifier.startsWith("@ds/") ||
        specifier === "design-system" ||
        specifier.startsWith("design-system/") ||
        specifier.startsWith("../design-system/") ||
        specifier.startsWith("../../design-system/");
      const isAllowed = allowedDirectImports.get(rel)?.has(specifier) ?? false;

      if (!isForbidden || isAllowed) continue;

      const index = match.index ?? 0;
      const { line, column, lineText } = lineInfo(source, index);
      findings.push({
        file: rel,
        line,
        column,
        label: "DS import surface",
        message:
          "Application code must import design-system TypeScript through @ds. The only direct DS import allowed in src/ is ../design-system/styles.css from src/main.tsx.",
        lineText,
      });
    }
  }
}

function normalizeInlineCss(source) {
  return stripComments(source)
    .replace(/\s+/g, " ")
    .replace(/\s*([{},:;])\s*/g, "$1")
    .trim();
}

function assertHtmlMountStyles(findings) {
  const allowedMountCss = normalizeInlineCss(`
    html,
    body,
    #root {
      height: 100%;
    }

    body {
      overflow: hidden;
    }
  `);

  for (const file of htmlMountStyleFiles) {
    const source = readFileSync(file, "utf8");
    const rel = relative(root, file).replaceAll("\\", "/");
    const styleBlocks = [...source.matchAll(/<style\b[^>]*>([\s\S]*?)<\/style>/gi)];

    if (styleBlocks.length !== 1) {
      findings.push({
        file: rel,
        line: 1,
        column: 1,
        label: "HTML mount bootstrap",
        message: "HTML entry files must contain exactly one inline style block for document mount bootstrap.",
        lineText: "<style>",
      });
      continue;
    }

    const match = styleBlocks[0];
    const css = match[1] ?? "";
    if (normalizeInlineCss(css) !== allowedMountCss) {
      const index = match.index ?? 0;
      const { line, column, lineText } = lineInfo(source, index);
      findings.push({
        file: rel,
        line,
        column,
        label: "HTML mount bootstrap",
        message: "Inline HTML styles are limited to html/body/#root height and body overflow. Move product styling to DS tokens or owning component modules.",
        lineText,
      });
    }
  }
}

function assertDsStylesEntry(findings) {
  const expectedImports = [
    '@import url("tokens/fonts.css");',
    '@import url("tokens/colors.css");',
    '@import url("tokens/typography.css");',
    '@import url("tokens/spacing.css");',
    '@import url("tokens/elevation.css");',
    '@import url("tokens/base.css");',
  ];
  const source = readFileSync(dsStylesEntryPath, "utf8");
  const rel = relative(root, dsStylesEntryPath).replaceAll("\\", "/");
  const actualImports = stripComments(source)
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);

  const matches =
    actualImports.length === expectedImports.length &&
    actualImports.every((line, index) => line === expectedImports[index]);

  if (!matches) {
    findings.push({
      file: rel,
      line: 1,
      column: 1,
      label: "design-system/styles.css entry",
      message:
        "design-system/styles.css must contain only the approved @import lines, in canonical order. Adding a new entry layer requires updating this guard in the same change.",
      lineText: actualImports[0] ?? "",
    });
  }
}

const findings = [];

for (const file of walk(srcRoot)) {
  const source = readFileSync(file, "utf8");
  recordPatternFindings({
    file,
    source,
    regex: /(?<!-)['"`.]rq-[A-Za-z0-9_-]+/g,
    label: "rq-* in src/",
    message: "rq-* classes are DS-owned. In src/, use ex-* hooks or --rq-* variables instead.",
    findings,
  });
}

assertSrcDesignSystemImports(findings);

for (const file of walk(dsRoot)) {
  const source = readFileSync(file, "utf8");
  recordPatternFindings({
    file,
    source,
    regex: /(?<!-)['"`.]ex-[A-Za-z0-9_-]+/g,
    label: "ex-* in design-system/",
    message: "ex-* classes are Explorer-owned. DS components must expose rq-* hooks or CSS variables.",
    findings,
  });
}

assertDsStylesEntry(findings);
assertHtmlMountStyles(findings);

if (findings.length > 0) {
  for (const f of findings) {
    console.error(`${f.file}:${f.line}:${f.column}  [${f.label}]`);
    console.error(`  ${f.lineText}`);
    console.error(`  ${f.message}`);
    console.error("");
  }
  console.error(`CSS architecture guard: ${findings.length} violation(s).`);
  process.exit(1);
}

console.log("CSS architecture guard passed.");
