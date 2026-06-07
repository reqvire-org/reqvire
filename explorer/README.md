# Reqvire Explorer (SPA shell)

Native static single-page Explorer for the Reqvire Project Store. Built with
Vite + TypeScript + React, Radix Themes 3 + `@radix-ui/react-icons`, and
compiled Tailwind (no CDN-loaded framework or stylesheet, no runtime Tailwind
compiler).

The exported/served `index.html` **is** this bundle: the Rust HTML export
(`core/build.rs` + `core/src/export.rs::write_explorer_index`) embeds the built
`dist/` and injects the Project Store seed before the bundle script. Explorer
views are SPA routes; standalone Explorer/report HTML entry points are not
canonical outputs.

Every route renders natively from the store (model, knowledge graph, traces,
ontologies, KN2, coverage, resources, files, search, and the element-detail
modal). The Knowledge Graph route hosts the current
Sigma/Graphology project graph as a right-rail specialist view. The Model route
hosts List/Grid file-manager modes plus Sunburst/Icicle containment projection
modes through native D3 partition views, and Ontologies uses the
committed Sigma ontology renderer when exported ontology graph data is present.
The shared left Explorer pane starts with active-view controls. Model/file routes
then show the shared project tree; specialist tool-rail views do not inherit that
tree unless they define their own navigator.
The shared shell also owns the right `Inspector` lane: views with inspector
content use the same 390px lane, vertical `Inspector` collapse strip, and
right tool rail instead of defining route-local right-side geometry.
Files renders a Reqvire-native read-only file manager over Project Store
`folders` and `files`: folder tree, breadcrumbs, list/grid modes, sortable
columns, shared Inspector-lane search, icon/color legends, file selection, source-page
secondary actions, and modeled-element rows that open the shared element-detail
modal. It intentionally does not import a third-party file-manager stylesheet or
mount an external file-manager widget, so it stays inside the Explorer design
system.

## Build & embed flow

`vite build` emits deterministic, unhashed assets (`assets/explorer.js` +
`assets/explorer.css`, Tailwind compiled in — no CDN). `core/build.rs` copies
`dist/` into `OUT_DIR` so `cargo build` embeds it. **Build the explorer before
`cargo build`** (CI/`make` do this); the compiled Explorer bundle is a required
build input.

## Commands

```bash
cd explorer
npm install
npm run dev        # local dev server (uses src/store/devFixture.ts)
npm run typecheck  # tsc --noEmit
npm run test       # vitest (route parsing + store loader)
npm run build      # tsc --noEmit && vite build -> dist/ (index.html + deterministic assets)
```

## Interface expected from Task-50 (Rust HTML export)

The SPA reads an immutable browser-local **Project Store** seed. Task-50 owns
producing it from `core/src/html/store.rs`. The frontend resolves the seed in
this order (see `src/store/loadStore.ts`):

1. `window.reqvireProjectStore` — a global object, **or**
2. The JSON object literal inside `<script id="reqvire-project-store">`.

The current export script in `store.rs` already emits exactly this shape:

```html
<script id="reqvire-project-store">
  const reqvireProjectStore = { /* ExplorerProjectStore JSON */ };
</script>
```

### Required of Task-50

- **Inject the seed before the SPA bundle runs.** The `#reqvire-project-store`
  script tag (or the `window.reqvireProjectStore` global) must be present in the
  exported `index.html` before the Explorer's module script executes.
- **Mount point.** Exported `index.html` must contain `<div id="root"></div>`
  and include the built Explorer bundle (`assets/explorer.js` and
  `assets/explorer.css`, emitted by `npm run build`). The SPA must not depend on the previous runtime in `store.rs`
  that mounts separate HTML pages — that iframe/page-mount path is replaced by native
  view modules and can be removed once the bundle is wired in.
- **Schema version.** `schema_version` must equal the constant in
  `src/store/types.ts` (`EXPECTED_SCHEMA_VERSION`), kept in sync with
  `SCHEMA_VERSION` in `store.rs`. A mismatch renders a visible non-fatal banner;
  a missing/malformed seed fails closed (`MissingStoreNotice`).
- **Schema shape.** Top-level sections, mirrored in `src/store/types.ts`:
  `schema_version`, `project`, `folders`, `files`, `resources`, `elements`,
  `relations`, `attachments`, `concept_refs`, `submodels`, `traces`, `coverage`,
  `ontology`, `knowledge_graph`, `search`, `summaries`, `routes`.

`src/store/types.ts` is the authoritative TypeScript mirror of the Rust structs.
The opaque report projections (`submodels`, `traces`, `coverage`, `ontology`,
`knowledge_graph`) are typed `unknown` and will be narrowed as the corresponding
rich views are built.

### Scope boundary

Task-49 did **not** change the Rust Project Store schema or export data. The
only Rust-facing requirement is that the export injects the seed and includes
the built bundle as described above.
```
