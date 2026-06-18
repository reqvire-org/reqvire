# Elements

### Interactive Mermaid Diagrams

The system shall produce interactive visual representations of relationships within the System model in the form of Mermaid diagrams, enabling users to explore relations, navigate the model structure, and understand dependencies.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Diagram Relation Filtering Specification](Specifications.md#diagram-relation-filtering-specification)
  * definedBy: [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * definedBy: [Mermaid Interactive Capabilities Specification](Specifications.md#mermaid-interactive-capabilities-specification)
  * derive: [Diagram Generation](#diagram-generation)
  * derivedFrom: [Model Reports](ReportingRequirements.md#model-reports)
---

### Diagram Generation

When requested, the system shall automatically generate diagrams with relation filtering and save them to the required locations of the model.

#### Metadata
  * type: requirement

#### Relations
  * derive: [File Diagram Reused Contract Context Display](#file-diagram-reused-contract-context-display)
  * derive: [Interactive Mermaid Diagram Node Behavior](#interactive-mermaid-diagram-node-behavior)
  * derive: [SysML-Compatible Relationship Rendering](#sysml-compatible-relationship-rendering)
  * derivedFrom: [Interactive Mermaid Diagrams](#interactive-mermaid-diagrams)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
---

### File Diagram Reused Contract Context Display

The system shall display element reused_contract_context in file-based mermaid diagrams as clickable links to referenced contract elements below the element name within the node box.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [File Diagram Reused Contract Context Display Contract Specification](Specifications.md#file-diagram-reused-contract-context-display-contract-specification)
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
---

### Interactive Mermaid Diagram Node Behavior

The system shall implement interactive click behavior for Mermaid diagram nodes that redirects to the referenced element.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Mermaid Diagram Link Behavior](Behaviors.md#mermaid-diagram-link-behavior)
  * definedBy: [Interactive Mermaid Diagram Node Behavior Contract Specification](Specifications.md#interactive-mermaid-diagram-node-behavior-contract-specification)
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
---

### SysML-Compatible Relationship Rendering

The system shall implement a relationship rendering engine that adheres to SysML notation standards following clearly defined specifications, ensuring diagram consistency and standards compliance.

#### Details
The system shall render relationships using:
- SysML stereotypes («deriveReqt», «verify», «satisfy»)
- Appropriate line styles (dashed or solid)
- Open (hollow) arrowheads
- Correct arrow directions based on hierarchy semantics

Each relation type has specific visual properties and directional semantics defined in the specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [SysML Rendering Specification](Specifications.md#sysml-rendering-specification)
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
---

