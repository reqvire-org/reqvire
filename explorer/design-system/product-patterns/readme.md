# Product Patterns

Product patterns are reusable Reqvire Explorer visual patterns. They sit above
the primitive `ds-*` component layer and below app containers.

## Ownership

- `components/` owns generic primitives, `ds-*` hooks, and `--ds-*` component variables.
- `product-patterns/` owns reusable UX/product patterns, `ux-*` hooks, and `--ux-*` product variables.
- Product patterns may accept domain-shaped data and callbacks.
- Product patterns must not import from `src/`, use store/router hooks, access workers, or read `window`.
- Application and showcase code import product patterns from the public `@ds` barrel.

## Groups

- `shell/` - application shell layout targets such as `AppShell`, `ShellPane`, `PaneResizer`, `ShellMain`, `RouteFrame`, `RouteLayout`, and `RoutePanel`.
- `chrome/` - workspace chrome targets such as `WorkspaceToolbar`.
- `side-pane/` - Explorer side-pane frames, selections, filters, trees, and legends.
- `detail/` - detail dialogs and relation, attachment, concept, and ontology detail bodies.
- `content/` - document, markdown, diagram, and code preview frames.
- `feedback/` - product notices and help content.

## Naming

Names should be general inside their folder context. Do not repeat `Explorer`
or `Reqvire` unless the name would be ambiguous without it. Domain nouns such
as `Element`, `Relation`, and `Ontology` are fine when they name actual product
concepts.
