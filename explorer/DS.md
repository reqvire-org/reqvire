# Design System

Reference for the Reqvire Explorer design system: architecture, token contract,
component catalogue, palette API, build pipeline, and lint checks.

---

## Architecture

The design system is a self-contained subdirectory (`design-system/`) inside
the Explorer workspace. It ships:

- **CSS tokens** — the complete visual language as CSS custom properties
- **TSX components** — UI primitives and reusable UX/product patterns as typed React components
- **Palette API** — a TypeScript module for programmatic color access
- **Barrel export** — a single `index.ts` that re-exports every component,
  product pattern, and palette symbol; the only import surface the app should
  use

### Ownership contract

The namespace boundary is strict:

- `ds-*` classes and `--ds-*` component variables are owned by reusable
  primitives under `design-system/components/`.
- `ux-*` classes and `--ux-*` product variables are owned by the reusable
  UX/product-pattern layer: `design-system/product-patterns/`.
- Application code MUST NOT emit new `ds-*` hooks.
- After migration, application code MUST NOT emit `ux-*` hooks either; `src/`
  consumes product patterns through props, data, state, callbacks, and
  composition.
- Primitive design-system components MUST NOT emit `ux-*` hooks.
- Product patterns MUST NOT emit `ds-*` hooks. They customize primitives only
  through documented props or documented `--ds-*` variables on `ux-*` wrappers.
- Product patterns and showcase examples MUST NOT assign primitive
  interaction/state policy variables such as `--ds-*-sel-*`,
  `--ds-*-hover-*`, `--ds-*-active-*`, `--ds-*-focus-*`, or
  `--ds-*-off-*`. If a product pattern needs different selected, hover,
  active, focus, off, or disabled behavior, add a typed primitive prop or
  variant such as `density` or `variant` and implement that state CSS inside
  `design-system/components/`.
- After the product-pattern refactor, `src/` MUST NOT define visual CSS rules;
  it should own behavior, routing, store access, state, effects, workers, and
  callback wiring only.
- Application CSS and Linaria blocks MUST NOT target design-system internals
  such as `.ds-treeitem`, `.ds-tabs`, `.ds-search__input`, or
  `.ds-togglerow__label`.

UX/product patterns customize design-system primitives only through documented
props or documented `--ds-*` component variables. If a needed customization
does not exist, add it to the design-system component API instead of reaching
into its internal markup.

The app consumes the design system in two ways:

| Surface | How |
|---------|-----|
| CSS | `src/main.tsx` imports `../design-system/styles.css` directly |
| Components / product patterns / palette | `import { … } from "@ds"` (path alias → `design-system/index.ts`) |

The CSS entry is the only direct app import from `design-system/`. Component,
product-pattern, palette, and helper imports go through `@ds`; direct
implementation paths remain forbidden by the public import contract.

Explorer document mount mechanics currently live inline in `index.html`:
`html`, `body`, and `#root` height, plus the shell-level body overflow policy.
There is no `src/styles.css`, `src/global.css`, or `src/html.css` styling
layer. If `src/app-mount.css` is introduced, it is the only app-side CSS file
exception and must contain only the same mount bootstrap. Those names hide
ownership and invite product styling to accumulate outside component modules.
The architecture and CSS ownership guards keep document bootstrap separate
from product styling.

