/*
 * CSS/namespace architecture guard.
 *
 * Enforces:
 *   - ds-* classes/selectors are reusable DS primitive-owned only.
 *   - ux-* classes/selectors are reusable UX/product-pattern-owned only.
 *   - Application UI imports DS TypeScript through @ds only.
 *   - design-system/styles.css remains the import-only public CSS entry.
 *   - HTML entry inline styles remain mount-bootstrap only.
 *   - Showcase CSS is demo scaffolding only and cannot restyle rendered components.
 *
 * Product patterns may set documented --ds-* CSS variables on ux-* wrappers.
 * That is the primitive customization API and is intentionally not treated as
 * ds-* class ownership.
 */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const srcRoot = resolve(root, "src");
const dsRoot = resolve(root, "design-system");
const dsComponentsRoot = resolve(dsRoot, "components");
const dsProductPatternsRoot = resolve(dsRoot, "product-patterns");
const dsShowcaseRoot = resolve(dsRoot, "showcase");
const dsStylesEntryPath = resolve(dsRoot, "styles.css");
const htmlMountStyleFiles = [
  resolve(root, "index.html"),
  resolve(dsRoot, "showcase/index.html"),
];

const inlineStyleBoundaryFiles = new Set([
  "src/App.tsx",
  "src/lib/ontologyGraphRenderer.ts",
  "src/rendering/MarkdownContent.tsx",
  "src/test/setupCssTokens.ts",
  "src/views/GraphLibraryViews.tsx",
]);

const primitiveShowcasePages = new Set([
  "design-system/showcase/pages/ComponentsPage.tsx",
  "design-system/showcase/pages/ControlsPage.tsx",
  "design-system/showcase/pages/CorePage.tsx",
  "design-system/showcase/pages/DataPage.tsx",
  "design-system/showcase/pages/NavigationPage.tsx",
  "design-system/showcase/pages/TokensPage.tsx",
]);

const productPatternImportPattern =
  /^(AppShell|Shell|Pane|SidePane|Workspace|Detail|ElementDetail|RelationList|Ontology|FileBrowser|Document|MarkdownFrame|DiagramBlock|CodePreview|CodeToolbar|CodeBody|RendererNotice|StoreNotice|Help|Report|Coverage|Trace|Resource)/;
const allowedShowcaseStateClasses = new Set(["is-active"]);

const allowedProductPatternDsVariables = new Set([
  "--ds-coderef-min-w",
  "--ds-coderef-ow",
  "--ds-coderef-text-align",
  "--ds-coderef-ws",
  "--ds-search-icon-left",
  "--ds-search-icon-sz",
  "--ds-search-input-bg",
  "--ds-search-input-border",
  "--ds-search-input-color",
  "--ds-search-input-fs",
  "--ds-search-input-h",
  "--ds-search-input-p",
  "--ds-search-input-placeholder-color",
  "--ds-section-head-p",
  "--ds-stat-display",
  "--ds-stat-jc",
  "--ds-stat-min-w",
  "--ds-tab-h",
  "--ds-table-min-w",
  "--ds-table-td-border",
  "--ds-table-td-p",
  "--ds-table-th-bg",
  "--ds-table-th-border",
  "--ds-table-th-fw",
  "--ds-tablewrap-bg",
  "--ds-tablewrap-border",
  "--ds-tablewrap-radius",
  "--ds-tabs-border-bottom",
  "--ds-tabs-h",
  "--ds-togglerow-bg",
  "--ds-togglerow-border",
  "--ds-togglerow-gap",
  "--ds-togglerow-h",
  "--ds-togglerow-jc",
  "--ds-togglerow-label-min-w",
  "--ds-togglerow-label-of",
  "--ds-togglerow-label-toe",
  "--ds-togglerow-label-ws",
  "--ds-togglerow-line-color",
  "--ds-togglerow-line-h",
  "--ds-togglerow-line-min-h",
  "--ds-togglerow-line-p",
  "--ds-togglerow-line-swatch-bg",
  "--ds-togglerow-line-swatch-border",
  "--ds-togglerow-line-swatch-w",
  "--ds-togglerow-meta-ai",
  "--ds-togglerow-meta-bg",
  "--ds-togglerow-meta-color",
  "--ds-togglerow-meta-display",
  "--ds-togglerow-meta-fw",
  "--ds-togglerow-meta-h",
  "--ds-togglerow-meta-jc",
  "--ds-togglerow-meta-lh",
  "--ds-togglerow-meta-min-w",
  "--ds-togglerow-meta-p",
  "--ds-togglerow-meta-radius",
  "--ds-togglerow-min-h",
  "--ds-togglerow-p",
  "--ds-togglerow-radius",
  "--ds-togglerow-shadow",
  "--ds-togglerow-static-cursor",
  "--ds-treeitem-count-ml",
  "--ds-treeitem-h",
  "--ds-treeitem-icon-color",
  "--ds-treeitem-label-flex",
  "--ds-treeitem-lh",
  "--ds-treeitem-pr",
  "--ds-treeitem-twist-color",
  "--ds-treeitem-twist-w",
]);

