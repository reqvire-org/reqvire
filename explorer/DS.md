# Design System

Reference for the Reqvire Explorer design system: architecture, token contract,
component catalogue, palette API, build pipeline, and lint checks.

---

## Architecture

The design system is a self-contained subdirectory (`design-system/`) inside
the Explorer workspace. It ships:

- **CSS tokens** — the complete visual language as CSS custom properties
- **TSX components** — all UI primitives as typed React components
- **Palette API** — a TypeScript module for programmatic color access
- **Barrel export** — a single `index.ts` that re-exports every component and
  palette symbol; the only import surface the app should use

The app consumes the design system in two ways:

| Surface | How |
|---------|-----|
| CSS | `src/design-system.css` does a single `@import "../design-system/styles.css"` |
| Components / palette | `import { … } from "@ds"` (path alias → `design-system/index.ts`) |

No third-party UI framework (Tailwind, Radix Themes, etc.) is in the
dependency tree. All visual primitives are owned here.

---

## Directory layout

```
design-system/
├── index.ts                   # Barrel — the only public import surface
├── palette.ts                 # Programmatic color API
├── styles.css                 # CSS entry point (@import only)
│
├── tokens/
│   ├── fonts.css              # @font-face for Geist + Geist Mono
│   ├── colors.css             # Color ramps + semantic aliases + dark theme
│   ├── typography.css         # Type scale, weights, line heights, .rq-eyebrow / .rq-mono
│   ├── spacing.css            # 4px grid, radii, control heights, icon sizes, layout
│   ├── elevation.css          # Shadows, motion, z-index stack
│   └── base.css               # Minimal reset + body defaults
│
├── components/
│   ├── components.css         # Aggregates all component CSS (imported by styles.css)
│   ├── core/                  # Alert · Badge · Button · Card · Icon · IconButton · Modal
│   ├── data/                  # Chip · ElementIcon · RelationPill · Stat · Table · TypeBadge
│   ├── controls/              # SearchInput · SegmentedControl · Tabs · ToggleRow
│   └── navigation/            # Breadcrumb · SidebarSection · TreeItem
│
├── assets/
│   └── fonts/                 # Geist-Variable.woff2, GeistMono-Variable.woff2
│
├── vite.bundle.config.ts      # Builds _ds_bundle.js (standalone kit only)
├── _ds_bundle.js              # Generated IIFE bundle (not used by the app build)
└── _adherence.oxlintrc.json   # Generated import-contract config (oxlint)
```

---

## Token system

Every CSS value in the app must reference a token via `var(--token)`. Raw `px`
values, hex colors, font stacks, numeric font weights, durations, and easing
functions are allowed **only** inside custom-property declarations and at-rule
conditions. Anything else is a lint error.

### Token taxonomy and naming

Design tokens are organized into three layers. Keep the layers distinct:

| Layer | What it means | Example |
|-------|---------------|---------|
| Primitive tokens | Raw choices in the visual language | `--rose-600`, `--space-8`, `--text-sm` |
| Semantic tokens | Product decisions that explain purpose | `--accent`, `--bg-surface`, `--text-muted`, `--border-focus` |
| Component tokens | Narrow component knobs that still resolve to semantic intent | `--rq-tabs-border-bottom`, `--dialog-w` |

Primitive tokens are the available options. Semantic tokens are the choices the
interface is allowed to make. Components and app CSS should normally use the
semantic layer, because the token name must explain **why** the value exists,
not only what value it currently holds.

Do not name tokens after raw color or scale ideas when the usage has product
meaning. Use examples like this when reviewing token additions:

```css
/* bad — describes a paint chip, not intent */
--action-fill-red: var(--rose-600);

/* good — describes where and why the color is used */
--action-primary-bg: var(--accent);
```

The same rule applies beyond color. A spacing token like
`--spacing-small` is usually too vague: it does not say whether it is for a
gap, an inset, a row height, or a readable content measure. Prefer names that
describe the relationship being spaced:

```css
/* bad — only says the value is small */
--layout-space-small: var(--space-2);

/* good — says what relationship the space controls */
--gap-icon-label: var(--space-2);
--inset-control-inline: var(--space-4);
--inset-control-block: var(--space-2);
--gap-section-stack: var(--space-8);
```

Aim for the middle of the specificity range:

- **Too generic**: a token can be reused everywhere, but changing it affects
  unrelated UI. Example: one `--spacing-small` controlling icon gaps, card
  padding, table cell padding, and toolbar rhythm.
