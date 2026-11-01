// Renderer colors for Canvas, WebGL and SVG drawing. Read a palette once per render or
// context change with paletteFor(element), look colors up inside per-item loops, and
// redraw from observePalette(element, callback). Nothing here inserts DOM nodes.
import {
  brands,
  contexts,
  defaultContext,
  palette,
  systemThemes,
  themes,
  type ColorToken,
  type Palette,
  type PaletteColor,
  type PaletteContext,
} from "./tokens/themes.generated";

export type { ColorToken, Palette, PaletteColor, PaletteContext };

const themeSelector = themes.map((name) => `[data-theme="${name}"]`).join(", ");
const brandSelector = brands.map((name) => `[data-brand="${name}"]`).join(", ");
const darkScheme = "(prefers-color-scheme: dark)";

/** The nearest ancestor (or the element itself) matching a selector, crossing shadow roots. */
function closest(element: Element, selector: string): Element | null {
  let node: Element | null = element;
  while (node) {
    const found = node.closest(selector);
    if (found) return found;
    const root = node.getRootNode();
    node = typeof ShadowRoot !== "undefined" && root instanceof ShadowRoot ? root.host : null;
  }
  return null;
}

function systemTheme(): string | null {
  // Like the generated sheet, the OS scheme applies only to a root without data-theme.
  if (!systemThemes || typeof matchMedia !== "function" || document.documentElement.hasAttribute("data-theme")) return null;
  return matchMedia(darkScheme).matches ? systemThemes.dark : systemThemes.light;
}

/** The palette context an element renders in: its nearest declared data-theme and data-brand,
 * else the OS color scheme (with followSystemTheme) or the default theme and brand. */
export function paletteContext(element: Element): PaletteContext {
  const theme = closest(element, themeSelector)?.getAttribute("data-theme") ?? systemTheme() ?? themes[0];
  const brand = brandSelector ? closest(element, brandSelector)?.getAttribute("data-brand") : null;
  const key = brand && brand !== (brands as readonly string[])[0] ? `${brand}/${theme}` : theme;
  return (contexts as readonly string[]).includes(key) ? (key as PaletteContext) : defaultContext;
}

/** Every color token resolved for the element's context. Read it once per render, outside
 * per-node and per-edge loops; it is a static object, so lookups cost nothing. */
export function paletteFor(element: Element): Palette {
  return palette[paletteContext(element)];
}

// Colors outside the palette, normalized once by one offscreen 2D context.
const cacheSize = 256;
const colors = new Map<string, PaletteColor | null>();
let context2d: OffscreenCanvasRenderingContext2D | CanvasRenderingContext2D | null | undefined;

type Subscriber = { element: Element; callback: (palette: Palette, context: PaletteContext) => void; context: PaletteContext };
const subscribers = new Set<Subscriber>();
let observer: MutationObserver | null = null;
let media: MediaQueryList | null = null;

function refresh(records?: MutationRecord[]) {
  let changed = false;
  for (const subscriber of subscribers) {
    if (!subscribers.has(subscriber)) continue; // unsubscribed by an earlier callback
    // Only an attribute change on the element or an ancestor can change its context.
    if (records && subscriber.element.getRootNode() === document && !records.some((record) => record.target === subscriber.element || record.target.contains(subscriber.element))) continue;
    const context = paletteContext(subscriber.element);
    if (context === subscriber.context) continue;
    subscriber.context = context;
    changed = true;
    subscriber.callback(palette[context], context);
  }
  if (changed) colors.clear();
}
const onMedia = () => refresh();

/** Calls back with the new palette whenever the element's effective context changes: a
 * data-theme or data-brand change on it or an ancestor, or the OS color scheme with
 * followSystemTheme. Returns the unsubscribe function. One shared observer serves all. */
export function observePalette(element: Element, callback: (palette: Palette, context: PaletteContext) => void): () => void {
  const subscriber: Subscriber = { element, callback, context: paletteContext(element) };
  subscribers.add(subscriber);
  if (!observer && typeof MutationObserver === "function") {
    observer = new MutationObserver((records) => refresh(records));
    observer.observe(document.documentElement, { subtree: true, attributes: true, attributeFilter: ["data-theme", "data-brand"] });
  }
  if (!media && systemThemes && typeof matchMedia === "function") {
    media = matchMedia(darkScheme);
    media.addEventListener("change", onMedia);
  }
  return () => {
    subscribers.delete(subscriber);
    if (subscribers.size) return;
    observer?.disconnect();
    observer = null;
    media?.removeEventListener("change", onMedia);
    media = null;
  };
}

function canvas2d() {
  if (context2d !== undefined) return context2d;
  const canvas = typeof OffscreenCanvas === "function" ? new OffscreenCanvas(1, 1) : typeof document !== "undefined" ? Object.assign(document.createElement("canvas"), { width: 1, height: 1 }) : null;
  context2d = (canvas?.getContext("2d", { willReadFrequently: true }) as OffscreenCanvasRenderingContext2D | CanvasRenderingContext2D | null | undefined) ?? null;
  return context2d;
}

const hex = (value: number) => value.toString(16).padStart(2, "0");
function fromChannels(r: number, g: number, b: number, a: number): PaletteColor {
  const alpha = Math.round(a * 10000) / 10000;
  return { css: alpha >= 1 ? `#${hex(r)}${hex(g)}${hex(b)}` : `rgba(${r}, ${g}, ${b}, ${alpha})`, rgba: [r, g, b, alpha] };
}

