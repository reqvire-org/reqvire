# Elements

### Browser Trace Diagram Generation

When displaying Explorer trace views, the browser client shall generate per-verification Mermaid roll-up diagrams from Project Store trace data without requiring CLI Markdown or Mermaid report output.

#### Details
Trace diagrams define the report-owned trace diagram data and roll-up behavior. WebExplorer trace rendering reuses this behavior through contract bindings from the Traces view requirement.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Browser Trace Diagram Generation Contract Specification](Specifications.md#browser-trace-diagram-generation-contract-specification)
  * derive: [Interactive Mermaid Diagram Node Behavior](#interactive-mermaid-diagram-node-behavior)
  * derivedFrom: [Model Reports](ReportingRequirements.md#model-reports)
  * satisfiedBy: [ReportViews.tsx](../../../explorer/src/views/ReportViews.tsx)
---

### Interactive Mermaid Diagram Node Behavior

The Explorer trace view shall implement interactive click behavior for Mermaid diagram nodes rendered from browser-generated trace roll-up diagrams.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Interactive Mermaid Diagram Node Behavior Contract Specification](Specifications.md#interactive-mermaid-diagram-node-behavior-contract-specification)
  * derivedFrom: [Browser Trace Diagram Generation](#browser-trace-diagram-generation)
  * satisfiedBy: [ReportViews.tsx](../../../explorer/src/views/ReportViews.tsx)
---

### Reqvire Relationship Rendering

The system shall implement a relationship rendering engine that uses Reqvire relation names and consistent visual conventions, ensuring diagram consistency and relation semantics remain clear.

#### Details
The system shall render relationships using:
- Reqvire relation labels (`derive`, `verifiedBy`, `satisfiedBy`, and related canonical relation names)
- Appropriate line styles (dashed or solid)
- Open (hollow) arrowheads
- Correct arrow directions based on hierarchy semantics

Each relation type has specific visual properties and directional semantics defined in the specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Reqvire Relation Rendering Specification](Specifications.md#reqvire-relation-rendering-specification)
  * derivedFrom: [Browser Trace Diagram Generation](#browser-trace-diagram-generation)
  * satisfiedBy: [ReportViews.tsx](../../../explorer/src/views/ReportViews.tsx)
---