- **Too specific**: a design change requires editing many duplicate tokens.
  Example: `--button-padding-top`, `--search-input-padding-top`,
  `--menu-item-padding-top`, and `--table-cell-padding-top` all changing
  together for one density adjustment.
- **Right level**: the token captures a reusable design relationship with a
  predictable blast radius. Example: `--inset-control-y` changes vertical
  padding for control-like components without touching card layout.

Before adding a token, ask two questions:

1. If this value changes, will the affected UI match the token name's intent?
2. If a broad design adjustment happens, will this token scale across the
   relevant use cases without requiring many parallel edits?

If either answer is no, rename or re-scope the token before adding it.

### Layout intent

Use normal document flow as the default layout model: flex, grid, intrinsic
sizing, and responsive constraints. This is the code equivalent of making
layout intent explicit: alignment, gaps, wrapping, and resizing behavior are
visible in the structure.

Use `position: absolute` only when the element is intentionally removed from
flow, such as:

- notification/count badges anchored to a parent
- modal scrims and floating overlays
- popovers/tooltips
- canvas/SVG labels that are positioned by renderer coordinates

Do not use absolute positioning for ordinary page, panel, card, toolbar, form,
or list layout. If a layout can be expressed with flex or grid, use flex or
grid.

### Design file hygiene

Every design-system example, mock, and translated production surface should
answer these checks before it is considered ready:

- **Name everything semantically.** Layers, components, variants, tokens, and
  exported parts must be named for purpose, not appearance. A reader should be
  able to tell what a thing does and where it belongs without inspecting its
  current color, size, or coordinates.
- **Use flow layout and variables.** Layout intent must be encoded through
  flow primitives, responsive constraints, and token variables. Spacing,
  sizing, colors, typography, states, and density should come from variables so
  the design can change without editing each instance.
- **Annotate behavior and states.** Document interaction states, empty states,
  loading states, disabled states, responsive behavior, and any intentionally
  absolute overlays. If a behavior would not be obvious from the component
  name and tokens, annotate it.

### Loading order

`design-system/styles.css` imports tokens in this order — each layer can
reference tokens defined earlier:

1. `fonts.css` — `@font-face` declarations (no tokens needed)
2. `colors.css` — primitive ramps, then semantic aliases (`--bg-*`, `--text-*`, `--border-*`, etc.)
3. `typography.css` — `--font-sans/mono`, type scale, weights, line heights
4. `spacing.css` — spacing grid, radii, control heights, layout constants
5. `elevation.css` — shadows, motion, z-index
6. `base.css` — `*` reset + `body` defaults
7. `components/components.css` — all component styles

### Colors (`tokens/colors.css`)

Three primitive ramps plus semantic aliases:

| Ramp | Purpose |
|------|---------|
| `--slate-0` … `--slate-950` | Neutral text and dark surfaces |
| `--warm-0` … `--warm-300` | Product surfaces (canvas, panels, hovers) |
| `--rose-50` … `--rose-800` | Brand accent (interactive chrome only) |

**Six element-type semantic hues** — never repurpose for decoration:

| Token | Color | Used for |
|-------|-------|---------|
| `--capability` | blue `#bbdefb` | Capability nodes/chips |
| `--requirement` | deep purple `#673ab7` | Requirement nodes/chips |
| `--refinement` | orange `#ff9800` | All refinement sub-types |
| `--verification` | green `#4caf50` | All verification sub-types |
| `--ontology` | gold `#b08a00` | Ontology nodes |
| `--resource` | amber `#ffca28` | Files / resources |

Each type ships three channels: `--<type>` (fill), `--<type>-ink` (text on
tint), `--<type>-tint` (light chip background). The `-tint-d` variant is the
dark-mode tint; dark theme activates it automatically via the overrides in
`colors.css`.

**Semantic aliases** (always use these in components, not raw ramps):

- Surfaces: `--bg-canvas`, `--bg-surface`, `--bg-raised`, `--bg-overlay`, `--bg-sunken`, `--bg-hover`, `--bg-active`
- Text: `--text-strong`, `--text-body`, `--text-secondary`, `--text-muted`, `--text-faint`, `--text-inverse`, `--text-link`, `--text-code`
- Borders: `--border-subtle`, `--border-default`, `--border-strong`, `--border-focus`
- Accent: `--accent`, `--accent-hover`, `--accent-active`, `--accent-fg`, `--accent-subtle`, `--accent-ring`
- Selection: `--bg-selected`, `--border-selected`
- Graph edges: `--edge-default`, `--edge-derive`, `--edge-satisfy`, `--edge-trace`, `--edge-attach`
- Extended RDF palette: `--rdf-class`, `--rdf-objprop`, `--rdf-dtprop`, `--rdf-nodeshape`, `--rdf-propshape`, etc.

