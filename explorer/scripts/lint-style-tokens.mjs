/*
 * Style token guard.
 *
 * Enforces tokenized visual policy in app CSS, Linaria CSS-in-TS(X), and
 * reusable DS component/showcase CSS. Raw design values are allowed only in
 * source-of-truth token declarations, local component variables, and at-rule
 * conditions. Showcase examples are scanned too because they are the visual
 * regression surface for the design system.
 */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

const scanRoots = [
  resolve(root, "src"),
  resolve(root, "design-system/components"),
  resolve(root, "design-system/product-patterns"),
  resolve(root, "design-system/showcase"),
];

const excludedPathParts = [
  "/node_modules/",
  "/dist/",
  "/dist-kit/",
  "/dist-showcase/",
  ".test.",
  "_ds_bundle.js",
  "_ds_manifest.json",
  "_adherence.oxlintrc.json",
];

const checks = [
  {
    kind: "raw-px",
    regex: /-?\b\d+(?:\.\d+)?px\b/g,
    message: "Use a spacing, size, radius, border, or typography token instead of raw px.",
  },
  {
    kind: "raw-rem",
    regex: /-?\b\d+(?:\.\d+)?rem\b/g,
    message: "Use a spacing, size, or component variable instead of raw rem.",
  },
  {
    kind: "raw-color",
    regex: /#[0-9a-fA-F]{3,8}\b|\b(?:rgba?|hsla?|hwb|lab|lch|oklab|oklch)\(/g,
    message: "Use a design-system color token instead of a raw color value.",
  },
  {
    kind: "color-mix",
    regex: /\bcolor-mix\(/g,
    message: "Define mixed colors in design-system tokens, then consume the semantic token here.",
  },
  {
    kind: "raw-filter-function",
    regex: /\b(?:blur|brightness|contrast|drop-shadow|grayscale|hue-rotate|invert|saturate|sepia)\(/g,
    message: "Define visual filter effects in design-system tokens, then consume the semantic token here.",
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
  {
    kind: "raw-z-index",
    regex: /z-index\s*:\s*(?!\s*(?:var\(|auto\b|calc\(\s*var\())[^;]+/gi,
    message: "Use a --z-* token instead of a raw z-index value.",
  },
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

    for (let i = start; i < end; i += 1) chars[i] = " ";
  }

  return chars.join("");
}

function maskAllowedRegions(source) {
  let masked = maskBalancedAtRuleBlocks(source, ["theme"]);
  masked = masked.replace(/--[\w-]+\s*:[^;{}]*/g, (m) => " ".repeat(m.length));
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

function extractTaggedTemplateBodies(source, tagName) {
  const bodies = [];
  const pattern = new RegExp(`\\b${tagName}\\s*\``, "g");
  for (const match of source.matchAll(pattern)) {
    const bodyStart = (match.index ?? 0) + match[0].length;
    let escaped = false;
    for (let i = bodyStart; i < source.length; i += 1) {
      const char = source[i];
      if (escaped) {
        escaped = false;
        continue;
      }
      if (char === "\\") {
        escaped = true;
        continue;
      }
      if (char === "`") {
        bodies.push({ source: source.slice(bodyStart, i), offset: bodyStart });
        break;
      }
    }
  }
  return bodies;
}

function extractInlineStyleLiteralBodies(source) {
  const bodies = [];
  const marker = "style={{";
  let index = source.indexOf(marker);
  while (index !== -1) {
    const bodyStart = index + marker.length;
    let depth = 2;
    let quote = "";
    let escaped = false;
    for (let i = bodyStart; i < source.length; i += 1) {
      const char = source[i];
      if (escaped) {
        escaped = false;
        continue;
      }
      if (char === "\\") {
        escaped = true;
        continue;
      }
      if (quote) {
        if (char === quote) quote = "";
        continue;
      }
      if (char === "\"" || char === "'" || char === "`") {
        quote = char;
        continue;
      }
      if (char === "{") depth += 1;
      if (char === "}") depth -= 1;
      if (depth === 0) {
        bodies.push({ source: source.slice(bodyStart, i - 1), offset: bodyStart });
        index = source.indexOf(marker, i + 1);
        break;
      }
    }
    if (index !== -1 && index < bodyStart) break;
  }
  return bodies;
}

function recordFindings(fileSource, rel, fragment, context, findings) {
  const searchable = maskAllowedRegions(stripComments(fragment.source));
  for (const check of checks) {
    for (const match of searchable.matchAll(check.regex)) {
      const absoluteIndex = fragment.offset + (match.index ?? 0);
      const { line, column, lineText } = lineInfo(fileSource, absoluteIndex);
      findings.push({
        file: rel,
        line,
        column,
        kind: check.kind,
        value: match[0],
        message: check.message,
        source: lineText,
        context,
      });
    }
  }
}

const findings = [];
for (const scanRoot of scanRoots) {
  for (const file of walk(scanRoot)) {
    const rel = relative(root, file).replaceAll("\\", "/");
    const source = readFileSync(file, "utf8");

    if (file.endsWith(".css")) {
      recordFindings(source, rel, { source, offset: 0 }, "css-file", findings);
      continue;
    }

    for (const fragment of extractTaggedTemplateBodies(source, "css")) {
      recordFindings(source, rel, fragment, "linaria-css-template", findings);
    }

    for (const fragment of extractInlineStyleLiteralBodies(source)) {
      recordFindings(source, rel, fragment, "inline-style-literal", findings);
    }
  }
}

if (findings.length > 0) {
  for (const f of findings) {
    console.error(`${f.file}:${f.line}:${f.column} ${f.kind} ${f.value} [${f.context}]`);
    console.error(`  ${f.message}`);
    console.error(`  ${f.source}`);
  }
  console.error(`\nStyle token guard failed: ${findings.length} raw value(s).`);
  process.exit(1);
}

console.log("Style token guard passed: CSS, Linaria, and inline style literals are token-driven.");
