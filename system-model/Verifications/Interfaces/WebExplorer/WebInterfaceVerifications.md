# Elements

### Web Explorer Interface Verification Objective

This objective groups verification that the served Web Explorer renders model data, assets, diagrams, navigation, responsive layouts, and export flows correctly.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Component Reuse Verification](#component-reuse-verification)
  * derive: [Contract Bindings Link Serving Verification](#contract-bindings-link-serving-verification)
  * derive: [Diagram Contract Bindings Display Verification](#diagram-contract-bindings-display-verification)
  * derive: [Element Detail Inline Concept Reference Verification](#element-detail-inline-concept-reference-verification)
  * derive: [Explorer Serve Verification](#explorer-serve-verification)
  * derive: [Export Command Verification](#export-command-verification)
  * derive: [Mobile Responsiveness Verification](#mobile-responsiveness-verification)
  * derive: [Model Containment Contract Bindings Links Verification](#model-containment-contract-bindings-links-verification)
  * derive: [Model View Element Navigation Test](#model-view-element-navigation-test)
  * derive: [Ontology Model Viewer Analysis Verification](#ontology-model-viewer-analysis-verification)
  * derive: [Responsive Design Verification](#responsive-design-verification)
  * derive: [Serve Command Verification](#serve-command-verification)
  * derive: [SPA Explorer Store Contract Verification](#spa-explorer-store-contract-verification)
  * derive: [Thesaurus Project Store Projection Verification](#thesaurus-project-store-projection-verification)
---

### Component Reuse Verification

This analysis verifies Explorer components are reused across routes without duplicated route-local renderers.

#### Details
Expected checks:
- `index.html` is the single Explorer SPA shell entry point.
- Browser chrome is implemented by shared Explorer components.
- Separate Explorer/report document entry points are not emitted.
- Source code is organized in reusable route and shell modules.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Component-Based Explorer Architecture](../../../Interfaces/WebExplorer/ExplorerRendering.md#component-based-explorer-architecture)
---

### Contract Bindings Link Serving Verification

This test verifies that the served Explorer preserves contract_bindings identifier links to referenced contract elements.

#### Details

##### Acceptance Criteria:
- System shall preserve all contract-identifier contract_bindings referenced by elements
- Contract Bindings identifier links shall resolve to referenced contract elements in Explorer content and element detail workflows
- Duplicate contract_bindings (same contract referenced multiple times) shall be processed consistently

##### Test Criteria:
- Create model with elements having contract_bindings
- Run the Explorer through the serve workflow or a Project Store fixture
- Verify contract_bindings links resolve to contract element records and source anchors
- Verify identifier targets are navigable from rendered content routes and element modals

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Contract Bindings Link Serving](../../../Interfaces/WebExplorer/Capabilities.md#contract-bindings-link-serving)
---

### Diagram Contract Bindings Display Verification

This test verifies that diagrams display contract_bindings links within element boxes.

#### Details

##### Acceptance Criteria:
- Element boxes in diagrams shall include bound contract element names
- Contract Bindings shall be prefixed with paperclip icon (📎)
- Contract Bindings shall appear below element name using line breaks
- Contract Bindings display shall not break diagram rendering
- Model and Traces diagram labels shall not expose full `file#fragment` contract_bindings identifiers as visible node text

##### Test Criteria:
- Create model with element having contract_bindings
- Generate diagram (format or model command)
- Verify Mermaid output contains multiline labels with contract_bindings
- Verify bound contract element names appear with 📎 prefix
- Verify Model route/source content and Traces route data use compact contract_bindings labels and still render Mermaid containers for the final graph where Mermaid output is present
- Verify diagram renders correctly with contract_bindings labels

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Diagram Contract Bindings Display](../../../Interfaces/WebExplorer/Capabilities.md#diagram-contract-bindings-display)
---

### Element Detail Inline Concept Reference Verification

This component test verifies that regular element-detail modals render authored concept references as inline native concept links instead of as a separate source subsection or ontology-node fallback.

#### Details

##### Acceptance Criteria:
- The regular element-detail modal shall hide the authored `#### Concept References` source subsection from rendered body content.
- Referenced native SKOS concepts shall be matched in prose by preferred label, alternative labels, and authored reference label.
- Matching prose terms shall render as inline links using the standard Explorer link color with no resting background and underline only on hover or focus, rather than as badges, glyphs, pills, or a separate Concept References section.
- Activating an inline concept-reference link shall open the referenced native `concept` element modal.
- Authored model concept references shall not open ontology-node modals.

##### Test Criteria:
- Render an element-detail modal for an element with a native concept reference whose label appears in the main body.
- Assert no `Concept References` section heading or raw concept IRI appears in the rendered modal body.
- Assert the matching prose term is rendered as an inline concept-reference control.
- Click the inline concept-reference control and assert the native concept element identifier is opened.
- Render an element-detail modal where the body uses an alternative label for the referenced native concept.
- Assert the alternative-label prose term opens the same native concept element.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [ElementDetailModal.test.tsx](../../../../explorer/src/components/ElementDetailModal.test.tsx)
  * verify: [SPA Explorer Shell and Project Store](../../../Interfaces/WebExplorer/Capabilities.md#spa-explorer-shell-and-project-store)
---

### Explorer Serve Verification

This test verifies that the system serves the native SPA Explorer shell with Model route containment modes and Project Store data.

#### Details

##### Acceptance Criteria:
- System shall serve `index.html` as the primary SPA Explorer shell and browser-local Project Store host
- `index.html` shall contain a Project Store seed before Explorer views render
- The Model route shall display folders, files, elements, and the project graph through native List, Grid, and Graph modes.
- The Model project tree shall initialize with only the root project node expanded, so first-level folders/files are visible while deeper folder and file element rows remain collapsed until user action or selected-descendant reveal behavior requires expansion.
- Modeled-element Grid cards shall use a single leading element marker, keep the title close to that marker, and render adjacent type badges without repeating the marker dot, shape, or glyph.
- Graph mode shall render the project knowledge graph with pan/zoom, search/focus, selected-node state, and graph filters in the Model left pane.
- The native Explorer shell shall not render primary left-pane view links; Ontologies and Traces are reached as specialist Explorer views, while the project Knowledge Graph is reached as Model Graph mode.
- The native Explorer shell shall expose the shared collapsible vertical `Explorer` edge strip and a compact right tool rail; views with contextual evidence use left-pane selected-item links and shared detail modals instead of defining route-local right-side geometry.
- Old Explorer page URLs shall not be generated; equivalent content shall be reachable through SPA routes and source-document links.
- Links in diagrams and text shall resolve through Explorer content routes or Project Store source-content records
- Paths in served content shall maintain the original relative structure
- Project Store file records shall expose source content generated directly from modeled element source files and existing graph-referenced local implementation/evidence/resource files, without depending on generated Markdown files on disk.
- Relation-backed implementation files, evidence files, scripts, images, and other local resource targets shall remain Project Store resources for relation semantics, and existing repository-relative local targets shall appear in the Model tree under their full path.
- System should work in environments without Git repositories

##### Test Criteria:
- Command exits with success (0) return code
- The served root URL returns `index.html`
- `/assets/project-store.js` contains an Explorer Project Store seed
- The Project Store seed includes required sections for files, resources, elements, relations, contract_bindings, concept references, submodels, traces, coverage, ontology, knowledge graph, search, summaries, and routes
- The Project Store seed distinguishes modeled file containers from modeled resources/evidence files
- Project Store file records include normalized source Markdown content derived from the registry for modeled files and raw source-preview content for existing registry-linked local resources
- Relation-backed local resource targets are present as resources with source-preview content when the local file exists, and existing resource-only paths appear in the Project Store `files` and `folders` hierarchy under their full repository-relative path
- Nonexistent local targets, unsupported parsed pages, unrelated repository files, and external URLs are absent from the Project Store file-tree hierarchy
- A plain `assets/project-store.js` request after a direct filesystem edit returns the already-materialized runtime store and does not regenerate model data from disk
- Hash routes for primary Model, file deep links, Ontologies, and Traces views plus supporting Coverage, Resources, Elements, and Search workflows are declared; the project Knowledge Graph is not a separate hash route
- Retired Explorer page URLs are absent from generated output and canonical route mappings
- Explorer content preserves the structure and information from the source files
- Model tree initial render shows only root-level project children and does not eagerly expand child folders or file element rows.
- Modeled-element Grid card markup renders one `ElementIcon` per element card and uses text-only type badges when the icon is already present.
- Mermaid click links resolve through canonical Explorer routes or source anchors
- Both GitHub-style URLs and direct file paths in mermaid click links are handled correctly
- Paths should not have duplicated folder names (e.g., specifications/specifications)
- Missing embedded asset paths return 404 while non-asset browser routes return the SPA shell for client-side routing
- Existing repository-relative static assets referenced by Markdown image or document links return the file bytes with an appropriate content type
- Repository asset requests reject parent-directory traversal and unsupported static asset extensions

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Served Explorer Browser Interface](../../../Interfaces/WebExplorer/Capabilities.md#served-explorer-browser-interface)
---

### Export Command Verification

This test verifies that the export command writes a complete self-contained static Explorer site to the specified output directory.

#### Details

##### Acceptance Criteria:
- System shall write `index.html` to the output directory
- System shall write `assets/project-store.js` containing `window.reqvireProjectStore`
- System shall write `ontologies.ttl` to the output directory
- System shall write all other embedded SPA bundle assets to the output directory
- System shall copy repository-local static assets referenced by rendered workspace content using their repository-relative output paths
- Output directory shall be self-contained and serve correctly from a static file host

##### Test Criteria:
- Run `reqvire export --output <tmpdir>` on a minimal model workspace
- Verify `index.html` exists and contains the Explorer SPA shell
- Verify `assets/project-store.js` exists and contains `reqvireProjectStore`
- Verify `ontologies.ttl` exists
- Verify an exported workspace containing a Markdown image such as `![Diagram](images/diagram.png)` includes `images/diagram.png` in the output and the exported Explorer renders it without a broken image request
- Verify the exported assets reference no external CDN resources

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-export-command/test.sh)
  * verify: [Export Command](../../../Interfaces/WebExplorer/Capabilities.md#export-command)
---

### Mobile Responsiveness Verification

This test verifies the Explorer is usable on mobile devices.

#### Details
Expected checks:
- Desktop and mobile viewports can use the Explorer shell without horizontal page overflow.
- Left Explorer pane and right tool rail remain compact and usable without a top header.
- Touch targets remain usable on common mobile viewports.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Mobile-Friendly Explorer](../../../Interfaces/WebExplorer/ExplorerRendering.md#mobile-friendly-explorer)
  * verify: [Responsive Explorer Rendering](../../../Interfaces/WebExplorer/ExplorerRendering.md#responsive-explorer-rendering)
---

### Model Containment Contract Bindings Links Verification

This test verifies that the served Explorer Model containment data preserves contract_bindings links from modeled elements to referenced contract elements.

#### Details

##### Acceptance Criteria:
- Elements with contract_bindings shall expose contract_bindings records or equivalent Project Store containment contract_bindings records
- Element contract_bindings records that target contract elements shall use the shared Explorer element-role and subtype glyph contract rather than a report-specific symbol
- Element contract_bindings shall be clickable from supported Explorer surfaces and navigate to the referenced element detail/source route

##### Test Criteria:
- Create model with element having contract_bindings
- Run the Explorer through the serve workflow or a Project Store fixture
- Verify Model List/Grid data contains contract_bindings records or equivalent Project Store containment contract_bindings records
- Verify contract-bindings-element records have links to element definitions or element-detail routes

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Containment View Contract Bindings Links](../../../Interfaces/WebExplorer/Capabilities.md#containment-view-contract-bindings-links)
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
  * verify: [Model View Element Navigation](../../../Interfaces/WebExplorer/Capabilities.md#model-view-element-navigation)
---

### Ontology Model Viewer Analysis Verification

This analysis verifies that the Ontologies page behaves as an ontology model viewer rather than a raw RDF triple viewer.

#### Details
Expected analysis checks:
- Confirm the primary Ontologies visualization, search data, and ontology node modal construct metadata are built from `SemanticIndex.ontology_projection`, the same generated ontology construct projection used by full semantic export.
- Confirm ontology graph data includes explicit `authored`, `concepts`, `reqvire-context`, and `external-source` layer semantics; authored structural ontology/projection facts are controlled by the Core layer, SKOS concept nodes, SKOS concept taxonomy edges, plus one-way `mapsToConcept` bridge edges are controlled by the Concepts layer, generated semantic context is hidden until its layer is enabled and is limited to model-to-term `declaresTerm` and `referencesTerm` provenance, and external-source vocabulary is a separate opt-in layer.
- Confirm the primary viewer does not render `rdf:type` edges, OWL/RDFS metaclass nodes, RDF list plumbing, anonymous SHACL property-shape blank nodes, or generic literal plumbing as the main user-facing graph.
- Confirm classes, object properties, datatype properties, RDF properties, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic resources are classified into distinct semantic kinds when present, while property terms are projected as graph relationship semantics rather than standalone graph nodes.
- Confirm datatype-property literal values are not graph nodes or visibility filter layers, but remain searchable and appear in the selected subject node modal as predicate/value evidence.
- Confirm a named IRI typed only by a declared ontology class, without explicit `owl:NamedIndividual`, is shown in graph data and the ontology node modal as a named individual while retaining its `∈` membership construct evidence.
- Confirm visual coloring is driven by semantic kind, not by source provenance, so a class referenced by SHACL remains class-colored, property metadata remains property-typed in links/modal/search, and actual SHACL node shapes and property shapes use SHACL-specific colors.
- Confirm RDF, RDFS, XSD, OWL reserved vocabulary and core SHACL shape syntax do not require local External Ontology source files and are not presented as imported external-source vocabulary. The external-source layer is reserved for used external subset triples derived from actual `#### External Ontology` dependencies, unused raw external terms are absent from graph data and search, and external-source metadata identifies `used_subset` materialization.
- Confirm object and datatype properties are first-class relationship semantics with aggregated domain and range information rendered as labeled links and modal property usage evidence, not as standalone graph nodes.
- Confirm SHACL node-shape target classes and property usage rows receive derived slot/facet modal sections from property-shape paths, datatype/class range constraints, node kind, cardinality, pattern, allowed values, and source-shape evidence.
- Confirm target-class slot/facet sections are labeled as class slots, property usage sections are labeled as usages of the selected property by target classes, and repeated usages with different target classes or source shapes are not presented as duplicate property definitions.
- Confirm raw SHACL evidence is shown only when direct raw constraints are bound to the inspected node and that classes or property metadata with only normalized SHACL overlays do not show an empty raw-evidence section.
- Confirm equivalence groups use stable deterministic identifiers derived from canonical member lists.
- Confirm domain/range, subclass/member-of, disjointness, equivalence groups, inverse properties, property chains, property characteristics, class-expression/restriction constructs, SHACL overlays, provenance/source citations, and symbols are represented from generated semantic projection constructs when present in the ontology input.
- Confirm class-expression nodes used as property domain/range expressions display contextual labels for actual union-valued ontology constraints while preserving expression members and property usage evidence in the ontology node modal, and confirm `define` does not render as a `Capability ∪ Requirement` range expression because contract ownership is requirement-only.
- Confirm ontology viewer symbols are defined with semantic meaning, raw Unicode code point, rendered Unicode character, allowed viewer usage, tooltip text, and accessible labels.
- Confirm ontology modal badges render the symbol and semantic label without rendering the raw Unicode code point as visible badge text, and that visible badge labels prefer domain wording such as `Subclass`.
- Confirm the ontology node modal uses a single-column content flow with RDF type, full URI, OWL document, description, and notation at the top before projection constructs, literal values, raw evidence, and sources.
- Confirm subclass and membership badges are directional and are not mirrored onto superclass or class-object nodes solely because those nodes are construct targets.
- Confirm source citations remain available as modal/search evidence, link to served source route fragments, and the served `ontologies.ttl` artifact remains available for raw RDF/Turtle auditability and downstream tooling.
- Confirm OWL ontology document IRIs typed as `owl:Ontology` do not render as ontology graph nodes, `rdfs:isDefinedBy` does not render as a canvas edge, and selected authored ontology terms expose their OWL document IRI in modal/search metadata.
- Confirm the Ontologies SPA route opens directly on the headerless Explorer shell, fills the available viewport between the persistent left Explorer pane/strip and the right tool rail, places the `.ttl` download action in the Ontologies left pane with the summary controls, and does not render a retired route-local action bar, raw Turtle/source-block list, page header preamble, shared content-card footer, or route-local right sidebar.
- Confirm the Ontologies SPA route paints the shell, graph canvas, and design-system spinner loading notice before deferred Sigma/ForceAtlas renderer construction starts, and clears the loading notice after the ontology renderer is mounted.
- Confirm the ontology graph uses the Sigma.js, Graphology, and ForceAtlas2 rendering engine already used by the project knowledge graph while preserving ontology-specific projection and filter semantics, and uses Sigma curved-arrow edge programs to separate parallel edges and labels between the same nodes.
- Confirm ontology graph property and construct edge labels are anchored on the same curved connector geometry as the rendered Sigma edge rather than at unrelated straight-line chord positions.
- Confirm normal ontology property relationships render as solid labeled Sigma arrows, while OWL set-operator/class-expression member links use a dedicated Sigma/WebGL edge program, render as unlabeled dashed structural connectors with an open diamond marker at the anonymous construct/source side and an arrowhead at the member target side, retain expression kind/member evidence in node labels and the ontology node modal, and do not draw connector strokes or markers from the edge-label canvas hook.
- Confirm ontology render nodes use one construct-class contract derived from semantic type and projection construct evidence, and that restriction/class-expression classification drives glyph rendering, visibility filters, construct-only node gating, and focused-neighborhood behavior consistently.
- Confirm OWL set-operator/class-expression and restriction nodes render as compact construct circles through Sigma `nodeProgramClasses` and `@sigma/node-image` rather than as ordinary named ontology classes, with Sigma providing the circular node-color background, a construct-kind node label, and an inline SVG pictogram providing a bold semantic glyph without PNG/raster sprites, SVG border detail, transparent square image backgrounds, or custom construct-node hover overlays; out-of-focus construct nodes must dim through the same Sigma reducer path as ordinary nodes.
- Confirm glyph-only construct circles appear only for actual OWL anonymous construct nodes, such as `owl:unionOf`, `owl:intersectionOf`, `owl:complementOf`, and `owl:Restriction`, when their `Class expressions` or `Restrictions` visibility controls and focused edge visibility rules make the construct neighborhood visible.
- Confirm non-property construct labels use `@sigma/edge-curve`'s curved-label renderer so labels follow the same curve geometry as the Sigma edge program.
- Confirm subclass connectors render through a dedicated Sigma/WebGL notation edge program with the same dashed connector stroke style as class-expression construct links and a hollow triangle marker at the superclass target side, while keeping readable `Subclass of` labels anchored on the connector.
- Confirm restriction constructs render as explicit restriction glyph nodes with Sigma-native restriction connector arrows for `on property` and filler/target evidence, and are not flattened into ordinary domain/range property edges.
- Confirm ontology node labels use Sigma default node-label and hover rendering, selected or hovered ordinary ontology nodes display their full labels even when normal unfocused labels are truncated for density, and construct glyph nodes show construct-kind labels while showing their symbols through Sigma image-node rendering.
- Confirm the ontology graph uses ontology-diagram visual conventions: class-like concepts, restrictions, and class expressions render as compact circular or elliptical anchors; properties render as labeled domain/range relationship edges near their anchors rather than graph boxes; datatype/resource/SHACL nodes remain visually distinct; relation edges use directed ontology-diagram connector lines with visible direction arrowheads; and the graph supports Sigma pan/zoom, selected-node centering, and reset-driven layout.
- Confirm ontology relationship edges are hidden in the default full-graph view and appear only for edges in the active focus tree, plus enabled member/filler edges one semantic step beyond visible construct-only nodes, through Sigma's native edge reducer and `@sigma/edge-curve` edge programs, with relation visibility controls limiting which focused edges are eligible. With no pinned selection, hover fades unrelated nodes. With a pinned selection, nodes outside the selected focus tree are hidden; rolling over a visible node inside that selected tree opens the rollover tree, dims selected-tree nodes outside the rollover tree to the same low-strength treatment, and keeps nodes outside both trees hidden.
- Confirm Sigma z-index and highlighted-node rendering are enabled so the active ontology focus tree is painted through Sigma's normal focus path, selected/focused nodes render above focused-neighbor nodes, focused-neighbor nodes render above focused edges, focused edges render above unrelated or muted graph items, and focused edges are not rendered through a separate focused-edge canvas overlay.
- Confirm generic SHACL overlay edges render as unlabeled overlay lines while retaining modal/projection evidence, unless an edge carries a more specific ontology/SHACL relation label.
- Confirm relation connectors and arrowheads remain visible but subtle enough not to overflow or dominate labels.
- Confirm circular class-anchor size is bounded and grows from graph connection degree rather than label length, so highly connected concepts are visually emphasized while low-degree concepts remain compact.
- Confirm graph nodes, property link labels, modal badges, and legend swatches resolve through the ontology semantic role palette consistently, including separate role tokens for class anchors, SKOS concepts, property semantics, datatypes, named individuals, SHACL shapes, resources, restrictions, class expressions, external references, and the shared graph canvas surface.
- Confirm search, focus, modal detail, filters, and the compact legend operate over semantic ontology roles and OWL constructs rather than generic RDF predicate edges.
- Confirm ontology graph nodes can be dragged through Sigma pointer events, updating in-memory Graphology coordinates so users can uncover overlapped relation lines or labels, and confirm the visible view controls expose `Reset` without a separate `Fit` button.
- Confirm the Ontologies and Model/project graph left legend/filter panels use the shared graph control width and selected-control treatment: active controls use selected-control background/foreground tokens and inactive/hover controls use shared warm-neutral surface tokens.
- Confirm selected Model Graph nodes expose a selected-element link in the left Explorer pane that opens the shared element-detail modal, and selected Ontologies graph nodes expose a selected ontology-node link in the left Explorer pane that opens the ontology element modal.
- Confirm the detailed semantic type color key and construct notation key are passive, while the `Show` group contains the active role and relation visibility controls.
- Confirm Ontologies left-pane overlay controls expose Core, Concepts, Semantic Context, and External Sources as active layer filters with node counts, and that toggling layer controls changes graph visibility without changing ontology node modal evidence for selected nodes. Confirm Core and Concepts are shown by default, Concepts can be hidden for structural-only inspection, Core can be hidden for thesaurus-only inspection, Concepts uses the `--rdf-concept` role token for SKOS concept nodes, and no generated inverse `mappedFrom` bridge edge is rendered.
- Confirm the passive type legend exposes separate color swatches only for semantic kinds that can render as nodes, including classes, SKOS concepts, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic resources; property kinds remain visible through property links, modal badges, search metadata, and relation visibility controls.
- Confirm the `Show` visibility group exposes one shared button design for datatype property links, object property links, class disjointness, restrictions, class expressions, SHACL shapes, resources, and external references without replacing Reqvire's richer passive type and notation legends, and that active means shown on the canvas.
- Confirm ontology terms and class-membership context are not exposed as hideable toggles, property links are controlled through datatype-property and object-property visibility controls, and the single SHACL shapes role filter controls both SHACL shape nodes and SHACL overlay relations.
- Confirm selected and hovered focus trees are computed from currently visible relation filters, so disabling object-property, datatype-property, restriction, class-expression, disjointness, or SHACL relation categories removes neighbors connected only by those hidden relations from the focused subgraph without filtering evidence from the ontology element modal; visible construct-only nodes expand the focus through their enabled construct links so union/intersection/complement members and restriction fillers are visible from the selected context.
- Confirm the default filter state opens with datatype-property links, object-property links, class disjointness, restrictions, class expressions, SHACL shapes, resources, and external references shown, while ontology terms and class-membership context remain always available.
- Confirm role filters are hard gates for node visibility, so disabling SHACL shapes hides both SHACL shape nodes and SHACL overlay relations without requiring a second SHACL slot-overlay checkbox.
- Confirm the passive `Notation` legend covers domain/range, subclass, membership, disjointness, equivalence, inverse, property chain, property characteristic, restriction, class-expression, and SHACL-overlay constructs without exposing those rows as a second construct-filter panel.
- Confirm relation styling is passive visual notation rather than a selectable filter, and the active `Show` relation controls determine which clutter categories are shown.
- Confirm equivalence, inverse-property, property-chain, and property-characteristic constructs remain visible as passive legend/modal evidence rather than active filters while they have no direct canvas-visible toggle effect.
- Confirm visibility controls affect construct-only canvas nodes, construct-specific canvas edges, and graph node badges without making nodes visible when their role filter is disabled, while the ontology node modal continues to show the selected node's full evidence.
- Confirm graph-registry provenance and generated-projection provenance are not exposed as graph-wide filter axes; source/provenance evidence remains available in the ontology node modal and Project Store data.
- Confirm active filters combine inclusively within one category and narrow together across different active categories on the canvas without narrowing the ontology element modal evidence.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Ontology Construct Grouping](../../../Interfaces/WebExplorer/Capabilities.md#ontology-construct-grouping)
  * verify: [Ontology Property-Centric Visualization](../../../Interfaces/WebExplorer/Capabilities.md#ontology-property-centric-visualization)
  * verify: [Ontology Symbol and Badge Vocabulary](../../../Interfaces/WebExplorer/Capabilities.md#ontology-symbol-and-badge-vocabulary)
  * verify: [OWL Semantic Ontology Projection](../../../Interfaces/WebExplorer/Capabilities.md#owl-semantic-ontology-projection)
  * verify: [Ontology Projection Subgraph Materialization](../../../Reports/ModelReports/ReportingRequirements.md#ontology-projection-subgraph-materialization)
---

### Responsive Design Verification

This test verifies responsive breakpoints and compiled Explorer design-system CSS integration.

#### Details
Expected checks:
- Explorer layout works at mobile, tablet, and desktop widths.
- No layout breaks or overlapping controls are present.
- Compiled CSS and font assets are local and no runtime CSS compiler or CDN framework is required.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Explorer Design System Styling](../../../Interfaces/WebExplorer/ExplorerRendering.md#explorer-design-system-styling)
  * verify: [Responsive Explorer Rendering](../../../Interfaces/WebExplorer/ExplorerRendering.md#responsive-explorer-rendering)
---

### SPA Explorer Store Contract Verification

This test verifies that `index.html` is the central SPA Explorer shell and contains or loads the normalized browser-local Project Store required by all current views.

#### Details

##### Acceptance Criteria:
- `index.html` shall be the primary Explorer shell and Project Store host, served as a native SPA built with Vite/TypeScript/React and the Reqvire Explorer design system.
- The served shell shall reference local compiled bundle, stylesheet, design-system, and font assets with no CDN-loaded framework, no CDN-loaded styling runtime, and no runtime CSS compiler.
- The Project Store seed shall be present before view rendering and shall include a schema/version marker.
- The Project Store `project` section shall include repository and branch metadata when Git metadata is available, and the Model tree root shall render that identity instead of a generic `Project` label.
- The store shall expose normalized top-level sections for project, folders, files, resources, elements, relations, contract_bindings, concept references, thesaurus, submodels, traces, coverage, ontology, knowledge graph, search, summaries, and routes.
- File containers and modeled resources shall be represented as separate record families with explicit cross-references when the same path appears in both roles.
- Route definitions shall include canonical hash routes for current views and element/file/search detail workflows.
- Element-detail routes shall open a Project Store-backed scrollable modal/dialog in the Explorer shell instead of using the source content route as the primary element navigation target.
- Element-detail modal headers shall show only the actual element type as a text badge and shall not show an additional type-family/kind badge, marker dot, shape, or glyph when the element type is more specific.
- Opening a related element from within the element-detail modal shall show a compact back control whose accessible label names the previous element and shall not render a duplicate visible `From:` context line.
- Regular element-detail modals shall render model concept references as standard link-colored inline links on matching prose terms, with underline only on hover or focus, hide the authored `#### Concept References` source subsection, match native concept preferred labels, alternative labels, and authored reference labels, expose the concept IRI as tooltip/location metadata, and open the referenced native concept element modal when activated.
- Concept relation lists in the element-detail modal shall exclude the selected concept itself, and reciprocal ontology edges for the same concept pair shall be deduplicated before rendering.
- Element-detail modals shall expose source navigation as a secondary action using the source anchor.
- The Coverage route shall render a left-pane coverage explorer with section counts for Overview, Capability coverage, Unverified requirements, Unimplemented requirements, Unsatisfied verifications, and Orphaned verifications, while keeping summary cards, charts, and legends out of the left pane.
- Separate Explorer/report document entry points shall not be generated.
- Missing or malformed store seed data shall be detectable by automated checks and visible to users as an Explorer diagnostic.

##### Test Criteria:
- Run `reqvire serve` on a minimal model with at least one capability, requirement, verification relation, satisfiedBy evidence file, contract_bindings or concept-reference fact, and ontology term when available.
- Run the Explorer component/unit tests that cover element-detail modal back context rendering, inline native concept-reference modal routing, alternative-label concept-reference matching, hidden authored concept-reference subsection rendering, and exclusion of self-referential concept relation rows from the modal.
- Parse the generated store seed from `index.html` or its referenced static asset.
- Assert all required top-level store sections exist.
- Assert the `project` section exposes repository and branch metadata when running from a Git repository, and that Explorer tree roots render the combined repository/branch label.
- Assert at least one Markdown source path appears in `files`.
- Assert at least one implementation or evidence path referenced by `satisfiedBy` or contract_bindings appears in `resources`.
- Assert `files` records carry source navigation metadata while `resources` records carry referring-fact evidence.
- Assert canonical routes include primary `#/model`, specialist `#/ontologies` and `#/traces`, and supporting `#/files`, `#/files/<path>`, `#/coverage`, `#/resources`, `#/elements/<identifier>`, and `#/search`; do not require a separate Containment route or a separate Knowledge Graph route/page file.
- Assert at least one Explorer element link or search result targets `#/elements/<identifier>` and that the element-detail UI contains a modal/dialog marker plus a secondary source link.
- Assert element-detail modal headers render the actual element type as the only visible type badge and do not render a marker dot, shape, or glyph inside that badge; for example a `behavior` element shall not also show a separate `contract` badge.
- Assert the Model view List/Grid modes render from Project Store `folders` and `files` without an iframe or third-party file-manager widget, expose breadcrumb navigation, sortable file rows, grid cards, central workspace search, icon/color legends, source-page secondary actions, and modeled-element rows that open the shared element-detail modal. Assert clicking anywhere on a Grid mode folder/file card opens or selects that card's item, while the source-page secondary action remains a separate control. Assert `#/files` and `#/files/<path>` deep-link into that behavior without creating a separate primary Filesystem view.
- Assert the Model Graph mode paints the shell, graph canvas, and design-system spinner loading notice before deferred Sigma/ForceAtlas graph construction starts, clears the loading notice after renderer startup, and keeps full-graph ForceAtlas layout quality while using cached adjacency/focus lookup for interaction.
- Assert the Model tree, grid cards, modeled-element lists, relation/contract_bindings endpoints, and element legends use the shared Explorer `ElementIcon` type glyphs, that capability, semantic-contract, and verification-objective elements use their own role colors as plain squares with no glyph, that verification-objective uses the darker verification-objective token distinct from concrete verification, that inline concept-reference terms in element content use standard link color with no glyph or pill and underline only on hover or focus, that evidence-file artifacts use the neutral/default treatment, and that contract-family subtypes keep the shared contract color while rendering distinct glyph marks for `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output`.
- Assert selecting a folder, file, or modeled element in the left Model project tree updates the active Model workspace mode: List/Grid browse the selected folder or file, Graph focuses the matching graph node when one exists, and modeled-element rows open the shared element-detail modal without leaving the Model workspace.
- Assert the Search route's left-pane result-type controls do not render a duplicate passive legend for the same result-type colors and labels.
- Assert the Coverage route's left Explorer pane renders the coverage explorer section rows with counts, that selecting a row scrolls or selects the matching central Coverage section, and that the left pane does not duplicate the Coverage dashboard summaries or legend content.
- Assert the Explorer builds its ranked search index in a browser worker after the initial shell render, keeps non-search Explorer views interactive during indexing, and returns BM25-style ranked results that prioritize title matches over path/result-kind matches and body/content matches.
- Assert prefix and fuzzy search terms can find matching Project Store search documents, and result-kind controls filter ranked results without rebuilding the index.
- Assert `index.html` loads local compiled SPA bundle/stylesheet assets and contains no framework/styling CDN reference and no runtime CSS compiler.
- Assert canonical SPA routes are sufficient for Explorer navigation and that no retired Explorer page adapters or separate Explorer/report implementations are emitted.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [SPA Explorer Shell and Project Store](../../../Interfaces/WebExplorer/Capabilities.md#spa-explorer-shell-and-project-store)
---

### Serve Command Verification

This test verifies that the serve command starts an HTTP server for the embedded Explorer and generated model runtime data.

#### Details

##### Acceptance Criteria:
- System shall start HTTP server on specified host and port
- System shall display clickable terminal link to the server URL
- System shall serve index.html when accessing root URL
- System shall serve embedded Explorer assets and generated Project Store data with correct paths
- System shall serve generated `ontologies.ttl`
- System shall expose `/mcp` on the same listener only when `--enable-mcp` is present
- System shall expose embedded MCP mutation tools only when both `--enable-mcp` and `--enable-mutations` are present
- System shall return index.html for non-asset browser routes
- System shall not return Explorer `index.html` for `/mcp` requests when embedded MCP is enabled
- System shall return 404 for missing asset paths
- System shall set correct Content-Type headers for different file types
- System shall run in quiet mode without verbose runtime-generation output
- System shall not automatically open browser window
- System shall display instructions for Ctrl-C stop

##### Test Criteria:
- Command starts successfully and displays server URL with instructions
- Server responds to HTTP requests on specified port
- Root URL (/) serves index.html
- HTML files are served with text/html content type
- SVG files are served with image/svg+xml content type
- Missing embedded asset paths return 404 status
- Non-asset browser routes return index.html for SPA fallback
- `reqvire serve --enable-mcp` accepts MCP protocol requests at `/mcp` while root and SPA routes still serve Explorer content
- `reqvire serve --enable-mcp` omits mutation tools from MCP `tools/list`
- `reqvire serve --enable-mcp --enable-mutations` includes mutation tools in MCP `tools/list`
- After an embedded MCP mutation changes model files, a subsequent `assets/project-store.js` request returns regenerated Project Store data that reflects the current workspace.
- Runtime data responses include no-store cache control to avoid stale browser datastores after mutation.
- `reqvire serve --enable-mutations` without `--enable-mcp` fails CLI argument validation
- Runtime-generation verbose output is suppressed (quiet mode active)

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Serve Command](../../../Interfaces/WebExplorer/Capabilities.md#serve-command)
  * verify: [Serve Command Embedded MCP Endpoint](../../../Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint)
---

### Thesaurus Project Store Projection Verification

This test verifies that the Thesaurus Explorer route is backed by a native Project Store thesaurus projection instead of ontology graph provenance.

#### Details

##### Acceptance Criteria:
- The exported Project Store shall include a top-level `thesaurus` projection with `schemes` and `concepts`.
- Each concept-scheme row shall preserve distinct SKOS identity and native Reqvire `concept-scheme` element identity.
- Each concept row shall preserve distinct SKOS identity and native Reqvire `concept` element identity.
- Concept rows shall expose scheme membership, taxonomy parent identity, related concept identity, SKOS authoring fields, source navigation, model usage, and ontology mapping usage needed by the Thesaurus route.
- Thesaurus map activation shall use native concept or concept-scheme element IDs from the `thesaurus` projection, not ontology graph node source/provenance data.
- Ontology bridge evidence from `reqvire:mapsToConcept` shall appear as mapping usage without making the mapped ontology term the concept's navigation target.

##### Test Criteria:
- Export a model containing a native concept scheme, native concepts, concept taxonomy, related concepts, and an ontology term mapped to one concept through `reqvire:mapsToConcept`.
- Parse `assets/project-store.js` and assert the top-level `thesaurus` projection exists.
- Assert concept-scheme `element_id` resolves to a Project Store element with type `concept-scheme`.
- Assert each concept `element_id` resolves to a Project Store element with type `concept`.
- Assert the narrower concept's `parent_id` references the broader concept SKOS id.
- Assert related concept ids, labels, scope note, source href, and ontology `maps_to` evidence are preserved.
- Assert the concept `element_id` differs from ontology graph provenance for the same SKOS concept node, preventing Thesaurus map clicks from opening ontology elements.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-thesaurus-project-store/test.sh)
  * verify: [Thesaurus View Generation](../../../Interfaces/WebExplorer/Capabilities.md#thesaurus-view-generation)
---