**Dark theme** — activated by `data-theme="dark"` or `.dark` on any ancestor
element. All semantic aliases re-resolve to navy-slate surfaces (`#0e141d` →
`#1e2836`). Tints switch to their `-d` alpha variants. No JavaScript required.

### Typography (`tokens/typography.css`)

**Fonts** — Geist (UI/display) and Geist Mono (IDs, paths, type slugs). Both
are variable `.woff2` files vendored in `design-system/assets/fonts/` and
served via relative `@font-face` URLs in `tokens/fonts.css`. No CDN.

**Type scale** — the product is dense; default UI text is `--text-sm` (13px):

| Token | Size | Use |
|-------|------|-----|
| `--text-micro` | 11px | Eyebrow / section labels (uppercase) |
| `--text-caption` | 12px | Meta, counts, tags |
| `--text-sm` | 13px | Default UI text — rows, controls, chips |
| `--text-base` | 14px | Body / reading copy in panels |
| `--text-lg` | 17px | Card titles, panel headings |
| `--text-xl` | 21px | Dialog titles, view headers |
| `--text-2xl` | 26px | Big numbers, hero counts |

Weights: `--weight-regular` (400), `--weight-medium` (500), `--weight-semibold`
(600), `--weight-bold` (700).

Two utility classes are defined here:
- `.rq-eyebrow` — 11px semibold uppercase with `0.07em` tracking; sidebar section labels
- `.rq-mono` — Geist Mono at 0.92em; IDs, paths, relation slugs

### Spacing (`tokens/spacing.css`)

4px base grid. Key tokens:

- **Grid**: `--space-1` (2px) … `--space-32` (64px)
- **Radii**: `--radius-xs` (4px) → `--radius-xl` (14px) → `--radius-pill` (999px)
- **Control heights**: `--control-xs` (22px), `--control-sm` (28px), `--control-md` (34px), `--control-lg` (40px)
- **Icon sizes**: `--icon-xs` (13px) → `--icon-lg` (20px)
- **Layout**: `--sidebar-w` (300px), `--rail-w` (52px), `--header-h` (52px), `--content-max` (1180px)
- **Borders**: `--border-w` (1px), `--border-w-thick` (2px), `--border-w-heavy` (3px), `--focus-w` (2px)

### Elevation (`tokens/elevation.css`)

- **Shadows**: `--shadow-xs` → `--shadow-xl`. Cards use `xs/sm`; popovers/modals use `lg/xl`. Dark mode re-pitches all shadows to black alpha.
- **Motion**: `--dur-fast` (130ms), `--dur-base` (200ms), `--dur-slow` (320ms). Easing: `--ease-standard` (`cubic-bezier(0.2,0,0,1)`). Quick and mechanical — no bounce.
- **Z-index stack**: `--z-rail` (20) → `--z-sticky` (40) → `--z-popover` (60) → `--z-overlay` (80) → `--z-modal` (90) → `--z-toast` (100)

---

## Component catalogue

All components are TSX, located in `design-system/components/`, and exported
from `design-system/index.ts`. Every component accepts standard HTML attributes
on its root element unless noted.

### Core (`components/core/`)

| Component | Description |
|-----------|-------------|
| `Alert` | Inline status message strip — info / warning / danger / success variants |
| `Badge` | Compact count or label pill |
| `Button` | Primary / secondary / ghost / danger; `sm` / `md` / `lg` sizes |
| `Card` | White surface card with 1px border and xs shadow; selected state adds accent ring |
| `Icon` | Lucide-geometry SVG icons. Props: `name` (see `ICON_NAMES`), `size`, `className` |
| `IconButton` | Icon-only button wrapper — ghost / subtle / primary; same sizes as Button |
| `Modal` | Radix Dialog-backed overlay. Sub-components: `ModalContent`, `ModalHeader`, `ModalTitle`, `ModalDescription`, `ModalBody`, `ModalFooter`, `ModalClose` |

### Data (`components/data/`)

| Component | Description |
|-----------|-------------|
| `Chip` | Small inline label, optionally with a left dot swatch |
| `ElementIcon` | Colored model-element glyph — square / diamond / hub shape; sized `sm/md/lg` |
| `RelationPill` | Relation row chip: kind label + colored pip + target label; optional href |
| `Stat` / `StatRow` | Key-value stat pair; `StatRow` renders a horizontal run of stats |
| `Table` | Full table set: `TableViewport` (scroll container), `TableHead`, `TableBody`, `TableRow`, `TableHeader`, `TableCell`, `TableSortButton` |
| `TypeBadge` | Element-type badge with optional tinted fill; uses the element-type color system |