`@ds` is the only public TypeScript import surface. Application code MUST NOT
import from `@ds/*`, `design-system/*`, or component/product-pattern/palette
implementation paths directly.

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
│   ├── typography.css         # Type scale, weights, line heights, .ds-eyebrow / .ds-mono
│   ├── spacing.css            # 4px grid, radii, control heights, icon sizes, layout
│   ├── elevation.css          # Shadows, motion, z-index stack
│   └── base.css               # Minimal reset + body defaults
│
├── components/
│   ├── core/                  # Alert · Badge · BrandMark · Button · Card · Icon · IconButton · Modal
│   ├── data/                  # Chip · CodeRef · ElementIcon · RelationPill · Stat · Table · TypeBadge
│   ├── controls/              # SearchInput · SegmentedControl · Tabs · ToggleRow
│   └── navigation/            # Breadcrumb · SidebarSection · TreeItem
│
├── product-patterns/
│   ├── shell/                 # AppShell · ShellPane · PaneResizer · ShellMain · route frames
│   ├── chrome/                # WorkspaceToolbar and later pane chrome
│   ├── side-pane/             # Explorer side-pane frames, filters, trees, legends
│   ├── detail/                # Detail dialog bodies and relation/detail lists
│   ├── content/               # Document, markdown, diagram, and code preview frames
│   ├── thesaurus/             # Standalone SKOS concept browser and concept detail map
│   └── feedback/              # Product notices and help content
│
├── hooks/
│   └── useLatestRef.ts        # Stable event-callback ref for renderer boundaries
│
├── assets/
│   ├── fonts/                 # Geist-Variable.woff2, GeistMono-Variable.woff2
│   └── logo-mark.svg          # Canonical reusable Reqvire brand mark
│
└── vite.bundle.config.ts      # Builds standalone kit output when requested
```

Generated bundles, generated CSS, generated adherence config, and `dist-*`
directories are build/runtime artifacts. They MUST NOT be tracked as source in
this repository, with no release-artifact exception. Source of truth is TS/TSX,
CSS tokens, config, docs, and explicitly owned static assets.

### Asset ownership

The asset boundary is strict:

| Path | Source of truth for |
|------|---------------------|
| `design-system/assets/` | Assets consumed by design-system components or tokens, including fonts and reusable brand source SVGs |
| `.vite/generated-assets/` | Ignored build output generated from source assets, including favicons and app icons |
| `public/assets/` | Optional non-DS browser/runtime files that must be referenced by raw URL |
| `design-system/showcase/public/assets/` | Optional showcase-only mock public resources for raw URL use cases |
| `dist*/assets/` | Build/serve output assembled from source assets and generated runtime data |

Assets used by reusable components belong in `design-system/assets/`, not
`src/assets/`. Public assets are only for files that must be browser-addressable
by raw URL and are not reusable DS/brand assets. The app build merges
`design-system/assets/`, generated assets, and `public/assets/` into
`dist/assets/`; matching filenames are an error, not an override. Favicons and
platform app icons are generated from `design-system/assets/logo-mark.svg` by
`npm run generate:icons`; do not hand-edit or track those derived PNG/ICO files
as source. Real Explorer project data is served at `assets/project-store.js`
by `reqvire serve`; that runtime data file is not checked into
`public/assets/` or `design-system/assets/`. The showcase renders the real app
with `src/store/devFixture.ts` injected by
`design-system/showcase/MockShell.tsx`; showcase public resources belong
under `design-system/showcase/public/assets/` only when a mock must use a raw
browser URL.

---

## Token system

Every visual CSS value in the app and design-system source must reference a
token or component variable via `var(--token)`. Raw `px` values, hex colors,
font stacks, numeric font weights, durations, and easing functions are allowed
**only** inside token/custom-property declarations and at-rule conditions.
Anything else is a lint error.

The checks are intentionally conservative. If a new token category, CSS entry
layer, raw-value allowance, global selector, generated asset class, import
surface, or namespace exception is truly needed, the same change must update
the relevant lint guard and documentation. A passing guard is the approval
mechanism; do not bypass it with ad hoc exclusions or unguarded conventions.

Design-system components expose visual policy through semantic props and
`--ds-*` component variables. They MUST NOT expose arbitrary CSS color-string
props. Dynamic product coloring, such as ontology or graph role colors, is
mapped by the application into semantic roles, token names, or documented
component variables before it reaches a design-system primitive. Interaction
state policy remains primitive-owned: product patterns and showcase examples
may set documented context, density, and composition variables, but selected,
hover, active, focus, off, and disabled styling belongs in the primitive API.

### Token taxonomy and naming

Design tokens are organized into three layers. Keep the layers distinct:

| Layer | What it means | Example |
|-------|---------------|---------|
| Primitive tokens | Raw choices in the visual language | `--rose-600`, `--space-8`, `--text-sm` |
| Semantic tokens | Product decisions that explain purpose | `--accent`, `--bg-surface`, `--text-muted`, `--border-focus` |
| Component tokens | Narrow component knobs that still resolve to semantic intent | `--ds-tabs-border-bottom`, `--ds-modal-w` |

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

Component-specific styling lives with the owning TSX component through Linaria:
primitive components use `ds-*` hooks, while UX/product patterns use `ux-*`
hooks. Application `src/` code should not be a styling layer; it consumes
product patterns and owns behavior, routing, store access, state, and effects.
The only inline HTML style in
`index.html` and `design-system/showcase/index.html` is the document mount
bootstrap:

```css
html,
body,
#root {
  height: 100%;
}

