# Elements

### HTML Export

The system SHALL generate comprehensive HTML documentation with all model artifacts by creating a temporary working copy, generating all reports in that copy, and exporting to the output directory.

#### Metadata
  * type: requirement

#### Relations
  * derive: [SPA Explorer Shell and Project Store](#spa-explorer-shell-and-project-store)
  * derive: [Attachment Export](#attachment-export)
  * derive: [Containment View Attachment Links](#containment-view-attachment-links)
  * derive: [Diagram Attachment Display](#diagram-attachment-display)
  * derive: [Local Linked File Export](#local-linked-file-export)
  * derive: [Model-Centric View Generation](#model-centric-view-generation)
  * derive: [Project Knowledge Graph View](#project-knowledge-graph-view)
  * derive: [Ontologies View Generation](#ontologies-view-generation)
  * derive: [Web Interface Color Scheme](#web-interface-color-scheme)
  * derivedFrom: [Web Interface](../Interfaces.md#web-interface)
  * refinedBy: [Web Interface Navigation Behavior](Behaviors.md#web-interface-navigation-behavior)
  * refinedBy: [D3.js Containment Tree Specification](Specifications.md#d3js-containment-tree-specification)
  * refinedBy: [HTML Branding Specification](Specifications.md#html-branding-specification)
  * refinedBy: [HTML Export Pipeline Specification](Specifications.md#html-export-pipeline-specification)
  * refinedBy: [HTML Navigation Bar Specification](Specifications.md#html-navigation-bar-specification)
  * refinedBy: [Web Interface Style Specification](Specifications.md#web-interface-style-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * satisfiedBy: [layouts.rs](../../../core/src/html/layouts.rs)
  * satisfiedBy: [mod.rs](../../../core/src/html/mod.rs)
  * satisfiedBy: [knowledgegraph.rs](../../../core/src/html/pages/knowledgegraph.rs)
  * satisfiedBy: [ontologies.rs](../../../core/src/html/pages/ontologies.rs)
  * satisfiedBy: [html_export.rs](../../../core/src/html_export.rs)
  * satisfiedBy: [index_generator.rs](../../../core/src/index_generator.rs)
  * verifiedBy: [CLI Help Structure Verification](../CLI/Verifications/CLIVerifications.md#cli-help-structure-verification)
  * verifiedBy: [HTML Export Verification](Verifications/WebInterfaceVerifications.md#html-export-verification)
---

### SPA Explorer Shell and Project Store

The system shall generate `index.html` as the single-page Reqvire Explorer shell and seed it with a normalized browser-local Project Store that supports the primary Explorer routes and supporting report/detail views as the canonical browser experience.

#### Details
The SPA Explorer shell shall:
- Be a native static single-page application built with Vite, TypeScript, and React, using Radix Themes 3 with `@radix-ui/react-icons` and compiled Tailwind, exported as `index.html` plus deterministic `assets/explorer.js` and `assets/explorer.css` bundles with no CDN-loaded framework or stylesheet and no runtime Tailwind compiler.
- Treat `index.html` as the primary browser entry point and central Project Store host.
- Render the primary Model route as a native SPA view module reading from the Project Store. The Model route shall host List, Grid, Sunburst, and Icicle modes over the Project Store filesystem/model and containment projections.
- Render specialist Knowledge Graph, Ontologies, Traces, and KN2 routed views from right vertical tool-rail icons, and render supporting Search, file deep links, Coverage, Resources, and element-detail workflows from the same Project Store without making them primary left-pane navigation modes.
- Open element-detail routes in an in-shell scrollable modal backed by Project Store element records, preserving the current Explorer view context behind the modal.
- Do not generate standalone Explorer/report HTML entry points.
- Seed a normalized project snapshot that distinguishes exported filesystem/source file containers from modeled resource and evidence-file targets.
- Keep containment, model, knowledge graph, verification traces, coverage, resources, ontology, search, summaries, and route state as view-neutral store projections rather than separate page-local data models.
- Preserve the current relation model: capabilities attach ontology and are specified/verified; requirements own refinements, satisfaction evidence, verification evidence, and reusable refinement attachments.

#### Metadata
  * type: requirement

#### Concept References
  * Project Store: reqvire:BrowserLocalProjectStore
  * Explorer Route: reqvire:ExplorerRoute
  * File Container: reqvire:FileContainer
  * Resource Reference: reqvire:ModeledResource

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [SPA Explorer Store Contract Refinement Specification](Specifications.md#spa-explorer-store-contract-refinement-specification)
  * refinedBy: [Explorer Store Seed Data Output Specification](../../Functional/Output/Specifications.md#explorer-store-seed-data-output-specification)
  * verifiedBy: [SPA Explorer Store Contract Verification](Verifications/WebInterfaceVerifications.md#spa-explorer-store-contract-verification)
---

### Project Knowledge Graph View

The system shall expose a Knowledge Graph view during export and serve workflows as a specialist Explorer route. The view presents the actual parsed project graph as current elements and facts.

#### Details
The Knowledge Graph view shall:
- Answer what actual project elements and facts exist right now.
- Be reached through the canonical `index.html#/knowledge-graph` Explorer route from the right vertical tool rail; a separate Knowledge Graph HTML entry point shall not be generated as standalone Explorer output.
- Render the four system-model layers as first-class graph nodes: ontologies, capabilities, requirements, and verifications. Requirement-owned refinements may appear as subordinate requirement detail/contract nodes when actual project facts reference them, but they are not a separate system-model layer.
- Render actual relation facts, attachment facts, concept-reference facts, file targets, and external targets as graph edges or resource nodes.
- Use Reqvire capability-root submodels as the structural graph partitioning contract; attachments, concept references, verification/satisfaction, and trace facts are overlays rather than submodel boundaries.
- Treat structural ownership/backbone relations separately from cross-layer evidence relations: `derive` and `specify` organize the capability/requirement submodel backbone, while requirement-owned `refine`, `attach`, `satisfiedBy`, `verifiedBy`, `trace`, and concept-reference facts connect subordinate details or layers as inspectable overlays.
- Reuse the dense Explorer graph-and-inspector interaction pattern used by the ontology viewer, while focusing on project instances rather than ontology vocabulary definitions.
- Provide inspector evidence for element type, identifier, source location, governance, metadata, incoming facts, outgoing facts, attachments, and concept references.
- Keep ontology vocabulary exploration in the Ontologies view; the Knowledge Graph view may show ontology terms only when they are referenced by actual project elements.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Project Knowledge Graph View Refinement Specification](Specifications.md#project-knowledge-graph-view-refinement-specification)
  * satisfiedBy: [knowledgegraph.rs](../../../core/src/html/pages/knowledgegraph.rs)
  * satisfiedBy: [kn2.rs](../../../core/src/html/pages/kn2.rs)
  * verifiedBy: [HTML Generation Integration Verification](Verifications/HTMLGenerationVerifications.md#integration-test-verification)
---

### Ontologies View Generation

The system shall expose an Ontologies Explorer view during export and serve workflows that presents ontology and SHACL semantics through an OWL-aware model viewer instead of a raw RDF triple graph.

#### Details
The Ontologies view shall:
- Use RDF/Turtle and SHACL blocks as the parsed semantic source, while avoiding raw RDF serialization artifacts as the primary user-facing visualization.
- Present ontology concepts as typed visual entities such as classes, named individuals, SHACL shapes, datatypes, restrictions, and class expressions, while keeping literal constraints and datatype-property literal values as inspector/search evidence rather than primary graph nodes.
- Treat ontology properties as labeled relationship semantics between their domain/range terms, not as standalone graph nodes, while retaining domain, range, inverse, equivalence, chain, and characteristic information as inspector/search evidence when those axioms are present.
- Derive class slots and slot facets from SHACL target-class/property-shape constraints so users can inspect datatype or object range, cardinality, pattern, node-kind, and allowed-value constraints without navigating raw SHACL blank nodes.
- Show named properties as reusable slots in the selected class or term inspector, including each target class and source shape that uses that property as a `sh:path`, so repeated property usages are understandable rather than presented as duplicate property definitions.
- Use a defined ontology symbol and badge vocabulary so rendered symbols are stable, documented, and accessible.
- Consume generated ontology projection facts from the semantic export context so the HTML explorer and `reqvire ontologies --full` describe the same ontology constructs.
- Prioritize graph canvas space by using a dense full-height viewer layout with a scroll-contained shared `Inspector` lane.
- Separate normalized ontology constructs and SHACL-derived slots/facets from optional raw SHACL evidence; empty raw-evidence sections shall not be shown.
- Apply viewer filters as explicit visibility contracts: the single `Show` control group determines coarse node/resource visibility and relation visibility, the SHACL shapes control governs both SHACL shape nodes and SHACL overlay relations, datatype/object property controls are named as property-link controls, and other construct notation remains available as passive legend and inspector evidence.
- Keep linked source citations in the viewer and the exported `ontologies.ttl` artifact available for traceability and downstream tooling without rendering raw Turtle blocks as the primary page content.

#### Metadata
  * type: requirement

#### Attachments
  * [Ontology Collection Output Specification](../../Functional/Output/Specifications.md#ontology-collection-output-specification)

#### Relations
  * derive: [OWL Semantic Ontology Projection](#owl-semantic-ontology-projection)
  * derive: [Ontology Property-Centric Visualization](#ontology-property-centric-visualization)
  * derive: [Ontology Construct Grouping](#ontology-construct-grouping)
  * derive: [Ontology Symbol and Badge Vocabulary](#ontology-symbol-and-badge-vocabulary)
  * derivedFrom: [HTML Export](#html-export)
  * trace: [Ontology Projection Subgraph Materialization](../../Functional/Output/Reporting.md#ontology-projection-subgraph-materialization)
  * refinedBy: [Ontologies View Generation Refinement Specification](Specifications.md#ontologies-view-generation-refinement-specification)
  * refinedBy: [Ontology Rendering Details](OntologyRenderingDetails.md#ontology-rendering-details)
---

### OWL Semantic Ontology Projection

The system shall transform semantic-index RDF quads into generated ontology projection facts that suppress RDF serialization mechanics and expose stable ontology concepts for both full semantic export and HTML visualization.

#### Details
The projection shall classify terms by semantic role, preserve source traceability, attach generated direct-authored OWL/RDFS/SHACL construct data to `SemanticIndex`, expose that data as a reusable ontology projection subgraph inside the existing in-memory RDF projection, derive normalized slot/facet records from SHACL property shapes, and omit primary rendering of `rdf:type` edges, RDF list plumbing, metaclass resources, and anonymous blank-node implementation details unless those nodes represent a meaningful ontology construct.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Ontologies View Generation](#ontologies-view-generation)
  * refinedBy: [OWL Semantic Ontology Projection Refinement Specification](Specifications.md#owl-semantic-ontology-projection-refinement-specification)
  * verifiedBy: [Ontology Model Viewer Analysis Verification](Verifications/WebInterfaceVerifications.md#ontology-model-viewer-analysis-verification)
---

### Ontology Property-Centric Visualization

The system shall render OWL object properties and datatype properties as first-class relationship semantics: properties appear as labeled domain/range edges and inspector evidence, rather than standalone graph boxes.

#### Details
The property visualization shall aggregate many domain and range classes without multiplying identical property arcs, distinguish object-property ranges from datatype-property ranges, and show property semantics through compact edge labels, badges, or inspector sections on selected classes, individuals, and terms. When a named property is used by multiple SHACL property shapes, the inspector shall present those as target-class usages with source-shape evidence rather than as duplicate property definitions.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Ontologies View Generation](#ontologies-view-generation)
  * refinedBy: [Ontology Property-Centric Visualization Refinement Specification](Specifications.md#ontology-property-centric-visualization-refinement-specification)
  * verifiedBy: [Ontology Model Viewer Analysis Verification](Verifications/WebInterfaceVerifications.md#ontology-model-viewer-analysis-verification)
---

### Ontology Construct Grouping

The system shall present multi-node OWL constructs as explicit semantic groups instead of exposing their low-level RDF representation.

#### Details
Construct grouping shall cover equivalence groups, inverse properties, property-chain axioms, property characteristics, and SHACL shape overlays when those constructs are present in the collected ontology or semantic-contract content.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Ontologies View Generation](#ontologies-view-generation)
  * refinedBy: [Ontology Construct Grouping Refinement Specification](Specifications.md#ontology-construct-grouping-refinement-specification)
  * verifiedBy: [Ontology Model Viewer Analysis Verification](Verifications/WebInterfaceVerifications.md#ontology-model-viewer-analysis-verification)
---

### Ontology Symbol and Badge Vocabulary

The system shall define a canonical Unicode symbol vocabulary for ontology viewer badges, edge labels, group headers, tooltips, and inspector fields.

#### Details
The symbol vocabulary shall define each symbol with its semantic meaning, raw Unicode code point, rendered Unicode character, and allowed viewer usage locations. Symbols shall supplement text labels and accessible descriptions; they shall not be the only carrier of meaning.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Ontologies View Generation](#ontologies-view-generation)
  * refinedBy: [Ontology Symbol and Badge Vocabulary Refinement Specification](Specifications.md#ontology-symbol-and-badge-vocabulary-refinement-specification)
  * verifiedBy: [Ontology Model Viewer Analysis Verification](Verifications/WebInterfaceVerifications.md#ontology-model-viewer-analysis-verification)
---

### Local Linked File Export

The system shall preserve local file references in exported markdown content so linked local assets remain usable in exported HTML pages.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Local Linked File Export Refinement Specification](Specifications.md#local-linked-file-export-refinement-specification)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * satisfiedBy: [html_export.rs](../../../core/src/html_export.rs)
  * verifiedBy: [HTML Export Local Linked Files Verification](Verifications/WebInterfaceVerifications.md#html-export-local-linked-files-verification)
---

### Attachment Export

The system shall preserve attachment identifier links to referenced refinement elements during HTML export to preserve document completeness and enable navigation.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Attachment Export Refinement Specification](Specifications.md#attachment-export-refinement-specification)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * verifiedBy: [Attachment Export Verification](Verifications/WebInterfaceVerifications.md#attachment-export-verification)
---

### Containment View Attachment Links

The system shall display attachment links as children of elements in the containment D3.js tree to provide quick access to associated documents.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Containment View Attachment Links Refinement Specification](Specifications.md#containment-view-attachment-links-refinement-specification)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * verifiedBy: [Containment Attachment Links Verification](Verifications/WebInterfaceVerifications.md#containment-attachment-links-verification)
---

### Diagram Attachment Display

The system shall display attachment links within element boxes in generated diagrams to show document associations visually.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Diagram Attachment Display Refinement Specification](Specifications.md#diagram-attachment-display-refinement-specification)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Diagram Attachment Display Verification](Verifications/WebInterfaceVerifications.md#diagram-attachment-display-verification)
---

### Model-Centric View Generation

The system shall generate a model-centric visualization during HTML export showing model roots with nested relations containing full element details.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Style Specification](../../Functional/Output/Specifications.md#mermaid-diagram-style-specification)

#### Relations
  * derive: [Model View Element Navigation](#model-view-element-navigation)
  * derivedFrom: [HTML Export](#html-export)
  * refinedBy: [Model-Centric View Generation Refinement Specification](Specifications.md#model-centric-view-generation-refinement-specification)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
---

### Model View Element Navigation

The system shall make element names in the model-centric view clickable links that navigate to the element's definition in its source file.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model-Centric View Generation](#model-centric-view-generation)
  * refinedBy: [Model View Element Navigation Refinement Specification](Specifications.md#model-view-element-navigation-refinement-specification)
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
  * refinedBy: [Color Scheme Specification](../../Functional/Output/Specifications.md#color-scheme-specification)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * satisfiedBy: [layouts.rs](../../../core/src/html/layouts.rs)
  * satisfiedBy: [styles.rs](../../../core/src/html/styles.rs)
---

### Serve Command

The system SHALL provide a serve command that exports comprehensive HTML documentation and serves it via an HTTP server for browsing.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Two-Pass Validation Behavior](../../Functional/Core/Behaviors.md#two-pass-validation-behavior)
  * [Validation Error Reporting Behavior](../../Functional/Core/Behaviors.md#validation-error-reporting-behavior)
  * [HTML Export Pipeline Specification](Specifications.md#html-export-pipeline-specification)

#### Relations
  * derivedFrom: [Web Interface](../Interfaces.md#web-interface)
  * refinedBy: [Serve Command Refinement Specification](Specifications.md#serve-command-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [serve.rs](../../../cli/src/serve.rs)
  * verifiedBy: [Serve Command Verification](Verifications/WebInterfaceVerifications.md#serve-command-verification)
---