### Controls (`components/controls/`)

| Component | Description |
|-----------|-------------|
| `SearchInput` | Controlled text input with a leading search icon and optional clear button |
| `SegmentedControl` | Mutually exclusive button group; active segment gets dark-ink fill |
| `Tabs` | Top tab bar with active underline; each item is `{ id, label }` |
| `ToggleRow` | Labeled boolean toggle (checkbox-backed) for filter/overlay panels |

### Navigation (`components/navigation/`)

| Component | Description |
|-----------|-------------|
| `Breadcrumb` | Slash-delimited path bar; last item is active (non-link) |
| `SidebarSection` | Collapsible section with `.rq-eyebrow` header and optional action slot |
| `TreeItem` | Indented tree row: indent level, expand chevron, icon slot, label, optional count |

---

## Palette API (`design-system/palette.ts`)

Exported via `@ds`. Provides programmatic access to element-type colors without
hardcoding token strings.

### Types

| Type | Values |
|------|--------|
| `ElementRole` | `"capability"`, `"requirement"`, `"refinement"`, `"source"`, `"constraint"`, `"behavior"`, `"state"`, `"input-output"`, `"verification"`, `"specification"`, `"semantic-contract"`, `"ontology"`, `"resource"`, `"other"` |
| `ElementType` | `ElementRole` minus `"other"` |
| `PaletteChannel` | `"fill"` \| `"ink"` \| `"tint"` |
| `DesignSystemColorToken` | Union of every valid color token string (`--capability`, `--text-body`, etc.) |
| `ElementIconShape` | `"square"` \| `"diamond"` \| `"hub"` |

### Constants

```ts
ELEMENT_ROLE_TOKENS  // { capability: { fill, ink, tint }, requirement: { … }, … }
ELEMENT_TYPES        // { capability: { color, shape, role }, … }
DESIGN_SYSTEM_COLOR_TOKENS  // readonly tuple of every non-role color token
```

### Functions

```ts
// Resolve an element type/family string to its canonical role
elementRole(type?: string | null, family?: string | null): ElementRole

// Get the CSS token name for a role + channel
roleColorToken(role: string | null | undefined, channel?: PaletteChannel): DesignSystemColorToken

// Get the resolved CSS value string for a role + channel
roleColorValue(role: string, channel?: PaletteChannel): string

// Resolve a token to its computed value (browser) or var(...) fallback (SSR)
cssVar(token: DesignSystemColorToken): string

// Replace var(--token) references with resolved hex values safe for Mermaid
replaceCssVarsForMermaid(source: string): string

// Emit Mermaid classDef lines for all element types (call after DOM ready)
getMermaidClassDefs(): readonly string[]
```

`cssVar()` and `getMermaidClassDefs()` must be called in a browser context with
the design system CSS loaded so that `getComputedStyle` can resolve tokens.
`replaceCssVarsForMermaid()` normalizes computed colors to hex for Mermaid's
parser, which does not accept `var()`.

---

## Build pipeline

### Application build

```
npm run build
```

Runs in sequence:

1. **`npm run lint`** — adherence check + style check (see Lint section below)
2. **`tsc --noEmit`** — TypeScript type check
3. **`npm run build:ds-bundle`** — builds `design-system/_ds_bundle.js` (standalone kit)
4. **`vite build`** — emits `dist/` with deterministic asset names (`assets/explorer.js`, `assets/explorer.css`)

The build fails at the first failing step. All four must pass before `dist/` is emitted.

### DS bundle (`_ds_bundle.js`)

```
npm run build:ds-bundle
```

Configured in `design-system/vite.bundle.config.ts`. Produces an IIFE
(`ReqvireExplorerDesignSystem_48409e`) from `design-system/index.ts`. React and
ReactDOM are external globals. The bundle is used by the standalone design
system HTML kit (`design-system/Example.html`) — it is **not** part of the
application bundle that `vite build` produces.

---

## Lint and checks

```
npm run lint
```

Runs two checks sequentially.

### 1. Adherence (import contract)

```
npm run lint:adherence
```

```
npm run generate:adherence && oxlint --deny-warnings --config design-system/_adherence.oxlintrc.json src design-system
```

