/*
 * Style token guard.
 *
 * Enforces the design-system token contract on app CSS: raw design values
 * (px lengths, colors, font stacks, numeric weights, durations, easings) are
 * allowed ONLY where values are *defined* — custom-property declarations and
 * @media/@container/@supports conditions. Everything else must reference a
 * token via var(). Any finding fails the run; there is no baseline.
 */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const srcRoot = resolve(root, "src");

const checks = [
  {
    kind: "raw-px",
    regex: /-?\b\d+(?:\.\d+)?px\b/g,
    message: "Use a spacing, size, radius, border, or typography token instead of raw px.",
  },
  {
    kind: "raw-color",
    regex: /#[0-9a-fA-F]{3,8}\b|\b(?:rgba?|hsla?|hwb|lab|lch|oklab|oklch)\(/g,
    message: "Use a design-system color token instead of a raw color value.",
  },
  {
    kind: "raw-font-family",
    regex: /font-family\s*:\s*(?!\s*(?:var\(|inherit\b))[^;]+/gi,
    message: "Use var(--font-sans) or var(--font-mono) instead of a raw font-family stack.",
  },
  {
    kind: "raw-font-weight",
    regex: /font-weight\s*:\s*\d+/gi,
    message: "Use a --weight-* token instead of a numeric font-weight.",
  },
  {
    kind: "raw-duration",
    regex: /\b\d+(?:\.\d+)?m?s\b/g,
    message: "Use a --dur-* token instead of a raw duration.",
  },
  {
    kind: "raw-easing",
    regex: /\bcubic-bezier\(|(?<=[\s,:])ease(?:-in|-out|-in-out)?\b/g,
    message: "Use an --ease-* token instead of a raw easing.",
  },
];

function walk(dir) {
  const files = [];
  for (const name of readdirSync(dir)) {
    const path = resolve(dir, name);
    const stat = statSync(path);
    if (stat.isDirectory()) {
      files.push(...walk(path));
    } else if (stat.isFile() && path.endsWith(".css")) {
      files.push(path);
    }
  }
  return files;
}

function stripComments(source) {
  return source.replace(/\/\*[\s\S]*?\*\//g, (match) => " ".repeat(match.length));
}

function maskBalancedAtRuleBlocks(source, atRules) {
  const pattern = new RegExp(`@(?:${atRules.join("|")})\\b`, "g");
  const chars = [...source];

  for (const match of source.matchAll(pattern)) {
    const start = match.index ?? 0;
    const open = source.indexOf("{", start);
    if (open === -1) continue;

    let depth = 0;
    let end = open;
    for (let i = open; i < source.length; i += 1) {
      if (source[i] === "{") depth += 1;
      if (source[i] === "}") depth -= 1;
      if (depth === 0) {
        end = i + 1;
        break;
      }
    }

    for (let i = start; i < end; i += 1) {
      chars[i] = " ";
    }
  }

  return chars.join("");
}

/* Blank out positions where raw values are legitimate:
   custom-property declarations and at-rule conditions. */
function maskAllowedRegions(source) {
  let masked = maskBalancedAtRuleBlocks(source, ["theme"]);
  // --custom-prop: <value>  (the whole declaration)
  masked = masked.replace(/--[\w-]+\s*:[^;{}]*/g, (m) => " ".repeat(m.length));
  // @media / @container / @supports / @layer / @source <prelude> — up to the opening brace/semicolon
  masked = masked.replace(/@(?:media|container|supports|layer|source)[^{;]*/g, (m) => " ".repeat(m.length));
  return masked;
}

function lineInfo(source, index) {
  const before = source.slice(0, index);
  const line = before.split("\n").length;
  const column = index - before.lastIndexOf("\n");
  const lineText = source.split("\n")[line - 1]?.trim() ?? "";
  return { line, column, lineText };
}

const findings = [];
for (const file of walk(srcRoot)) {
  const rel = relative(root, file).replaceAll("\\", "/");
  const source = readFileSync(file, "utf8");
  const searchable = maskAllowedRegions(stripComments(source));

  for (const check of checks) {
    for (const match of searchable.matchAll(check.regex)) {
      const { line, column, lineText } = lineInfo(source, match.index ?? 0);
      findings.push({
        file: rel,
        line,
        column,
        kind: check.kind,
        value: match[0],
        message: check.message,
        source: lineText,
      });
    }
  }
}

if (findings.length > 0) {
  for (const f of findings) {
    console.error(`${f.file}:${f.line}:${f.column} ${f.kind} ${f.value}\n  ${f.message}\n  ${f.source}`);
  }
  console.error(`\nStyle token guard failed: ${findings.length} raw value(s) in app CSS.`);
  process.exit(1);
}

console.log("Style token guard passed: app CSS is fully token-driven.");
