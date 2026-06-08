/*
 * CSS architecture guard.
 *
 * Enforces the two-prefix contract:
 *   rq-*  — Design System primitives. Must only be DEFINED in design-system/**
 *   ex-*  — Explorer app shell.       Must only be DEFINED in src/**
 *
 * A "definition" is any CSS selector line that references the guarded prefix.
 * The check is strict: the prefix must not appear in selectors at all in the
 * wrong file tree, even as a descendant combinator target.
 *
 * Property declarations (lines starting with an identifier followed by `:`)
 * and at-rules (lines starting with `@`) are excluded from the check.
 */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function walk(dir) {
  const files = [];
  for (const name of readdirSync(dir)) {
    const path = resolve(dir, name);
    const stat = statSync(path);
    if (stat.isDirectory()) files.push(...walk(path));
    else if (stat.isFile() && path.endsWith(".css")) files.push(path);
  }
  return files;
}

/* Replace block comments with blank lines to preserve line numbers. */
function stripComments(src) {
  return src.replace(/\/\*[\s\S]*?\*\//g, (m) => {
    const lines = m.split("\n");
    return lines.map((l, i) => (i === 0 || i === lines.length - 1 ? " ".repeat(l.length) : "")).join("\n");
  });
}

/*
 * Returns true if the line looks like a CSS property declaration.
 * Property declarations start with an identifier (word chars + hyphens)
 * followed by `:`. Selector lines start with `.`, `#`, `[`, `*`, `&`, `>`,
 * `+`, `~`, or an element name not followed by `:` in the same token.
 */
function isPropertyDeclaration(line) {
  const t = line.trimStart();
  // At-rules, empty lines, closing braces: not selectors, not properties
  if (!t || t.startsWith("@") || t.startsWith("}") || t === "{") return true;
  // Property: identifier (may start with --) followed by optional whitespace then colon
  return /^[\w-]+\s*:/.test(t);
}

const checks = [
  {
    dir: resolve(root, "src"),
    badPrefix: ".rq-",
    label: "rq-* in src/",
    message:
      "rq-* classes are Design System primitives. In src/, set CSS custom properties " +
      "on the parent element instead of targeting rq-* selectors directly.",
  },
  {
    dir: resolve(root, "design-system"),
    badPrefix: ".ex-",
    label: "ex-* in design-system/",
    message:
      "ex-* classes belong to the Explorer app shell. Move the definition to src/styles.css.",
  },
];

const findings = [];

for (const check of checks) {
  for (const file of walk(check.dir)) {
    const rel = relative(root, file).replaceAll("\\", "/");
    const source = readFileSync(file, "utf8");
    const clean = stripComments(source);
    const cleanLines = clean.split("\n");
    const sourceLines = source.split("\n");

    for (let i = 0; i < cleanLines.length; i++) {
      if (isPropertyDeclaration(cleanLines[i])) continue;
      if (cleanLines[i].includes(check.badPrefix)) {
        findings.push({
          file: rel,
          line: i + 1,
          lineText: sourceLines[i].trim(),
          label: check.label,
          message: check.message,
        });
      }
    }
  }
}

if (findings.length > 0) {
  for (const f of findings) {
    console.error(`${f.file}:${f.line}  [${f.label}]`);
    console.error(`  ${f.lineText}`);
    console.error(`  ${f.message}`);
    console.error("");
  }
  console.error(`CSS architecture guard: ${findings.length} violation(s).`);
  process.exit(1);
}

console.log("CSS architecture guard passed.");