`scripts/generate-adherence-config.mjs` reads `design-system/index.ts`, extracts
every exported PascalCase component name, then writes `_adherence.oxlintrc.json`
with a `no-restricted-imports` rule that bans direct imports from any component
path. The rule forces all imports to go through `@ds`.

Banned import patterns:
```
design-system/components/**
../design-system/components/**
components/controls/**
components/core/**
components/data/**
components/navigation/**
ui_kits/explorer/**
```

The barrel file itself (`design-system/index.ts`) is exempted from the rule.
The config is regenerated on every lint run so it always reflects the current
barrel.

### 2. Style checks

```
npm run lint:style
```

```
stylelint "src/**/*.css" && node scripts/lint-style-tokens.mjs
```

**stylelint** checks property/selector correctness via `.stylelintrc.json`.

**Style token guard** (`scripts/lint-style-tokens.mjs`) enforces the token
contract on all CSS files under `src/`. It walks every `.css` file, strips
comments, blanks out custom-property declarations and at-rule conditions (where
raw values are legitimate), then runs six pattern checks on the remainder:

| Check | What it flags |
|-------|--------------|
| `raw-px` | Raw `px` lengths not inside a custom-property or at-rule |
| `raw-color` | Hex colors, `rgb()`, `hsl()`, `oklch()`, etc. |
| `raw-font-family` | `font-family:` not starting with `var(` or `inherit` |
| `raw-font-weight` | `font-weight:` with a numeric value |
| `raw-duration` | Raw `ms` / `s` durations |
| `raw-easing` | `cubic-bezier(` or `ease(-in|-out|-in-out)` |

Any finding exits 1. There is no baseline or grandfather list — the check is
either clean or it fails.

---

## How to use

### Importing components

```tsx
import { Button, Icon, TypeBadge, ElementIcon } from "@ds";
import { cssVar, elementRole, roleColorValue } from "@ds";
import type { ElementRole, PaletteChannel } from "@ds";
```

Never import from a component path directly:

```tsx
// wrong
import { Button } from "../../design-system/components/core/Button";

// correct
import { Button } from "@ds";
```

### Using tokens in CSS

```css
/* correct — token reference */
.my-panel {
  background: var(--bg-surface);
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-md);
  padding: var(--space-8);
  font-size: var(--text-sm);
  color: var(--text-body);
  box-shadow: var(--shadow-sm);
  transition: background var(--dur-fast) var(--ease-standard);
}

/* wrong — raw values */
.my-panel {
  background: #f8f6f1;
  border: 1px solid #d8d2c6;
  border-radius: 8px;
  padding: 16px;
}
```

Custom properties are the one allowed location for raw values:

```css
/* allowed — defining a token */
:root {
  --dialog-w: 1120px;
  --sidebar-custom: 260px;
}
```

### Element-type color in TSX

```tsx
import { roleColorValue, elementRole } from "@ds";

// Get a CSS value string for the fill of a given element type
const fill = roleColorValue("requirement", "fill");     // "var(--requirement)"
const tint = roleColorValue("verification", "tint");    // "var(--verification-tint)"

// Resolve a raw type string from the store to a canonical role
const role = elementRole(element.element_type, element.type_family);
```

### Rendering an element glyph

```tsx
import { ElementIcon } from "@ds";

<ElementIcon type="verification" size="md" />
<ElementIcon type="refinement" size="sm" />
```

### Dark mode

Apply `data-theme="dark"` to `<html>` or any container. All `--bg-*`,
`--text-*`, `--border-*`, and type-tint tokens re-resolve automatically via CSS.
No JavaScript color recalculation needed.

### Adding a new component

1. Create `design-system/components/<group>/MyComponent.tsx`
2. Add exports to `design-system/index.ts` (component + types)
3. Run `npm run lint:adherence` — the config regenerates from the barrel
4. Style the component using only `var(--token)` references
5. Run `npm run lint:style` to verify the CSS is token-clean

---

## Dependencies

The design system itself has no runtime dependencies beyond React. The application
workspace dependencies relevant to the design system:

| Package | Role |
|---------|------|
| `react`, `react-dom` | Component runtime |
| `@vitejs/plugin-react` | JSX transform for dev + build |
| `oxlint` | Import contract enforcement |
| `stylelint` | CSS property/selector lint |
| `vite` | Dev server, application build, DS bundle |
| `typescript` | Type checking |

Removed in the current version: `tailwindcss`, `@tailwindcss/vite`,
`lucide-react`, `clsx`, `tailwind-merge`, `@radix-ui/react-icons`,
`@radix-ui/themes`.
