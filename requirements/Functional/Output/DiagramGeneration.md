# Elements

### Interactive Mermaid Diagrams

The system shall produce interactive visual representations of relationships within the System model in the form of Mermaid diagrams, enabling users to explore relations, navigate the model structure, and understand dependencies.

#### Metadata
  * type: user-requirement

#### Attachments
  * [Diagram Relation Filtering Specification](Specifications.md#diagram-relation-filtering-specification)

#### Relations
  * derive: [Diagram Generation](#diagram-generation)
  * derivedFrom: [Model Reports](Reporting.md#model-reports)
  * derivedFrom: [Generate Diagrams](../../UserStories.md#generate-diagrams)
  * satisfiedBy: [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * satisfiedBy: [Mermaid Diagram Style Specification](Specifications.md#mermaid-diagram-style-specification)
  * satisfiedBy: [Mermaid Interactive Features Specification](Specifications.md#mermaid-interactive-features-specification)
---

### Diagram Generation

When requested, the system shall automatically generate diagrams with relation filtering and save them to the required locations of the model.

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * [Mermaid Interactive Features Specification](Specifications.md#mermaid-interactive-features-specification)

#### Relations
  * derive: [File Diagram Attachment Display](#file-diagram-attachment-display)
  * derive: [Interactive Mermaid Diagram Node Behavior](#interactive-mermaid-diagram-node-behavior)
  * derive: [SysML-Compatible Relationship Rendering](#sysml-compatible-relationship-rendering)
  * derivedFrom: [Interactive Mermaid Diagrams](#interactive-mermaid-diagrams)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * satisfiedBy: [Diagram Relation Filtering Specification](Specifications.md#diagram-relation-filtering-specification)
  * verifiedBy: [Diagram Generation Test](Verifications/DiagramVerifications.md#diagram-generation-test)
  * verifiedBy: [Diagram Relation Filtering Verification](Verifications/DiagramVerifications.md#diagram-relation-filtering-verification)
  * verifiedBy: [Visualize Model Relationships Verification](Verifications/DiagramVerifications.md#visualize-model-relationships-verification)
---

### File Diagram Attachment Display

The system shall display element attachments in file-based mermaid diagrams as clickable links below the element name within the node box.

#### Details
- Attachments shall appear below the element name using `<br/>` line separator
- Each attachment shall be prefixed with 📎 icon
- Attachment filenames shall be shown (not full paths)
- Clicking an attachment shall navigate to the attachment file
- Multiple attachments shall each appear on separate lines
- Elements without attachments show only their name

Format: `Element Name<br/>📎 attachment1.md<br/>📎 attachment2.md`

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * [Mermaid Interactive Features Specification](Specifications.md#mermaid-interactive-features-specification)

#### Relations
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [File Diagram Attachment Test](Verifications/DiagramVerifications.md#file-diagram-attachment-test)
---

### Interactive Mermaid Diagram Node Behavior

The system shall implement interactive click behavior for Mermaid diagram nodes that redirects to the referenced element.

#### Details
Clickable mermaid diagrams links by default must use relative links to the git repository.

CLI flag options must be provided that can change default behavior to use stable github repository links:
  * diagrams click links are not working on Github if not using stable github repository links
  * from another side that pollutes PR diffs thus choice must be given to the user
  * Commands that generate diagrams (`generate-diagrams`, `export`, `serve`) must expose `--links-with-blobs` CLI flag for that purpose
  * The flag defaults to `false` (use relative paths)

When generating diagram node links and when `--links-with-blobs` flag is set to `true`, the system shall:
- Use stable git repository links (`{repository-url}/blob/{commit-hash}/{file-path}`) when git repository information is available
- Fallback to relative markdown links when git repository information is not available
- Use the current commit hash to ensure links remain stable even as the repository evolves
- Match the same link format used in traceability matrices and change impact reports
- Preserve interactive behavior across all generated diagrams

The `traces` command shall always use relative paths (hardcoded to `false`, no flag needed).

The `change-impact` command shall continue to use GitHub blob URLs by default (unchanged behavior).

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Interactive Features Specification](Specifications.md#mermaid-interactive-features-specification)

#### Relations
  * derivedFrom: [Diagram Generation](#diagram-generation)
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

#### Attachments
  * [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * [Diagram Relation Filtering Specification](Specifications.md#diagram-relation-filtering-specification)

#### Relations
  * derivedFrom: [Diagram Generation](#diagram-generation)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * satisfiedBy: [SysML Rendering Specification](Specifications.md#sysml-rendering-specification)
---

### Trace Relation Non-Directional Behavior

The system shall treat trace relations as non-directional for circular dependency detection while maintaining their traceability purpose, ensuring that trace relations do not participate in cycle detection algorithms.

#### Details
The trace relation behavior shall include:

1. **Circular Dependency Exclusion**:
   - Trace relations shall not be traversed during circular dependency detection
   - The cycle detection algorithm shall skip trace relations to prevent false positive cycles
   - Trace relations exist solely for traceability and documentation purposes

2. **Non-Propagation Behavior**:
   - Changes shall not propagate through trace relations
   - Trace relations shall not be included in change impact analysis
   - Impact trees shall not traverse trace relation connections

3. **Bidirectional Traceability**:
   - Trace relations shall provide bidirectional navigational capability
   - Users can navigate from source to target and target to source
   - Both directions are semantically equivalent for traceability purposes

4. **Validation Behavior**:
   - Trace relations shall be validated for target existence
   - Trace relations shall not require type compatibility validation
   - Trace relations can connect any element type to any other element type

This ensures that trace relations serve their intended purpose of establishing lightweight traceability connections without creating artificial dependency constraints or participating in architectural validation logic.

#### Metadata
  * type: requirement

#### Relations
  * verifiedBy: [Invalid Relations Test](../Core/Verifications/ValidationVerifications.md#invalid-relations-test)
  * verifiedBy: [Trace Relations No Cycles Verification](../Processing/Verifications/TraceVerifications.md#trace-relations-no-cycles-verification)
---