const primitiveStatePolicyVariablePattern = /--ds-[\w-]+-(?:sel|hover|active|focus|off)(?:-|$)/;

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

function walk(dir, options = {}) {
  const excludedParts = options.includeShowcase
    ? excludedPathParts.filter((part) => part !== "/showcase/")
    : excludedPathParts;
  const files = [];
  for (const name of readdirSync(dir)) {
    const path = resolve(dir, name);
    const normalized = path.replaceAll("\\", "/");
    if (excludedParts.some((part) => normalized.includes(part))) continue;
    const stat = statSync(path);
    if (stat.isDirectory()) {
      files.push(...walk(path, options));
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

function projectRelative(file) {
  return relative(root, file).replaceAll("\\", "/");
}

function isInside(parent, file) {
  const normalizedParent = `${parent.replaceAll("\\", "/")}/`;
  const normalizedFile = file.replaceAll("\\", "/");
  return normalizedFile.startsWith(normalizedParent);
}

function recordPatternFindings({ file, source, regex, label, message, findings }) {
  const rel = projectRelative(file);
  const clean = stripComments(source);
  for (const match of clean.matchAll(regex)) {
    const index = match.index ?? 0;
    const { line, column, lineText } = lineInfo(source, index);
    findings.push({ file: rel, line, column, label, message, lineText });
  }
}

function assertDsVariableCustomizationContract(findings) {
  const assignmentPattern = /--ds-[A-Za-z0-9_-]+\s*:/g;
  const scannedRoots = [
    {
      root: srcRoot,
      owner: "Application code",
      prefix: "Application",
    },
    {
      root: dsProductPatternsRoot,
      owner: "Product patterns",
      prefix: "Product-pattern",
    },
    {
      root: dsShowcaseRoot,
      owner: "Showcase examples",
      prefix: "Showcase",
    },
  ];

  for (const scanned of scannedRoots) {
    for (const file of walk(scanned.root, { includeShowcase: scanned.root === dsShowcaseRoot })) {
      const source = readFileSync(file, "utf8");
      const clean = stripComments(source);

      for (const match of clean.matchAll(assignmentPattern)) {
        const variable = match[0].replace(/\s*:$/, "");
        const index = match.index ?? 0;

        if (primitiveStatePolicyVariablePattern.test(variable)) {
          const { line, column, lineText } = lineInfo(source, index);
          findings.push({
            file: projectRelative(file),
            line,
            column,
            label: "Primitive state policy variable",
            message:
              `${scanned.owner} must not set primitive interaction/state policy variables. Add a typed primitive prop or variant and implement the state CSS inside design-system/components.`,
            lineText,
          });
          continue;
        }

        if (!allowedProductPatternDsVariables.has(variable)) {
          const { line, column, lineText } = lineInfo(source, index);
          findings.push({
            file: projectRelative(file),
            line,
            column,
            label: "Undocumented primitive variable",
            message:
              `${scanned.prefix} --ds-* customizations are deny-by-default. Add this variable to the documented guard allowlist only after deciding it is context/density/composition, not primitive state policy.`,
            lineText,
          });
        }
      }
    }
  }
}

function assertNoInlineVisualStyles(findings) {
  assertNoInlineVisualStylesInRoot({
    root: dsComponentsRoot,
    findings,
    owner: "Reusable DS components",
    labelSuffix: "design-system component",
    messagePrefix:
      "Reusable DS components must not use inline style. Use semantic props, data attributes, token classes, or SVG presentation attributes instead.",
  });
  assertNoInlineVisualStylesInRoot({
    root: dsProductPatternsRoot,
    findings,
    owner: "Product patterns",
    labelSuffix: "product-pattern",
    messagePrefix:
      "Product patterns must not use inline style. Move dynamic visual values into a reusable primitive prop/API in design-system/components, then compose that primitive here.",
  });
  assertNoInlineVisualStylesInRoot({
    root: dsShowcaseRoot,
    findings,
    owner: "Showcase examples",
    labelSuffix: "showcase",
    messagePrefix:
      "Showcase examples must not use inline style. Move demo styling into showcase.css with showcase-* classes so examples exercise the same architecture as Explorer.",
    includeShowcase: true,
  });
  assertNoInlineVisualStylesInRoot({
    root: srcRoot,
    findings,
    owner: "Application UI",
    labelSuffix: "application UI",
    messagePrefix:
      "Application UI must not use inline style. Use DS/product-pattern props, tokens, classes, or an explicitly allowlisted renderer boundary.",
    allowedRelativeFiles: inlineStyleBoundaryFiles,
  });
}

function assertNoInlineVisualStylesInRoot({
  root,
  findings,
  labelSuffix,
  messagePrefix,
  includeShowcase = false,
  allowedRelativeFiles = new Set(),
}) {
  const checks = [
    {
      regex: /\bstyle\s*=\s*\{/g,
      label: `Inline style in ${labelSuffix}`,
      message: messagePrefix,
    },
    {
      regex: /\bCSSProperties\b/g,
      label: `CSSProperties in ${labelSuffix}`,
      message: `${messagePrefix} CSSProperties-based visual plumbing is not allowed here.`,
    },
    {
      regex: /\.style\./g,
      label: `Imperative style mutation in ${labelSuffix}`,
      message: `${messagePrefix} Imperative style mutation is not allowed here.`,
    },
    {
      regex: /\.style\.setProperty\s*\(/g,
      label: `Imperative style mutation in ${labelSuffix}`,
      message: `${messagePrefix} setProperty-based visual plumbing is not allowed here.`,
    },
  ];

  for (const file of walk(root, { includeShowcase })) {
    const rel = projectRelative(file);
    if (allowedRelativeFiles.has(rel)) continue;
    const source = readFileSync(file, "utf8");
    for (const check of checks) {
      recordPatternFindings({
        file,
        source,
        regex: check.regex,
        label: check.label,
        message: check.message,
        findings,
      });
    }
  }
}

function assertExportedPropsOmitStyle(findings) {
  const reactAttrPattern =
    /\b(?:HTMLAttributes|ButtonHTMLAttributes|AnchorHTMLAttributes|ImgHTMLAttributes|InputHTMLAttributes|TableHTMLAttributes|ThHTMLAttributes|TdHTMLAttributes|SVGAttributes)\s*</;
  const propDeclarationPattern = /\bexport\s+(?:interface|type)\s+\w*Props\b/;
  const roots = [dsComponentsRoot, dsProductPatternsRoot];

  for (const scanRoot of roots) {
    for (const file of walk(scanRoot)) {
      const source = readFileSync(file, "utf8");
      const lines = stripComments(source).split("\n");
      const sourceLines = source.split("\n");
      const rel = projectRelative(file);

      lines.forEach((line, index) => {
        if (!propDeclarationPattern.test(line) || !reactAttrPattern.test(line)) return;
        if (/\bOmit\s*</.test(line) && /["']style["']/.test(line)) return;

        findings.push({
          file: rel,
          line: index + 1,
          column: line.search(/export/) + 1,
          label: "Style prop in public DS API",
          message:
            'Exported DS/component prop types that extend React DOM attributes must omit "style". Visual customization goes through semantic props, tokens, data attributes, or documented variables.',
          lineText: sourceLines[index].trim(),
        });
      });
    }
  }
}

function assertNoArbitraryVisualStringProps(findings) {
  const roots = [dsComponentsRoot, dsProductPatternsRoot];
  const arbitraryStringPattern =
    /\b(?:background|color|fill|stroke|pipColorToken|accentColorToken|colorToken)\??\s*:\s*string\b|`\s*--\$\{string\}\s*`/g;

  for (const scanRoot of roots) {
    for (const file of walk(scanRoot)) {
      const source = readFileSync(file, "utf8");
      recordPatternFindings({
        file,
        source,
        regex: arbitraryStringPattern,
        label: "Arbitrary visual string prop",
        message:
          "DS/public visual APIs must not accept arbitrary visual strings. Use semantic variants or typed token unions such as DesignSystemColorToken.",
        findings,
      });
    }
  }
}

function assertShowcasePrimitivePagesStayPrimitive(findings) {
  const importPattern = /\bimport\s*\{([^}]+)\}\s*from\s*["']@ds["']/g;

  for (const file of walk(resolve(dsShowcaseRoot, "pages"), { includeShowcase: true })) {
    const rel = projectRelative(file);
    if (!primitiveShowcasePages.has(rel)) continue;

    const source = readFileSync(file, "utf8");
    const clean = stripComments(source);
    for (const match of clean.matchAll(importPattern)) {
      const names = (match[1] ?? "")
        .split(",")
        .map((part) => part.replace(/\btype\b/g, "").trim())
        .filter(Boolean);
      for (const name of names) {
        const importedName = name.split(/\s+as\s+/)[0]?.trim() ?? "";
        if (!productPatternImportPattern.test(importedName)) continue;

        const index = match.index ?? 0;
        const { line, column, lineText } = lineInfo(source, index);
        findings.push({
          file: rel,
          line,
          column,
          label: "Product pattern in primitive showcase page",
          message:
            "Showcase primitive pages may demonstrate primitives/tokens only. Product vocabulary and Explorer compositions belong in ProductPatternsPage or MocksPage.",
          lineText,
        });
      }
    }
  }
}

function assertShowcasePagesUseScaffoldClasses(findings) {
  const classPattern = /\bclassName\s*=\s*["']([^"']+)["']/g;

  for (const file of walk(resolve(dsShowcaseRoot, "pages"), { includeShowcase: true })) {
    const rel = projectRelative(file);
    const source = readFileSync(file, "utf8");
    const clean = stripComments(source);

    for (const match of clean.matchAll(classPattern)) {
      const classes = (match[1] ?? "").split(/\s+/).filter(Boolean);
      const badClass = classes.find((name) => !name.startsWith("showcase-") && !allowedShowcaseStateClasses.has(name));
      if (!badClass) continue;

      const index = match.index ?? 0;
      const { line, column, lineText } = lineInfo(source, index);
      findings.push({
        file: rel,
        line,
        column,
        label: "Non-showcase class in showcase page",
        message:
          "Showcase page scaffolding must use showcase-* classes. Primitive ds-* and product ux-* classes must come from rendered DS components/patterns, not page markup.",
        lineText,
      });
    }
  }
}

function assertShowcaseCssDoesNotRestyleRenderedComponents(findings) {
  const file = resolve(dsShowcaseRoot, "showcase.css");
  const source = readFileSync(file, "utf8");
  const clean = stripComments(source);
  const rel = projectRelative(file);
  const selectorPattern = /([^{}]+)\{/g;
  const renderedComponentTargetPattern = /(?:\.ds-[A-Za-z0-9_-]+|\.ux-[A-Za-z0-9_-]+|\[data-product-pattern\b)/;
  const nestedDomTargetPattern =
    /\.showcase-[A-Za-z0-9_-]+(?:(?!,|\{).)*(?:\s|[>+~])(?:a|button|code|img|input|pre|select|svg|table|tbody|td|textarea|th|thead|tr)\b/;

  for (const match of clean.matchAll(selectorPattern)) {
    const selectorBlock = match[1] ?? "";
    const blockIndex = match.index ?? 0;
    const selectors = selectorBlock
      .split(",")
      .map((selector) => selector.trim())
      .filter(Boolean);

    for (const selector of selectors) {
      if (!selector.includes(".showcase-")) continue;

      const targetsRenderedComponent = renderedComponentTargetPattern.test(selector);
      const targetsNestedDom = nestedDomTargetPattern.test(selector);
      if (!targetsRenderedComponent && !targetsNestedDom) continue;

      const selectorIndex = blockIndex + Math.max(selectorBlock.indexOf(selector), 0);
      const { line, column, lineText } = lineInfo(source, selectorIndex);
      findings.push({
        file: rel,
        line,
        column,
        label: "Showcase restyles rendered component",
        message:
          "showcase-* CSS is demo scaffolding only. It must not target ds-* primitives, ux-* product hooks, data-product-pattern internals, or DOM elements inside rendered components; move the behavior into the owning component/pattern API.",
        lineText,
      });
    }
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
    const rel = projectRelative(file);
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
    const rel = projectRelative(file);
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
  const rel = projectRelative(dsStylesEntryPath);
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
    regex: /(?<!-)['"`.]ds-[A-Za-z0-9_-]+/g,
    label: "ds-* in src/",
    message: "ds-* classes are primitive-owned. src/ should consume design-system APIs instead of emitting visual hooks.",
    findings,
  });
  recordPatternFindings({
    file,
    source,
    regex: /(?<!-)['"`.]ux-[A-Za-z0-9_-]+/g,
    label: "ux-* in src/",
    message:
      "ux-* classes are product-pattern-owned. src/ should compose exported product patterns rather than emitting visual hooks.",
    findings,
  });
}

assertSrcDesignSystemImports(findings);

for (const file of walk(dsRoot)) {
  if (isInside(dsProductPatternsRoot, file)) continue;

  const source = readFileSync(file, "utf8");
  recordPatternFindings({
    file,
    source,
    regex: /(?<!-)['"`.]ux-[A-Za-z0-9_-]+/g,
    label: "ux-* in design-system/",
    message:
      "ux-* classes are UX/product-pattern-owned. Reusable primitives must expose ds-* hooks, semantic props, or CSS variables instead.",
    findings,
  });
}

for (const file of walk(dsProductPatternsRoot)) {
  const source = readFileSync(file, "utf8");
  recordPatternFindings({
    file,
    source,
    regex: /(?<!-)['"`.]ds-[A-Za-z0-9_-]+/g,
    label: "ds-* in product-patterns/",
    message:
      "ds-* classes are reusable DS primitive-owned. Product patterns must emit ux-* hooks and customize primitives through documented props or --ds-* variables.",
    findings,
  });
}

assertDsStylesEntry(findings);
assertHtmlMountStyles(findings);
assertDsVariableCustomizationContract(findings);
assertNoInlineVisualStyles(findings);
assertExportedPropsOmitStyle(findings);
assertNoArbitraryVisualStringProps(findings);
assertShowcasePrimitivePagesStayPrimitive(findings);
assertShowcasePagesUseScaffoldClasses(findings);
assertShowcaseCssDoesNotRestyleRenderedComponents(findings);

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
