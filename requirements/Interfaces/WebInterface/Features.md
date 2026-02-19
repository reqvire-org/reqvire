# Elements

### HTML Export

The system SHALL generate comprehensive HTML documentation with all model artifacts by creating a temporary working copy, generating all reports in that copy, and exporting to the output directory.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Attachment Export](#attachment-export)
  * derive: [Containment View Attachment Links](#containment-view-attachment-links)
  * derive: [Diagram Attachment Display](#diagram-attachment-display)
  * derive: [Model-Centric View Generation](#model-centric-view-generation)
  * derive: [Web Interface Color Scheme](#web-interface-color-scheme)
  * derivedFrom: [Web Interface](../Interfaces.md#web-interface)
  * refinedBy: [Web Interface Navigation Behavior](Behaviors.md#web-interface-navigation-behavior)
  * refinedBy: [D3.js Containment Tree Specification](Specifications.md#d3js-containment-tree-specification)
  * refinedBy: [HTML Branding Specification](Specifications.md#html-branding-specification)
  * refinedBy: [HTML Export Pipeline Specification](Specifications.md#html-export-pipeline-specification)
  * refinedBy: [HTML Navigation Bar Specification](Specifications.md#html-navigation-bar-specification)
  * refinedBy: [Web Interface Style Specification](Specifications.md#web-interface-style-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * satisfiedBy: [layouts.rs](../../../core/src/html/layouts.rs)
  * satisfiedBy: [mod.rs](../../../core/src/html/mod.rs)
  * satisfiedBy: [html_export.rs](../../../core/src/html_export.rs)
  * satisfiedBy: [index_generator.rs](../../../core/src/index_generator.rs)
  * verifiedBy: [CLI Help Structure Verification](../CLI/Verifications/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [HTML Export Verification](Verifications/WebInterfaceVerifications.md#html-export-verification)
---

### Attachment Export

The system shall preserve attachment identifier links to referenced refinement elements during HTML export to preserve document completeness and enable navigation.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Attachment Export Refinement Specification](Specifications.md#attachment-export-refinement-specification)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * verifiedBy: [Attachment Export Verification](Verifications/WebInterfaceVerifications.md#attachment-export-verification)
---

### Containment View Attachment Links

The system shall display attachment links as children of elements in the containment D3.js tree to provide quick access to associated documents.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Containment View Attachment Links Refinement Specification](Specifications.md#containment-view-attachment-links-refinement-specification)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * verifiedBy: [Containment Attachment Links Verification](Verifications/WebInterfaceVerifications.md#containment-attachment-links-verification)
---

### Diagram Attachment Display

The system shall display attachment links within element boxes in generated diagrams to show document associations visually.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Diagram Attachment Display Refinement Specification](Specifications.md#diagram-attachment-display-refinement-specification)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Diagram Attachment Display Verification](Verifications/WebInterfaceVerifications.md#diagram-attachment-display-verification)
---

### Model-Centric View Generation

The system shall generate a model-centric visualization during HTML export showing root requirements with nested relations containing full element details.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derive: [Model View Element Navigation](#model-view-element-navigation)
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Model-Centric View Generation Refinement Specification](Specifications.md#model-centric-view-generation-refinement-specification)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
---

### Model View Element Navigation

The system shall make element names in the model-centric view clickable links that navigate to the element's definition in its source file.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model-Centric View Generation](#model-centric-view-generation)
  * refinedBy: [Model View Element Navigation Refinement Specification](Specifications.md#model-view-element-navigation-refinement-specification)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * verifiedBy: [Model View Element Navigation Test](Verifications/WebInterfaceVerifications.md#model-view-element-navigation-test)
---

### Web Interface Color Scheme

The system shall implement a consistent color scheme across all HTML pages following clearly defined specifications optimized for MBSE and requirements management applications.

#### Details
The color scheme shall provide:
- Primary branding colors for navigation and UI elements
- Element type-specific colors for visual differentiation
- Status indicator colors for verification and error states
- Interactive state colors for hover effects and links
- D3.js containment tree node styling with type-specific icons

The system shall ensure color consistency between:
- HTML page styling
- Mermaid diagram rendering
- D3.js containment tree visualization

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Color Scheme Specification](../../Functional/Output/Specifications.md#color-scheme-specification)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * satisfiedBy: [layouts.rs](../../../core/src/html/layouts.rs)
  * satisfiedBy: [styles.rs](../../../core/src/html/styles.rs)
---

### Serve Command

The system SHALL provide a serve command that exports comprehensive HTML documentation and serves it via an HTTP server for browsing.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [HTML Export Pipeline Specification](Specifications.md#html-export-pipeline-specification)

#### Relations
  * derivedFrom: [Web Interface](../Interfaces.md#web-interface)
  * refinedBy: [Serve Command Refinement Specification](Specifications.md#serve-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [serve.rs](../../../cli/src/serve.rs)
  * trace: [Validate Command](../CLI/Commands.md#validate-command)
  * verifiedBy: [Serve Command Verification](Verifications/WebInterfaceVerifications.md#serve-command-verification)
---
