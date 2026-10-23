# Report screens

`TracesScreen` owns the grouped verification overview and selected `TraceFlow`.
`TraceNavigation` owns its folder/file/verification tree, filtering and summary.
The application prepares trace DTOs only for the selected verification.

`CoverageReportScreen` owns gauges, breakdowns, capability drilldown, gaps and
scoped/empty states. It consumes the published projection without evaluating
coverage. `CoverageNavigationPane` owns the matching search and capability
navigation. Callbacks route inspections; fixture callbacks keep them local.

Coverage uses the Model-style quick filter above its Whole Model navigation,
without another section heading. The application owns the query; navigation
retains matching names or identifiers and their ancestors with temporary
search disclosure. Filtering changes visibility without changing scope,
coverage data or selection URLs. Gap rows show a trailing type badge without a
duplicate leading element icon.

`CoverageDrilldown` owns the title and Verification/Implementation headers.
Headers and hierarchy rows share their column grid and insets; narrow panels
place the title above the metric headers. Rows retain bars, percentages,
counts, meaningful blocker counts and each metric's accessible name.
`CoveragePanel` omits its header when its content supplies one.

`flowLayoutEngine` runs ELK in a dedicated local worker. Each job terminates its
worker on completion, failure or cancellation. The flow owner rejects obsolete
results and keeps the accepted map while replacement work is pending. Both
production and showcase use this service; consumers can inject the same typed
`FlowLayoutEngine` contract for tests or another compatible service.

Model and verification cards share `--space-flow-node-width` and
`--space-flow-node-height` from foundation Data. The pattern measures an unscaled
CSS probe before requesting layout; the same accepted dimensions feed ELK and
React Flow. Size changes cancel obsolete layout work without clearing the last
valid map. There is no separate JavaScript design-size default.

Existing report, layout-input, layout-lifecycle and application integration tests
cover the data adapters and shared UI. Run `npm test` and both builds. Inspect
Traces and Coverage in the Mocks tab with matching selection and filters.


### Registered trace browser checks

`TraceFlow.ads.contract.ts` registers `trace-flow-layout` and `trace-flow-expand`
against the real `patterns/reports/TraceFlow` scenario.
`TraceFlow.ads.behavior.spec.ts` uses the actual layout worker and rendered CSS
dimensions, verifies direction changes, and checks that expansion preserves the
mounted renderer and restores focus. Run them with
`npm run test:ads -- --unit patterns/reports/TraceFlow` from `explorer/`.
The existing colocated Vitest lifecycle/layout tests and application integration
tests remain separate checks.
