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

`bun run build` runs source checks, unit/integration tests and registered ADS
browser checks. Any failure stops the build:

```
prebuild: lint:ds + assets:ds
build:   lint:style + typecheck + test + build:app + test:ads
```

`@reqvire-org/ads` is pinned to `0.3.38` as a development dependency; project
architecture and preview settings live in `ds.config.json`.
Bun 1.4.1+ manages dependencies and runs MCP and ADS compilation.
Node 24 runs Vitest with a 1,200 MiB heap ceiling and at most two workers, and the
existing Vite app/kit builds with a 1,600 MiB heap ceiling. The 121-node flow
integration case stalls and grows memory when Vitest is forced onto Bun; it
passes under Node. `scripts/run-node.mjs` verifies a real Node executable,
rejects Bun's `node` shim, and prepends the selected Node directory for child
processes. It prefers the pinned Hermit Node, then PATH or installed nvm Node 24;
set `REQVIRE_NODE` to select an explicit executable. Missing real Node fails
immediately. `bun run test` preserves this explicitly selected runtime.
The Bun runtime exceeded a bounded 4 GiB process scope for this Vite workload,
including with `--smol`; the Node build passed in that scope. The app, MCP previews and
optional design-system bundle share the ADS Linaria processing contract.
`bun run lint` runs ADS architecture and configured usage checks once
(`lint:ds`), followed by Biome CSS checks (`lint:style`). Toast usage policy lives
in `ds.config.json` under `usage`; ADS enforces it directly in local checks,
guarded acceptance and validation. No project lint plugin or Oxlint pass remains.
Build hooks run the same ADS gate and asset generation before checked builds.
Document presentation is owned by the Linaria `documentClasses()` helper in
`patterns/shell/DocumentSkin.ts`, applied to the body by both browser mounts.
Its base cascade layer preserves component precedence and styles portals too.
The foundation keeps document defaults, native normalization and accessibility.
For the source-level tour, start with the
[local design-system guide](design-system/readme.md). Use ADS skills for
implementation workflows; shared rules belong to the
[ADS tooling contract](https://github.com/reqvire-org/ads/blob/main/docs/ADS.md).

The public `ExplorerWorkspace` and screen patterns are shared by production
adapters and the fixture showcase. Run `bun run dev:showcase` and select Composition → Explorer workspace to inspect
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


Biome uses `biome.css.json` and runs only on CSS; formatting is disabled. ADS
checks architecture and declared notification usage in TypeScript/JavaScript. The duplicate-selector rule is in
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
install after changing Bun configuration. `REQVIRE_BUILD_EXPLORER=1`
also requests a checked frontend build during Cargo compilation.

The application consumes design-system source through `@ds`; it does not require a
standalone kit. `bun run build:all` builds the checked application plus that
optional kit. `bun run build:ds-bundle` uses
`design-system/vite.bundle.config.ts` and external Preact globals; its
generated outputs are separate from the application bundle and are not tracked.
The bundle-only commands require checks to have run first.

---

## Commands

```bash
cd explorer
bun install
bun run dev        # local dev server (uses src/store/devFixture.ts)
bun run typecheck  # tsc --noEmit
bun run test           # Vitest application and colocated ADS unit tests
bun run test:ads   # registered browser checks through ADS's isolated preview host
bun run build      # source checks + unit/integration tests + SPA build + ADS browser checks
bun run ads-mcp          # stdio MCP + showcase; guarded editing enabled
bun run ads-read-mcp     # stdio MCP + showcase; read-only
bun run dev:showcase    # MCP client showcase; open the printed URL
bun run build:showcase  # checked MCP preview resource graph -> .ds/preview/
bun run build:mcp-previews -- --watch  # advanced: compiler without the managed host
```

### Browser verification

`playwright.ads.config.ts` discovers colocated `.ads.behavior.spec.ts` files.
The owners' `.ads.contract.ts` files register the checks with the `widget`
profile in `ds.config.json`. `bun run test:ads` first validates and compiles the
preview graph, then calls `ads verify .`, which starts
and closes an isolated MCP preview host and supplies the selected cases to the
project's `test:behavior` runner. No running MCP session or fixed port is needed.
The managed preview watcher reuses the matching validated graph, keeping cold
compilation outside its bounded App startup wait. Preparation failures stop
verification; browser timeouts and interaction assertions remain unchanged.
After building previews, run `bunx --no-install ads verify . --unit
components/controls/WorktreeSelector` to select one owner. Run the lower-level `test:behavior` command through ADS so it receives
the verification request and scenario URLs.

Registered checks cover branch-picker selection/dismissal, toast actions and
promise/focus transitions, and real trace-worker layout/expanded-view lifecycle.
They complement the existing Vitest and application browser tests; they do not
certify unregistered behavior or replace routing, worktree/store and Rust-server
end-to-end checks. Vitest excludes the Playwright specs and retains the ordinary
application tests under `src/` and colocated ADS unit tests.

Local browser checks use installed Google Chrome, matching MCP capture settings.
Set `REQVIRE_TEST_BROWSER` (or `CHROME_BIN`) to an absolute browser executable
path to use another Chrome/Chromium installation. CI installs the project's
locked Playwright Chromium with `bun --bun x --no-install playwright install --with-deps
chromium --only-shell` before building. Browser installation is explicit, never
part of the source build. Failure evidence is saved in `.ds/verification/`.
`types/ads-verification.d.ts` supplies the missing development-helper declaration
for the installed ADS helper without relaxing production TypeScript checks.

`build:app` and the MCP preview compiler remain compilation commands; the preview
compiler runs native ADS architecture gates. The checked build and acceptance
policy run the separately configured project scripts.
This keeps verification from recursively invoking itself. PR/release workflows
use the checked build, and PR CI retains the application browser suite after
building the Rust CLI. The bundle-only commands are for those internal stages.

---

## Interface expected from `reqvire serve`

The SPA reads an immutable browser-local **Project Store** seed produced from
`crates/reqvire-core/src/html/store.rs`. The frontend resolves the seed in this order (see
`src/store/loadStore.ts`):

1. `window.reqvireProjectStore` — a global set by `assets/project-store.js`
   loaded by the SPA before Preact mounts.
2. The dev fixture, only during `bun run dev` (no seed → `MissingStoreNotice`
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
catalogue app or hand-maintained registry is needed. Colocated `.ads.scenario.tsx`
and `.ads.fixture.ts` files render the public `@ds` API. Shared normalized example
records and callback adapters live with the workspace composition; focused
screen scenarios supply their own initial view/mode. Full application route and
store harnesses remain under `src/test/`.

`ds.config.json` declares `framework: "preact"`, the `.ds/preview/manifest.json`
resource graph and its managed watcher. Starting ADS MCP also starts the website
and compiler; use the URL from `ads_status`. Components, styles, fonts and runtime
execute only inside the sandboxed App. The host's theme selects the project's
own skin. Failed checks/builds retain the previous preview with stale diagnostics.
Token fixtures declare literal `{ token, property, probe }` samples. The public
`TokenSamples` review helper applies each real token to its declared CSS property;
the App audits the rendered bindings in addition to static coverage. Build
freshness checks source revisions as well as emitted resources.

## ADS MCP and showcase

From the repository root:

```sh
cd explorer
bun install --frozen-lockfile
bun run ads-mcp -- --port 4191
```

Open **http://127.0.0.1:4191/**. Use `bun run ads-read-mcp -- --port 4191` instead
for read-only MCP; run one command at a time. Both start the showcase and managed
preview compiler. `bun run dev:showcase` starts the browser-only runtime.
These launch commands use the exact published ADS version in `bun.lock`, as do
build checks and the preview compiler. Bun 1.3.13 or newer is required.

The repository's `.mcp.json` launches ADS with guarded mutations from the
repository root using:

```sh
bun explorer/node_modules/@reqvire-org/ads/dist/cli.mjs mcp explorer --mutations
```

MCP clients launching scripts should use `bun --silent run ads-mcp` or
`bun --silent run ads-read-mcp` to keep stdout exclusive to JSON-RPC. Restart the
MCP connection after changing client configuration or upgrading ADS.

The managed preview compiler keeps a persistent Vite graph and reuses unchanged
module transforms after edits. Validation still gates publication. One-off
`build:mcp-previews` commands remain checked, fresh builds.

For visual review through MCP, discover a scenario with `ads_scenarios` and call
`ads_capture` with its exact ID. Screenshot-enabled patch/transaction dry runs
produce current/candidate PNG resources without applying source changes. Captures
are requested explicitly; ADS retains at most these two slots per scenario/target.
Read `ads://guides/captures` for settings, named targets and resource freshness.
The project declares Playwright as a development dependency and configures
`mcp.capture.channel: "chrome"`, which requires a locally installed Google Chrome.
To use Playwright Chromium instead, remove that channel setting and run
`bun --bun x --no-install playwright install chromium` from `explorer/`.

React Flow canvas structure is adapted from xyflow 12.12.0 under MIT. Its shared project-owned styling and retained source notice live in the [FlowCanvasSkin guide](design-system/patterns/graphs/FlowCanvasSkin/README.md). The graph engine remains an installed dependency.