body {
  overflow: hidden;
}
```

Body typography, colors, background, scrollbar styling, reset rules, and
selection styling belong to `design-system/tokens/base.css`. Product layout
dimensions belong to the shell or owning view/component module.

### Colors (`tokens/colors.css`)

Three primitive ramps plus semantic aliases:

| Ramp | Purpose |
|------|---------|
| `--slate-0` … `--slate-950` | Neutral text and dark surfaces |
| `--warm-0` … `--warm-300` | Product surfaces (canvas, panels, hovers) |
| `--rose-50` … `--rose-800` | Brand accent (interactive chrome only) |

**Element-type semantic hues** — never repurpose for decoration:

| Token | Color | Used for |
|-------|-------|---------|
| `--capability` | blue `#bbdefb` | Capability nodes/chips |
| `--requirement` | deep purple `#673ab7` | Requirement nodes/chips |
| `--contract` | orange `#ff9800` | All requirement-owned contract sub-types |
| `--verification` | green `#4caf50` | All verification sub-types |
| `--ontology` | gold `#b08a00` | Ontology nodes |
| `--concept` | yellow `#e7c94a` | Native SKOS concept elements |
| `--concept-scheme` | darker yellow `#cda83a` | Native SKOS concept-scheme roots |
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
- Extended RDF palette: `--rdf-class`, `--rdf-concept`, `--rdf-objprop`, `--rdf-dtprop`, `--rdf-nodeshape`, `--rdf-propshape`, etc. `--rdf-concept` is the canonical yellow SKOS graph-node token. Native Reqvire `concept` and `concept-scheme` elements use the semantic aliases `--concept` and `--concept-scheme` so element badges stay distinct from passive RDF notation. `--concept-reference` is the darker yellow token for concept-reference facts and graph edges from model elements to concept nodes.

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
- `.ds-eyebrow` — 11px semibold uppercase with `0.07em` tracking; sidebar section labels
- `.ds-mono` — Geist Mono at 0.92em; IDs, paths, relation slugs

### Spacing (`tokens/spacing.css`)

4px base grid. Key tokens:

- **Grid**: `--space-1` (2px) … `--space-32` (64px)
- **Radii**: `--radius-xs` (4px) → `--radius-xl` (14px) → `--radius-pill` (999px)
- **Control heights**: `--control-xs` (22px), `--control-sm` (28px), `--control-md` (34px), `--control-lg` (40px)
- **Icon sizes**: `--icon-xs` (13px) → `--icon-lg` (20px)
- **Layout**: `--navigation-rail-width` (52px), `--app-header-height` (52px), `--content-max` (1180px), `--row-height-compact` (30px), `--compact-column-min` (112px)
- **Stack rhythm**: `--stack-gap-compact` (`0`) — default vertical gap for contiguous menu, table, filter, legend, and relation rows
- **Borders**: `--border-w` (1px), `--border-w-thick` (2px), `--border-w-heavy` (3px), `--focus-w` (2px)

List-like UI is contiguous by default: menu items, table/list rows, filter
rows, legend rows, and relation rows sit directly next to each other
vertically. Use section margins, row padding, borders, or selection/hover fills
for structure; add vertical row gaps only when the component is explicitly a
card grid or tile layout.

Explorer shell pane widths are not design-system tokens. They are
application-owned `--ux-*` variables defined on the shell root:
`--ux-left-pane-width`, `--ux-left-pane-collapsed-width`, and
`--ux-current-left-width`.

### Elevation (`tokens/elevation.css`)

- **Shadows**: `--shadow-xs` → `--shadow-xl`. Cards use `xs/sm`; popovers/modals use `lg/xl`. Dark mode re-pitches all shadows to black alpha.
- **Effects**: filter/effect formulas such as `--filter-danger-hover`, `--filter-scrim-blur`, and `--filter-highlight-glow`. Raw `blur(...)`, `brightness(...)`, `drop-shadow(...)`, etc. belong here, not in component CSS.
- **Motion**: `--dur-fast` (130ms), `--dur-base` (200ms), `--dur-slow` (320ms). Easing: `--ease-standard` (`cubic-bezier(0.2,0,0,1)`). Quick and mechanical — no bounce.
- **Z-index stack**: `--z-rail` (20) → `--z-sticky` (40) → `--z-popover` (60) → `--z-overlay` (80) → `--z-modal` (90) → `--z-toast` (100)
- **Local layering**: `--z-local-base`, `--z-local-raised`, and `--z-local-overlay` for component-internal stacking. Raw numeric `z-index` values are not allowed in component/app CSS.

Token-only visual formulas are deliberate: color derivation (`color-mix(...)`),
filter/effect functions, raw shadow recipes, motion curves/durations, and
z-index numbers are source-of-truth values. Components consume semantic tokens
or component variables that resolve to these tokens.

---

## Component catalogue

All components are TSX, located in `design-system/components/`, and exported
from `design-system/index.ts`. Every component accepts standard HTML attributes
on its root element unless noted.

### Core (`components/core/`)

