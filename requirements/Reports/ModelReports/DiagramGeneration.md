# Elements

### Interactive Mermaid Diagrams

The system shall produce interactive visual representations of relationships within the System model in the form of Mermaid diagrams, enabling users to explore relations, navigate the model structure, and understand dependencies.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Diagram Generation](#diagram-generation)
  * derivedFrom: [Model Reports](ReportingRequirements.md#model-reports)
  * refinedBy: [Diagram Relation Filtering Specification](Specifications.md#diagram-relation-filtering-specification)
  * refinedBy: [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * refinedBy: [Mermaid Interactive Capabilities Specification](Specifications.md#mermaid-interactive-capabilities-specification)
---

### Diagram Generation

When requested, the system shall automatically generate diagrams with relation filtering and save them to the required locations of the model.

#### Metadata
  * type: requirement

#### Relations
  * derive: [File Diagram Attachment Display](#file-diagram-attachment-display)
  * derive: [Interactive Mermaid Diagram Node Behavior](#interactive-mermaid-diagram-node-behavior)
  * derive: [SysML-Compatible Relationship Rendering](#sysml-compatible-relationship-rendering)
  * derivedFrom: [Interactive Mermaid Diagrams](#interactive-mermaid-diagrams)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
---

### File Diagram Attachment Display

The system shall display element attachments in file-based mermaid diagrams as clickable links to referenced refinement elements below the element name within the node box.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * refinedBy: [File Diagram Attachment Display Refinement Specification](Specifications.md#file-diagram-attachment-display-refinement-specification)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
---

### Interactive Mermaid Diagram Node Behavior

The system shall implement interactive click behavior for Mermaid diagram nodes that redirects to the referenced element.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * refinedBy: [Mermaid Diagram Link Behavior](Behaviors.md#mermaid-diagram-link-behavior)
  * refinedBy: [Interactive Mermaid Diagram Node Behavior Refinement Specification](Specifications.md#interactive-mermaid-diagram-node-behavior-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
---

### SysML-Compatible Relationship Rendering

The system shall implement a relationship rendering engine that adheres to SysML notation standards following clearly defined specifications, ensuring diagram consistency and standards compliance.

#### Details
The system shall render relationships using:
- SysML stereotypes («deriveReqt», «verify», «satisfy», «trace»)
- Appropriate line styles (dashed or solid)
- Open (hollow) arrowheads
- Correct arrow directions based on hierarchy semantics

Each relation type has specific visual properties and directional semantics defined in the specification.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * refinedBy: [SysML Rendering Specification](Specifications.md#sysml-rendering-specification)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
---

### Trace Relation Non-Directional Behavior

The system shall treat trace relations as non-directional for circular dependency detection while maintaining their traceability purpose, ensuring that trace relations do not participate in cycle detection algorithms.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * refinedBy: [Trace Relation Non-Directional Behavior Refinement Specification](Specifications.md#trace-relation-non-directional-behavior-refinement-specification)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
  * verifiedBy: [Invalid Relations Test](../../Verifications/Operations/Validation/ValidationVerifications.md#invalid-relations-test)
  * verifiedBy: [Trace Relations No Cycles Verification](../../Verifications/Verification/Traceability/TraceVerifications.md#trace-relations-no-cycles-verification)
---
