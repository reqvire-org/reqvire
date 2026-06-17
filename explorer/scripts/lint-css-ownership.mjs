/*
 * CSS ownership guard.
 *
 * This detects visual CSS rule definitions and product class hooks outside the
 * design-system ownership boundary. It runs as a focused guard and as part of
 * default lint so src visual CSS regressions fail normal validation.
 */
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const srcRoot = resolve(root, "src");
const dsRoot = resolve(root, "design-system");
const bootstrapCssPath = resolve(srcRoot, "app-mount.css");

const excludedPathParts = [
  "/node_modules/",
  "/dist/",
  "/dist-kit/",
  "/dist-showcase/",
  "/.vite/",
  "_ds_bundle.js",
  "_ds_manifest.json",
  "_adherence.oxlintrc.json",
];

const allowedBootstrapCss = normalizeCss(`
html,
body,
#root {
  height: 100%;
}

body {
  overflow: hidden;
}
`);

function walk(dir) {
  const files = [];
  if (!existsSync(dir)) return files;

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

function stripComments(source) {
  return source.replace(/\/\*[\s\S]*?\*\//g, (match) => " ".repeat(match.length));
}

function normalizeCss(source) {
  return stripComments(source)
    .replace(/\s+/g, " ")
    .replace(/\s*([{},:;])\s*/g, "$1")
    .trim();
}

function rel(file) {
  return relative(root, file).replaceAll("\\", "/");
}

function lineInfo(source, index) {
  const before = source.slice(0, index);
  const line = before.split("\n").length;
  const column = index - before.lastIndexOf("\n");
  const lineText = source.split("\n")[line - 1]?.trim() ?? "";
  return { line, column, lineText };
}

function recordFinding(findings, file, source, index, label, message) {
  const { line, column, lineText } = lineInfo(source, index);
  findings.push({
    file: rel(file),
    line,
    column,
    label,
    message,
    lineText,
  });
}

function isInside(parent, file) {
  const normalizedParent = `${parent.replaceAll("\\", "/")}/`;
  const normalizedFile = file.replaceAll("\\", "/");
  return normalizedFile.startsWith(normalizedParent);
}

function assertNoExternalCssFiles(findings) {
  for (const file of walk(root)) {
    if (!file.endsWith(".css")) continue;
    if (isInside(dsRoot, file)) continue;

    const source = readFileSync(file, "utf8");
    if (file === bootstrapCssPath && normalizeCss(source) === allowedBootstrapCss) continue;

    const message =
      file === bootstrapCssPath
        ? "src/app-mount.css is allowed only for document mount bootstrap. Move product styling into design-system product patterns."
        : "CSS files outside design-system/ are not an app styling layer. Move visual rules into design-system/components or design-system/product-patterns.";
    recordFinding(findings, file, source, 0, "CSS ownership", message);
  }
}

function findTaggedTemplates(source, tagPattern) {
  const findings = [];
  for (const match of source.matchAll(tagPattern)) {
    findings.push(match.index ?? 0);
  }
  return findings;
}

function assertNoSrcProductClassHooks(findings) {
  const classHookPattern = /(?<!-)['"`.]ux-[A-Za-z0-9_-]+/g;

  for (const file of walk(srcRoot)) {
    if (!/\.(tsx?|jsx?)$/.test(file)) continue;

    const source = readFileSync(file, "utf8");
    const clean = stripComments(source);
    for (const match of clean.matchAll(classHookPattern)) {
      recordFinding(
        findings,
        file,
        source,
        match.index ?? 0,
        "Product class ownership",
        "ux-* hooks are owned by design-system/product-patterns. src code should consume UX/product patterns through props, state, callbacks, and data, not emit visual product hooks.",
      );
    }
  }
}

function assertNoExternalLinariaRules(findings) {
  const allowedRoots = [
    resolve(dsRoot, "components"),
    resolve(dsRoot, "product-patterns"),
    resolve(dsRoot, "showcase"),
  ];

  for (const file of walk(root)) {
    if (!/\.(tsx?|jsx?)$/.test(file)) continue;
    if (allowedRoots.some((allowedRoot) => isInside(allowedRoot, file))) continue;

    const source = readFileSync(file, "utf8");
    const clean = stripComments(source);
    const hasLinariaImport = /from\s+["']@linaria\/(?:atomic|react)["']/.test(clean);
    if (!hasLinariaImport) continue;

    const cssMatches = findTaggedTemplates(clean, /\bcss\s*`/g);
    const styledMatches = findTaggedTemplates(clean, /\bstyled(?:\s*\.\s*[A-Za-z][\w-]*|\s*\([^)]*\))\s*`/g);

    for (const index of cssMatches) {
      recordFinding(
        findings,
        file,
        source,
        index,
        "Linaria ownership",
        "Linaria css templates outside design-system product-pattern/component ownership define product UI styling. Move the visual rule to design-system/product-patterns; keep only non-visual behavior in src.",
      );
    }

    for (const index of styledMatches) {
      recordFinding(
        findings,
        file,
        source,
        index,
        "Linaria ownership",
        "Linaria styled definitions outside design-system product-pattern/component ownership define product UI styling. Move the visual rule to design-system/product-patterns; keep only non-visual behavior in src.",
      );
    }
  }
}

const findings = [];
assertNoExternalCssFiles(findings);
assertNoSrcProductClassHooks(findings);
assertNoExternalLinariaRules(findings);

if (findings.length > 0) {
  for (const f of findings) {
    console.error(`${f.file}:${f.line}:${f.column}  [${f.label}]`);
    console.error(`  ${f.lineText}`);
    console.error(`  ${f.message}`);
    console.error("");
  }
  console.error(`CSS ownership guard: ${findings.length} finding(s).`);
  process.exit(1);
}

console.log("CSS ownership guard passed: visual CSS is owned by design-system product patterns/components.");