| Component | Description |
|-----------|-------------|
| `Alert` | Inline status message strip — default / info / warning / danger / success variants |
| `Badge` | Compact count or label pill |
| `BrandMark` | Reqvire brand mark image backed by `design-system/assets/logo-mark.svg`; decorative by default when requested |
| `Button` | `tone`: primary / accent / secondary / ghost / danger / link; `size`: `sm` / `md` / `lg` |
| `Card` | Tokenized raised surface card with optional padding, interactivity, selection, and accent strip |
| `Icon` | Lucide-geometry SVG icons. Props: `name` (see `ICON_NAMES`), `size`, `className` |
| `IconButton` | Icon-only button wrapper — `tone`: secondary / ghost; `size`: `sm` / `md`; optional active state |
| `Modal` | In-house portal-backed dialog overlay. Sub-components: `ModalContent`, `ModalHeader`, `ModalTitle`, `ModalDescription`, `ModalBody`, `ModalFooter`, `ModalClose` |

### Data (`components/data/`)

| Component | Description |
|-----------|-------------|
| `Chip` | Button-backed inline label with optional icon, count, and active state |
| `CodeRef` | Monospace code/path reference with optional line anchor and quiet wrapping |
| `ElementIcon` | Colored model-element glyph — square / diamond / hub shape; sized `sm/md/lg` |
| `RelationPill` | Relation row chip: kind label + `ElementIcon` marker + target label; optional href |
| `Stat` / `StatRow` | Key-value stat pair; `StatRow` renders a horizontal run of stats |
| `Table` | Full table set: `TableViewport` (scroll container), `TableHeaderGroup`, `TableBody`, `TableRow`, `TableHeaderCell`, `TableCell`, `TableSortButton` |
| `TypeBadge` | Element-type badge with optional tinted fill; uses the element-type color system |

### Controls (`components/controls/`)

| Component | Description |
|-----------|-------------|
| `SearchInput` | Controlled text input with a leading search icon and optional clear button |
| `SegmentedControl` | Mutually exclusive button group; active segment gets dark-ink fill |
| `Tabs` | Underline or pill tab bar; each item is `{ value, label, icon, badge }` |
| `ToggleRow` | Labeled boolean toggle (checkbox-backed) for filter/overlay panels |

### Navigation (`components/navigation/`)

| Component | Description |
|-----------|-------------|
| `Breadcrumb` | Slash-delimited path bar; last item is active (non-link) |
| `SidebarSection` | Collapsible section with `.ds-eyebrow` header and optional action slot |
| `TreeItem` | Indented tree row: indent level, expand chevron, icon slot, label, optional count |

### Product Patterns (`product-patterns/`)

Product patterns are Explorer-specific visual patterns exported through `@ds`.
They may use domain-shaped props and callback props, but they must not import
from `src/`, use store/router hooks, access workers, or read `window`.
They may customize primitives only through documented props or documented
context/density/composition `--ds-*` variables. They must not redefine primitive
state policy variables; state variants belong in primitive props and primitive
CSS.

Renderer-backed product patterns and app renderer boundaries use the canonical
stable-callback rule: expensive renderer lifecycle effects depend on renderer
data and configuration only, while UI callbacks are read at event time through
`useLatestRef`. Do not put route/modal/open callbacks into Sigma, React Flow,
Mermaid, or other canvas/SVG renderer mount-effect dependency arrays merely so
event handlers can see the latest callback. This keeps modal opens, shell
re-renders, and route metadata updates from tearing down and rebuilding
long-lived renderers. The public helper is exported from `@ds` as
`useLatestRef`; application renderer boundaries and design-system product
patterns may use it for event callbacks.

Shared route chrome must be consumed through the canonical exported product
pattern directly. Do not keep view-specific aliases or compatibility wrapper
components for shared patterns such as `WorkspaceShell`; update callers to the
canonical name.

They must not use inline `style={...}`, `CSSProperties`, ad hoc CSS-variable
objects, computed colors, or imperative `.style.*` mutations. Dynamic visual
values belong behind reusable primitive APIs in `design-system/components/**`;
product patterns compose those primitives through typed semantic/token props.

| Group | Exported targets |
|-------|------------------|
| `shell/` | `AppShell`, `ShellPane`, `PaneResizer`, `ShellMain`, `RouteFrame`, `RouteLayout`, `RoutePanel` |
| `chrome/` | `WorkspaceToolbar`, `PaneChromeHeader`, `ReqvireRailMark` |
| `side-pane/` | `SidePaneFrame`, pane filters, summaries, selections, trees, legends, and action rows |
| `detail/` | `DetailDialog`, element detail content, relation and Contract Bindings lists, concept references, and ontology detail bodies |
| `content/` | `WorkspaceShell`, `DocumentPanel`, `MarkdownFrame`, `DiagramBlockFrame`, `CodePreviewFrame`, `CodeToolbar`, `CodeBody`, `RendererNotice` |
| `thesaurus/` | `ThesaurusExplorer` for standalone SKOS concept schemes, concept taxonomy, concept references, and ontology `mapsToConcept` bridges |
| `feedback/` | `StoreNotice`, `HelpContent`, `HelpDialog` |

