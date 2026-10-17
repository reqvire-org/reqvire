# Reqvire Explorer (SPA shell)

Native static single-page Explorer for the Reqvire Project Store. Built with
Vite + TypeScript + Preact 11 and an in-tree design system; no Tailwind, no Radix
Themes, no CDN dependencies.

The served `index.html` **is** this bundle: `crates/reqvire-core/build.rs` embeds the built
`dist/`, and `reqvire serve` supplies the generated Project Store as
`assets/project-store.js`. Explorer views are SPA routes.

Every route renders natively from the store (model, graph, traces,
ontologies, coverage, resources, files, search, and the element-detail modal).
The Model route offers List/Grid browsing plus a Sigma/Graphology graph mode.
The Ontologies route uses a Sigma ontology renderer when ontology graph data is
present.

---

## Build & embed flow

`npm run build` is lint-gated:

```
prebuild: lint:ds + assets:ds
build:   lint:style + typecheck + build:app
```

`@reqvire-org/ads` is pinned to `0.3.15` as a development dependency; project
architecture and preview settings live in `ds.config.json`.
The app, MCP preview compiler and optional design-system bundle share the ADS
Linaria tag resolver to preserve custom-property arithmetic during extraction.
`npm run lint` runs shared ADS architecture checks and core/registered Oxlint
rules (`lint:ds`), then Biome CSS correctness checks (`lint:style`). Build hooks
refresh ignored `.ds/oxlintrc.json`, validate source, and generate configured
icons before application and standalone showcase builds. Renderer exceptions
and non-state hooks are explicit config; Toast usage rules remain component-owned.
Document presentation is owned by the Linaria `documentClasses()` helper in
`patterns/shell/DocumentSkin.ts`, applied to the body by both browser mounts.
Its base cascade layer preserves component precedence and styles portals too.
The foundation keeps document defaults, native normalization and accessibility.
For the source-level tour, start with the
[local design-system guide](design-system/readme.md). Use ADS skills for
implementation workflows; shared rules belong to the
[ADS tooling contract](https://github.com/reqvire-org/ads/blob/main/docs/ADS.md).

The public `ExplorerWorkspace` and screen patterns are shared by production
adapters and the fixture showcase. Run `npm run dev:showcase` and select Composition → Explorer workspace to inspect
the real UI with local data. Tokens, Components and Patterns provide independently
mountable scenarios. Fixture code has no application imports; full-app
routing/store tests remain under `src/test/`.

Owned UI uses native Preact imports, hooks, DOM events and JSX. ReactFlow and
`react-markdown` use `preact/compat`; npm aliases also resolve their CommonJS
imports to Preact. `preact.config.ts` shares the bundler aliases across the app,
showcase and optional design-system bundle. No React renderer is shipped. DOM tests use
Preact Testing Library with explicit cleanup flushing for Preact 11, and lifecycle
tests exercise unmount/remount behavior directly.

ADS checks have no production runtime. Heavy Sigma renderers stay behind lazy
imports; ELK and ForceAtlas layouts use cancellable local browser workers.
Application projections, caches, worktree loading and routing remain in app
adapters. The migration preserves existing layout profiles and lifecycle tests.
See the [graph guide](design-system/patterns/graphs/README.md#forceatlas-browser-profiling)
for browser profiling commands and interpretation.


Biome uses `biome.css.json` and runs only on CSS; formatting is disabled. Oxlint
continues to check TypeScript/JavaScript. The duplicate-selector rule is in
Biome's nursery group, so its tested version is pinned. This replaces Stylelint
and removes its unpatched `braces` dependency chain. See
[GHSA-vfj7-8cjw-p6xm](https://github.com/advisories/GHSA-vfj7-8cjw-p6xm).

`vite build` emits deterministic, unhashed assets (`assets/explorer.js` +
`assets/explorer.css`). The asset contract validates `design-system/assets/`,
generated browser assets, and optional `public/assets/` raw URL inputs, then
assembles the build output under `dist/assets/`; matching asset filenames fail
the build. `crates/reqvire-core/build.rs` copies `dist/` into `OUT_DIR` so `cargo build`
embeds it. **Build the Explorer before `cargo build`** — the compiled bundle is
a required build input.

From the repository root, `make explorer` installs dependencies when its
successful-install receipt is absent or older than the package manifests, then
runs the checked application build. `make explorer-deps` performs dependency
setup only. Remove `explorer/node_modules/.reqvire-deps.stamp` to force a fresh
install after changing Node or npm configuration. `REQVIRE_BUILD_EXPLORER=1`
also requests a checked frontend build during Cargo compilation.

The application consumes design-system source through `@ds`; it does not require a
standalone kit. `npm run build:all` builds the checked application plus that
optional kit. `npm run build:ds-bundle` uses
`design-system/vite.bundle.config.ts` and external Preact globals; its
generated outputs are separate from the application bundle and are not tracked.
The bundle-only commands require checks to have run first.

---

## Commands

```bash
cd explorer
npm install
npm run dev        # local dev server (uses src/store/devFixture.ts)
npm run typecheck  # tsc --noEmit
npm run test       # vitest (route parsing, store loader, component tests)
npm run build      # lint + typecheck + generated design-system artifacts + vite build -> dist/
npm run ads-mcp          # stdio MCP + showcase; guarded editing enabled
npm run ads-read-mcp     # stdio MCP + showcase; read-only
npm run dev:showcase    # MCP client showcase; open the printed URL
npm run build:showcase  # checked MCP preview resource graph -> .ds/preview/
npm run build:mcp-previews -- --watch  # advanced: compiler without the managed host
```

---

## Interface expected from `reqvire serve`

The SPA reads an immutable browser-local **Project Store** seed produced from
`crates/reqvire-core/src/html/store.rs`. The frontend resolves the seed in this order (see
`src/store/loadStore.ts`):

1. `window.reqvireProjectStore` — a global set by `assets/project-store.js`
   loaded by the SPA before Preact mounts.
2. The dev fixture, only during `npm run dev` (no seed → `MissingStoreNotice`
   in production builds).

The runtime seed has this shape:

```js
window.reqvireProjectStore = { /* ExplorerProjectStore JSON */ };
```

### Runtime contract

- **Serve the seed at the runtime URL.** The SPA loader requests
  `assets/project-store.js` before mounting Preact. That runtime script must
  assign `window.reqvireProjectStore`; it is generated by `reqvire serve`, not
  checked into Explorer source.
- **Mount point.** `index.html` must contain `<div id="root"></div>` and
  include `assets/explorer.js` and `assets/explorer.css`.
- **Schema version.** `schema_version` must equal `EXPECTED_SCHEMA_VERSION` in
  `src/store/types.ts`, kept in sync with `SCHEMA_VERSION` in `store.rs`. A
  mismatch renders a visible non-fatal banner; a missing or malformed seed fails
  closed.
- **Schema shape.** Top-level keys mirrored in `src/store/types.ts`:
  `schema_version`, `project`, `folders`, `files`, `resources`, `elements`,
  `relations`, `attachments`, `concept_refs`, `submodels`, `traces`, `coverage`,
  `ontology`, `knowledge_graph`, `search`, `summaries`, `routes`.
  The `attachments` key is the Project Store compatibility field for
  Contract Bindings entries.

`src/store/types.ts` is the authoritative TypeScript mirror of the Rust structs.

### Scope boundary

Explorer owns the browser shell and design system. Rust owns normalized Project
Store data and the embedded Vite bundle served by `reqvire serve`.

The showcase is ADS tooling's minimal MCP client and Apps host. It discovers
Tokens, Components, Patterns and Composition scenarios; the project owns their
implementations, fixture inputs and native Preact mount adapter. No second
catalogue app or hand-maintained registry is needed. Colocated `.scenario.tsx`
and `.fixture.ts` files render the public `@ds` API. Shared normalized example
records and callback adapters live with the workspace composition; focused
screen scenarios supply their own initial view/mode. Full application route and
store harnesses remain under `src/test/`.

`ds.config.json` declares `framework: "preact"`, the `.ds/preview/manifest.json`
resource graph and its managed watcher. Starting ADS MCP also starts the website
and compiler; use the URL from `ads_status`. Components, styles, fonts and runtime
execute only inside the sandboxed App. The host's theme selects the project's
own skin. Failed checks/builds retain the previous preview with stale diagnostics.
Token fixtures declare literal `{ token, property, probe }` samples. The public
`TokenSamples` pattern applies each real token to its declared CSS property;
the App audits the rendered bindings in addition to static coverage. Build
freshness checks source revisions as well as emitted resources.

## ADS MCP and showcase

From the repository root:

```sh
cd explorer
npm ci
npm run ads-mcp -- --port 4191
```

Open **http://127.0.0.1:4191/**. Use `npm run ads-read-mcp -- --port 4191` instead
for read-only MCP; run one command at a time. Both start the showcase and managed
preview compiler. `npm run dev:showcase` starts the browser-only runtime.
These launch commands use `npx --yes @reqvire-org/ads@0.3.15`; build checks and
the preview compiler use the same published version installed by `npm ci`.

The repository's `.mcp.json` launches ADS with guarded mutations from the
repository root using:

```sh
npx --yes @reqvire-org/ads@0.3.15 mcp explorer --mutations
```

MCP clients launching the npm scripts should use `npm --silent run ads-mcp` or
`npm --silent run ads-read-mcp` to keep stdout exclusive to JSON-RPC. Restart the
MCP connection after changing client configuration or upgrading ADS.

For visual review through MCP, discover a scenario with `ads_scenarios` and call
`ads_capture` with its exact ID. Screenshot-enabled patch/transaction dry runs
produce current/candidate PNG resources without applying source changes. Captures
are requested explicitly; ADS retains at most these two slots per scenario/target.
Read `ads://guides/captures` for settings, named targets and resource freshness.
The project declares Playwright as a development dependency and configures
`mcp.capture.channel: "chrome"`, which requires a locally installed Google Chrome.
To use Playwright Chromium instead, remove that channel setting and run
`npx playwright install chromium` from `explorer/`.
