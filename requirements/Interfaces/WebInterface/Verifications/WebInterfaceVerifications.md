# Elements

### Attachment Export Verification

This test verifies that HTML export preserves attachment identifier links to referenced refinement elements.

#### Details

##### Acceptance Criteria:
- System shall preserve all refinement-identifier attachments referenced by elements
- Attachment identifier links shall resolve to referenced refinement elements in exported HTML
- Duplicate attachments (same refinement referenced multiple times) shall be processed consistently

##### Test Criteria:
- Create model with elements having attachments
- Run HTML export command
- Verify attachment links in exported HTML resolve to refinement element anchors
- Verify identifier targets are navigable from rendered pages

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-attachment-export/test.sh)
  * verify: [Attachment Export](../Capabilities.md#attachment-export)
---

### Containment Attachment Links Verification

This test verifies that the D3.js containment tree view displays attachments as children of elements.

#### Details

##### Acceptance Criteria:
- Elements with attachments shall show attachments as child nodes in D3.js tree
- Element attachments (refinements) shall use wrench icon (🔧) with type `attachment-element`
- Element attachments shall be clickable and navigate to the referenced element

##### Test Criteria:
- Create model with element having attachments
- Run HTML export command
- Verify containment.html contains d3-tree JSON data with attachment nodes
- Verify attachment-element nodes have links to element definitions

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-attachment-export/test.sh)
  * verify: [Containment View Attachment Links](../Capabilities.md#containment-view-attachment-links)
---

### Diagram Attachment Display Verification

This test verifies that diagrams display attachment links within element boxes.

#### Details

##### Acceptance Criteria:
- Element boxes in diagrams shall include attached refinement element names
- Attachments shall be prefixed with paperclip icon (📎)
- Attachments shall appear below element name using line breaks
- Attachment display shall not break diagram rendering

##### Test Criteria:
- Create model with element having attachments
- Generate diagram (format or model command)
- Verify Mermaid output contains multiline labels with attachments
- Verify attached refinement element names appear with 📎 prefix
- Verify diagram renders correctly with attachment labels

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-attachment-export/test.sh)
  * verify: [Diagram Attachment Display](../Capabilities.md#diagram-attachment-display)
---

### HTML Export Verification

This test verifies that the system exports specifications into HTML format with generated containment view (D3.js tree) and saves them in the designated output location.

#### Details

##### Acceptance Criteria:
- System should export specifications to HTML format
- HTML files should be saved in the designated output location
- HTML output should maintain the structure and content of the original specifications
- System shall generate containment.md in the temporary working directory during HTML export
- containment.md shall be converted to containment.html in the output directory
- containment.html shall contain an interactive D3.js collapsible tree showing containment hierarchy
- containment.html shall display folders, files, and elements in a hierarchical tree
- containment.html shall serve as the primary entry point for HTML documentation
- Links in diagrams and text must be converted to use .html instead of .md
- Paths in HTML files should maintain the original relative structure
- System should work in environments without Git repositories

##### Test Criteria:
- Command exits with success (0) return code
- HTML files are generated at the expected location with .html extension
- Output directory contains containment.html file
- containment.html contains D3.js tree visualization
- containment.html includes d3-tree JSON data block
- HTML content preserves the structure and information from the source files
- Links in HTML files use .html extension instead of .md
- Mermaid click links are properly converted from .md to .html
- Both GitHub-style URLs and direct file paths in mermaid click links are handled correctly
- Paths should not have duplicated folder names (e.g., specifications/specifications)
- The .git directory is not present in export output

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-export/test.sh)
  * verify: [HTML Export](../Capabilities.md#html-export)
---

### Ontology Model Viewer Analysis Verification

This analysis verifies that the Ontologies page behaves as an ontology model viewer rather than a raw RDF triple viewer.

#### Details
Expected analysis checks:
- Confirm the primary Ontologies visualization, search data, and inspector construct metadata are built from `SemanticIndex.ontology_projection`, the same generated ontology construct projection used by full semantic export.
- Confirm the primary viewer does not render `rdf:type` edges, OWL/RDFS metaclass nodes, RDF list plumbing, anonymous SHACL property-shape blank nodes, or generic literal plumbing as the main user-facing graph.
- Confirm classes, object properties, datatype properties, RDF properties, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic resources are classified into distinct semantic node kinds when present.
- Confirm datatype-property literal values are not graph nodes or visibility filter layers, but remain searchable and appear in the selected subject node inspector as predicate/value evidence.
- Confirm a named IRI typed only by a declared ontology class, without explicit `owl:NamedIndividual`, is shown in graph data and the inspector as a named individual while retaining its `∈` membership construct evidence.
- Confirm visual coloring is driven by semantic node kind, not by source provenance, so a class or property referenced by SHACL remains class/property-colored while actual SHACL node shapes and property shapes use SHACL-specific colors.
- Confirm built-in XSD, RDF, RDFS, OWL, and SHACL namespace references are available as an external-reference layer that is hidden by default and can be enabled for datatype/range audit.
- Confirm object and datatype properties are first-class visual entities with aggregated domain and range information.
- Confirm SHACL node-shape target classes and named property nodes receive derived slot/facet inspector sections from property-shape paths, datatype/class range constraints, node kind, cardinality, pattern, allowed values, and source-shape evidence.
- Confirm target-class slot/facet sections are labeled as class slots, named-property slot/facet sections are labeled as usages of the selected property by target classes, and repeated usages with different target classes or source shapes are not presented as duplicate property definitions.
- Confirm raw SHACL evidence is shown only when direct raw constraints are attached to the inspected node and that class/property nodes with only normalized SHACL overlays do not show an empty raw-evidence section.
- Confirm equivalence groups use stable deterministic identifiers derived from canonical member lists.
- Confirm domain/range, subclass/member-of, disjointness, equivalence groups, inverse properties, property chains, property characteristics, class-expression/restriction constructs, SHACL overlays, provenance/source citations, and symbols are represented from generated semantic projection constructs when present in the ontology input.
- Confirm class-expression nodes used as property domain/range expressions display contextual labels such as `refine range: Capability ∪ Requirement`, while preserving expression members and property usage evidence in the inspector.
- Confirm semantic-query-contract refinements define the direct-authored construct extraction patterns while raw query text remains out of ontology collection and full semantic export.
- Confirm ontology viewer symbols are defined with semantic meaning, raw Unicode code point, rendered Unicode character, allowed viewer usage, tooltip text, and accessible labels.
- Confirm ontology inspector badges render the symbol and semantic label without rendering the raw Unicode code point as visible badge text, and that visible badge labels prefer domain wording such as `Subclass`.
- Confirm subclass and membership badges are directional and are not mirrored onto superclass or class-object nodes solely because those nodes are construct targets.
- Confirm source citations remain available as inspector/search evidence, link to the exported source HTML page fragments, and the exported `ontologies.ttl` artifact remains available for raw RDF/Turtle auditability and downstream tooling.
- Confirm `ontologies.html` opens directly on the explorer, fills the available viewport below the fixed navigation bar, places the `.ttl` download action in the same compact single-line sidebar footer as the summary counts, and does not render the old top/sidebar action bar, raw Turtle/source-block list, page header preamble, or shared content-card footer.
- Confirm search, focus, inspector, filters, and the compact legend operate over semantic ontology roles and OWL constructs rather than generic RDF predicate edges.
- Confirm the detailed semantic type color key is passive, while visibility controls are separate multi-select filter-in toggles that are active by default for grouped semantic roles, relation edges, construct overlay kinds, and data origins.
- Confirm the passive type legend exposes separate color swatches for classes, object properties, datatype properties, RDF properties, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic resources.
- Confirm the grouped role filters expose ontology terms, properties, SHACL shapes, resources, relations, and external references instead of making every detailed type swatch clickable.
- Confirm role and origin filters are hard gates for node visibility, so disabling SHACL shapes hides SHACL shape nodes even when SHACL-overlay construct filters remain enabled.
- Confirm construct filters cover domain/range, subclass, membership, disjointness, equivalence, inverse, property chain, property characteristic, restriction, class-expression, and SHACL-overlay constructs.
- Confirm construct filters affect construct-specific edges, node badges, derived slot/facet sections, and inspector projection-construct rows without making nodes visible when their role or origin filter is disabled.
- Confirm ontology origin filters can distinguish authored Reqvire ontology/SHACL content, graph-registry extracted model facts, and generated ontology projection constructs, and that generated construct edges disappear when construct origin is disabled even if registry origin remains enabled.
- Confirm active filters combine inclusively within one category and narrow together across different active categories.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Ontologies View Generation](../Capabilities.md#ontologies-view-generation)
  * verify: [OWL Semantic Ontology Projection](../Capabilities.md#owl-semantic-ontology-projection)
  * verify: [Ontology Property-Centric Visualization](../Capabilities.md#ontology-property-centric-visualization)
  * verify: [Ontology Construct Grouping](../Capabilities.md#ontology-construct-grouping)
  * verify: [Ontology Symbol and Badge Vocabulary](../Capabilities.md#ontology-symbol-and-badge-vocabulary)
  * verify: [Ontology Projection Subgraph Materialization](../../../Functional/Output/Reporting.md#ontology-projection-subgraph-materialization)
---

### HTML Export Local Linked Files Verification

This test verifies that HTML export preserves local linked-file references used in exported markdown content.

#### Details

##### Acceptance Criteria:
- Exported HTML pages shall preserve local file reference paths for markdown-rendered links and images
- Local non-markdown files referenced by exported markdown content shall exist in the export output
- Exported HTML pages shall render local file references without converting asset paths to HTML document paths

##### Test Criteria:
- Create model content with local file references
- Run HTML export command
- Verify exported HTML contains the expected local `href` and `<img src>` paths
- Verify the referenced local files exist in the output tree

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-export-local-images/test.sh)
  * verify: [Local Linked File Export](../Capabilities.md#local-linked-file-export)
---

### Model View Element Navigation Test

Test verifies that element names in the model-centric view are clickable links.

#### Test Steps
1. Run `reqvire model` command to generate model report
2. Verify output contains element headers as markdown links
3. Verify links follow format `[Element Name](file_path#fragment)`

#### Expected Results
- Element names are rendered as markdown links
- Links point to source file with element fragment

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [Model View Element Navigation](../Capabilities.md#model-view-element-navigation)
---

### Serve Command Verification

This test verifies that the serve command exports HTML to a temporary directory and starts an HTTP server that serves the model documentation.

#### Details

##### Acceptance Criteria:
- System shall export HTML artifacts to a temporary directory
- System shall start HTTP server on specified host and port
- System shall display clickable terminal link to the server URL
- System shall serve containment.html when accessing root URL
- System shall serve all exported HTML files with correct paths
- System shall serve static assets (SVG diagrams, CSS, etc.)
- System shall return 404 for non-existent files
- System shall set correct Content-Type headers for different file types
- System shall run in quiet mode (suppress verbose export output)
- System shall not automatically open browser window
- System shall display instructions for Ctrl-C stop

##### Test Criteria:
- Command starts successfully and displays server URL with instructions
- Server responds to HTTP requests on specified port
- Root URL (/) serves containment.html
- HTML files are served with text/html content type
- SVG files are served with image/svg+xml content type
- Non-existent paths return 404 status
- Export verbose output is suppressed (quiet mode active)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Serve Command](../Capabilities.md#serve-command)
---