Product-pattern names should be general inside their folder context. Do not
repeat `Explorer` or `Reqvire` unless the name would be ambiguous without it.
Domain nouns such as `Element`, `Relation`, and `Ontology` are fine when they
name actual product concepts.

The showcase must import product patterns from `@ds` and drive examples from
showcase-local fixtures. It must not import store hooks, router state, or app
containers. The full-app mock harness is isolated to
`design-system/showcase/MockShell.tsx`, which injects fixture data and renders
`src/App`.

Showcase primitive pages demonstrate primitives only. They may show states,
props, and generic composition mechanics, but they must not present product
vocabulary or product compositions as raw primitive examples. Product concepts
such as panes, filters, graph legends, model trees, detail dialogs, reports,
and Explorer chrome belong in `ProductPatternsPage` and must use exported
product-pattern components from `@ds`.
Showcase scaffolding can wrap those components with `showcase-*` containers,
but it must not reach into their internals with `.ds-*`, `.ux-*`,
`[data-product-pattern]`, element selectors, or DOM descendants. Move that
behavior into the owning component/pattern contract instead.

---

## Palette API (`design-system/palette.ts`)

Exported via `@ds`. Provides programmatic access to element-type colors without
hardcoding token strings.

### Types

| Type | Values |
|------|--------|
| `ElementRole` | `"capability"`, `"requirement"`, `"contract"`, `"source"`, `"constraint"`, `"behavior"`, `"state"`, `"input-output"`, `"verification"`, `"specification"`, `"semantic-contract"`, `"ontology"`, `"concept"`, `"concept-scheme"`, `"concept-reference"`, `"resource"`, `"other"` |
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

1. **`npm run lint`** — generated artifact, adherence, and style checks (see Lint section below)
2. **`npm run generate:icons`** — derives favicon/app-icon PNG/ICO outputs
   from `design-system/assets/logo-mark.svg` into ignored `.vite/generated-assets/`
3. **`tsc --noEmit`** — TypeScript type check
4. **`npm run build:ds-bundle`** — emits generated standalone DS kit artifacts
   outside tracked source
5. **`vite build`** — emits `dist/` with deterministic asset names (`assets/explorer.js`, `assets/explorer.css`)

The build fails at the first failing step. All steps must pass before `dist/` is emitted.

### Showcase dev/build

```
npm run dev:showcase
npm run build:showcase
```

Both commands run `npm run lint` first, then generate browser icons and start
or build the Vite showcase. This is intentional: showcase pages exercise the
same design-system contracts as the real Explorer, so namespace, token,
product-pattern, and primitive-state-policy violations must fail before a
developer can preview the showcase.

### Standalone DS artifacts

```
npm run build:ds-bundle
```

Configured in `design-system/vite.bundle.config.ts`. Produces standalone kit
artifacts from `design-system/index.ts`. React and ReactDOM are external
globals. These files are generated outputs and are **not** tracked source. They
are also **not** part of the application bundle that `vite build` produces.

---

## Lint and checks

```
npm run lint
```

Runs four checks sequentially.

### Scripted guard rules

Current scripted guard rules under `explorer/scripts/`:

**Default lint pipeline**

`npm run lint` runs:

1. `lint:artifacts`
2. `lint:adherence`
3. `lint:style`
4. `lint:css-ownership`

**1. Generated artifacts**

`lint-generated-artifacts.mjs`

Rejects tracked/generated source artifacts:

- `explorer/_ds_bundle.js`
- `explorer/design-system/_ds_bundle.js`
- `explorer/design-system/reqvire-explorer.css`
- `explorer/design-system/_ds_manifest.json`
- `explorer/design-system/_adherence.oxlintrc.json`
- `explorer/design-system/dist-kit/`
- `explorer/design-system/dist-showcase/`
- `explorer/dist/`

Rejects generated assets in source asset roots:

- `project-store.js`
- generated favicon/app-icon files
- `site.webmanifest`
- `browserconfig.xml`

Rejects asset collisions between:

- `design-system/assets/`
- `public/assets/`
- `design-system/showcase/public/assets/`

**2. Import adherence**

`generate-adherence-config.mjs` + oxlint

Generated oxlint config rejects direct DS internal imports:

- `@ds/*`
- `design-system/components/**`
- `../design-system/components/**`
- `../../design-system/components/**`
- `design-system/product-patterns/**`
- `../design-system/product-patterns/**`
- `../../design-system/product-patterns/**`
- `components/controls/**`
- `components/core/**`
- `components/data/**`
- `components/navigation/**`
- `product-patterns/**`

