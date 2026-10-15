# Showcase frame

`ShowcaseHeader`, `ShowcaseNavigation`, and `ShowcaseRegion` are public APIs for
minimal, neutral showcase chrome and predefined example layouts. Import them
through `@ds`. Fixtures supply content, state, and callbacks; this pattern owns
CSS and markup. It accepts no arbitrary `style` or `className` props.

The header provides light/dark switching and a minimize button. The showcase app
applies `data-theme` to the document so patterns, components, and toast portals
use the same theme. Minimize removes the complete header without replacing
example content; refresh restores it. Neutral chrome has dedicated token values.

Pages are Tokens, Components, Patterns, and Mocks. Component groups are examples,
and mocks compose public DS patterns with local fixture data. Full application
integration harnesses and their route/store tests live under `src/test/`.

The header/navigation adapts the DS tooling repository's working example:
https://github.com/reqvire-org/design-system/tree/main/design-system/patterns/showcase
Its MIT notice is retained in `LICENSE.source.md`. This source is project-owned;
the installed DS tool does not supply a UI catalogue.

Navigation uses the layers, box, layout-grid, and grid section icons at small size,
with muted opacity and a subtle neutral active border. It does not use product Tabs.
