export type ElementIconShape = "square" | "diamond" | "hub";
export type ElementRole =
  | "capability"
  | "requirement"
  | "contract"
  | "source"
  | "constraint"
  | "behavior"
  | "state"
  | "input-output"
  | "verification-objective"
  | "test-verification"
  | "formal-proof-verification"
  | "analysis-verification"
  | "inspection-verification"
  | "demonstration-verification"
  | "verification"
  | "specification"
  | "semantic-query"
  | "semantic-contract"
  | "ontology"
  | "concept"
  | "concept-scheme"
  | "concept-reference"
  | "resource"
  | "other";
export type ElementType = Exclude<ElementRole, "other">;
export type PaletteChannel = "fill" | "ink" | "tint";
type CssTokenName = `--${string}`;

export const ELEMENT_ROLE_TOKENS = {
  capability: { fill: "--color-role-capability", ink: "--color-role-capability-ink", tint: "--color-role-capability-tint" },
  requirement: { fill: "--color-role-requirement", ink: "--color-role-requirement-ink", tint: "--color-role-requirement-tint" },
  contract: { fill: "--color-role-contract", ink: "--color-role-contract-ink", tint: "--color-role-contract-tint" },
  source: { fill: "--color-role-contract", ink: "--color-role-contract-ink", tint: "--color-role-contract-tint" },
  constraint: { fill: "--color-role-contract", ink: "--color-role-contract-ink", tint: "--color-role-contract-tint" },
  behavior: { fill: "--color-role-contract", ink: "--color-role-contract-ink", tint: "--color-role-contract-tint" },
  state: { fill: "--color-role-contract", ink: "--color-role-contract-ink", tint: "--color-role-contract-tint" },
  "input-output": { fill: "--color-role-contract", ink: "--color-role-contract-ink", tint: "--color-role-contract-tint" },
  "verification-objective": {
    fill: "--color-role-verification-objective",
    ink: "--color-role-verification-objective-ink",
    tint: "--color-role-verification-objective-tint",
  },
  "test-verification": { fill: "--color-role-verification", ink: "--color-role-verification-ink", tint: "--color-role-verification-tint" },
  "formal-proof-verification": { fill: "--color-role-verification", ink: "--color-role-verification-ink", tint: "--color-role-verification-tint" },
  "analysis-verification": { fill: "--color-role-verification", ink: "--color-role-verification-ink", tint: "--color-role-verification-tint" },
  "inspection-verification": { fill: "--color-role-verification", ink: "--color-role-verification-ink", tint: "--color-role-verification-tint" },
  "demonstration-verification": { fill: "--color-role-verification", ink: "--color-role-verification-ink", tint: "--color-role-verification-tint" },
  verification: { fill: "--color-role-verification", ink: "--color-role-verification-ink", tint: "--color-role-verification-tint" },
  specification: { fill: "--color-role-contract", ink: "--color-role-contract-ink", tint: "--color-role-contract-tint" },
  "semantic-query": { fill: "--color-role-ontology", ink: "--color-role-ontology-ink", tint: "--color-role-ontology-tint" },
  "semantic-contract": { fill: "--color-role-semantic-contract", ink: "--color-role-semantic-contract-ink", tint: "--color-role-semantic-contract-tint" },
  ontology: { fill: "--color-role-ontology", ink: "--color-role-ontology-ink", tint: "--color-role-ontology-tint" },
  concept: { fill: "--color-role-concept", ink: "--color-role-concept-ink", tint: "--color-role-concept-tint" },
  "concept-scheme": { fill: "--color-role-concept-scheme", ink: "--color-role-concept-scheme-ink", tint: "--color-role-concept-scheme-tint" },
  "concept-reference": { fill: "--color-role-concept-reference", ink: "--color-role-concept-reference-ink", tint: "--color-role-concept-reference-tint" },
  resource: { fill: "--color-role-resource", ink: "--color-role-resource-ink", tint: "--color-role-resource-tint" },
  other: { fill: "--color-role-other", ink: "--color-role-other-ink", tint: "--color-role-other-tint" },
} as const;

