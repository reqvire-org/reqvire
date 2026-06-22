# Elements

### Traces View

The Explorer Traces view shall render verification traceability from the browser-local Project Store as native SPA content.

#### Details
The view is the `#/traces` specialist route implemented by the Explorer `TracesView` module.

The Traces view shall:
- render a left-pane verification trace tree grouped by trace source file;
- render central trace rows grouped by source file and concrete verification element;
- show per-verification metadata, directly verified requirements, and trace-tree counts from Project Store trace projection data;
- render each verification roll-up diagram from the stored Mermaid trace tree;
- open modeled elements through the shared element-detail modal when trace rows or diagrams reference modeled elements;
- use design-system report patterns and role tokens rather than route-local visual policy.

#### Metadata
  * type: specification

#### Relations
  * define: [Traces View Generation](Capabilities.md#traces-view-generation)
---
