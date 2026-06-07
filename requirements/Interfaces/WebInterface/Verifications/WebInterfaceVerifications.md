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
- Verify Model Sunburst/Icicle data contains D3 tree attachment nodes or equivalent Project Store containment attachment records
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
- Model and Traces diagram labels shall not expose full `file#fragment` attachment identifiers as visible node text

##### Test Criteria:
- Create model with element having attachments
- Generate diagram (format or model command)
- Verify Mermaid output contains multiline labels with attachments
- Verify attached refinement element names appear with 📎 prefix
- Verify exported Model route/source pages and Traces route data use compact attachment labels and still render Mermaid containers for the final graph where Mermaid output is present
- Verify diagram renders correctly with attachment labels

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-attachment-export/test.sh)
  * verify: [Diagram Attachment Display](../Capabilities.md#diagram-attachment-display)
---

### HTML Export Verification

This test verifies that the system exports specifications into HTML format with the native SPA Explorer shell and Model route containment modes, then saves them in the designated output location.

#### Details

##### Acceptance Criteria:
- System should export specifications to HTML format
- HTML files should be saved in the designated output location
- HTML output should maintain the structure and content of the original specifications
- System shall generate `index.html` as the primary SPA Explorer shell and browser-local Project Store host
- `index.html` shall contain a Project Store seed before Explorer views render
- The Model route shall display folders, files, and elements through native List, Grid, Sunburst, and Icicle modes.
- Sunburst and Icicle Model modes shall render containment as D3 partition views with click-to-drill/zoom and breadcrumb navigation.
- The native Explorer shell shall not render a top header or primary left-pane view links; Knowledge Graph, Ontologies, Traces, and KN2 are reached from right vertical tool-rail icons.
- The native Explorer shell shall expose shared collapsible vertical `Explorer` and `Inspector` edge strips; views with contextual evidence use the shell-owned 390px right `Inspector` lane instead of defining route-local right-side geometry.
- Old Explorer page URLs shall not be generated; equivalent content shall be reachable through SPA routes and source-document links.
- Links in diagrams and text must be converted to use .html instead of .md
- Paths in HTML files should maintain the original relative structure
- System should work in environments without Git repositories

##### Test Criteria:
- Command exits with success (0) return code
- HTML files are generated at the expected location with .html extension
- Generated KN2 output exposes concept-reference facts as a relation toggle/filter, and disabling that relation can remove concept reference targets from the visible graph.
- Output directory contains `index.html`
- `index.html` contains an Explorer shell marker and Project Store seed
- The Project Store seed includes required sections for files, resources, elements, relations, attachments, concept references, submodels, traces, coverage, ontology, knowledge graph, search, summaries, and routes
- The Project Store seed distinguishes file containers from modeled resources/evidence files
- Hash routes for primary Model, file deep links, Knowledge Graph, Ontologies, Traces, and KN2 views plus supporting Coverage, Resources, Elements, and Search workflows are declared
- Retired Explorer page URLs are absent from generated output and canonical route mappings
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

### SPA Explorer Store Contract Verification

This test verifies that `index.html` is the central SPA Explorer shell and contains or loads the normalized browser-local Project Store required by all current views.

#### Details

##### Acceptance Criteria:
- `index.html` shall be the primary Explorer shell and Project Store host, exported as a native static SPA built with Vite/TypeScript/React using Radix Themes 3, `@radix-ui/react-icons`, and compiled Tailwind.
- The exported shell shall reference local compiled bundle and stylesheet assets with no CDN-loaded framework, no CDN-loaded Tailwind, and no runtime Tailwind compiler.
- The Project Store seed shall be present before view rendering and shall include a schema/version marker.
- The store shall expose normalized top-level sections for project, folders, files, resources, elements, relations, attachments, concept references, submodels, traces, coverage, ontology, knowledge graph, search, summaries, and routes.
- File containers and modeled resources shall be represented as separate record families with explicit cross-references when the same path appears in both roles.
- Route definitions shall include canonical hash routes for current views and element/file/search detail workflows.
- Element-detail routes shall open a Project Store-backed scrollable modal/dialog in the Explorer shell instead of using the generated source page as the primary element navigation target.
- Element-detail modals shall expose source-page navigation as a secondary action using the exported source anchor.
- Standalone Explorer/report HTML entry points shall not be generated.
- Missing or malformed store seed data shall be detectable by automated checks and visible to users as an Explorer diagnostic.

##### Test Criteria:
- Run HTML export on a minimal model with at least one capability, requirement, verification relation, satisfiedBy evidence file, attachment or concept-reference fact, and ontology term when available.
- Parse the generated store seed from `index.html` or its referenced static asset.
- Assert all required top-level store sections exist.
- Assert at least one exported Markdown source path appears in `files`.
- Assert at least one implementation or evidence path referenced by `satisfiedBy`, `trace`, or an attachment appears in `resources`.
- Assert `files` records carry exported HTML/source navigation metadata while `resources` records carry referring-fact evidence.
- Assert canonical routes include primary `#/model`, right-tool specialist `#/knowledge-graph`, `#/ontologies`, `#/traces`, and `#/kn2`, and supporting `#/files`, `#/files/<path>`, `#/coverage`, `#/resources`, `#/elements/<identifier>`, and `#/search`; do not require a separate Containment route and do not generate a standalone Knowledge Graph page.
- Assert at least one generated Explorer element link or search result targets `#/elements/<identifier>` and that the element-detail UI contains a modal/dialog marker plus a secondary source-page link.
- Assert the Model view List/Grid modes render from Project Store `folders` and `files` without an iframe or third-party file-manager widget, expose breadcrumb navigation, sortable file rows, grid cards, shared Inspector-lane search, icon/color legends, source-page secondary actions, and modeled-element rows that open the shared element-detail modal. Assert `#/files` and `#/files/<path>` deep-link into that behavior without creating a separate primary Filesystem view.
- Assert `index.html` loads local compiled SPA bundle/stylesheet assets and contains no Tailwind/framework CDN reference and no runtime Tailwind compiler.
- Assert canonical SPA routes are sufficient for Explorer navigation and that no retired Explorer page adapters or separate Explorer/report implementations are emitted.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [SPA Explorer Shell and Project Store](../Capabilities.md#spa-explorer-shell-and-project-store)
---

### Ontology Model Viewer Analysis Verification

This analysis verifies that the Ontologies page behaves as an ontology model viewer rather than a raw RDF triple viewer.

#### Details
Expected analysis checks:
- Confirm the primary Ontologies visualization, search data, and inspector construct metadata are built from `SemanticIndex.ontology_projection`, the same generated ontology construct projection used by full semantic export.
- Confirm the primary viewer does not render `rdf:type` edges, OWL/RDFS metaclass nodes, RDF list plumbing, anonymous SHACL property-shape blank nodes, or generic literal plumbing as the main user-facing graph.
- Confirm classes, object properties, datatype properties, RDF properties, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic resources are classified into distinct semantic kinds when present, while property terms are projected as graph relationship semantics rather than standalone graph nodes.
- Confirm datatype-property literal values are not graph nodes or visibility filter layers, but remain searchable and appear in the selected subject node inspector as predicate/value evidence.
- Confirm a named IRI typed only by a declared ontology class, without explicit `owl:NamedIndividual`, is shown in graph data and the inspector as a named individual while retaining its `∈` membership construct evidence.
- Confirm visual coloring is driven by semantic kind, not by source provenance, so a class referenced by SHACL remains class-colored, property metadata remains property-typed in links/inspector/search, and actual SHACL node shapes and property shapes use SHACL-specific colors.
- Confirm built-in XSD, RDF, RDFS, OWL, and SHACL namespace references are available as an external-reference layer that is hidden by default and can be enabled for datatype/range audit.
- Confirm object and datatype properties are first-class relationship semantics with aggregated domain and range information rendered as labeled links and inspector property usage evidence, not as standalone graph nodes.
- Confirm SHACL node-shape target classes and property usage rows receive derived slot/facet inspector sections from property-shape paths, datatype/class range constraints, node kind, cardinality, pattern, allowed values, and source-shape evidence.
- Confirm target-class slot/facet sections are labeled as class slots, property usage sections are labeled as usages of the selected property by target classes, and repeated usages with different target classes or source shapes are not presented as duplicate property definitions.
- Confirm raw SHACL evidence is shown only when direct raw constraints are attached to the inspected node and that classes or property metadata with only normalized SHACL overlays do not show an empty raw-evidence section.
- Confirm equivalence groups use stable deterministic identifiers derived from canonical member lists.
- Confirm domain/range, subclass/member-of, disjointness, equivalence groups, inverse properties, property chains, property characteristics, class-expression/restriction constructs, SHACL overlays, provenance/source citations, and symbols are represented from generated semantic projection constructs when present in the ontology input.
- Confirm class-expression nodes used as property domain/range expressions display contextual labels for actual union-valued ontology constraints while preserving expression members and property usage evidence in the inspector, and confirm `refine` does not render as a `Capability ∪ Requirement` range expression because refinement ownership is requirement-only.
- Confirm semantic-query-contract refinements define the direct-authored construct extraction patterns while raw query text remains out of ontology collection and full semantic export.
- Confirm ontology viewer symbols are defined with semantic meaning, raw Unicode code point, rendered Unicode character, allowed viewer usage, tooltip text, and accessible labels.
- Confirm ontology inspector badges render the symbol and semantic label without rendering the raw Unicode code point as visible badge text, and that visible badge labels prefer domain wording such as `Subclass`.
- Confirm subclass and membership badges are directional and are not mirrored onto superclass or class-object nodes solely because those nodes are construct targets.
- Confirm source citations remain available as inspector/search evidence, link to the exported source HTML page fragments, and the exported `ontologies.ttl` artifact remains available for raw RDF/Turtle auditability and downstream tooling.
- Confirm the Ontologies SPA route opens directly on the headerless Explorer shell, fills the available viewport between the persistent left Explorer pane/strip and the shared right `Inspector` lane/tool rail, places the `.ttl` download action in the canonical ontology control/footer location for the active renderer mode, and does not render a retired route-local action bar, raw Turtle/source-block list, page header preamble, or shared content-card footer.
- Confirm the ontology graph uses the Sigma.js, Graphology, and ForceAtlas2 rendering engine already used by the project knowledge graph while preserving ontology-specific projection and filter semantics, and uses Sigma curved-arrow edge programs to separate parallel edges and labels between the same nodes.
- Confirm ontology graph property and construct edge labels are anchored on the same curved connector geometry as the rendered Sigma edge rather than at unrelated straight-line chord positions.
- Confirm normal ontology property relationships render as solid labeled Sigma arrows, while OWL set-operator/class-expression member links use a dedicated Sigma/WebGL edge program, render as unlabeled dashed structural connectors with an open diamond marker at the anonymous construct/source side and an arrowhead at the member target side, retain expression kind/member evidence in node labels and the inspector, and do not draw connector strokes or markers from the edge-label canvas hook.
- Confirm ontology render nodes use one construct-class contract derived from semantic type and projection construct evidence, and that restriction/class-expression classification drives glyph rendering, visibility filters, construct-only node gating, and focused-neighborhood behavior consistently.
- Confirm OWL set-operator/class-expression and restriction nodes render as compact construct circles through Sigma `nodeProgramClasses` and `@sigma/node-image` rather than as ordinary named ontology classes, with Sigma providing the circular node-color background, a construct-kind node label, and an inline SVG pictogram providing a bold semantic glyph without PNG/raster sprites, SVG border detail, transparent square image backgrounds, or custom construct-node hover overlays; out-of-focus construct nodes must dim through the same Sigma reducer path as ordinary nodes.
- Confirm glyph-only construct circles appear only for actual OWL anonymous construct nodes, such as `owl:unionOf`, `owl:intersectionOf`, `owl:complementOf`, and `owl:Restriction`, when their `Class expressions` or `Restrictions` visibility controls and focused edge visibility rules make the construct neighborhood visible.
- Confirm non-property construct labels use `@sigma/edge-curve`'s curved-label renderer so labels follow the same curve geometry as the Sigma edge program.
- Confirm subclass connectors render through a dedicated Sigma/WebGL notation edge program with the same dashed connector stroke style as class-expression construct links and a hollow triangle marker at the superclass target side, while keeping readable `Subclass of` labels anchored on the connector.
- Confirm restriction constructs render as explicit restriction glyph nodes with Sigma-native restriction connector arrows for `on property` and filler/target evidence, and are not flattened into ordinary domain/range property edges.
- Confirm ontology node labels use Sigma default node-label and hover rendering, selected or hovered ordinary ontology nodes display their full labels even when normal unfocused labels are truncated for density, and construct glyph nodes show construct-kind labels while showing their symbols through Sigma image-node rendering.
- Confirm the ontology graph uses ontology-diagram visual conventions: class-like concepts, restrictions, and class expressions render as compact circular or elliptical anchors; properties render as labeled domain/range relationship edges near their anchors rather than graph boxes; datatype/resource/SHACL nodes remain visually distinct; relation edges use directed ontology-diagram connector lines with visible direction arrowheads; and the graph supports Sigma pan/zoom, selected-node centering, and reset-driven layout.
- Confirm ontology relationship edges are hidden in the default full-graph view and appear only for edges incident to the hovered or selected focus node set, plus enabled member/filler edges one semantic step beyond visible construct-only nodes, through Sigma's native edge reducer and `@sigma/edge-curve` edge programs, with relation visibility controls limiting which focused edges are eligible.
- Confirm Sigma z-index and highlighted-node rendering are enabled so the hovered or selected ontology focus neighborhood is painted through Sigma's normal focus path, selected/focused nodes render above focused-neighbor nodes, focused-neighbor nodes render above focused edges, focused edges render above unrelated or muted graph items, and focused edges are not rendered through a separate focused-edge canvas overlay.
- Confirm generic SHACL overlay edges render as unlabeled overlay lines while retaining inspector/projection evidence, unless an edge carries a more specific ontology/SHACL relation label.
- Confirm relation connectors and arrowheads remain visible but subtle enough not to overflow or dominate labels.
- Confirm circular class-anchor size is bounded and grows from graph connection degree rather than label length, so highly connected concepts are visually emphasized while low-degree concepts remain compact.
- Confirm graph nodes, property link labels, inspector badges, and legend swatches use the muted ontology-diagram palette consistently, including blue class anchors, muted property semantics, yellow datatypes, purple named individuals, red SHACL shapes, and a low-contrast gray-green canvas.
- Confirm search, focus, inspector, filters, and the compact legend operate over semantic ontology roles and OWL constructs rather than generic RDF predicate edges.
- Confirm ontology graph nodes can be dragged through Sigma pointer events, updating in-memory Graphology coordinates so users can uncover overlapped relation lines or labels, and confirm the visible view controls expose `Reset` without a separate `Fit` button.
- Confirm the Ontologies and Model/project graph left legend/filter panels use the KN2 graph control width and selected-control treatment: 220px panels, black active buttons with white text/checkmarks, warm-neutral inactive/hover controls, and neutral compact right-inspector headings with no black title bar.
- Confirm the right-inspector headings and content sit inside the shared collapsible `Inspector` lane and remain aligned with the shell-owned right edge strip and tool rail across Model, Knowledge Graph, Ontologies, Traces, KN2, Resources, and Files views.
- Confirm the detailed semantic type color key and construct notation key are passive, while the `Show` group contains the active role and relation visibility controls.
- Confirm the passive type legend exposes separate color swatches only for semantic kinds that can render as nodes, including classes, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic resources; property kinds remain visible through property links, inspector badges, search metadata, and relation visibility controls.
- Confirm the `Show` visibility group exposes one shared button design for terms, datatype property links, object property links, class membership, class disjointness, restrictions, class expressions, SHACL shapes, resources, and external references without replacing Reqvire's richer passive type and notation legends, and that active means shown on the canvas.
- Confirm the grouped role filters expose ontology terms, SHACL shapes, resources, and external references instead of making every detailed type swatch clickable; property links are controlled through datatype-property and object-property visibility controls, and the single SHACL shapes role filter controls both SHACL shape nodes and SHACL overlay relations.
- Confirm selected and hovered focus neighborhoods are computed from currently visible relation filters, so disabling object-property, datatype-property, membership, restriction, class-expression, disjointness, or SHACL relation categories removes neighbors connected only by those hidden relations from the focused/highlighted subgraph without filtering evidence from the inspector; visible construct-only nodes expand the focus through their enabled construct links so union/intersection/complement members and restriction fillers are visible from the selected context.
- Confirm the default filter state opens with datatype-property links, object-property links, and class-membership edges checked, while SHACL shapes, generic resources/individuals, disjointness, equivalence, inverse-property overlays, property chains, property characteristics, restrictions, class expressions, and external references are inactive until selected.
- Confirm role filters are hard gates for node visibility, so disabling SHACL shapes hides both SHACL shape nodes and SHACL overlay relations without requiring a second SHACL slot-overlay checkbox.
- Confirm the passive `Notation` legend covers domain/range, subclass, membership, disjointness, equivalence, inverse, property chain, property characteristic, restriction, class-expression, and SHACL-overlay constructs without exposing those rows as a second construct-filter panel.
- Confirm relation styling is passive visual notation rather than a selectable filter, and the active `Show` relation controls determine which clutter categories are shown.
- Confirm equivalence, inverse-property, property-chain, and property-characteristic constructs remain visible as passive legend/inspector evidence rather than active filters while they have no direct canvas-visible toggle effect.
- Confirm visibility controls affect construct-only canvas nodes, construct-specific canvas edges, and graph node badges without making nodes visible when their role filter is disabled, while the inspector continues to show the selected node's full evidence.
- Confirm graph-registry provenance and generated-projection provenance are not exposed as graph-wide filter axes; source/provenance evidence remains available in the inspector and export data.
- Confirm active filters combine inclusively within one category and narrow together across different active categories on the canvas without narrowing the shared right `Inspector` lane.

#### Metadata
  * type: analysis-verification

#### Relations
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
- System shall serve index.html when accessing root URL
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
- Root URL (/) serves index.html
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
