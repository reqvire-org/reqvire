# Elements

### Trace Diagram Projection Data

The system shall provide per-verification trace diagram projection data derived from report trace trees without requiring human-readable Markdown or Mermaid report output.

#### Details
Trace diagrams define report-owned trace diagram data, roll-up semantics, relation direction, and node target metadata. Browser rendering reuses this behavior through contract bindings from the consuming interface requirement.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Trace Diagram Projection Data Contract Specification](Specifications.md#trace-diagram-projection-data-contract-specification)
  * derive: [Trace Diagram Node Target Data](#trace-diagram-node-target-data)
  * derivedFrom: [Model Reports](ReportingRequirements.md#model-reports)
---

### Relation Diagram Semantics

The system shall provide relation diagram semantics that use Reqvire relation names and consistent visual conventions, ensuring diagram consistency and relation semantics remain clear for downstream renderers.

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
  * derivedFrom: [Trace Diagram Projection Data](#trace-diagram-projection-data)
---

### Trace Diagram Node Target Data

The system shall include stable model element identifiers, source-relative paths, or route-neutral target metadata for trace diagram nodes so consuming interfaces can resolve node activation consistently.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Trace Diagram Node Target Data Contract Specification](Specifications.md#trace-diagram-node-target-data-contract-specification)
  * derivedFrom: [Trace Diagram Projection Data](#trace-diagram-projection-data)
---