function normalize(value: string): PaletteColor | null {
  const ctx = canvas2d();
  if (!ctx) return null;
  // An invalid color leaves fillStyle unchanged; two sentinels tell it apart from a valid one.
  ctx.fillStyle = "#000000";
  ctx.fillStyle = value;
  const first = String(ctx.fillStyle);
  ctx.fillStyle = "#ffffff";
  ctx.fillStyle = value;
  if (first !== String(ctx.fillStyle)) return null;
  const legacy = first.match(/^#([\da-f]{6})$/i) ?? first.match(/^rgba\((\d+), (\d+), (\d+), ([\d.]+)\)$/);
  if (legacy) return legacy.length === 2 ? fromChannels(parseInt(legacy[1].slice(0, 2), 16), parseInt(legacy[1].slice(2, 4), 16), parseInt(legacy[1].slice(4), 16), 1) : fromChannels(Number(legacy[1]), Number(legacy[2]), Number(legacy[3]), Number(legacy[4]));
  // Wider color syntax: draw one pixel; the canvas clips to sRGB.
  ctx.clearRect(0, 0, 1, 1);
  ctx.fillRect(0, 0, 1, 1);
  const [r, g, b, a] = ctx.getImageData(0, 0, 1, 1).data;
  return fromChannels(r, g, b, a / 255);
}

/** A color for a value outside the palette: a palette token (--name or var(--name)) from the
 * element's context, currentColor or another token from the element's computed style, or any
 * absolute CSS color. Absolute strings are normalized once by an offscreen canvas and kept in a
 * bounded cache; the cache clears when an observed context changes. Null when not a color. */
export function resolveColor(element: Element, value: string): PaletteColor | null {
  let text = value.trim();
  const token = text.match(/^var\(\s*(--[\w-]+)\s*\)$/)?.[1] ?? (text.startsWith("--") ? text : null);
  if (token) {
    const own = (paletteFor(element) as Record<string, PaletteColor | undefined>)[token];
    if (own) return own;
    text = getComputedStyle(element).getPropertyValue(token).trim();
  } else if (/^currentcolor$/i.test(text) || text.includes("var(")) {
    const style = getComputedStyle(element);
    text = /^currentcolor$/i.test(text) ? style.color : text.replace(/var\(\s*(--[\w-]+)\s*\)/g, (_match, name: string) => style.getPropertyValue(name).trim());
  }
  if (!text) return null;
  if (colors.has(text)) {
    const cached = colors.get(text) ?? null;
    colors.delete(text);
    colors.set(text, cached);
    return cached;
  }
  const color = normalize(text);
  colors.set(text, color);
  if (colors.size > cacheSize) colors.delete(colors.keys().next().value as string);
  return color;
}

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
} as const satisfies Record<ElementRole, Record<PaletteChannel, ColorToken>>;

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
] as const satisfies readonly ColorToken[];

type ElementRoleToken = {
  [Role in keyof typeof ELEMENT_ROLE_TOKENS]: (typeof ELEMENT_ROLE_TOKENS)[Role][keyof (typeof ELEMENT_ROLE_TOKENS)[Role]];
}[keyof typeof ELEMENT_ROLE_TOKENS];

export type DesignSystemColorToken = ElementRoleToken | (typeof DESIGN_SYSTEM_COLOR_TOKENS)[number];
export type ExplorerColorToken = DesignSystemColorToken;

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

/** A design-system color token resolved for the element's context (the document root by
 * default) as #rrggbb, or #rrggbbaa with alpha; var(<token>) without a document. Reads the
 * generated palette: no DOM insertion or computed-style read per call, so read colors once per
 * render pass with paletteFor(element) where many are needed. */
export function cssVar(token: DesignSystemColorToken, element?: Element): string {
  if (typeof document === "undefined") return `var(${token})`;
  // Untyped callers may name a token outside the palette: it stays a var() reference.
  const color = (paletteFor(element ?? document.documentElement) as Record<string, PaletteColor | undefined>)[token];
  return color ? hexColor(color) : `var(${token})`;
}

/** A palette color as #rrggbb, or #rrggbbaa with alpha, for renderers that take hex strings. */
export function paletteHex(colors: Palette, token: ColorToken): string {
  return hexColor(colors[token]);
}

/** Mermaid source with var(--token) references replaced by the element's palette colors (the
 * document root by default). Re-render from observePalette when the element's theme changes. */
export function replaceCssVarsForMermaid(source: string, element?: Element) {
  if (typeof document === "undefined") return source;
  const root = element ?? document.documentElement;
  const colors = paletteFor(root) as Record<string, PaletteColor | undefined>;
  return source.replace(/var\((--[a-z0-9-]+)\)/gi, (match, token: string) => {
    const color = colors[token] ?? resolveColor(root, match);
    return color ? hexColor(color) : match;
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
  return cssVar(token);
}

function hexColor({ rgba: [r, g, b, a] }: PaletteColor): string {
  const channels = `#${hex(r)}${hex(g)}${hex(b)}`;
  return a >= 1 ? channels : `${channels}${hex(Math.max(0, Math.min(255, Math.round(a * 255))))}`;
}
