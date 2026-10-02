# Elements

### Traces View

The Explorer Traces view shall render verification traceability from the browser-local Project Store as native SPA content.

#### Details
The view is the `#/traces` specialist route implemented by the Explorer `TracesView` module. Detailed trace data, left-pane, row, Mermaid roll-up, modal navigation, and design-system behavior shall follow the Explorer verification trace rendering specification and browser trace diagram generation contract.

In the design-system Mocks view, the same Traces navigation item and `#/traces` route open the native trace-flow preview inside the real Explorer shell. The preview composes its side pane and workspace through the application's view slots, with showcase-local trace fixtures and the shared `TraceFlow` product pattern. Its card, direction, disclosure, focus, and detail-navigation behavior follows the native trace-flow showcase section of the Explorer verification trace rendering specification.

#### Metadata
  * type: specification

#### Relations
  * define: [Traces View Generation](Capabilities.md#traces-view-generation)
---
