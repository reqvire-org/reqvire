# Web Interface

## HTML Export
### Index Generation

The system shall implement an IndexGenerator component that is automatically invoked during HTML export to create a hierarchical index with links to documents and elements.

#### Details
The index generator shall:
1. Traverse all specifications and documents in the model
2. Group elements by file and section
3. Create a hierarchical index with links to documents and elements
4. Generate summary statistics including total files, sections, and elements
5. Generate the index as index.md during HTML export
6. Be integrated into the HTML export pipeline (see [HTML Export](#html-export))

The index generation is automatically performed as part of the HTML export process and saves the result as index.md in the temporary working directory, which is then converted to index.html when exported.

#### Relations
  * derivedFrom: [Documentation Index for HTML Export](../../UserRequirements.md#documentation-index-for-html-export)
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [../../../core/src/index_generator.rs](../../../core/src/index_generator.rs)
  * satisfiedBy: [../../../core/src/html_export.rs](../../../core/src/html_export.rs)
---

### HTML Navigation Enhancement

The system shall enhance the HTML generator to process SpecificationIndex.md as a special file, adding navigation elements and ensuring it serves as the primary entry point.

#### Details
SpecificationIndex.md file must be saved as index.html file when exported to the HTML output directory.

#### Relations
  * derivedFrom: [Documentation Index for HTML Export](../../UserRequirements.md#documentation-index-for-html-export)
  * satisfiedBy: [../../../core/src/html.rs](../../../core/src/html.rs)
  * satisfiedBy: [../../../core/src/html_export.rs](../../../core/src/html_export.rs)
  * verifiedBy: [../../Verifications/Misc.md#html-export-verification](../../Verifications/Misc.md#html-export-verification)
---

### Model HTML Pages Design

The system SHALL design and implement HTML pages with consistent layout, styling, and navigation for browsing the MBSE model.

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [../../../core/src/html.rs](../../../core/src/html.rs)
  * satisfiedBy: [../../../core/src/html_export.rs](../../../core/src/html_export.rs)
---

### HTML Export

The system SHALL generate comprehensive HTML documentation with all model artifacts by creating a temporary working copy, generating all reports in that copy, and exporting to the output directory.

#### Details
**Working Directory Setup:**
- Create temporary working directory (e.g., in /tmp)
- Use graph registry to identify all model files and artifacts
- Copy all identified files to temporary directory preserving structure
- Copy all related system elements (following satisfiedBy and other relations)

**Generation Pipeline (in temporary directory):**
Execute all generation commands treating temporary directory as repository root:
1. Generate all Mermaid diagrams in markdown files
2. Generate index.md (model structure overview)
3. Generate matrix.svg (traceability matrix visualization)
4. Generate traces.md (verification upward traceability)
5. Generate coverage.md (verification coverage report)

**HTML Conversion:**
- Convert all markdown files to HTML with embedded styles
- Process Mermaid diagrams for web rendering
- Convert internal .md links to .html links
- Preserve directory structure

**Output:**
- Accept `--output` option (default: 'html')
- Create output folder if not existing
- Copy generated HTML and all artifacts from temp directory to output directory
- Add .gitignore file to output directory ignoring all files except itself
- Clean up temporary working directory

**Source Protection:**
- Never modify original repository files
- All generation happens in isolated temporary directory

#### Relations
  * derivedFrom: [Export HTML specifications](../../UserRequirements.md#export-html-specifications)
  * derivedFrom: [CLI Interface Structure](CLI.md#cli-interface-structure)
  * satisfiedBy: [../../../core/src/html_export.rs](../../../core/src/html_export.rs)
  * satisfiedBy: [../../../core/src/html.rs](../../../core/src/html.rs)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
---

### Export Related System Elements

The system shall ensure that any related system elements are also copied into output folder to ensure consistency of exported model.

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [../../../core/src/html_export.rs](../../../core/src/html_export.rs)
  * satisfiedBy: [../../../core/src/html.rs](../../../core/src/html.rs)
---

### Model Diagram SVG Generation

The export command shall generate model.svg that visualizes the entire model structure using nested containment boxes (folders > files > sections > elements) with relations connecting elements, rendered in a style consistent with existing mermaid diagrams.

#### Details
The model diagram shall:
- Use nested containment boxes to show the hierarchy: folders contain files, files contain sections, sections contain elements
- Display elements with their relations connecting them inside their containing sections
- Use consistent visual styling (colors, fonts, line styles) matching existing mermaid diagrams generated by the system
- Render as a standalone SVG file suitable for embedding in HTML documentation
- Be generated as part of the HTML export pipeline alongside other artifacts (index.md, traces.md, matrix.svg, coverage.md)
- Follow the same generation approach as matrix.svg using mermaid syntax or direct SVG generation

The diagram helps users quickly understand the overall model structure and navigate complex requirement hierarchies visually.

#### Relations
  * derivedFrom: [Complete Model Structure Visualization](../../UserRequirements.md#complete-model-structure-visualization)
  * derivedFrom: [HTML Export](#html-export)
  * verifiedBy: [../../Verifications/DiagramsTests.md#model-diagram-svg-verification](../../Verifications/DiagramsTests.md#model-diagram-svg-verification)
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
  * derivedFrom: [Browse Model via Web Interface](../../UserRequirements.md#browse-model-via-web-interface)
  * derivedFrom: [HTML Export](#html-export)
  * trace: [Validate Command](CLI.md#validate-command)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [../../../cli/src/serve.rs](../../../cli/src/serve.rs)
  * verifiedBy: [../../Verifications/Misc.md#serve-command-verification](../../Verifications/Misc.md#serve-command-verification)
---