export const ELEMENT_TYPES: Record<ElementType, { color: string; shape: ElementIconShape; role: ElementRole; glyph?: string }> = {
  capability: { color: "var(--color-role-capability)", shape: "square", role: "capability" },
  requirement: { color: "var(--color-role-requirement)", shape: "square", role: "requirement" },
  contract: { color: "var(--color-role-contract)", shape: "diamond", role: "contract", glyph: "C" },
  source: { color: "var(--color-role-contract)", shape: "diamond", role: "source", glyph: "↗" },
  constraint: { color: "var(--color-role-contract)", shape: "diamond", role: "constraint", glyph: "!" },
  behavior: { color: "var(--color-role-contract)", shape: "diamond", role: "behavior", glyph: "→" },
  state: { color: "var(--color-role-contract)", shape: "diamond", role: "state", glyph: "●" },
  "input-output": { color: "var(--color-role-contract)", shape: "diamond", role: "input-output", glyph: "↔" },
  "verification-objective": {
    color: "var(--color-role-verification-objective)",
    shape: "square",
    role: "verification-objective",
  },
  "test-verification": { color: "var(--color-role-verification)", shape: "square", role: "verification", glyph: "T" },
  "formal-proof-verification": { color: "var(--color-role-verification)", shape: "square", role: "verification", glyph: "FP" },
  "analysis-verification": { color: "var(--color-role-verification)", shape: "square", role: "verification", glyph: "A" },
  "inspection-verification": { color: "var(--color-role-verification)", shape: "square", role: "verification", glyph: "I" },
  "demonstration-verification": { color: "var(--color-role-verification)", shape: "square", role: "verification", glyph: "D" },
  verification: { color: "var(--color-role-verification)", shape: "square", role: "verification" },
  specification: { color: "var(--color-role-contract)", shape: "diamond", role: "specification", glyph: "≡" },
  "semantic-query": { color: "var(--color-role-ontology)", shape: "square", role: "semantic-query", glyph: "Q" },
  "semantic-contract": { color: "var(--color-role-semantic-contract)", shape: "square", role: "semantic-contract" },
  ontology: { color: "var(--color-role-ontology)", shape: "square", role: "ontology" },
  concept: { color: "var(--color-role-concept)", shape: "square", role: "concept" },
  "concept-scheme": { color: "var(--color-role-concept-scheme)", shape: "square", role: "concept-scheme" },
  "concept-reference": { color: "var(--color-role-concept-reference)", shape: "square", role: "concept-reference" },
  resource: { color: "var(--color-role-resource)", shape: "square", role: "resource" },
};

export const DESIGN_SYSTEM_COLOR_TOKENS = [
  "--color-role-verification-type-test",
  "--color-role-verification-type-formal-proof",
  "--color-role-verification-type-analysis",
  "--color-role-verification-type-inspection",
  "--color-role-verification-type-demonstration",
  "--accent",
  "--accent-active",
  "--accent-hover",
  "--accent-ring",
  "--accent-subtle",
  "--bg-active",
  "--bg-canvas",
  "--bg-hover",
  "--bg-overlay",
  "--bg-raised",
  "--bg-selected",
  "--bg-sunken",
  "--bg-surface",
  "--border-default",
  "--border-focus",
  "--border-selected",
  "--border-strong",
  "--border-subtle",
  "--color-role-edge-bind",
  "--color-role-edge-default",
  "--color-role-edge-derive",
  "--color-role-edge-satisfy",
  "--color-role-edge-trace",
  "--color-role-concept-reference",
  "--color-role-concept-reference-ink",
  "--color-role-concept-reference-tint",
  "--color-role-concept",
  "--color-role-concept-ink",
  "--color-role-concept-tint",
  "--color-role-concept-scheme",
  "--color-role-concept-scheme-ink",
  "--color-role-concept-scheme-tint",
  "--color-role-node-generic-fill",
  "--color-role-ontology-ink",
  "--color-role-rdf-class",
  "--color-role-rdf-classexpr",
  "--color-role-rdf-concept",
  "--color-role-rdf-concept-scheme",
  "--color-role-rdf-datatype",
  "--color-role-rdf-dtprop",
  "--color-role-rdf-individual",
  "--color-role-rdf-nodeshape",
  "--color-role-rdf-objprop",
  "--color-role-rdf-propshape",
  "--color-role-rdf-rdfprop",
  "--color-role-rdf-resource",
  "--color-role-rdf-restriction",
  "--color-role-rdf-shacl",
  "--color-role-requirement-ink",
  "--color-paper",
  "--color-ink",
  "--success",
  "--success-bg",
  "--text-body",
  "--text-code",
  "--text-faint",
  "--text-inverse",
  "--text-link",
  "--text-muted",
  "--text-secondary",
  "--text-strong",
  "--warning",
  "--warning-bg",
  "--danger",
  "--danger-bg",
  "--info",
  "--info-bg",
] as const satisfies readonly CssTokenName[];

