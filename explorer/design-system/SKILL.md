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
- `components/<group>/` — typed React primitives (TSX; props are the source of truth) (`Alert`, `Badge`, `BrandMark`, `Button`, `Card`, `Icon`, `IconButton`, `Modal`, `CodeRef`, `ElementIcon`, `TypeBadge`, `RelationPill`, `Chip`, `Stat`, `Table`, `ToggleRow`, `SegmentedControl`, `SearchInput`, `Tabs`, `TreeItem`, `Breadcrumb`, `SidebarSection`).
- `product-patterns/<group>/` — typed reusable UX/product-pattern targets (`shell`, `chrome`, `side-pane`, `detail`, `content`, `feedback`) that emit `ux-*` visual hooks.
- `index.ts` — the barrel; production code imports primitives, product patterns, and the palette API from here (the app aliases it as `@ds`).
- `palette.ts` — programmatic color API: element-type → token mapping, runtime CSS-variable resolution, Mermaid class defs.
- `assets/logo-mark.svg` — 200x200 source brand mark (reconstruction; replace with the official logo when available). Favicons and app icons are generated from it by `npm run generate:icons`, not hand-maintained as source.

## Using the components
Production Explorer code imports components, product patterns, and palette symbols from `@ds`.
For the local showcase, run `npm run dev:showcase` or `npm run build:showcase`; both commands run `npm run lint` before the Vite step so showcase-only rule breaks fail immediately.
Generated bundles, generated CSS, generated browser icons, generated adherence
config, and dist output are build/runtime artifacts, never tracked source.

## Non-negotiables
- Keep the **element-type color code** meaningful: capability=blue, requirement=violet, refinement=orange, verification=green, ontology=gold, resource=amber, other=slate. Never reuse these hues decoratively.
- Rose is the only chrome accent; primary buttons/active segments use dark slate ink. Dense 13px UI text. Geist. 4px grid. Soft, low shadows. No emoji, no decorative gradients, no hand-drawn SVG icons (use `Icon` / `ElementIcon`).
- Primitive components own interaction state styling. Product patterns and showcase examples may use documented primitive props and documented context/density/composition `--ds-*` variables, but must not assign primitive state-policy variables like `--ds-*-sel-*`, `--ds-*-hover-*`, `--ds-*-active-*`, `--ds-*-focus-*`, or `--ds-*-off-*`; add a primitive prop/variant instead.
- Product patterns must not use inline `style={...}`, `CSSProperties`, ad hoc CSS-variable objects, computed colors, or imperative `.style.*` mutations. If a dynamic visual value is legitimate, move it behind a reusable primitive API in `components/**` (for example `ElementIcon`, `TypeBadge`, `TokenSwatch`, `DonutMeter`, `BarMeterFill`) and compose that primitive from the product pattern.
- When reviewing or refactoring DS code, search for `style={`, `CSSProperties`, `--...` style objects, `setProperty(`, and `.style.` across `design-system` and `src`. Classify findings: primitive API plumbing may be acceptable; product-pattern and app visual policy is not.
- Showcase primitive pages demonstrate primitives only. Product vocabulary and product compositions such as panes, filters, graph legends, model trees, detail dialogs, reports, and Explorer chrome belong in `ProductPatternsPage` and must use exported product-pattern components from `@ds`.

## Before editing visual code
- Classify the ownership first: `tokens`, `components` primitive, `product-patterns`, `showcase` scaffold, or app logic. Put the visual rule in the owning layer, not where the bug happened to show up.
- If a component only looks correct after a parent wrapper dresses it, the contract is wrong. Move the visual contract into the primitive or product-pattern component that owns that UI.
- Product patterns must look correct when mounted alone. Shell/view wrappers may provide layout space and routing context, but not the final visual styling for the pattern.
- Consumer app code owns state, data loading, routing, selection, and events. It consumes DS/product-pattern components; it does not define visual CSS policy.
- If code needs dynamic visuals, do not use inline style. Prefer semantic props, typed token props, `data-*` attributes with token selectors, SVG presentation attributes, or a new primitive API.
- If a product pattern needs different hover/selection/focus/active/off behavior, do not override `--ds-*` state variables. Add a primitive variant or prop and keep that state CSS in `components/**`.
- If lint blocks a visual change, either change the DS API or update the conservative guard/allowlist in the same patch with a clear reason. Do not bypass the guard with local CSS, inline style, generated CSS, or consumer-owned selectors.

## What guards should catch
- No inline `style={...}`, `CSSProperties`, `.style.*`, or `setProperty(...)` in reusable DS components, product patterns, showcase pages, or app UI, except explicit renderer boundaries.
- No app or product-pattern CSS targeting `.ds-*` internals. Consumers use public props, `data-*`, documented CSS variables, or exported product-pattern components.
- No undocumented `--ds-*` assignments outside DS primitives. The allowlist is deny-by-default and should contain only context, density, or composition variables.
- No primitive state-policy variables outside primitives: `--ds-*-sel-*`, `--ds-*-hover-*`, `--ds-*-active-*`, `--ds-*-focus-*`, `--ds-*-off-*`.
- No raw visual values outside token/source-of-truth files: raw px/rem/colors, `color-mix`, raw filter functions, durations, easing, z-index, or font stacks.
- No generated artifacts tracked as source, no hand-maintained generated CSS/icons/bundles, and no direct DS internal imports from consumer app code.
- Exported component/product-pattern prop types that extend React DOM attributes must omit `"style"` by default.
- DS/public visual APIs must not accept arbitrary visual strings such as `color?: string`, `background?: string`, `pipColorToken`, or unconstrained ``--${string}``; use semantic variants or typed token unions such as `DesignSystemColorToken`.
- Showcase pages use `showcase-*` classes for scaffolding. Primitive showcase pages import primitives/tokens only; product-pattern and mock pages own Explorer/product vocabulary.
- `showcase-*` is not a customization escape hatch. Showcase CSS may style only the demo scaffold around examples; it must not target `.ds-*`, `.ux-*`, `[data-product-pattern]`, or DOM elements inside rendered components. If showcase needs a different size, placement, or state, add/use a public primitive or product-pattern API.

## What requires human or agent judgment
- Whether a `--ds-*` variable is legitimate context/density customization or hidden primitive styling.
- Whether a product pattern truly matches the real Explorer usage, especially after refactors. Prefer mounting the real exported product-pattern component in showcase rather than recreating a lookalike.
- Whether a showcase primitive example has crossed into product vocabulary. If it has panes, filters, graph legends, model trees, detail dialogs, reports, or Explorer chrome, move it to product patterns or mocks.
- Whether a dynamic visual value belongs in a generic primitive, a product-specific pattern, or an app-renderer boundary.
- Whether shell/view wrappers are doing layout only or are visually dressing child components. Dressing child components is a contract smell.
