# Product Patterns

Product patterns are reusable Reqvire Explorer visual patterns. They sit above
the primitive `ds-*` component layer and below app containers.

## Ownership

- `components/` owns generic primitives, `ds-*` hooks, and `--ds-*` component variables.
- `patterns/` owns reusable UX/product patterns, `ux-*` hooks, and `--ux-*` product variables.
- Product patterns may accept domain-shaped data and callbacks.
- Product patterns must not import from `src/` or use application store/router hooks.
- A component provides one cohesive UI capability; a pattern arranges capabilities
  into a reusable structure or workflow. Component count is not a classification rule.
  `WorktreeSelector` and `RendererNotice` live in `components/`; the workspace and
  document patterns compose them through public APIs. Layout parts may arrange slots
  without importing other components.
- Navigation destinations and navigation callbacks come from the application.
  Patterns render supplied links or invoke callbacks rather than writing router state.
- UI-owned renderer and layout services may use browser APIs and local workers. Keep their lifecycles cancellable and graph-only dependencies lazy. Application data caches, worktree loading and routing remain application-owned.
- Application and showcase code import product patterns from the public `@ds` barrel.

Renderer lifecycle effects depend on renderer data and configuration. Read UI
callbacks at event time through the public `useLatestRef` helper so route,
modal or shell updates do not rebuild long-lived Sigma, React Flow or Mermaid
renderers. Consume shared route chrome through its canonical exported pattern;
keep application views free of compatibility wrappers for the same UI.

## Groups

- `shell/` - shared production/showcase `ExplorerWorkspace`, pane sizing, Model screens and application shell layout targets such as `AppShell`, `ShellPane`, `PaneResizer`, `ShellMain`, `RouteFrame`, `RouteLayout`, and `RoutePanel`.
- `chrome/` - workspace chrome targets such as `WorkspaceToolbar`.
- `side-pane/` - Explorer side-pane frames, selections, filters, trees, and legends.
- `detail/` - detail dialogs and shared relation disclosure, Contract Bindings, concept, and ontology detail bodies.
- `content/` - document, markdown, diagram, and code preview frames.
- `feedback/` - product notices and help content.
- `graphs/` - lazy Sigma graph implementations, cancellable ForceAtlas workers and matching controls.
- `reports/` - Trace/Coverage screens and navigation, flow patterns and cancellable ELK workers.
- `thesaurus/` - shared concept hierarchy pane and concept map.
- `files/`, `search/`, `resources/` - public browsing, search and resource patterns.

## Naming

Names should be general inside their folder context. Do not repeat `Explorer`
or `Reqvire` unless the name would be ambiguous without it. Domain nouns such
as `Element`, `Relation`, and `Ontology` are fine when they name actual product
concepts.