type ElementRoleToken = {
  [Role in keyof typeof ELEMENT_ROLE_TOKENS]: (typeof ELEMENT_ROLE_TOKENS)[Role][keyof (typeof ELEMENT_ROLE_TOKENS)[Role]];
}[keyof typeof ELEMENT_ROLE_TOKENS];

export type DesignSystemColorToken = ElementRoleToken | (typeof DESIGN_SYSTEM_COLOR_TOKENS)[number];
export type ExplorerColorToken = DesignSystemColorToken;

const CSS_TOKEN_FALLBACKS: Partial<Record<DesignSystemColorToken, string>> = {
  "--color-role-capability": "#bbdefb",
  "--color-role-capability-ink": "#1565c0",
  "--color-role-capability-tint": "#e3f2fd",
  "--color-role-requirement": "#673ab7",
  "--color-role-requirement-ink": "#512da8",
  "--color-role-requirement-tint": "#ede7f6",
  "--color-role-contract": "#ff9800",
  "--color-role-contract-ink": "#e65100",
  "--color-role-contract-tint": "#fff3e0",
  "--color-role-semantic-contract": "#d32f2f",
  "--color-role-semantic-contract-ink": "#b71c1c",
  "--color-role-semantic-contract-tint": "#ffebee",
  "--color-role-verification": "#4caf50",
  "--color-role-verification-ink": "#2e7d32",
  "--color-role-verification-tint": "#e8f5e9",
  "--color-role-verification-objective": "#1b5e20",
  "--color-role-verification-objective-ink": "#0b3d12",
  "--color-role-verification-objective-tint": "#d7ead8",
  "--color-role-ontology": "#b08a00",
  "--color-role-ontology-ink": "#6f5600",
  "--color-role-ontology-tint": "#f4e3a1",
  "--color-role-concept": "#e7c94a",
  "--color-role-concept-ink": "#6f5600",
  "--color-role-concept-tint": "#fbf1cf",
  "--color-role-concept-scheme": "#cda83a",
  "--color-role-concept-scheme-ink": "#6f5600",
  "--color-role-concept-scheme-tint": "#f7e7b4",
  "--color-role-resource": "#ffca28",
  "--color-role-resource-ink": "#8d6e00",
  "--color-role-resource-tint": "#fff3cf",
  "--color-role-other": "#9e9e9e",
  "--color-role-other-ink": "#616161",
  "--color-role-other-tint": "#ececec",
  "--color-role-node-generic-fill": "#eceff1",
  "--color-role-edge-default": "#c0c8d4",
  "--color-role-edge-derive": "#673ab7",
  "--color-role-edge-satisfy": "#4caf50",
  "--color-role-edge-trace": "#97a2b4",
  "--color-role-edge-bind": "#2196f3",
  "--bg-canvas": "#fbfaf7",
  "--bg-sunken": "#f3f1eb",
  "--bg-surface": "#ffffff",
  "--border-default": "#d8d2c6",
  "--accent": "#e11d48",
  "--accent-ring": "rgba(225,29,72,0.32)",
  "--color-role-rdf-class": "#94a3b8",
  "--color-role-rdf-concept": "#e7c94a",
  "--color-role-rdf-concept-scheme": "#cda83a",
  "--color-role-concept-reference": "#a77718",
  "--color-role-concept-reference-ink": "#4a3608",
  "--color-role-concept-reference-tint": "#fbf1cf",
  "--color-role-rdf-objprop": "#64748b",
  "--color-role-rdf-dtprop": "#0f766e",
  "--color-role-rdf-rdfprop": "#115e59",
  "--color-role-rdf-individual": "#7c3aed",
  "--color-role-rdf-datatype": "#d6a43f",
  "--color-role-rdf-restriction": "#cbd5e1",
  "--color-role-rdf-classexpr": "#e2e8f0",
  "--color-role-rdf-nodeshape": "#dc2626",
  "--color-role-rdf-propshape": "#be123c",
  "--color-role-rdf-resource": "#14b8a6",
  "--color-role-rdf-shacl": "#ef4444",
  "--success": "#1f9d57",
  "--color-paper": "#ffffff",
  "--color-ink": "#0d1119",
  "--text-body": "#232b37",
  "--text-faint": "#97a2b4",
  "--text-muted": "#6b7688",
  "--text-strong": "#161d27",
};

