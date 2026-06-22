# Reqvire Explorer — Design System

A design system for **Reqvire Explorer**, the interactive viewer for [Reqvire](https://www.reqvire.org/) system models. Reqvire is a Git-native MBSE (model-based systems engineering) toolkit: engineering knowledge — capabilities, requirements, contracts, semantic contracts, verifications and traceability links — lives as semi-structured Markdown in a repository, and Reqvire compiles it into typed graph projections and reports.

The **Explorer** is the read/navigate surface over that graph. It is one application with several *modes*, switched from a top tab bar:

| Mode | What it shows |
|------|---------------|
| **Model** | File/folder tree + a grid of folder / source-file / modeled-element tiles. |
| **Knowledge Graph** | Force-directed graph of all elements, colored by type, with relation overlays. |
| **Ontologies** | RDF/SHACL graph of the modeling vocabulary, with a node inspector. |
| **Search** | Unified search across files, elements, resources, ontology terms, traces, coverage. |
| **Traces** | Grouped verification trace rows and requirement roll-up diagrams. |
| **Element detail** | A modal: metadata table, user-story content, prose details, and typed relations. |

> **Design intent.** The reference screenshots show *what exists and the direction* — not a pixel target. This system keeps the structure and semantics (the modes, the element-type color code, the sidebar + canvas + top-tab shell) and **elevates the craft**: warm product surfaces, slate text ramps, a rose chrome accent, Geist type, a 4px spacing grid, restrained elevation, and a clean component set.

## Sources

- **Product site:** https://www.reqvire.org/
- **Reference UI:** Explorer screenshots provided by the project owner (`uploads/explorer-*.png`) — Model browser, element modal, Graph, Ontologies, Search, and Traces. These are the source of truth for layout and the element-type semantics.
- The canonical implementation lives in this repository under `explorer/design-system/` and is documented in `../DS.md`.

---

## Content fundamentals

How Reqvire writes copy — match this when generating product text.

- **Voice — precise, declarative, engineering-register.** Labels are nouns or noun phrases: "Source file", "Outgoing relations", "Type family", "Verification Coverage Specification". No marketing tone, no exclamation.
- **User stories use first person, role-bolded.** Requirement/capability content follows the canonical form: *"As a **System Engineer**, I want a well-defined Reqvire model structure…, so that I can…"*. The role is bold; the sentence is one clause of want + rationale.
- **Casing.** Element and specification names are **Title Case** ("Containment Specification", "Defining Model Structure"). Type slugs and relation kinds are **lowercase / camelCase tokens** rendered in mono or as quiet tags: `capability`, `specification`, `semantic-contract`, `specifiedBy`, `verifiedBy`, `derivedFrom`. Section labels in the sidebar are **UPPERCASE**, letterspaced ("SUMMARY", "SHOW", "OVERLAYS", "TYPES", "RELATIONS").
- **Identifiers and paths are monospace**, always with a line anchor when relevant: `system-model/Capabilities.md:3`, `system-model/Capabilities.md#defining-model-structure`.
- **Counts are bare and tabular** — "6 items", "8 children", "3 elements", "Submodels 13  Elements 640  Relations 1090", "50 results". Numbers sit in pills or inline after a label.
- **No emoji.** Iconography is line-icons and the colored type glyphs. The tone is a developer tool, not a consumer app.
- **Metadata is key→value**, with an `(explicit)` suffix marking values authored in source vs. inherited ("priority: high (explicit)").
- **Empty / hint states are quiet and instructive, italic**: *"Select a file row to inspect its modeled elements."*, *"Search the project store."*, *"Select a graph node to inspect URI, RDF type, comments, and SHACL constraints."*

---

## Visual foundations

**Palette.** Warm product surfaces (`--warm-*`) for canvas, panels, hover fills and borders, with slate ramps (`--slate-*`) for text and dark surfaces. A single **rose** brand accent (`--accent`, rose-600 · #e11d48) for interactive chrome — focus rings, selection, links, the active tab underline. A separate **dark slate ink** (`--slate-900`) fills primary buttons and active segmented controls (the product fills these with near-black, not the accent). Color is otherwise reserved for **meaning**: the six-hue element-type code.

**Element-type semantics** (the heart of the system — never repurpose these hues for decoration):
capability = **#BBDEFB** (blue), requirement = **#673AB7** (deep purple), semantic-contract = SHACL red (`--semantic-contract`), contract = **#FF9800** (orange — covers source, specification, constraint, behavior, state, input-output), verification-objective = dark green (`--verification-objective`), concrete verification = **#4CAF50** (green — test, formal proof, analysis, inspection, demonstration), ontology = fill **#F4E3A1** + stroke **#B08A00**, native SKOS concept = `--concept`, native concept scheme = darker yellow `--concept-scheme`, concept-reference fact/edge = `--concept-reference`, file/resource = **#FFCA28** (amber), evidence-file/artifact/other = **#9E9E9E** (gray). These are the authoritative Reqvire element-type colors. Each ships a solid plus a light tint for chips and node fills. The Ontologies mode adds an extended RDF/SHACL palette (`--rdf-*`), including `--rdf-concept` and `--rdf-concept-scheme` for passive graph notation.

**Typography.** **Geist** for all UI and display; **Geist Mono** for IDs, source paths, type/relation slugs and ontology terms. The product is **dense** — default UI text is **13px** (`--text-sm`), reading copy 14px, sidebar section labels 11px uppercase with `0.07em` tracking. Weights live at 400/500/600; 700 is rare.

**Spacing & shape.** 4px base grid (`--space-*`). Use semantic layout tokens for repeated product structure, for example `--side-pane-content-inset-inline` for Explorer side-pane content alignment, `--side-pane-search-content-gap` for the gap below pane quick filters, and `--side-pane-summary-columns` for compact pane summary grids. Radii step from 4px (type-icon chips) → 6px (tags/inputs) → 8px (buttons/toggles) → 10px (cards) → 14px (dialogs) → pill. Controls are compact: 34px default height, 28px small. List-like UI is contiguous by default: menu items, table/list rows, filters, legends, and relation rows use `--stack-gap-compact` so boxes sit directly next to each other vertically.

**Surfaces & elevation.** Light, near-white canvas (`--bg-canvas`) with white panels/cards. Borders do most of the structural work; **shadows are soft and low** (`--shadow-xs/sm` on cards, `--shadow-lg/xl` only for popovers and modals). Cards are white with a 1px subtle border and `xs` shadow; **selected** cards switch to a dark 1px ring + a faint sunken fill (no accent border). No heavy gradients — the only gradient is a barely-there radial wash behind the graph canvas.

**Backgrounds & imagery.** No photography, no illustration. The "imagery" of the product *is* the data: node clouds, trace rows, RDF graphs. Keep backgrounds flat; let the colored nodes carry the visual energy.

**Motion.** Quick and mechanical — `--dur-fast` 130ms for hover/press, `--dur-base` 200ms for panels/overlays, easing `cubic-bezier(0.2,0,0,1)`. **No bounce.** Graph nodes scale up ~1.35× on hover. Respect `prefers-reduced-motion`.

**Loading.** Loading indicators use the core **`Spinner`** glyph component. Product views own the loading state and message, but spinner shape, color, size, and motion stay in the design system.

**Interaction states.** Hover = a one-step-darker warm fill (`--bg-hover`) and/or stronger border; quiet controls also darken their text. Press = the next sunken step (`--bg-active`). Focus-visible = a 3px rose ring (`--ring-focus`). Toggleable facets/segments invert to **dark ink fill + light text** when active. Legend toggles dim to ~45% and gray their swatch when off.

**Borders, transparency, blur.** Hairline 1px borders in `--border-subtle`/`--border-default`; `--border-strong` on hover. Transparency is used sparingly: the modal scrim is `rgba(13,17,25,.42)` with a 1.5px backdrop blur; node labels get a text-shadow "halo" of the canvas color so they stay legible over nodes.

**Dark theme.** A faithful **navy-slate** dark mode (layered `#0e141d → #1e2836`), activated with `data-theme="dark"` on any container. Tints switch to their `-d` (alpha) variants; shadows deepen; the accent lightens to a brighter rose. The product's theme toggle (sun/moon) lives bottom-right in the original and top-right in this kit's tab shell.

---

## Iconography

- **Line icons, Lucide geometry.** The Explorer uses a single stroked line-icon set (2px stroke, round caps, 24px grid) — search, box/cube, network/share, globe, activity, grid, list, table, database, settings-gear, sun/moon, help-circle, chevrons, folder/file, external-link, download. This system ships them CDN-free as the **`Icon`** component (`components/core/Icon.tsx`), whose path data is Lucide-derived (ISC). Use `<Icon name="…" />`; import `ICON_NAMES` for the full list. For anything outside the curated set, pass your own inline SVG to the icon-accepting props — do **not** hand-draw decorative SVG.
- **Element-type glyphs are their own system.** A model element is marked by a colored glyph, *not* a line icon: capability / requirement / verification / ontology / resource / semantic-contract → type-colored rounded squares with no text mark; contract-family elements → orange diamonds with subtype marks (`source`, `specification`, `constraint`, `behavior`, `state`, `input-output`) so related contracts keep the same hue while remaining visually distinct. This is the **`ElementIcon`** component.
- **No emoji, no unicode-as-icon.** The only non-icon glyph in the source is the subclass mark `⊆` beside ontology terms.
- **Logo.** A small **model-hub constellation** — six satellite nodes whose links converge at the model root carrying the element-type colors. `assets/logo-mark.svg`. ⚠ This is a **reconstruction** from the screenshots; replace with the official Reqvire SVG when available (see Caveats).

---

## Index / manifest

**Root**
- `styles.css` — the single entry point consumers link. `@import`s only.
- `tokens/` — `colors.css`, `typography.css`, `spacing.css`, `elevation.css`, `fonts.css`, `base.css`.
- `assets/logo-mark.svg` — 200x200 source brand mark (reconstruction); the `BrandMark` component references it and favicon/app-icon PNG/ICO outputs are generated from this file.
- `SKILL.md` — Agent-Skill manifest for downloading this system into Claude Code.

**Components** (`components/<group>/` — TSX source exported via `index.ts`)
- `core/` — **Alert**, **Button**, **IconButton**, **Badge**, **BrandMark**, **Card**, **Icon**, **Modal**
- `data/` — **CodeRef**, **ElementIcon**, **TypeBadge**, **RelationPill**, **Chip**, **Stat** (+ **StatRow**), **Table**
- `controls/` — **ToggleRow**, **SegmentedControl**, **SearchInput**, **Tabs**
- `navigation/` — **TreeItem**, **Breadcrumb**, **SidebarSection**

**Product patterns** (`product-patterns/<group>/` — reusable UX/product patterns exported via `index.ts`)
- `shell/` — **AppShell**, **ShellPane**, **PaneResizer**, **ShellMain**, **RouteFrame**, **RouteLayout**, **RoutePanel**
- `chrome/`, `files/`, `resources/`, `search/`, `reports/`, `side-pane/`, `detail/`, `content/`, `feedback/` — reusable UX/product visual patterns

**Ownership**
- `ds-*` classes and `--ds-*` component variables are design-system-only.
- `ux-*` classes and `--ux-*` variables are owned by the UX/product-pattern layer,
  under `design-system/product-patterns/`.
- Consumers import primitives, product patterns, and palette symbols from `@ds` only.
- Consumers do not target design-system internals; customization happens through documented props or `--ds-*` variables.

**Generated artifacts**
- Generated bundles, generated CSS, and `dist-*` outputs are not tracked source.
- Favicons and platform app icons are generated from `assets/logo-mark.svg`; do not hand-edit generated PNG/ICO variants in source.
- There is no release-artifact exception in this repository.
- Build/runtime outputs are recreated from TS/TSX, CSS tokens, config, and docs.

---

## Caveats

1. **Fonts** are Geist and Geist Mono, vendored in `assets/fonts/*.woff2` and referenced relatively from `tokens/fonts.css`, so the kit and standalone artifacts work offline with no app server.
2. **Logo** is a reconstruction of the dotted-graph mark, built from the element-type palette. Please provide the official Reqvire logo (SVG preferred) to replace `assets/logo-mark.svg`, then regenerate browser icons with `npm run generate:icons`.
3. **Canonical source.** This directory is source, not an exported artifact. Generated bundles, generated CSS, generated browser icons, and showcase dist output must be rebuilt, not edited.
