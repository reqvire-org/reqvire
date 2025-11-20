# WebInterface

## HTML Documentation Export

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
3. Generate model.md (model-centric visualization with nested relations from root requirements)
4. Generate whole-model.md (complete model diagram showing all elements and relations)
5. Generate traces.md (verification upward traceability)
6. Generate coverage.md (verification coverage report)

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
  * derivedFrom: [Model Export](../UserStories.md#model-export)
  * derivedFrom: [Web Interface](../Interfaces.md#web-interface)
  * derivedFrom: [CLI Interface Structure](CLI.md#cli-interface-structure)
  * derivedFrom: [Deterministic Output for All Generated Content](../ReqvireTool/Reporting.md#deterministic-output-for-all-generated-content)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---

### HTML Navigation Bar

The system SHALL provide a fixed navigation bar in all HTML pages with links to key model artifacts for easy access.

#### Details
The navigation bar must include:
- Home: Link to index.html (model structure overview)
- Containment: Link to containment.html (model containment diagram)
- Model: Link to model.html (model-centric view with nested relations)
- Whole Model: Link to whole-model.html (complete model diagram)
- Traces: Link to traces.html (verification upward traceability report)
- Coverage: Link to coverage.html (verification coverage report)

The navigation bar must be:
- Always visible (fixed position) while scrolling
- Consistent across all HTML pages
- Clearly visible and accessible

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [base.html](../../core/templates/base.html)
  * satisfiedBy: [model.html](../../core/templates/model.html)
---

### Index Generation

The system shall automatically generate an index document during HTML export that contains a structured summary of all specification documents and folders, serving as the primary entry point (index.html) for HTML documentation.

#### Details
The index document generator shall:
1. Traverse all specifications and documents in the model
2. Group elements by file and section
3. Create a hierarchical index with links to documents and elements
4. Generate summary statistics including total files, sections, and elements
5. Generate the index as index.md during HTML export
6. Be integrated into the HTML export pipeline (see [HTML Export](#html-export))

The index generation is automatically performed as part of the HTML export process and saves the result as index.md in the temporary working directory, which is then converted to index.html when exported.

#### Relations
  * derivedFrom: [Model Export](../UserStories.md#model-export)
  * satisfiedBy: [index_generator.rs](../../core/src/index_generator.rs)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * verifiedBy: [HTML Export Verification](../Verifications/Misc.md#html-export-verification)
---

### HTML Design

The system SHALL design and implement HTML pages with consistent layout, styling, and navigation for browsing the MBSE model.

#### Relations
  * derivedFrom: [Model Export](../UserStories.md#model-export)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * satisfiedBy: [base.html](../../core/templates/base.html)
  * satisfiedBy: [model.html](../../core/templates/model.html)
---

### Export Related System Elements

The system shall ensure that any related system elements are also copied into output folder to ensure consistency of exported model.

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
---

### Model-Centric View Generation

The system shall generate a model-centric visualization during HTML export showing root requirements with nested relations containing full element details.

#### Details
- Display root requirements (no hierarchical parent) as top-level entries
- Show relations nested inside elements with full target details recursively
- Include metadata about total elements and relations
- Generate mermaid diagrams showing all nested relations
- Output as markdown with embedded visualizations (model.html)

#### Relations
  * derivedFrom: [Model Export](../UserStories.md#model-export)
  * derivedFrom: [Complete Model Structure Visualization](../DiagramGeneration.md#complete-model-structure-visualization)
  * derivedFrom: [Model Visualization and Exploration](../DiagramGeneration.md#model-visualization-and-exploration)
  * derivedFrom: [HTML Navigation Bar](#html-navigation-bar)
  * satisfiedBy: [export.rs](../../core/src/export.rs)
  * satisfiedBy: [report_model.rs](../../core/src/report_model.rs)
---

### Whole Model Diagram Generation

The system shall generate a complete model diagram during HTML export showing all elements and their relationships.

#### Details
- Display all elements without filtering
- Show all forward relations (derive, satisfiedBy, verifiedBy, trace)
- Group elements by file and section
- Use mermaid diagram format
- Output as whole-model.html

#### Relations
  * derivedFrom: [Model Export](../UserStories.md#model-export)
  * derivedFrom: [HTML Navigation Bar](#html-navigation-bar)
  * satisfiedBy: [export.rs](../../core/src/export.rs)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)
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
  * derivedFrom: [Web Interface](../Interfaces.md#web-interface)
  * trace: [Validate Command](CLI.md#validate-command)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
  * satisfiedBy: [serve.rs](../../cli/src/serve.rs)
  * verifiedBy: [Serve Command Verification](../Verifications/Misc.md#serve-command-verification)
---