const CONTRACT_TYPES = new Set([
  "source",
  "specification",
  "constraint",
  "behavior",
  "state",
  "input-output",
]);

export function elementRole(type?: string | null, family?: string | null): ElementRole {
  const normalizedType = (type ?? "").toLowerCase();
  const normalizedFamily = (family ?? "").toLowerCase();

  if (normalizedType in ELEMENT_TYPES) {
    return ELEMENT_TYPES[normalizedType as ElementType].role;
  }
  if (normalizedType.includes("capability") || normalizedFamily === "capability") return "capability";
  if (normalizedType.includes("verification") || normalizedFamily === "verification") return "verification";
  if (normalizedType === "semantic-query") return "semantic-query";
  if (normalizedType.includes("ontology") || normalizedFamily === "ontology") return "ontology";
  if (normalizedType === "evidence-file" || normalizedType.includes("evidence")) return "other";
  if (normalizedType === "concept") return "concept";
  if (normalizedType === "concept-scheme") return "concept-scheme";
  if (normalizedType === "concept-reference" || normalizedType.includes("concept-reference")) return "concept-reference";
  if (normalizedType.includes("resource") || normalizedType === "file" || normalizedFamily === "resource") {
    return "resource";
  }
  if (CONTRACT_TYPES.has(normalizedType) && normalizedType in ELEMENT_TYPES) {
    return ELEMENT_TYPES[normalizedType as ElementType].role;
  }
  if (normalizedType === "contract" || normalizedFamily === "contract") return "contract";
  if (normalizedType.includes("requirement") || normalizedFamily === "requirement") return "requirement";
  if (normalizedType.includes("contract")) return "semantic-contract";
  if (normalizedType.includes("specification")) return "specification";
  return "other";
}

export function roleColorToken(role: string | null | undefined, channel: PaletteChannel = "fill"): DesignSystemColorToken {
  return ELEMENT_ROLE_TOKENS[elementRole(role)][channel];
}

export function roleColorValue(role: string, channel: PaletteChannel = "fill") {
  return cssVar(roleColorToken(role, channel));
}

export function cssVar(token: DesignSystemColorToken): string {
  if (typeof window === "undefined") return `var(${token})`;
  const resolved = resolveCssToken(token);
  return normalizeCssColor(resolved) ?? resolved;
}

function resolveCssToken(token: CssTokenName, seen = new Set<string>()): string {
  if (seen.has(token)) return `var(${token})`;
  seen.add(token);

  const value = window.getComputedStyle(document.documentElement).getPropertyValue(token).trim();
  if (!value) return CSS_TOKEN_FALLBACKS[token as DesignSystemColorToken] ?? `var(${token})`;

  return resolveCssValue(value, seen);
}

function resolveCssValue(value: string, seen = new Set<string>()): string {
  const normalized = value.trim();
  const nested = normalized.match(/^var\((--[a-z0-9-]+)(?:,\s*([^)]+))?\)$/i);
  if (!nested) return normalized;

  return resolveCssToken(nested[1] as CssTokenName, seen);
}

