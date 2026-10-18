# Reqvire Explorer design-system guide

Use the ADS workflow skills served by the configured MCP server. Start with
`ads_status`, discover public APIs with `ads_catalog`, and inspect their discovered
scenario IDs before editing. The local source guide is [readme.md](readme.md);
shared ownership and tooling rules belong to the ADS tooling contract.

- Owned UI uses native Preact 11 JSX, hooks, DOM events (`onInput` for live text)
  and SVG attributes (`stroke-width`, `vector-effect`). React-only dependencies
  use the shared aliases in `../preact.config.ts`; never introduce a React renderer.
- Production consumes the `@ds` public barrel. Components own interaction states;
  patterns own reusable product layouts. App adapters own routing, projections,
  live services and application state.
- Keep existing fixed token names and semantic element-role colors. Values and
  branding are project-owned. No inline visual styles or consumer overrides of
  primitive internals. Measured renderer writes and documented non-state hooks
  are explicitly configured in `../ds.config.json`.
- Atomic base and variant styles use matching property names: separate border
  width/style/color, paired start/end padding, individual side-pane spacing and
  explicit font properties where variants override typography. Keep the existing
  token hooks. Review computed sizes, colors and states in built previews;
  portal and variant-tracing limits require rendered inspection too.
- Colocate `Owner.ads.scenario.tsx` and `Owner.ads.fixture.ts`/`.tsx` with runtime source.
  Scenarios use public `@ds` entities and zero-input default views, with literal
  `scenario` metadata. Helpers use `.ads.helpers.ts[x]`; tests use `.ads.test.ts[x]`.
  Optional static owner registration lives in `.ads.contract.ts`; reusable demo
  framing lives in development-only `review/` and is imported from `@ds/review`.
- Token fixtures declare literal `{ token, property, probe }` samples and render
  `TokenSamples`. Keep bindings and coverage complete; every sample must visibly
  apply the real token to its declared property. The App checks rendered effects.
- Shared document presentation belongs to the Linaria `documentClasses()` helper
  in `patterns/shell/DocumentSkin.ts`. Both browser mounts apply it to the body,
  including portals; keep its base cascade layer so component styles win.
  Token sheets contain definitions. `base.css` has document defaults, native
  normalization, focus/selection and reduced-motion accessibility only.
- Larger previews live under `composition/`. Workspace scenarios share normalized
  records and state/callback adapters there. Focused pattern entries own their
  initial fixture settings. Render the same public screen as production; never
  rebuild a second screen from primitives.
- The showcase shell belongs to ADS tooling. Do not add project browser pages,
  navigation replicas or a second preview pipeline. `npm run dev:showcase` starts
  its MCP runtime and managed compiler. Reuse an existing `ads_status.showcaseUrl`.
- `showcase/mcp/main.tsx` mounts/unmounts native Preact inside the App iframe.
  The outer host never receives project CSS/runtime. Preserve theme updates,
  independently mountable scenarios and complete teardown of listeners, timers,
  portals, graph renderers and workers.
- Preserve lazy Sigma imports, worker execution, caches, memoized projections,
  cancellation and stale-result rejection. Sharing a screen does not require
  eagerly loading its renderer or moving app services into the DS.
- Tests belong beside owners or under `src/test/`; app harnesses remain under
  `src/test/fixtures/`. Use `@/test/render` for Preact 11 cleanup flushing.
- Run `npm run check`, relevant tests, `npm run build` and preview compilation.
  After edits, inspect MCP build freshness (`current`, `stale`, `sourceRevision`)
  and the real scenario; source coverage
  cannot prove visual, behavioral or production-screen parity.
- Keep APIs, behavior, validation, source references and applicable licenses in
  the owner-local README. Toast's Sonner notice stays in `components/core/toast/`.

MCP guarded editing is enabled in the repository `.mcp.json`. Discover units with
`ads_units`, read targeted source ranges, and submit one owner as a Git unified
patch through `ads_apply_patch`. Mandatory architecture, registered lint, type and
preview-build checks run before application. Other editors must run the same
normal project gates. Use `ads_capture` with a discovered scenario ID for an
on-demand screenshot. A guarded dry run with `screenshot: true` and explicit
`captures` selections produces current/candidate images without applying source.
Read the returned PNG resources and freshness metadata; images do not replace
behavior checks. Load the served `ads://guides/captures` reference for the full
contract. Local ADS development setup is in [../README.md](../README.md).
