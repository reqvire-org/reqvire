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

Existing report, layout-input, layout-lifecycle and application integration tests
cover the data adapters and shared UI. Run `npm test` and both builds. Inspect
Traces and Coverage in the Mocks tab with matching selection and filters.
