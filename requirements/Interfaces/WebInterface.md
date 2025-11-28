# Elements

### HTML Export

The system SHALL generate comprehensive HTML documentation with all model artifacts by creating a temporary working copy, generating all reports in that copy, and exporting to the output directory.

#### Details
**Working Directory Setup:**
- Create temporary working directory (e.g., in /tmp)
- Generate markdown files from registry with full relations (user-created and auto-generated inverse relations)
- Copy all related system elements (following satisfiedBy and other relations)
- Copy all attachment files to temporary directory preserving structure

**Generation Pipeline (in temporary directory):**
Execute all generation commands treating temporary directory as repository root:
1. Generate all Mermaid diagrams in markdown files
2. Generate index.md (interactive D3.js tree showing containment hierarchy - main entry point)
3. Generate model.md (model-centric visualization with nested relations from root requirements)
4. Generate traces.md (verification upward traceability)
5. Generate coverage.md (verification coverage report)

**HTML Conversion:**
- Convert all markdown files to HTML with embedded styles
- Process Mermaid diagrams for web rendering
- Convert internal .md links to .html links
- Preserve directory structure

**Output:**
- Accept optional `--output` option to specify output directory
- When `--output` is not specified, export to a temporary directory and print the path
- When `--output` is specified, create output folder if not existing
- Copy generated HTML and all artifacts from temp directory to output directory
- Clean up temporary working directory (except when output is temp directory)

**Source Protection:**
- Never modify original repository files
- All generation happens in isolated temporary directory

**Git Directory Exclusion:**
- The .git directory shall never be exported to the output folder
- This prevents internal git metadata from polluting the exported documentation

**Export Related System Elements:**
- Ensure that any related system elements are also copied into output folder to ensure consistency of exported model

**Containment Generation (D3.js Tree):**
The containment page (containment.html) shall display an interactive D3.js collapsible tree showing the containment hierarchy:
1. Root node representing the model root
2. Folder nodes that can be expanded/collapsed
3. File nodes containing element children
4. Element nodes with type-specific icons and colors
5. Attachment nodes as children of elements (element and file attachments)
6. Clickable elements that navigate to their definitions
7. Expand All / Collapse All buttons for tree control

The containment view serves as the primary entry point for HTML documentation, providing an interactive visual overview of the model structure.

**HTML Navigation Bar:**
The system shall provide a fixed navigation bar in all HTML pages with links to key model artifacts for easy access.

The navigation bar must include (left to right):
- Containment: Link to containment.html (interactive D3.js tree - main entry point)
- Model: Link to model.html (model-centric view with nested relations)
- Traces: Link to traces.html (verification upward traceability report)
- Coverage: Link to coverage.html (verification coverage report)

The navigation bar must be:
- Always visible (fixed position) while scrolling
- Consistent across all HTML pages
- Clearly visible and accessible

**HTML Design:**
The system shall design and implement HTML pages with consistent layout, styling, and navigation for browsing the System model.

**Logo and Branding:**
- The navigation bar shall display the Reqvire logo on the left side before the navigation links
- A favicon shall be included for browser tab identification
- Apple touch icons shall be included for mobile device support
- All brand assets shall be exported to an assets folder during HTML export

