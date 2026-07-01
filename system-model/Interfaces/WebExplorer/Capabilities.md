# Elements

### Export Command

The system SHALL provide an export command that writes the embedded Explorer SPA bundle, generated model data, and eligible Git-worktree static assets referenced by rendered workspace content to a local output directory, producing a self-contained static site suitable for deployment to GitHub Pages or any static file host.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)

#### Relations
  * definedBy: [Export Command Contract Specification](Specifications.md#export-command-contract-specification)
  * derivedFrom: [Web Interface](../InterfacesRequirements.md#web-interface)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [explorer_runtime.rs](../../../crates/reqvire-core/src/explorer_runtime.rs)
  * verifiedBy: [Export Command Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#export-command-verification)
---

### Serve Command

The system SHALL provide a serve command that launches a local Explorer HTTP server for browsing the current workspace model and eligible Git-worktree static assets referenced by rendered workspace content.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)
  * [Two-Pass Validation Behavior](../../Operations/Validation/Behaviors.md#two-pass-validation-behavior)
  * [Validation Error Reporting Behavior](../../Operations/Validation/Behaviors.md#validation-error-reporting-behavior)
  * [Explorer Serve Pipeline Specification](Specifications.md#explorer-serve-pipeline-specification)

#### Relations
  * definedBy: [Serve Command Contract Specification](Specifications.md#serve-command-contract-specification)
  * derivedFrom: [Web Interface](../InterfacesRequirements.md#web-interface)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [serve.rs](../../../crates/reqvire-cli/src/serve.rs)
  * verifiedBy: [Serve Command Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#serve-command-verification)
---

### Serve Command Embedded MCP Endpoint

The system shall allow the Explorer serve command to expose the Reqvire MCP Streamable HTTP endpoint at `/mcp` on the same HTTP listener when explicitly enabled.

#### Details
Detailed embedded endpoint, registry reuse, transport, mutation gating, route preservation, runtime refresh, and endpoint display rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Serve Command Contract Specification](Specifications.md#serve-command-contract-specification)

#### Relations
  * definedBy: [Serve Command Embedded MCP Endpoint Specification](../MCP/Specifications.md#serve-command-embedded-mcp-endpoint-specification)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [mcp.rs](../../../crates/reqvire-cli/src/mcp.rs)
  * satisfiedBy: [serve.rs](../../../crates/reqvire-cli/src/serve.rs)
  * verifiedBy: [Embedded MCP Serve Endpoint Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#embedded-mcp-serve-endpoint-verification)
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
  * derive: [Containment View Contract Bindings Links](#containment-view-contract-bindings-links)
  * derive: [Contract Bindings Link Serving](#contract-bindings-link-serving)
  * derive: [Diagram Contract Bindings Display](#diagram-contract-bindings-display)
  * derive: [Model-Centric View Generation](#model-centric-view-generation)
  * derive: [Ontologies View Generation](#ontologies-view-generation)
  * derive: [Project Knowledge Graph View](#project-knowledge-graph-view)
  * derive: [SPA Explorer Shell and Project Store](#spa-explorer-shell-and-project-store)
  * derive: [Thesaurus View Generation](#thesaurus-view-generation)
  * derive: [Traces View Generation](#traces-view-generation)
  * derive: [Web Interface Color Scheme](#web-interface-color-scheme)
  * derivedFrom: [Web Interface](../InterfacesRequirements.md#web-interface)
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [explorer_runtime.rs](../../../crates/reqvire-core/src/explorer_runtime.rs)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/html/mod.rs)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
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

### Containment View Contract Bindings Links

The system shall preserve element contract_bindings links in Model containment data and expose them from supported Explorer surfaces to provide quick access to associated contract elements and documents.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Containment View Contract Bindings Links Contract Specification](Specifications.md#containment-view-contract-bindings-links-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [containment.rs](../../../crates/reqvire-core/src/containment.rs)
  * verifiedBy: [Model Containment Contract Bindings Links Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#model-containment-contract-bindings-links-verification)
---

### Contract Bindings Link Serving

The system shall preserve contract_bindings identifier links to referenced contract elements in the served Explorer to preserve document completeness and enable navigation.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Contract Bindings Link Serving Contract Specification](Specifications.md#contract-bindings-link-serving-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
  * verifiedBy: [Contract Bindings Link Serving Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#contract-bindings-link-serving-verification)
---

### Diagram Contract Bindings Display

The system shall display contract_bindings links within element boxes in generated diagrams to show document associations visually.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Diagram Contract Bindings Display Contract Specification](Specifications.md#diagram-contract-bindings-display-contract-specification)
  * definedBy: [Explorer Mermaid Diagram Style Specification](Specifications.md#explorer-mermaid-diagram-style-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
  * satisfiedBy: [verification_trace.rs](../../../crates/reqvire-core/src/verification_trace.rs)
  * verifiedBy: [Diagram Contract Bindings Display Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#diagram-contract-bindings-display-verification)
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
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
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
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
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
- Consume generated ontology projection facts from the semantic export context so the Ontologies Explorer and `reqvire semantic export` describe the same ontology constructs.
- Use ontology-document ownership metadata from generated `rdfs:isDefinedBy`/ontology term declarations for term grouping, search, and modal evidence, while not rendering ontology document IRIs as graph nodes or definition links as canvas edges.
- Expose ontology graph layers so users can inspect authored structural semantic content by default, optionally show curated SKOS Concepts and their structural bridges, optionally overlay semantic context for model-to-term declaration/reference provenance, and separately enable only the used external source vocabulary subset when external ontology dependencies are present.
- Prioritize graph canvas space by using a dense full-height viewer layout, left-pane ontology controls, and modal detail for selected ontology nodes.
- Compute full-graph layout from currently visible ontology nodes and edges using bounded data-dependent layout settings so hidden layers do not distort visible ontology structure.
- Reorganize the selected ontology focus tree locally to reduce node and label overlap while preserving the stable full-graph layout outside the focused tree.
- Separate normalized ontology constructs and SHACL-derived slots/facets from optional raw SHACL evidence; empty raw-evidence sections shall not be shown.
- Apply viewer filters as explicit canvas visibility contracts: authored structural ontology semantics are enabled by default, Concepts, semantic context, and external source layers are independently controlled overlays, ontology terms and class-membership context stay available, role controls govern optional SHACL, resource, and external-reference visibility, and construct notation remains available as passive legend and modal evidence.
- Keep linked source citations in the viewer and the served `ontologies.ttl` artifact available for traceability and downstream tooling without rendering raw Turtle blocks as the primary page content.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  * satisfiedBy: [ontologyGraphRenderer.ts](../../../explorer/src/lib/ontologyGraphRenderer.ts)
  * satisfiedBy: [OntologiesView.tsx](../../../explorer/src/views/OntologiesView.tsx)
---

### OWL Semantic Ontology Projection

The system shall consume o-kernel construct classifications and Reqvire semantic-index context to produce generated ontology projection facts that suppress RDF serialization mechanics and expose stable ontology concepts for both full semantic export and Explorer visualization.

#### Details
Detailed semantic-index, source-traceability, projection-subgraph, SHACL slot/facet, RDF-mechanics suppression, and Explorer/export consumer rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Ontology Construct Classification Specification](../../Architecture/OntologyKernelSpecifications.md#ontology-construct-classification-specification)

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
Detailed domain/range aggregation, deduplicated relationship rendering, modal evidence, source citation, SHACL slot/facet, and property characteristic rules shall follow the associated specification.

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
- Render actual relation facts, contract_bindings facts, concept-reference facts, file targets, and external targets as graph edges or resource nodes. Concept-reference facts target SKOS concept nodes and must not create a separate concept-reference node type.
- Use Reqvire capability-root submodels as the structural graph partitioning contract; contract_bindings, concept references, verification/satisfaction, and trace facts are overlays rather than submodel boundaries.
- Treat structural ownership/backbone relations separately from cross-layer evidence relations: `derive` and `specify` organize the capability/requirement submodel backbone, while requirement-owned `define`, `reuse`, `satisfiedBy`, `verifiedBy`, and concept-reference facts connect subordinate details or layers as inspectable overlays.
- Compute the full graph baseline from currently visible nodes and edges using bounded data-dependent layout settings so hidden filters and overlays do not distort visible project graph structure.
- Reorganize the clicked node's visible neighborhood during pinned focus from a stable full-graph baseline so selected-node exploration reduces local overlap, restores the previous focus when it leaves scope, and does not rebuild the full graph or lose the global layout.
- Reuse the dense Explorer graph and modal-detail interaction pattern used by the ontology viewer, while focusing on project instances rather than ontology vocabulary definitions.
- Provide modal detail evidence for element type, identifier, source location, governance, metadata, incoming facts, outgoing facts, contract_bindings, and concept references.
- Keep ontology vocabulary exploration in the Ontologies view; the Knowledge Graph view may show ontology terms only when they are referenced by actual project elements.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Project Knowledge Graph View Contract Specification](Specifications.md#project-knowledge-graph-view-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
  * satisfiedBy: [GraphLibraryViews.tsx](../../../explorer/src/views/GraphLibraryViews.tsx)
  * verifiedBy: [SPA Explorer Store Contract Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#spa-explorer-store-contract-verification)
---

### SPA Explorer Shell and Project Store

The system shall provide `index.html` as the single-page Reqvire Explorer shell and seed it with a normalized browser-local Project Store that supports the primary Explorer routes and supporting report/detail views as the canonical browser experience.

#### Details
The SPA Explorer shell shall:
- Be a native single-page application built with Vite, TypeScript, and React, using the Reqvire Explorer design system and compiled CSS, served as `index.html` plus deterministic `assets/explorer.js` and `assets/explorer.css` bundles with no CDN-loaded framework or stylesheet and no runtime CSS compiler.
- Treat `index.html` as the primary browser entry point and central Project Store host.
- Expose project identity metadata in the Project Store, including effective workspace root label and eligible Git worktree names, paths, and source-control metadata when available, so Explorer navigation can group modeled files and resources by Git worktree identity instead of showing a generic project label.
- Render the primary Model route as a native SPA view module reading from the Project Store. The Model route shall host List, Grid, and Graph modes over the Project Store filesystem/model and knowledge-graph projections.
- Keep Model project-tree selection shared across List, Grid, and Graph modes so selecting a folder, file, or modeled element in the left Explorer tree updates the active Model workspace instead of opening a disconnected Filesystem view.
- Render Graph as a Model mode over the Project Store knowledge-graph projection, render specialist Ontologies and Traces routed views from top navigation and tool actions, and render supporting Search, file deep links, Coverage, Resources, and element-detail workflows from the same Project Store without making them primary left-pane navigation modes.
- Build the Search route's ranked MiniSearch index in a browser worker after the initial shell render, using Project Store search documents with boosted title, path, result-kind, and content fields so indexing does not block primary Explorer interaction.
- Open element-detail routes in an in-shell scrollable modal backed by Project Store element records, preserving the current Explorer view context behind the modal.
- When a user opens a related element from inside an element-detail modal, keep an in-modal back target so the user can return to the previously inspected element without losing the underlying Explorer route context.
- Show the actual element type as the single primary type badge in element-detail modal headers. Canonical type family may drive color and icon semantics, but it shall not be rendered as a second visible kind badge when the actual type is more specific.
- Render authored concept references in regular element-detail modal content and source-page content as quiet inline links on matching prose terms, using the referenced native SKOS concept's preferred label, alternative labels, or authored reference label for matching.
- Open inline concept-reference links to the native `concept` element modal. Concept references shall not route to ontology-node modals because authored model concept references resolve to native concept elements.
- Suppress the reserved `#### Concept References` source subsection from regular element-detail modal body rendering and source-page rendering; the subsection is authoring metadata and must not appear as a separate visible content block.
- Do not generate separate Explorer/report document entry points.
- Seed a normalized project snapshot that distinguishes modeled source-file containers from modeled resource and evidence-file targets. The Model tree shall render `Model` and `Resources` as the visible top-level branches. Under each branch, eligible Git worktree identity folders shall group the corresponding modeled source files or graph-linked resources, so single-repo workspaces still show the repository identity and multi-repo workspaces show each repository/worktree separately. The `Model` branch shall contain modeled element source files from the graph registry as file containers. The `Resources` branch shall expose graph-linked implementation, evidence, documentation, and resource targets as a separate resource/evidence hierarchy that preserves workspace-root-relative folder structure when a local path exists. Resource-only targets shall not be represented as file containers that imply modeled elements. Unrelated Git-worktree files and all non-Git workspace files remain absent.
- Keep containment, model, knowledge graph, verification traces, coverage, resources, ontology, search, summaries, and route state as view-neutral store projections rather than separate page-local data models.
- Preserve the current relation model: capabilities may author concept references and are specified by requirements; requirements own contracts, satisfaction evidence, verification evidence, reusable contract contract_bindings, and concept references. Capability coverage is computed from verified requirements.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)

#### Relations
  * definedBy: [Explorer Store Seed Data Output Specification](Specifications.md#explorer-store-seed-data-output-specification)
  * definedBy: [SPA Explorer Store Contract Specification](Specifications.md#spa-explorer-store-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [git_commands.rs](../../../crates/reqvire-core/src/git_commands.rs)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
  * satisfiedBy: [DetailDialog.tsx](../../../explorer/design-system/product-patterns/detail/DetailDialog.tsx)
  * satisfiedBy: [FileBrowser.tsx](../../../explorer/design-system/product-patterns/files/FileBrowser.tsx)
  * satisfiedBy: [App.tsx](../../../explorer/src/App.tsx)
  * satisfiedBy: [ElementDetailModal.tsx](../../../explorer/src/components/ElementDetailModal.tsx)
  * satisfiedBy: [ExplorerSidePane.tsx](../../../explorer/src/components/ExplorerSidePane.tsx)
  * satisfiedBy: [types.ts](../../../explorer/src/store/types.ts)
  * satisfiedBy: [FilesView.tsx](../../../explorer/src/views/FilesView.tsx)
  * verifiedBy: [SPA Explorer Store Contract Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#spa-explorer-store-contract-verification)
---

### Thesaurus View Generation

The system shall expose a Thesaurus Explorer view during serve workflows that presents standalone native concept schemes and concepts as curated SKOS terminology instead of as ontology children or filesystem folders.

#### Details
Detailed route source, concept-scheme grouping, navigation, modal, map rendering, source navigation, and ontology-boundary rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Thesaurus View Generation Contract Specification](Specifications.md#thesaurus-view-generation-contract-specification)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
  * satisfiedBy: [ExplorerSidePane.tsx](../../../explorer/src/components/ExplorerSidePane.tsx)
  * satisfiedBy: [types.ts](../../../explorer/src/store/types.ts)
  * satisfiedBy: [ThesaurusView.tsx](../../../explorer/src/views/ThesaurusView.tsx)
  * verifiedBy: [SPA Explorer Store Contract Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#spa-explorer-store-contract-verification)
  * verifiedBy: [Thesaurus Project Store Projection Verification](../../Verifications/Interfaces/WebExplorer/WebInterfaceVerifications.md#thesaurus-project-store-projection-verification)
---

### Traces View Generation

The system shall expose verification traceability as a specialist Explorer view backed by Project Store trace projections and rendered through the shared Explorer shell.

#### Details
Detailed route data, left-pane behavior, flow/row rendering, Mermaid roll-up diagram rendering, modal interaction, and trace-data ownership rules shall follow the associated specifications.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Trace Diagram Projection Data Contract Specification](../../Reports/ModelReports/Specifications.md#trace-diagram-projection-data-contract-specification)
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * definedBy: [Explorer Verification Trace Rendering Specification](Specifications.md#explorer-verification-trace-rendering-specification)
  * definedBy: [Traces View](TracesView.md#traces-view)
  * derivedFrom: [Served Explorer Browser Interface](#served-explorer-browser-interface)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
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
- Programmatic palette tokens for browser-rendered graph, Mermaid, and badge renderers

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
  * satisfiedBy: [containment.rs](../../../crates/reqvire-core/src/containment.rs)
  * satisfiedBy: [ElementIcon.tsx](../../../explorer/design-system/components/data/ElementIcon.tsx)
  * satisfiedBy: [palette.ts](../../../explorer/design-system/palette.ts)
  * satisfiedBy: [ContractBindingList.tsx](../../../explorer/design-system/product-patterns/detail/ContractBindingList.tsx)
  * satisfiedBy: [RelationEndpoint.tsx](../../../explorer/design-system/product-patterns/detail/RelationEndpoint.tsx)
  * satisfiedBy: [colors.css](../../../explorer/design-system/tokens/colors.css)
---
