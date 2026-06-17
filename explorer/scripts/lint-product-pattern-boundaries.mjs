/*
 * Product-pattern boundary guard.
 *
 * Enforces import ownership that path-pattern lint alone cannot cover:
 *   - design-system/product-patterns must not import Explorer src/* through
 *     relative paths, baseUrl src/* paths, or the @/* alias.
 *   - design-system/showcase/MockShell.tsx is the only showcase file allowed
 *     to import the real src/App integration harness.
 */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const srcRoot = resolve(root, "src");
const srcAppPath = resolve(srcRoot, "App");
const productPatternsRoot = resolve(root, "design-system/product-patterns");
const showcaseRoot = resolve(root, "design-system/showcase");
const mockShellPath = resolve(showcaseRoot, "MockShell.tsx");

const excludedPathParts = ["/node_modules/", "/dist/", "/dist-kit/", "/dist-showcase/", "/.vite/"];

function walk(dir) {
  const files = [];
  for (const name of readdirSync(dir)) {
    const path = resolve(dir, name);
    const normalized = path.replaceAll("\\", "/");
    if (excludedPathParts.some((part) => normalized.includes(part))) continue;

    const stat = statSync(path);
    if (stat.isDirectory()) {
      files.push(...walk(path));
    } else if (stat.isFile() && /\.(tsx?|jsx?)$/.test(path)) {
      files.push(path);
    }
  }
  return files;
}

function stripComments(source) {
  return source.replace(/\/\*[\s\S]*?\*\/|\/\/[^\n\r]*/g, (match) => " ".repeat(match.length));
}

function lineInfo(source, index) {
  const before = source.slice(0, index);
  const line = before.split("\n").length;
  const column = index - before.lastIndexOf("\n");
  const lineText = source.split("\n")[line - 1]?.trim() ?? "";
  return { line, column, lineText };
}

function rel(file) {
  return relative(root, file).replaceAll("\\", "/");
}

function normalized(path) {
  return path.replaceAll("\\", "/");
}

function isAtOrInside(parent, file) {
  const normalizedParent = normalized(parent);
  const normalizedFile = normalized(file);
  return normalizedFile === normalizedParent || normalizedFile.startsWith(`${normalizedParent}/`);
}

function isSrcAppPath(path) {
  const normalizedPath = normalized(path).replace(/\.(tsx?|jsx?)$/, "");
  return normalizedPath === normalized(srcAppPath);
}

function importedSpecifiers(source) {
  const clean = stripComments(source);
  const pattern =
    /\bimport\s+(?:type\s+)?(?:[^"']*?\s+from\s+)?["']([^"']+)["']|\bexport\s+(?:type\s+)?[^"']*?\s+from\s+["']([^"']+)["']|\bimport\s*\(\s*["']([^"']+)["']\s*\)/g;
  const specifiers = [];

  for (const match of clean.matchAll(pattern)) {
    specifiers.push({
      specifier: match[1] ?? match[2] ?? match[3] ?? "",
      index: match.index ?? 0,
    });
  }

  return specifiers;
}

function resolveRelativeSpecifier(fromFile, specifier) {
  if (!specifier.startsWith(".")) return null;
  return resolve(dirname(fromFile), specifier);
}

function record(findings, file, source, index, label, message) {
  const { line, column, lineText } = lineInfo(source, index);
  findings.push({ file: rel(file), line, column, label, message, lineText });
}

function assertProductPatternsDoNotImportSrc(findings) {
  for (const file of walk(productPatternsRoot)) {
    const source = readFileSync(file, "utf8");

    for (const { specifier, index } of importedSpecifiers(source)) {
      const resolvedRelative = resolveRelativeSpecifier(file, specifier);
      const importsSrc =
        specifier.startsWith("@/") ||
        specifier === "src" ||
        specifier.startsWith("src/") ||
        (resolvedRelative !== null && isAtOrInside(srcRoot, resolvedRelative));

      if (!importsSrc) continue;

      record(
        findings,
        file,
        source,
        index,
        "Product-pattern src import",
        "Product patterns must not import from src/*, including relative paths, baseUrl src/* paths, or the @/* alias. Pass data, state, and callbacks from app containers instead.",
      );
    }
  }
}

function assertShowcaseAppImportsStayInMockShell(findings) {
  for (const file of walk(showcaseRoot)) {
    if (normalized(file) === normalized(mockShellPath)) continue;

    const source = readFileSync(file, "utf8");
    for (const { specifier, index } of importedSpecifiers(source)) {
      const resolvedRelative = resolveRelativeSpecifier(file, specifier);
      const importsSrcApp =
        specifier === "@/App" ||
        /^@\/App\.(?:tsx?|jsx?)$/.test(specifier) ||
        specifier === "src/App" ||
        /^src\/App\.(?:tsx?|jsx?)$/.test(specifier) ||
        (resolvedRelative !== null && isSrcAppPath(resolvedRelative));

      if (!importsSrcApp) continue;

      record(
        findings,
        file,
        source,
        index,
        "Showcase App import",
        "Only design-system/showcase/MockShell.tsx may import src/App. Showcase pages must use @ds product patterns and showcase-local fixtures.",
      );
    }
  }
}

const findings = [];
assertProductPatternsDoNotImportSrc(findings);
assertShowcaseAppImportsStayInMockShell(findings);

if (findings.length > 0) {
  for (const f of findings) {
    console.error(`${f.file}:${f.line}:${f.column}  [${f.label}]`);
    console.error(`  ${f.lineText}`);
    console.error(`  ${f.message}`);
    console.error("");
  }
  console.error(`Product-pattern boundary guard: ${findings.length} violation(s).`);
  process.exit(1);
}

console.log("Product-pattern boundary guard passed.");