#### Attachments
  * [Web Interface Style Specification](DesignDocuments/WebInterfaceStyles.md#web-interface-style-specification)
  * [Web Interface Navigation Behavior](DesignDocuments/WebInterfaceStyles.md#web-interface-navigation-behavior)

#### Relations
  * derive: [Attachment Export](#attachment-export)
  * derive: [Containment View Attachment Links](#containment-view-attachment-links)
  * derive: [Diagram Attachment Display](#diagram-attachment-display)
  * derive: [Model-Centric View Generation](#model-centric-view-generation)
  * derive: [Web Interface Color Scheme](#web-interface-color-scheme)
  * derivedFrom: [Web Interface](Interfaces.md#web-interface)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * satisfiedBy: [export.rs](../../core/src/export.rs)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * satisfiedBy: [index_generator.rs](../../core/src/index_generator.rs)
  * satisfiedBy: [base.html](../../core/templates/base.html)
  * satisfiedBy: [model.html](../../core/templates/model.html)
  * verifiedBy: [CLI Help Structure Verification](Verifications/CLIVerifications.md#cli-help-structure-verification)
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

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [export.rs](../../core/src/export.rs)
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

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [containment.rs](../../core/src/containment.rs)
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

#### Attachments
  * [Mermaid Diagram Style Specification](../System/Output/DesignDocuments/DiagramStyles.md#mermaid-diagram-style-specification)

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
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

#### Attachments
  * [Mermaid Diagram Style Specification](../System/Output/DesignDocuments/DiagramStyles.md#mermaid-diagram-style-specification)

#### Relations
  * derive: [Model View Element Navigation](#model-view-element-navigation)
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [export.rs](../../core/src/export.rs)
  * satisfiedBy: [report_model.rs](../../core/src/report_model.rs)
---

### Model View Element Navigation

The system shall make element names in the model-centric view clickable links that navigate to the element's definition in its source file.

#### Details
- Element names displayed as headers shall be hyperlinks
- Links shall point to the element's source file with fragment identifier
- Format: `[Element Name](file_path#element-fragment)`
- Enables direct navigation from model view to element definition

#### Relations
  * derivedFrom: [Model-Centric View Generation](#model-centric-view-generation)
  * satisfiedBy: [report_model.rs](../../core/src/report_model.rs)
  * verifiedBy: [Model View Element Navigation Test](Verifications/WebInterfaceVerifications.md#model-view-element-navigation-test)
---

### Web Interface Color Scheme

The system shall implement a consistent color scheme across all HTML pages optimized for MBSE and requirements management applications.

#### Details
**Primary Colors:**
| Color Name | Hex Code | Usage |
|------------|----------|-------|
| Indigo | #3F51B5 | Navigation bar background, primary branding |
| Indigo Hover | #7986CB | Navigation hover states (lighter for visibility) |
| Indigo Active | #303F9F | Navigation active/pressed states |
| Off-White | #FAFAFA | Body background - reduces eye strain |
| White | #FFFFFF | Content background, navigation text |

**Element Type Colors:**
The web interface shall use consistent colors matching the diagram color scheme:
| Element Type | Color Name | Hex Code | Usage |
|--------------|------------|----------|-------|
| Requirement | Deep Purple | #673AB7 | Core requirements, goals |
| Verification | Emerald Green | #4CAF50 | Validation criteria, testing completion |
| Other | Cool Gray | #9E9E9E | Other element types, external dependencies |

**Status Indicator Colors:**
| Status | Color Name | Hex Code | Usage |
|--------|------------|----------|-------|
| Verified/Passing | Forest Green | #4CAF50 | Verified requirements, passing tests |
| Pending/Warning | Amber | #FFB74D | Unverified items, warnings |
| Failed/Error | Red | #F44336 | Error messages, validation errors |

**Interactive State Colors:**
| State | Color | Hex Code | Usage |
|-------|-------|----------|-------|
| Hover Highlight | Peach | #FFAB91 | Diagram node/edge hover effect |
| Node Hover Shadow | Peach (70%) | rgba(255,171,145,0.7) | Soft drop-shadow on node hover |
| Link Color | Indigo | #3F51B5 | Hyperlinks (matches nav bar, font-weight: 500) |
| Link Hover | Indigo | #3F51B5 | Hyperlinks with underline on hover |
| Nav Hover | Light Indigo | #7986CB | Navigation item hover background |

**Border Colors:**
| Element | Hex Code | Usage |
|---------|----------|-------|
| Header Borders | #EEEEEE | H1/H2 underlines |
| Table Borders | #EEEEEE | Tables, content separation |
| Content Border | #E0E0E0 | Content card borders |
| Code Background | #f6f8fa | Code blocks |
| Diagram Canvas | #FAFAFA | Mermaid diagram background |

**Typography:**
- Primary font: System font stack (sans-serif)
- Monospace font: For code blocks and element identifiers
- Line height: 1.6 for content readability
- Primary text: #212121 (dark gray - headings)
- Secondary text: #424242 (medium gray - body)
- Muted text: #757575 (light gray - de-emphasized)

**Layout:**
- Navigation bar: Fixed position, full width, 50px height
- Content container: Centered with max-width 95% for readability
- Content card: White background with subtle border and shadow
- Responsive design: Adapts to different screen sizes
- Box shadows: rgba(0,0,0,0.08) for subtle depth

**D3.js Containment Tree Colors:**
The containment tree shall use consistent colors for node types:
| Node Type | Color | Hex Code | Icon |
|-----------|-------|----------|------|
| folder | Gray | #9E9E9E | 📁 |
| file | Yellow | #FFCA28 | 📄 |
| user-requirement | Purple | #7E57C2 | 👤 |
| system-requirement | Deep Purple | #673AB7 | 📐 |
| requirement | Deep Purple | #673AB7 | 📐 |
| verification | Green | #4CAF50 | ✅ |
| refinement | Orange | #FF9800 | 🔧 |
| design-document | Brown | #8D6E63 | 📝 |
| attachment-element | Orange | #FF9800 | 🔧 |
| attachment-file | Blue Gray | #607D8B | 📎 |

**D3.js Tree Styling:**
- Meta nodes (attachments): Smaller circles (r=4), italic 11px font, 85% opacity
- Regular nodes: Standard circles (r=6), normal 13px font
- Tree animations: Smooth transitions for expand/collapse
- Node hover: Cursor pointer for interactive nodes

#### Attachments
  * [Color Scheme Specification](../System/Output/DesignDocuments/OutputFormats.md#color-scheme-specification)
  * [Web Interface Style Specification](DesignDocuments/WebInterfaceStyles.md#web-interface-style-specification)

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [containment.rs](../../core/src/containment.rs)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
  * satisfiedBy: [base.html](../../core/templates/base.html)
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

#### Relations
  * derivedFrom: [Web Interface](Interfaces.md#web-interface)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * satisfiedBy: [serve.rs](../../cli/src/serve.rs)
  * trace: [Validate Command](CLI.md#validate-command)
  * verifiedBy: [Serve Command Verification](Verifications/WebInterfaceVerifications.md#serve-command-verification)
---
