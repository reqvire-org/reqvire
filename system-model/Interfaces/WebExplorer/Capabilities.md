# Elements

### Export Command

The system SHALL provide an export command that writes the embedded Explorer SPA bundle, generated model data, and repository-local static assets referenced by rendered workspace content to a local output directory, producing a self-contained static site suitable for deployment to GitHub Pages or any static file host.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Export Command Contract Specification](Specifications.md#export-command-contract-specification)
  * derivedFrom: [Web Interface](../InterfacesRequirements.md#web-interface)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [explorer_runtime.rs](../../../core/src/explorer_runtime.rs)
  * verifiedBy: [Export Command Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#export-command-verification)
---

### Serve Command

The system SHALL provide a serve command that launches a local Explorer HTTP server for browsing the current workspace model and repository-local static assets referenced by rendered workspace content.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Two-Pass Validation Behavior](../../Operations/Validation/Behaviors.md#two-pass-validation-behavior)
  * [Validation Error Reporting Behavior](../../Operations/Validation/Behaviors.md#validation-error-reporting-behavior)
  * [Explorer Serve Pipeline Specification](Specifications.md#explorer-serve-pipeline-specification)

#### Relations
  * definedBy: [Serve Command Contract Specification](Specifications.md#serve-command-contract-specification)
  * derivedFrom: [Web Interface](../InterfacesRequirements.md#web-interface)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [serve.rs](../../../cli/src/serve.rs)
  * verifiedBy: [Serve Command Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#serve-command-verification)
---

### Served Explorer Browser Interface

The system SHALL serve the embedded Reqvire Explorer SPA with all model artifacts needed for browser inspection.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Web Interface Navigation Behavior](Behaviors.md#web-interface-navigation-behavior)
  * definedBy: [Explorer Branding Specification](Specifications.md#explorer-branding-specification)
  * definedBy: [Explorer Navigation Chrome Specification](Specifications.md#explorer-navigation-chrome-specification)
  * definedBy: [Explorer Serve Pipeline Specification](Specifications.md#explorer-serve-pipeline-specification)
  * definedBy: [Model Browser and Graph Specification](Specifications.md#model-browser-and-graph-specification)
  * definedBy: [Web Interface Style Specification](Specifications.md#web-interface-style-specification)
  * derive: [Containment View Reused Contract Context Links](#containment-view-reused-contract-context-links)
  * derive: [Diagram Reused Contract Context Display](#diagram-reused-contract-context-display)
  * derive: [Model-Centric View Generation](#model-centric-view-generation)
  * derive: [Ontologies View Generation](#ontologies-view-generation)
  * derive: [Project Knowledge Graph View](#project-knowledge-graph-view)
  * derive: [Reused Contract Context Link Serving](#reused-contract-context-link-serving)
  * derive: [SPA Explorer Shell and Project Store](#spa-explorer-shell-and-project-store)
  * derive: [Traces View Generation](#traces-view-generation)
  * derive: [Web Interface Color Scheme](#web-interface-color-scheme)
  * derivedFrom: [Web Interface](../InterfacesRequirements.md#web-interface)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [explorer_runtime.rs](../../../core/src/explorer_runtime.rs)
  * satisfiedBy: [mod.rs](../../../core/src/html/mod.rs)
  * satisfiedBy: [store.rs](../../../core/src/html/store.rs)
  * satisfiedBy: [App.tsx](../../../explorer/src/App.tsx)
  * satisfiedBy: [ContentView.tsx](../../../explorer/src/components/ContentView.tsx)
  * satisfiedBy: [MarkdownContent.tsx](../../../explorer/src/components/MarkdownContent.tsx)
  * satisfiedBy: [searchIndex.ts](../../../explorer/src/lib/searchIndex.ts)
  * satisfiedBy: [SearchIndexContext.tsx](../../../explorer/src/search/SearchIndexContext.tsx)
  * satisfiedBy: [GraphLibraryViews.tsx](../../../explorer/src/views/GraphLibraryViews.tsx)
  * satisfiedBy: [OntologiesView.tsx](../../../explorer/src/views/OntologiesView.tsx)
  * satisfiedBy: [SearchView.tsx](../../../explorer/src/views/SearchView.tsx)
  * satisfiedBy: [searchIndex.worker.ts](../../../explorer/src/workers/searchIndex.worker.ts)
  * verifiedBy: [CLI Help Structure Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [Explorer Serve Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#explorer-serve-verification)
---

### Containment View Reused Contract Context Links

The system shall preserve element reused_contract_context links in Model containment data and expose them from supported Explorer surfaces to provide quick access to associated contract elements and documents.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Containment View Reused Contract Context Links Contract Specification](Specifications.md#containment-view-reused-contract-context-links-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * verifiedBy: [Model Containment Reused Contract Context Links Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#model-containment-reused-contract-context-links-verification)
---

### Diagram Reused Contract Context Display

The system shall display reused_contract_context links within element boxes in generated diagrams to show document associations visually.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Diagram Reused Contract Context Display Contract Specification](Specifications.md#diagram-reused-contract-context-display-contract-specification)
  * definedBy: [Explorer Mermaid Diagram Style Specification](Specifications.md#explorer-mermaid-diagram-style-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Diagram Reused Contract Context Display Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#diagram-reused-contract-context-display-verification)
---

### Model-Centric View Generation

The system shall render a model-centric Explorer visualization showing model roots with nested relations containing full element details.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Model-Centric View Generation Contract Specification](Specifications.md#model-centric-view-generation-contract-specification)
  * derive: [Model View Element Navigation](#model-view-element-navigation)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [store.rs](../../../core/src/html/store.rs)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
---

### Model View Element Navigation

The system shall make element names in the model-centric view clickable links that navigate to the element's definition in its source file.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Model View Element Navigation Contract Specification](Specifications.md#model-view-element-navigation-contract-specification)
  * derivedFrom: [Model-Centric View Generation](#model-centric-view-generation)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * verifiedBy: [Model View Element Navigation Test](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#model-view-element-navigation-test)
---

### Ontologies View Generation

The system shall expose an Ontologies Explorer view during serve workflows that presents ontology and SHACL semantics through an OWL-aware model viewer instead of a raw RDF triple graph.

#### Details
The Ontologies view shall:
- Use RDF/Turtle and SHACL blocks as the parsed semantic source, while avoiding raw RDF serialization artifacts as the primary user-facing visualization.
- Present ontology concepts as typed visual entities such as classes, named individuals, SHACL shapes, datatypes, restrictions, and class expressions, while keeping literal constraints and datatype-property literal values as modal/search evidence rather than primary graph nodes.
- Treat ontology properties as labeled relationship semantics between their domain/range terms, not as standalone graph nodes, while retaining domain, range, inverse, equivalence, chain, and characteristic information as modal/search evidence when those axioms are present.
- Derive class slots and slot facets from SHACL target-class/property-shape constraints so users can inspect datatype or object range, cardinality, pattern, node-kind, and allowed-value constraints without navigating raw SHACL blank nodes.
- Show named properties as reusable slots in the selected class or term modal, including each target class and source shape that uses that property as a `sh:path`, so repeated property usages are understandable rather than presented as duplicate property definitions.
- Use a defined ontology symbol and badge vocabulary so rendered symbols are stable, documented, and accessible.
- Consume generated ontology projection facts from the semantic export context so the Ontologies Explorer and `reqvire ontologies --full` describe the same ontology constructs.
- Expose ontology graph layers so users can inspect authored semantic content by default, optionally overlay semantic context for model-to-term declaration/reference provenance, and separately enable external source vocabulary when external ontology source triples are present.
- Prioritize graph canvas space by using a dense full-height viewer layout, left-pane ontology controls, and modal detail for selected ontology nodes.
- Separate normalized ontology constructs and SHACL-derived slots/facets from optional raw SHACL evidence; empty raw-evidence sections shall not be shown.
- Apply viewer filters as explicit canvas visibility contracts: authored ontology semantics are enabled by default, semantic context and external source layers are opt-in, ontology terms and class-membership context stay available, role controls govern optional SHACL, resource, and external-reference visibility, and construct notation remains available as passive legend and modal evidence.
- Keep linked source citations in the viewer and the served `ontologies.ttl` artifact available for traceability and downstream tooling without rendering raw Turtle blocks as the primary page content.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Ontology Collection Output Specification](../../Reports/ModelReports/Specifications.md#ontology-collection-output-specification)
  * [Ontology Projection Subgraph Materialization Specification](../../Reports/ModelReports/Specifications.md#ontology-projection-subgraph-materialization-specification)

#### Relations
  * definedBy: [Ontology Rendering Details](OntologyRenderingDetails.md#ontology-rendering-details)
  * definedBy: [Ontologies View Generation Contract Specification](Specifications.md#ontologies-view-generation-contract-specification)
  * derive: [Ontology Construct Grouping](#ontology-construct-grouping)
  * derive: [Ontology Property-Centric Visualization](#ontology-property-centric-visualization)
  * derive: [Ontology Symbol and Badge Vocabulary](#ontology-symbol-and-badge-vocabulary)
  * derive: [OWL Semantic Ontology Projection](#owl-semantic-ontology-projection)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
---

### OWL Semantic Ontology Projection

The system shall transform semantic-index RDF quads into generated ontology projection facts that suppress RDF serialization mechanics and expose stable ontology concepts for both full semantic export and Explorer visualization.

#### Details
The projection shall classify terms by semantic role, preserve source traceability, reuse generated direct-authored OWL/RDFS/SHACL construct data to `SemanticIndex`, expose that data as a reusable ontology projection subgraph inside the existing in-memory RDF projection, derive normalized slot/facet records from SHACL property shapes, and omit primary rendering of `rdf:type` edges, RDF list plumbing, metaclass resources, and anonymous blank-node implementation details unless those nodes represent a meaningful ontology construct.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [OWL Semantic Ontology Projection Contract Specification](Specifications.md#owl-semantic-ontology-projection-contract-specification)
  * derivedFrom: [Ontologies View Generation](#ontologies-view-generation)
  * verifiedBy: [Ontology Model Viewer Analysis Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#ontology-model-viewer-analysis-verification)
---

### Ontology Construct Grouping

The system shall present multi-node OWL constructs as explicit semantic groups instead of exposing their low-level RDF representation.

#### Details
Construct grouping shall cover equivalence groups, inverse properties, property-chain axioms, property characteristics, and SHACL shape overlays when those constructs are present in the collected ontology or semantic-contract content.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ontology Construct Grouping Contract Specification](Specifications.md#ontology-construct-grouping-contract-specification)
  * derivedFrom: [Ontologies View Generation](#ontologies-view-generation)
  * verifiedBy: [Ontology Model Viewer Analysis Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#ontology-model-viewer-analysis-verification)
---

### Ontology Property-Centric Visualization

The system shall render OWL object properties and datatype properties as first-class relationship semantics: properties appear as labeled domain/range edges and ontology-node detail evidence, rather than standalone graph boxes.

#### Details
The property visualization shall aggregate many domain and range classes without multiplying identical property arcs, distinguish object-property ranges from datatype-property ranges, and show property semantics through compact edge labels, badges, or modal sections on selected classes, individuals, and terms. When a named property is used by multiple SHACL property shapes, the ontology node modal shall present those as target-class usages with source-shape evidence rather than as duplicate property definitions.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ontology Property-Centric Visualization Contract Specification](Specifications.md#ontology-property-centric-visualization-contract-specification)
  * derivedFrom: [Ontologies View Generation](#ontologies-view-generation)
  * verifiedBy: [Ontology Model Viewer Analysis Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#ontology-model-viewer-analysis-verification)
---

### Ontology Symbol and Badge Vocabulary

The system shall define a canonical Unicode symbol vocabulary for ontology viewer badges, edge labels, group headers, tooltips, and ontology modal fields.

#### Details
The symbol vocabulary shall define each symbol with its semantic meaning, raw Unicode code point, rendered Unicode character, and allowed viewer usage locations. Symbols shall supplement text labels and accessible descriptions; they shall not be the only carrier of meaning.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ontology Symbol and Badge Vocabulary Contract Specification](Specifications.md#ontology-symbol-and-badge-vocabulary-contract-specification)
  * derivedFrom: [Ontologies View Generation](#ontologies-view-generation)
  * verifiedBy: [Ontology Model Viewer Analysis Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#ontology-model-viewer-analysis-verification)
---

### Project Knowledge Graph View

The system shall expose a Project Knowledge Graph view during serve workflows as the Model route's Graph mode. The view presents the actual parsed project graph as current elements and facts.

#### Details
The Knowledge Graph view shall:
- Answer what actual project elements and facts exist right now.
- Be reached through the canonical `index.html#/model` Explorer route by selecting Graph mode; a separate Knowledge Graph route or document entry point shall not be generated.
- Render the four system-model layers as first-class graph nodes: ontologies, capabilities, requirements, and verifications. Requirement-owned contracts may appear as subordinate requirement detail/contract nodes when actual project facts reference them, but they are not a separate system-model layer.
- Render actual relation facts, reused_contract_context facts, concept-reference facts, file targets, and external targets as graph edges or resource nodes.
- Use Reqvire capability-root submodels as the structural graph partitioning contract; reused_contract_context, concept references, verification/satisfaction, and trace facts are overlays rather than submodel boundaries.
- Treat structural ownership/backbone relations separately from cross-layer evidence relations: `derive` and `specify` organize the capability/requirement submodel backbone, while requirement-owned `define`, `reuse`, `satisfiedBy`, `verifiedBy`, and concept-reference facts connect subordinate details or layers as inspectable overlays.
- Reuse the dense Explorer graph and modal-detail interaction pattern used by the ontology viewer, while focusing on project instances rather than ontology vocabulary definitions.
- Provide modal detail evidence for element type, identifier, source location, governance, metadata, incoming facts, outgoing facts, reused_contract_context, and concept references.
- Keep ontology vocabulary exploration in the Ontologies view; the Knowledge Graph view may show ontology terms only when they are referenced by actual project elements.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Project Knowledge Graph View Contract Specification](Specifications.md#project-knowledge-graph-view-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [store.rs](../../../core/src/html/store.rs)
  * satisfiedBy: [GraphLibraryViews.tsx](../../../explorer/src/views/GraphLibraryViews.tsx)
  * verifiedBy: [SPA Explorer Store Contract Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#spa-explorer-store-contract-verification)
---

### Reused Contract Context Link Serving

The system shall preserve reused_contract_context identifier links to referenced contract elements in the served Explorer to preserve document completeness and enable navigation.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Reused Contract Context Link Serving Contract Specification](Specifications.md#reused-contract-context-link-serving-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [store.rs](../../../core/src/html/store.rs)
  * verifiedBy: [Reused Contract Context Link Serving Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#reused-contract-context-link-serving-verification)
---

### SPA Explorer Shell and Project Store

The system shall provide `index.html` as the single-page Reqvire Explorer shell and seed it with a normalized browser-local Project Store that supports the primary Explorer routes and supporting report/detail views as the canonical browser experience.

#### Details
The SPA Explorer shell shall:
- Be a native single-page application built with Vite, TypeScript, and React, using the Reqvire Explorer design system and compiled CSS, served as `index.html` plus deterministic `assets/explorer.js` and `assets/explorer.css` bundles with no CDN-loaded framework or stylesheet and no runtime CSS compiler.
- Treat `index.html` as the primary browser entry point and central Project Store host.
- Expose project identity metadata in the Project Store, including repository name and current branch when Git metadata is available, so Explorer navigation roots identify the served repository snapshot instead of showing a generic project label.
- Render the primary Model route as a native SPA view module reading from the Project Store. The Model route shall host List, Grid, and Graph modes over the Project Store filesystem/model and knowledge-graph projections.
- Keep Model project-tree selection shared across List, Grid, and Graph modes so selecting a folder, file, or modeled element in the left Explorer tree updates the active Model workspace instead of opening a disconnected Filesystem view.
- Render Graph as a Model mode over the Project Store knowledge-graph projection, render specialist Ontologies and Traces routed views from top navigation and tool actions, and render supporting Search, file deep links, Coverage, Resources, and element-detail workflows from the same Project Store without making them primary left-pane navigation modes.
- Build the Search route's ranked MiniSearch index in a browser worker after the initial shell render, using Project Store search documents with boosted title, path, result-kind, and content fields so indexing does not block primary Explorer interaction.
- Open element-detail routes in an in-shell scrollable modal backed by Project Store element records, preserving the current Explorer view context behind the modal.
- When a user opens a related element from inside an element-detail modal, keep an in-modal back target so the user can return to the previously inspected element without losing the underlying Explorer route context.
- Show the actual element type as the single primary type badge in element-detail modal headers. Canonical type family may drive color and icon semantics, but it shall not be rendered as a second visible kind badge when the actual type is more specific.
- Do not generate separate Explorer/report document entry points.
- Seed a normalized project snapshot that distinguishes modeled source-file containers from modeled resource and evidence-file targets. The Model tree shall contain modeled element files plus existing repository-relative local implementation/evidence/resource files referenced by graph-registry facts, while unrelated repository files remain absent.
- Keep containment, model, knowledge graph, verification traces, coverage, resources, ontology, search, summaries, and route state as view-neutral store projections rather than separate page-local data models.
- Preserve the current relation model: capabilities may author concept references and are specified/verified; requirements own contracts, satisfaction evidence, verification evidence, reusable contract reused_contract_context, and concept references.

#### Concept References
  * Project Store: reqvire:BrowserLocalProjectStore
  * Explorer Route: reqvire:ExplorerRoute
  * File Container: reqvire:FileContainer
  * Resource Reference: reqvire:ModeledResource

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Explorer Store Seed Data Output Specification](Specifications.md#explorer-store-seed-data-output-specification)
  * definedBy: [SPA Explorer Store Contract Specification](Specifications.md#spa-explorer-store-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [git_commands.rs](../../../core/src/git_commands.rs)
  * satisfiedBy: [store.rs](../../../core/src/html/store.rs)
  * satisfiedBy: [DetailDialog.tsx](../../../explorer/design-system/product-patterns/detail/DetailDialog.tsx)
  * satisfiedBy: [FileBrowser.tsx](../../../explorer/design-system/product-patterns/files/FileBrowser.tsx)
  * satisfiedBy: [App.tsx](../../../explorer/src/App.tsx)
  * satisfiedBy: [ElementDetailModal.tsx](../../../explorer/src/components/ElementDetailModal.tsx)
  * satisfiedBy: [ExplorerSidePane.tsx](../../../explorer/src/components/ExplorerSidePane.tsx)
  * satisfiedBy: [types.ts](../../../explorer/src/store/types.ts)
  * satisfiedBy: [FilesView.tsx](../../../explorer/src/views/FilesView.tsx)
  * verifiedBy: [SPA Explorer Store Contract Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#spa-explorer-store-contract-verification)
---

### Traces View Generation

The system shall expose verification traceability as a specialist Explorer view backed by Project Store trace projections and rendered through the shared Explorer shell.

#### Details
The Traces view shall:
- Present verification trace flow and trace rows from browser-local Project Store data generated by the serve pipeline.
- Use the shared Explorer left pane for trace file selection, flow/rows mode controls, selected verification links, and compact summary evidence.
- Render flow visualizations and Mermaid roll-up diagrams with the same Explorer design-system palette, typography, modal-detail pattern, and non-blocking diagram rendering behavior as other Explorer graph views.
- Keep trace rendering as a browser-interface concern while Functional Output owns the underlying trace tree construction and Project Store trace data projection.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * definedBy: [Explorer Verification Trace Rendering Specification](Specifications.md#explorer-verification-trace-rendering-specification)
  * definedBy: [TraceFlowView](TraceFlowView.md#traceflowview)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [store.rs](../../../core/src/html/store.rs)
  * satisfiedBy: [ReportViews.tsx](../../../explorer/src/views/ReportViews.tsx)
  * verifiedBy: [SPA Explorer Store Contract Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#spa-explorer-store-contract-verification)
---

### Web Interface Color Scheme

The system shall implement a consistent color scheme across the static Explorer following clearly defined specifications optimized for MBSE and requirements management applications.

#### Details
The color scheme shall provide:
- Semantic brand, chrome, surface, text, border, focus, and selection tokens for navigation and UI elements
- Element role tokens and glyphs for visual differentiation across files, model elements, resources, relation pills, graph nodes, badges, tiles, and modals
- Status tokens for verification, warning, error, and success states
- Interactive state tokens for hover, focus, selected, disabled, and active controls
- Programmatic palette tokens for browser-rendered graph, Mermaid, D3/Sankey, and badge renderers

The system shall ensure color consistency between:
- Explorer route styling
- Browser-rendered Mermaid diagram rendering
- Model List/Grid/Graph views
- Ontology graph visualization
- Trace flow visualization

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Explorer Color and Type Palette Specification](Specifications.md#explorer-color-and-type-palette-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * satisfiedBy: [ElementIcon.tsx](../../../explorer/design-system/components/data/ElementIcon.tsx)
  * satisfiedBy: [palette.ts](../../../explorer/design-system/palette.ts)
  * satisfiedBy: [RelationEndpoint.tsx](../../../explorer/design-system/product-patterns/detail/RelationEndpoint.tsx)
  * satisfiedBy: [ReusedContractContextList.tsx](../../../explorer/design-system/product-patterns/detail/ReusedContractContextList.tsx)
  * satisfiedBy: [colors.css](../../../explorer/design-system/tokens/colors.css)
---

