# Elements

### Traces View

The Explorer Traces view shall render verification traceability from the browser-local Project Store as native SPA content.

#### Details
The view is the `#/traces` specialist route implemented by the Explorer `TracesView` module. Served and exported Explorer use the shared `TraceFlow` product pattern with real, evaluated Project Store trace graphs. Selecting a verification opens its native React Flow/ELK map; selecting a file presents a compact verification overview. Detailed data ownership, capability context, left-pane selection, asynchronous layout, disclosure, focus, detail/source navigation, and context isolation follow the Explorer verification trace rendering specification.

The design-system Mocks view opens the same exported native pattern through its Traces navigation item and `#/traces` route inside the real Explorer shell. Showcase-local examples exercise the shared design and interaction contract.

#### Metadata
  * type: specification

#### Relations
  * define: [Traces View Generation](Capabilities.md#traces-view-generation)
---