Allowed public import surface is `@ds`.

**3. Product pattern boundaries**

`lint-product-pattern-boundaries.mjs`

Rejects:

- `design-system/product-patterns/**` importing anything from `src/`
- product patterns importing through relative paths into `src`
- product patterns importing `src/*`
- product patterns importing `@/*`
- showcase files importing `src/App`, except `design-system/showcase/MockShell.tsx`

**4. Style token guard**

`lint-style-tokens.mjs`

Scans:

- `src`
- `design-system/components`
- `design-system/product-patterns`
- `design-system/showcase`

Checks CSS files, Linaria `css` templates, and inline style literals.

Rejects raw visual values outside allowed token/custom-property/at-rule regions:

- raw `px`
- raw `rem`
- raw colors: hex, `rgb()`, `hsl()`, `oklch()`, etc.
- `color-mix(...)`
- raw filter functions: `blur()`, `brightness()`, `drop-shadow()`, etc.
- raw `font-family`
- numeric `font-weight`
- raw durations: `ms`, `s`
- raw easing: `ease`, `ease-in`, `cubic-bezier(...)`
- raw `z-index`

**5. CSS architecture**

`lint-css-architecture.mjs`

Rejects namespace ownership violations:

- `ds-*` emitted from `src/`
- `ux-*` emitted from `src/`
- `ux-*` emitted from primitive DS code outside `design-system/product-patterns/`
- `ds-*` emitted from `design-system/product-patterns/`

Enforces import surface:

- app code must import DS TypeScript through `@ds`
- only direct DS import allowed in `src/` is `../design-system/styles.css` from `src/main.tsx`

Enforces CSS entry files:

- `design-system/styles.css` must contain only approved token imports, in order
- `index.html` and `design-system/showcase/index.html` inline styles must be mount bootstrap only

Rejects undocumented primitive variable customization:

- any `--ds-*` assignment in `src/`, product patterns, or showcase unless allowlisted
- always rejects primitive state-policy vars outside primitives:
  - `--ds-*-sel-*`
  - `--ds-*-hover-*`
  - `--ds-*-active-*`
  - `--ds-*-focus-*`
  - `--ds-*-off-*`
- rejects unknown custom properties:
  - product patterns and app UI own `--ux-*`
  - showcase scaffolding owns `--showcase-*`
  - every other `var(--*)` must resolve to a token, documented `--ds-*`
    customization, or owned local variable

Rejects inline visual styling:

- `style={...}`
- `CSSProperties`
- `.style.*`
- `.style.setProperty(...)`

Applies to:

- DS components
- product patterns
- showcase
- app UI

With explicit renderer-boundary allowlist:

- `src/App.tsx`
- `src/lib/ontologyGraphRenderer.ts`
- `src/rendering/MarkdownContent.tsx`
- `src/test/setupCssTokens.ts`
- `src/views/GraphLibraryViews.tsx`

Rejects public API shape problems:

- exported DS/product-pattern prop types extending React DOM attrs must omit `"style"`
- arbitrary visual string props are rejected, e.g. `color?: string`, `background?: string`, `pipColorToken`, `accentColorToken`, unconstrained token strings

Showcase-specific rules:

- showcase page scaffolding classes must be `showcase-*`
- primitive showcase pages must not import product-pattern components
- `showcase-*` CSS must not target:
  - `.ds-*`
  - `.ux-*`
  - `[data-product-pattern]`
  - DOM elements inside rendered components, e.g. `svg`, `button`, `table`, etc.

**6. CSS ownership**

`lint-css-ownership.mjs`

Rejects:

- `.css` files outside `design-system/`, except strict bootstrap-only `src/app-mount.css`
- `ux-*` hooks emitted from `src/`
- Linaria `css` templates outside:
  - `design-system/components`
  - `design-system/product-patterns`
  - `design-system/showcase`
- Linaria `styled` definitions outside the same allowed ownership roots

So the big picture is:

- generated output is not source
- imports must go through public DS surfaces
- app `src/` is behavior/state, not visual ownership
- primitives own `ds-*`
- product patterns own `ux-*`
- showcase owns only `showcase-*` scaffolding
- visual values must be tokenized
- customization APIs are deny-by-default and documented through guards

### 1. Generated artifact guard

```
npm run lint:artifacts
```

`scripts/lint-generated-artifacts.mjs` fails if generated outputs are tracked
as source, including generated DS bundles, generated DS CSS, generated
adherence config, `dist/`, `design-system/dist-kit/`, and
`design-system/dist-showcase/`. It also rejects generated runtime/browser
assets such as `project-store.js`, generated favicon/app-icon variants, and
manifest/browserconfig files when they appear under source asset roots, and it
fails asset-name collisions between `design-system/assets/` and raw public
asset roots.

