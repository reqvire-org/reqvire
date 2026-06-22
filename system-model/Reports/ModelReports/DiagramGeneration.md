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

When requested through reporting commands, the system shall generate Mermaid diagrams with relation filtering in command output without mutating authored model files.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Interactive Mermaid Diagram Node Behavior](#interactive-mermaid-diagram-node-behavior)
  * derive: [Reqvire Relationship Rendering](#reqvire-relationship-rendering)
  * derivedFrom: [Interactive Mermaid Diagrams](#interactive-mermaid-diagrams)
  * satisfiedBy: [report_model.rs](../../../crates/reqvire-core/src/report_model.rs)
  * satisfiedBy: [diagrams.rs](../../../crates/reqvire-core/src/diagrams.rs)
---

### Interactive Mermaid Diagram Node Behavior

The system shall implement interactive click behavior for Mermaid diagram nodes emitted by model, containment, and trace report outputs.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Mermaid Diagram Link Behavior](Behaviors.md#mermaid-diagram-link-behavior)
  * definedBy: [Interactive Mermaid Diagram Node Behavior Contract Specification](Specifications.md#interactive-mermaid-diagram-node-behavior-contract-specification)
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [report_model.rs](../../../crates/reqvire-core/src/report_model.rs)
  * satisfiedBy: [diagrams.rs](../../../crates/reqvire-core/src/diagrams.rs)
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
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [diagrams.rs](../../../crates/reqvire-core/src/diagrams.rs)
---
