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
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * satisfiedBy: [layouts.rs](../../../core/src/html/layouts.rs)
  * satisfiedBy: [mod.rs](../../../core/src/html/mod.rs)
  * satisfiedBy: [html_export.rs](../../../core/src/html_export.rs)
  * satisfiedBy: [index_generator.rs](../../../core/src/index_generator.rs)
  * satisfiedBy: [Web Interface Navigation Behavior](Behaviors.md#web-interface-navigation-behavior)
  * satisfiedBy: [D3.js Containment Tree Specification](Specifications.md#d3js-containment-tree-specification)
  * satisfiedBy: [HTML Branding Specification](Specifications.md#html-branding-specification)
  * satisfiedBy: [HTML Export Pipeline Specification](Specifications.md#html-export-pipeline-specification)
  * satisfiedBy: [HTML Navigation Bar Specification](Specifications.md#html-navigation-bar-specification)
  * satisfiedBy: [Web Interface Style Specification](Specifications.md#web-interface-style-specification)
  * verifiedBy: [CLI Help Structure Verification](../CLI/Verifications/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [HTML Export Verification](Verifications/WebInterfaceVerifications.md#html-export-verification)
---

### Attachment Export

The system shall copy all attachment files referenced by elements during HTML export to preserve document completeness and enable navigation.

#### Details
During HTML export, the system shall:
- Identify all attachments from element.attachments across the model
- Copy each attachment file to the output directory preserving relative paths
- Skip duplicate attachments (same file referenced by multiple elements)
- Log attachment copying progress

This ensures exported documentation includes all referenced external documents (PDFs, design documents, etc.) for complete offline browsing.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * verifiedBy: [Attachment Export Verification](Verifications/WebInterfaceVerifications.md#attachment-export-verification)
---

### Containment View Attachment Links

The system shall display attachment links as children of elements in the containment D3.js tree to provide quick access to associated documents.

#### Details
For each element with attachments:
- Display attachments as child nodes in the D3.js tree
- Element attachments use wrench icon (🔧) with type `attachment-element`
- File attachments use paperclip icon (📎) with type `attachment-file`
- Element attachments are clickable and navigate to the referenced element
- File attachments show filename and path for reference

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * verifiedBy: [Containment Attachment Links Verification](Verifications/WebInterfaceVerifications.md#containment-attachment-links-verification)
---

### Diagram Attachment Display

The system shall display attachment links within element boxes in generated diagrams to show document associations visually.

#### Details
In Mermaid diagrams:
- Element boxes shall include attachment links below the element name
- Use paperclip icon (📎) prefix for each attachment
- Show filename only (not full path) for space efficiency
- Make attachment links clickable to open the document
- Format using Mermaid's multiline label syntax (`<br/>`)

Example Mermaid node:
```
elementId["Element Name<br/>📎 DesignDoc.md"]
```

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Diagram Attachment Display Verification](Verifications/WebInterfaceVerifications.md#diagram-attachment-display-verification)
---

### Model-Centric View Generation

The system shall generate a model-centric visualization during HTML export showing root requirements with nested relations containing full element details.

#### Details
- Display root requirements (no hierarchical parent) as top-level entries
- Show relations nested inside elements with full target details recursively
- Include metadata about total elements and relations
- Generate mermaid diagrams showing all nested relations
- Output as markdown with embedded visualizations (model.html)

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derive: [Model View Element Navigation](#model-view-element-navigation)
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
---

### Model View Element Navigation

The system shall make element names in the model-centric view clickable links that navigate to the element's definition in its source file.

#### Details
- Element names displayed as headers shall be hyperlinks
- Links shall point to the element's source file with fragment identifier
- Format: `[Element Name](file_path#element-fragment)`
- Enables direct navigation from model view to element definition

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model-Centric View Generation](#model-centric-view-generation)
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
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * satisfiedBy: [layouts.rs](../../../core/src/html/layouts.rs)
  * satisfiedBy: [styles.rs](../../../core/src/html/styles.rs)
  * satisfiedBy: [Color Scheme Specification](../../Functional/Output/Specifications.md#color-scheme-specification)
---

### Serve Command

The system SHALL provide a serve command that exports comprehensive HTML documentation and serves it via an HTTP server for browsing.

#### Details
`serve` command shall:
  - Accept `--host <HOST>` option to specify the bind address (default: localhost)
  - Accept `--port <PORT>` option to specify the server port (default: 8080)
  - Use a random temporary directory for HTML export
  - Run HTML Export to generate complete documentation in temporary directory
  - Start an HTTP server serving static files from the temporary directory
  - Display clickable server URL for user to open in browser
  - Display instructions to press Ctrl-C to stop server
  - Continue serving until terminated by the user (Ctrl-C)

#### Metadata
  * type: requirement

#### Attachments
  * [HTML Export Pipeline Specification](Specifications.md#html-export-pipeline-specification)

#### Relations
  * derivedFrom: [Web Interface](../Interfaces.md#web-interface)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [serve.rs](../../../cli/src/serve.rs)
  * trace: [Validate Command](../CLI/Commands.md#validate-command)
  * verifiedBy: [Serve Command Verification](Verifications/WebInterfaceVerifications.md#serve-command-verification)
---
