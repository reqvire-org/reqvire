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
  | "semantic-contract"
  | "ontology"
  | "resource"
  | "other";
export type ElementType = Exclude<ElementRole, "other">;
export type PaletteChannel = "fill" | "ink" | "tint";
type CssTokenName = `--${string}`;

export const ELEMENT_ROLE_TOKENS = {
  capability: { fill: "--capability", ink: "--capability-ink", tint: "--capability-tint" },
  requirement: { fill: "--requirement", ink: "--requirement-ink", tint: "--requirement-tint" },
  contract: { fill: "--contract", ink: "--contract-ink", tint: "--contract-tint" },
  source: { fill: "--contract", ink: "--contract-ink", tint: "--contract-tint" },
  constraint: { fill: "--contract", ink: "--contract-ink", tint: "--contract-tint" },
  behavior: { fill: "--contract", ink: "--contract-ink", tint: "--contract-tint" },
  state: { fill: "--contract", ink: "--contract-ink", tint: "--contract-tint" },
  "input-output": { fill: "--contract", ink: "--contract-ink", tint: "--contract-tint" },
  "verification-objective": {
    fill: "--verification-objective",
    ink: "--verification-objective-ink",
    tint: "--verification-objective-tint",
  },
  "test-verification": { fill: "--verification", ink: "--verification-ink", tint: "--verification-tint" },
  "formal-proof-verification": { fill: "--verification", ink: "--verification-ink", tint: "--verification-tint" },
  "analysis-verification": { fill: "--verification", ink: "--verification-ink", tint: "--verification-tint" },
  "inspection-verification": { fill: "--verification", ink: "--verification-ink", tint: "--verification-tint" },
  "demonstration-verification": { fill: "--verification", ink: "--verification-ink", tint: "--verification-tint" },
  verification: { fill: "--verification", ink: "--verification-ink", tint: "--verification-tint" },
  specification: { fill: "--contract", ink: "--contract-ink", tint: "--contract-tint" },
  "semantic-contract": { fill: "--semantic-contract", ink: "--semantic-contract-ink", tint: "--semantic-contract-tint" },
  ontology: { fill: "--ontology", ink: "--ontology-ink", tint: "--ontology-tint" },
  resource: { fill: "--resource", ink: "--resource-ink", tint: "--resource-tint" },
  other: { fill: "--other", ink: "--other-ink", tint: "--other-tint" },
} as const;

export const ELEMENT_TYPES: Record<ElementType, { color: string; shape: ElementIconShape; role: ElementRole; glyph?: string }> = {
  capability: { color: "var(--capability)", shape: "square", role: "capability" },
  requirement: { color: "var(--requirement)", shape: "square", role: "requirement" },
  contract: { color: "var(--contract)", shape: "diamond", role: "contract", glyph: "C" },
  source: { color: "var(--contract)", shape: "diamond", role: "source", glyph: "↗" },
  constraint: { color: "var(--contract)", shape: "diamond", role: "constraint", glyph: "!" },
  behavior: { color: "var(--contract)", shape: "diamond", role: "behavior", glyph: "→" },
  state: { color: "var(--contract)", shape: "diamond", role: "state", glyph: "●" },
  "input-output": { color: "var(--contract)", shape: "diamond", role: "input-output", glyph: "↔" },
  "verification-objective": {
    color: "var(--verification-objective)",
    shape: "square",
    role: "verification-objective",
  },
  "test-verification": { color: "var(--verification)", shape: "square", role: "verification", glyph: "T" },
  "formal-proof-verification": { color: "var(--verification)", shape: "square", role: "verification", glyph: "FP" },
  "analysis-verification": { color: "var(--verification)", shape: "square", role: "verification", glyph: "A" },
  "inspection-verification": { color: "var(--verification)", shape: "square", role: "verification", glyph: "I" },
  "demonstration-verification": { color: "var(--verification)", shape: "square", role: "verification", glyph: "D" },
  verification: { color: "var(--verification)", shape: "square", role: "verification" },
  specification: { color: "var(--contract)", shape: "diamond", role: "specification", glyph: "≡" },
  "semantic-contract": { color: "var(--semantic-contract)", shape: "square", role: "semantic-contract" },
  ontology: { color: "var(--ontology)", shape: "square", role: "ontology" },
  resource: { color: "var(--resource)", shape: "square", role: "resource" },
};