### 2. Adherence (import contract)

```
npm run lint:adherence
```

```
npm run generate:adherence && oxlint --deny-warnings --ignore-path=.oxlintignore --config .vite/_adherence.oxlintrc.json src design-system && node scripts/lint-product-pattern-boundaries.mjs
```

`scripts/generate-adherence-config.mjs` reads `design-system/index.ts`, extracts
every exported PascalCase component name, then writes the ignored generated
file `.vite/_adherence.oxlintrc.json` with a `no-restricted-imports` rule that
bans direct imports from component and product-pattern paths. The rule forces
application and showcase imports to go through `@ds`.

`scripts/lint-product-pattern-boundaries.mjs` is the path-aware import guard
for product ownership. It rejects `design-system/product-patterns/` imports
that resolve into `src/`, including relative paths, baseUrl `src/*` paths, and
the `@/*` alias. It also keeps the full-app showcase harness isolated:
`design-system/showcase/MockShell.tsx` is the only showcase file allowed to
import `src/App`, including through relative paths, baseUrl paths, or the
`@/*` alias.

Banned import patterns:
```
@ds/*
design-system/components/**
../design-system/components/**
../../design-system/components/**
design-system/product-patterns/**
../design-system/product-patterns/**
../../design-system/product-patterns/**
components/controls/**
components/core/**
components/data/**
components/navigation/**
product-patterns/**
../src/**
../../src/**
../../../src/**
```

The barrel file itself (`design-system/index.ts`) is exempted from the DS
internals rule. `design-system/showcase/MockShell.tsx` is the only showcase
file exempted from the relative `src/` import rule, because it is the full-app
mock integration harness. The config is regenerated on every lint run so it
always reflects the current barrel.

### 3. Style checks

```
npm run lint:style
```

```
stylelint "design-system/styles.css" "design-system/tokens/**/*.css" "design-system/components/**/*.css" "design-system/showcase/**/*.css" "src/**/*.css" --allow-empty-input && node scripts/lint-style-tokens.mjs && node scripts/lint-css-architecture.mjs
```

**stylelint** checks property/selector correctness via `.stylelintrc.json`.

**Style token guard** (`scripts/lint-style-tokens.mjs`) enforces the token
contract on CSS files, Linaria `css` templates, and inline style literals under
`src/`, `design-system/components/`, `design-system/product-patterns/`, and
`design-system/showcase/`. It
strips comments, blanks out custom-property declarations and at-rule conditions
(where raw values are legitimate), then runs conservative pattern checks on the
remainder:

| Check | What it flags |
|-------|--------------|
| `raw-px` | Raw `px` lengths not inside a custom-property or at-rule |
| `raw-rem` | Raw `rem` lengths not inside a custom-property or at-rule |
| `raw-color` | Hex colors, `rgb()`, `hsl()`, `oklch()`, etc. |
| `color-mix` | `color-mix(...)` outside token/source-of-truth files |
| `raw-filter-function` | `blur(...)`, `brightness(...)`, `drop-shadow(...)`, etc. outside token/source-of-truth files |
| `raw-font-family` | `font-family:` not starting with `var(` or `inherit` |
| `raw-font-weight` | `font-weight:` with a numeric value |
| `raw-duration` | Raw `ms` / `s` durations |
| `raw-easing` | `cubic-bezier(` or `ease(-in|-out|-in-out)` |
| `raw-z-index` | `z-index:` not using `var(--z-*)`, `auto`, or `calc(var(...))` |

Any finding exits 1. There is no baseline or grandfather list — the check is
either clean or it fails.