export function replaceCssVarsForMermaid(source: string) {
  return source.replace(/var\((--[a-z0-9-]+)\)/gi, (match, token: string) => {
    const colorToken = token as DesignSystemColorToken;
    const value = normalizeCssColor(resolveCssToken(token as CssTokenName)) ?? CSS_TOKEN_FALLBACKS[colorToken];
    return value ?? match;
  });
}

export function getMermaidClassDefs() {
  const classDef = (className: string, role: ElementRole, strokeWidth = "2px") => {
    const tokens = ELEMENT_ROLE_TOKENS[role];
    return `  classDef ${className} fill:${mermaidTokenColor(tokens.tint)},stroke:${mermaidTokenColor(tokens.fill)},stroke-width:${strokeWidth},color:${mermaidTokenColor("--text-body")};`;
  };

  return [
    classDef("capability", "capability", "2.5px"),
    classDef("systemRequirement", "requirement", "2px"),
    classDef("requirement", "requirement", "2px"),
    classDef("contract", "contract", "2px"),
    classDef("source", "source", "2px"),
    classDef("constraint", "constraint", "2px"),
    classDef("behavior", "behavior", "2px"),
    classDef("state", "state", "2px"),
    classDef("inputOutput", "input-output", "2px"),
    classDef("specification", "specification", "2px"),
    classDef("semanticContract", "semantic-contract", "2px"),
    classDef("verificationObjective", "verification-objective", "2px"),
    classDef("verification", "verification", "2px"),
    classDef("ontology", "ontology", "2px"),
    classDef("resource", "resource", "2px"),
    classDef("file", "resource", "2px"),
    classDef("folder", "resource", "2px"),
    classDef("default", "other", "1.5px"),
  ] as const;
}

function mermaidTokenColor(token: DesignSystemColorToken): string {
  const value = normalizeCssColor(cssVar(token));
  if (value) return value;
  const fallback = CSS_TOKEN_FALLBACKS[token];
  if (fallback) return fallback;
  throw new Error(`Missing Mermaid-safe CSS token value for ${token}`);
}

function normalizeCssColor(value: string | undefined): string | null {
  if (!value || typeof document === "undefined") return null;
  const probe = document.createElement("span");
  probe.style.color = "";
  probe.style.color = value.trim();
  if (!probe.style.color) return null;

  const parent = document.body ?? document.documentElement;
  parent.appendChild(probe);
  const computed = window.getComputedStyle(probe).color;
  probe.remove();

  const hex = colorToHex(computed);
  if (hex) return hex;

  if (/\bjsdom\b/i.test(window.navigator?.userAgent ?? "")) return null;

  // Wide-gamut path: browser returned color(display-p3 …) or similar.
  // Render one pixel through a canvas to clamp into sRGB hex.
  try {
    const canvas = document.createElement("canvas");
    canvas.width = canvas.height = 1;
    const ctx = canvas.getContext("2d");
    if (!ctx) return null;
    ctx.fillStyle = computed;
    ctx.fillRect(0, 0, 1, 1);
    const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
    return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
  } catch {
    return null;
  }
}

function colorToHex(color: string): string | null {
  const match = color.match(/^rgba?\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)(?:\s*,\s*([0-9.]+))?\s*\)$/i);
  if (!match) return null;

  const [r, g, b] = match.slice(1, 4).map((part) => {
    const value = Number.parseFloat(part);
    if (!Number.isFinite(value)) return null;
    return Math.max(0, Math.min(255, Math.round(value)));
  });
  if (r === null || g === null || b === null) return null;

  const alpha = match[4] === undefined ? 1 : Number.parseFloat(match[4]);
  const hex = [r, g, b].map((component) => component.toString(16).padStart(2, "0")).join("");
  if (!Number.isFinite(alpha) || alpha >= 1) return `#${hex}`;

  const alphaHex = Math.max(0, Math.min(255, Math.round(alpha * 255))).toString(16).padStart(2, "0");
  return `#${hex}${alphaHex}`;
}
