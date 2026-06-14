---
name: reqvire-design
description: Use this skill to generate well-branded interfaces and assets for Reqvire (the Reqvire Explorer — a Git-native MBSE knowledge-graph viewer), either for production or throwaway prototypes/mocks/etc. Contains essential design guidelines, colors, type, fonts, assets, and UI kit components for prototyping.
user-invocable: true
---

Read the `readme.md` file within this skill, and explore the other available files.

If creating visual artifacts (slides, mocks, throwaway prototypes, etc), copy assets out and create static HTML files for the user to view. If working on production code, you can copy assets and read the rules here to become an expert in designing with this brand.

If the user invokes this skill without any other guidance, ask them what they want to build or design, ask some questions, and act as an expert designer who outputs HTML artifacts _or_ production code, depending on the need.

## Where things are
- `readme.md` — the full design guide: product context, content fundamentals, visual foundations, iconography, and a file manifest. **Start here.**
- `styles.css` — single CSS entry point; link it to inherit all tokens, fonts and base styles. It is import-only and guarded.
- `tokens/` — colors (warm product surfaces, slate text ramps, rose accent, the six element-type hues), typography (Geist/Geist Mono), spacing, elevation, fonts.
- `components/<group>/` — typed React primitives (TSX; props are the source of truth) (`Alert`, `Badge`, `Button`, `Card`, `Icon`, `IconButton`, `Modal`, `ElementIcon`, `TypeBadge`, `RelationPill`, `Chip`, `Stat`, `Table`, `ToggleRow`, `SegmentedControl`, `SearchInput`, `Tabs`, `TreeItem`, `Breadcrumb`, `SidebarSection`).
- `index.ts` — the barrel; production code imports components and the palette API from here (the app aliases it as `@ds`).
- `palette.ts` — programmatic color API: element-type → token mapping, runtime CSS-variable resolution, Mermaid class defs.
- `assets/logo-mark.svg` — 200x200 source brand mark (reconstruction; replace with the official logo when available). Favicons and app icons are generated from it by `npm run generate:icons`, not hand-maintained as source.

## Using the components
Production Explorer code imports components and palette symbols from `@ds`.
For the local showcase, run `npm run dev:showcase` or `npm run build:showcase`.
Generated bundles, generated CSS, generated browser icons, generated adherence
config, and dist output are build/runtime artifacts, never tracked source.

## Non-negotiables
- Keep the **element-type color code** meaningful: capability=blue, requirement=violet, refinement=orange, verification=green, ontology=gold, resource=amber, other=slate. Never reuse these hues decoratively.
- Rose is the only chrome accent; primary buttons/active segments use dark slate ink. Dense 13px UI text. Geist. 4px grid. Soft, low shadows. No emoji, no decorative gradients, no hand-drawn SVG icons (use `Icon` / `ElementIcon`).