**CSS architecture guard** (`scripts/lint-css-architecture.mjs`) enforces
namespace ownership (`ds-*` only in primitive design-system components and
`ux-*` only in `design-system/product-patterns/`). It also keeps
`design-system/styles.css` as the import-only public CSS entry, keeps HTML entry
inline styles limited to document mount bootstrap, and blocks app code from
importing direct `design-system/*` paths, except the single
`../design-system/styles.css` import in `src/main.tsx`. The same guard scans
application code, `design-system/product-patterns/`, and
`design-system/showcase/` for
`--ds-*` assignments: customizations are deny-by-default, and primitive
interaction/state policy variables such as `--ds-*-sel-*`,
`--ds-*-hover-*`, `--ds-*-active-*`, `--ds-*-focus-*`, and `--ds-*-off-*`
are forbidden outside primitive components. Unknown custom properties are also
rejected outside their owning namespace: product patterns and application UI
own `--ux-*`, showcase scaffolding owns `--showcase-*`, and every other
`var(--*)` must resolve to a token, documented `--ds-*` customization, or an
owned local variable. The guard also rejects inline
visual styling in reusable components, product patterns, showcase pages, and
application UI: `style={...}`, `CSSProperties`, and imperative `.style.*` /
`setProperty(...)` mutation must live behind reusable primitive APIs such as
`ElementIcon`, `TypeBadge`, `TokenSwatch`, `DonutMeter`, or `BarMeterFill`, or
inside an explicitly allowlisted renderer boundary. Exported DS/product-pattern
prop types that extend React DOM attributes must omit `"style"`, and public
visual props must not accept arbitrary strings such as `color?: string`,
`background?: string`, or unconstrained ``--${string}`` token names; use
semantic variants or typed token unions such as `DesignSystemColorToken`.
Showcase page scaffolding must use `showcase-*` classes, and primitive showcase pages
may import primitives/tokens only. Product vocabulary and Explorer
compositions belong in product-pattern or mock showcase pages.
`showcase-*` is not a styling escape hatch: showcase CSS may style only the
demo scaffold around examples. It must not target `.ds-*`, `.ux-*`,
`[data-product-pattern]`, or DOM elements inside rendered components. If the
showcase needs a different size, placement, or state, add or use a public
primitive/product-pattern API and exercise that API from the page.
These checks are conservative by design: adding a new token layer, entry import,
global selector allowance, documented `--ds-*` customization, primitive
dynamic-style API, or namespace exception requires updating this guard and this
document in the same change.

### CSS ownership guard

```
npm run lint:css-ownership
```

`scripts/lint-css-ownership.mjs` is default-enforced by `npm run lint` so src
visual CSS regressions fail normal validation. It fails on:

- `.css` files outside `design-system/`, except a strictly bootstrap-only
  `src/app-mount.css` if that file exists
- `ux-*` product class hooks emitted from `src/`
- Linaria `css` and `styled` definitions outside
  `design-system/components/` and `design-system/product-patterns/`

`App.tsx` is not a visual exception. It may own route/store/bootstrap/event
wiring, pane width state, graph state, and callbacks, but shell layout rules
belong in `design-system/product-patterns/shell`.

Findings include file, line, column, evidence, and ownership guidance. Current
violations are lint failures, not allowlist targets.

---

## How to use

### Importing components and product patterns

```tsx
import { AppShell, Button, ElementIcon, Icon, TypeBadge, WorkspaceToolbar } from "@ds";
import { cssVar, elementRole, roleColorValue } from "@ds";
import type { ElementRole, PaletteChannel } from "@ds";
```

Never import from a component path directly:

```tsx
// wrong
import { Button } from "../../design-system/components/core/Button";
import { AppShell } from "../design-system/product-patterns/shell";

// correct
import { Button } from "@ds";
import { AppShell } from "@ds";
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

Custom-property declarations are the one allowed location for raw values, but
they still need to live with their owner. Design tokens belong in
`design-system/tokens/`; application variables belong on the shell root or the
owning Explorer component module. HTML entry inline styles must not define
product variables.

```css
/* allowed — component-owned app variable */
.ux-detail-dialog {
  --ux-detail-dialog-w: 1120px;
  width: min(var(--ux-detail-dialog-w), calc(100vw - var(--space-24)));
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
<ElementIcon type="contract" size="sm" />
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

### Adding a new product pattern

1. Create `design-system/product-patterns/<group>/MyPattern.tsx`
2. Export it from the group barrel and `design-system/product-patterns/index.ts`
3. Add an explicit public export to `design-system/index.ts`
4. Keep behavior, store access, routing, workers, and browser globals in `src/`
5. Use `ux-*` class hooks and `--ux-*` variables only inside product patterns
6. Add showcase coverage with mock data under `design-system/showcase/fixtures`
   when the pattern changes user-visible Explorer UI

---

## Dependencies

The design system itself has no runtime dependencies beyond React and ReactDOM.
The application workspace dependencies relevant to the design system:

| Package | Role |
|---------|------|
| `react`, `react-dom` | Component runtime |
| `@linaria/atomic`, `@linaria/core`, `@linaria/react` | Component-scoped CSS authoring |
| `@wyw-in-js/babel-preset`, `@wyw-in-js/vite` | Linaria/WyW extraction in dev + build |
| `@vitejs/plugin-react` | JSX transform for dev + build |
| `oxlint` | Import contract enforcement |
| `stylelint` | CSS property/selector lint |
| `vite` | Dev server, application build, DS bundle |
| `typescript` | Type checking |

Removed in the current version: `tailwindcss`, `@tailwindcss/vite`,
`lucide-react`, `clsx`, `tailwind-merge`, `@radix-ui/react-icons`,
`@radix-ui/themes`.