export const DESIGN_SYSTEM_COLOR_TOKENS = [
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
  "--edge-attach",
  "--edge-default",
  "--edge-derive",
  "--edge-satisfy",
  "--edge-trace",
  "--node-generic-fill",
  "--ontology-ink",
  "--rdf-class",
  "--rdf-classexpr",
  "--rdf-datatype",
  "--rdf-dtprop",
  "--rdf-individual",
  "--rdf-nodeshape",
  "--rdf-objprop",
  "--rdf-propshape",
  "--rdf-rdfprop",
  "--rdf-resource",
  "--rdf-restriction",
  "--rdf-shacl",
  "--requirement-ink",
  "--slate-0",
  "--slate-950",
  "--success",
  "--success-tint",
  "--text-body",
  "--text-code",
  "--text-faint",
  "--text-inverse",
  "--text-link",
  "--text-muted",
  "--text-secondary",
  "--text-strong",
  "--warning",
  "--warning-tint",
  "--danger",
  "--danger-tint",
  "--info",
  "--info-tint",
] as const satisfies readonly CssTokenName[];

type ElementRoleToken = {
  [Role in keyof typeof ELEMENT_ROLE_TOKENS]: (typeof ELEMENT_ROLE_TOKENS)[Role][keyof (typeof ELEMENT_ROLE_TOKENS)[Role]];
}[keyof typeof ELEMENT_ROLE_TOKENS];

export type DesignSystemColorToken = ElementRoleToken | (typeof DESIGN_SYSTEM_COLOR_TOKENS)[number];
export type ExplorerColorToken = DesignSystemColorToken;

const CSS_TOKEN_FALLBACKS: Partial<Record<DesignSystemColorToken, string>> = {
  "--capability": "#bbdefb",
  "--capability-ink": "#1565c0",
  "--capability-tint": "#e3f2fd",
  "--requirement": "#673ab7",
  "--requirement-ink": "#512da8",
  "--requirement-tint": "#ede7f6",
  "--contract": "#ff9800",
  "--contract-ink": "#e65100",
  "--contract-tint": "#fff3e0",
  "--semantic-contract": "#d32f2f",
  "--semantic-contract-ink": "#b71c1c",
  "--semantic-contract-tint": "#ffebee",
  "--verification": "#4caf50",
  "--verification-ink": "#2e7d32",
  "--verification-tint": "#e8f5e9",
  "--verification-objective": "#1b5e20",
  "--verification-objective-ink": "#0b3d12",
  "--verification-objective-tint": "#d7ead8",
  "--ontology": "#b08a00",
  "--ontology-ink": "#6f5600",
  "--ontology-tint": "#f4e3a1",
  "--resource": "#ffca28",
  "--resource-ink": "#8d6e00",
  "--resource-tint": "#fff3cf",
  "--other": "#9e9e9e",
  "--other-ink": "#616161",
  "--other-tint": "#ececec",
  "--node-generic-fill": "#eceff1",
  "--edge-default": "#c0c8d4",
  "--edge-derive": "#673ab7",
  "--edge-satisfy": "#4caf50",
  "--edge-trace": "#97a2b4",
  "--edge-attach": "#2196f3",
  "--bg-canvas": "#fbfaf7",
  "--bg-sunken": "#f3f1eb",
  "--bg-surface": "#ffffff",
  "--border-default": "#d8d2c6",
  "--accent": "#e11d48",
  "--accent-ring": "rgba(225,29,72,0.32)",
  "--rdf-class": "#94a3b8",
  "--rdf-objprop": "#64748b",
  "--rdf-dtprop": "#0f766e",
  "--rdf-rdfprop": "#115e59",
  "--rdf-individual": "#7c3aed",
  "--rdf-datatype": "#d6a43f",
  "--rdf-restriction": "#cbd5e1",
  "--rdf-classexpr": "#e2e8f0",
  "--rdf-nodeshape": "#dc2626",
  "--rdf-propshape": "#be123c",
  "--rdf-resource": "#14b8a6",
  "--rdf-shacl": "#ef4444",
  "--success": "#1f9d57",
  "--slate-0": "#ffffff",
  "--slate-950": "#0d1119",
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
  if (normalizedType.includes("ontology") || normalizedFamily === "ontology") return "ontology";
  if (normalizedType === "evidence-file" || normalizedType.includes("evidence")) return "other";
  if (normalizedType === "concept-reference" || normalizedType.includes("concept-reference")) return "resource";
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
