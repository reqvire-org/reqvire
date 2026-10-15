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
- Colocate `Owner.scenario.tsx` and `Owner.fixture.ts`/`.tsx` with runtime source.
  Scenarios use public `@ds` entities and zero-input default views, with literal
  `scenario` metadata. Helpers use `.scenario-adapter.tsx` or `fixtures/`.
- Token fixtures enumerate real foundation names and render `TokenSamples`.
  Keep token metadata and reachable fixture lists complete when changing tokens.
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
  After edits, inspect MCP build freshness and the real scenario; source coverage
  cannot prove visual, behavioral or production-screen parity.
- Keep APIs, behavior, validation, source references and applicable licenses in
  the owner-local README. Toast's Sonner notice stays in `components/core/toast/`.

MCP guarded editing is enabled in the repository `.mcp.json`. Discover units with
`ads_units`, read targeted source ranges, and submit one owner as a Git unified
patch through `ads_apply_patch`. Mandatory architecture, registered lint, type and
preview-build checks run before application. Other editors must run the same
normal project gates. Local ADS development setup is in [../README.md](../README.md).
