# Elements

### Model Reports

When requested the system shall provide human readable and machine readable System model reports with deterministic output and consistent ordering.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Interactive Mermaid Diagrams](DiagramGeneration.md#interactive-mermaid-diagrams)
  * derive: [Collect Capability and Requirement Context](#collect-capability-and-requirement-context)
  * derive: [JSON Element Size Estimate Exposure](#json-element-size-estimate-exposure)
  * derive: [Model Structure and Summaries](#model-structure-and-summaries)
  * derive: [Provide Validation Reports](#provide-validation-reports)
  * derive: [Requirement Implementation Coverage Report](#requirement-implementation-coverage-report)
  * derive: [Resources Report](#resources-report)
  * derive: [Verification Coverage Report](#verification-coverage-report)
  * specify: [Provide Reports](../../Capabilities/ReportsAndQuery.md#provide-reports)
  * refinedBy: [Deterministic Output Specification](Specifications.md#deterministic-output-specification)
  * refinedBy: [JSON Output Structure](Specifications.md#json-output-structure)
  * refinedBy: [Markdown Report Style Specification](Specifications.md#markdown-report-style-specification)
  * refinedBy: [Report Command Catalog Specification](Specifications.md#report-command-catalog-specification)
  * refinedBy: [Text Output Formatting](Specifications.md#text-output-formatting)
  * refinedBy: [Traceability Reporting Specification](../../Refinements.md#traceability-reporting-specification)
---

### JSON Element Size Estimate Exposure

The system shall expose element-level `size_estimate` records in JSON model evidence outputs when the model was built with size estimates enabled.

#### Details
- JSON outputs that serialize model elements shall include `size_estimate` when element size estimates are enabled.
- Non-JSON outputs shall not include size-estimate fields and shall remain unchanged.
- Report-level aggregate size summaries are out of scope.
- The initial JSON evidence outputs in scope are model evidence outputs that serialize elements directly or as nested relation targets.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [JSON Element Size Estimate Output Specification](Specifications.md#json-element-size-estimate-output-specification)
  * verifiedBy: [JSON Element Size Estimate Output Verification](Verifications/ReportingVerifications.md#json-element-size-estimate-output-verification)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
---

### Collect Capability and Requirement Context

The system shall collect and consolidate context from a capability or requirement element, including directional capability and requirement traversal, inherited capability ontology context, refinedBy targets, attached requirement contract contents, and source citations in text or JSON format.

#### Details
The system shall define:
- Content collection rules for elements, refinedBy targets, and attachments
- Output format specifications for text and JSON modes
- Direction-based traversal over capability hierarchy, requirement hierarchy, and the `specify`/`specifiedBy` bridge where defined by the collect traversal specification

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [Collect Content Specification](Specifications.md#collect-content-specification)
  * refinedBy: [Collect Output Format Specification](Specifications.md#collect-output-format-specification)
  * satisfiedBy: [report_collect.rs](../../../core/src/report_collect.rs)
---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the System model, including counts and types of connections, ontology-root and capability-root starting contexts, and JSON output.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Containment View Report](#containment-view-report)
  * derive: [Model Diagram Output Formats](#model-diagram-output-formats)
  * derive: [Requirement Submodels Report](#requirement-submodels-report)
  * derive: [Search Report Generator](#search-report-generator)
  * derivedFrom: [Model Reports](#model-reports)
---

### Containment View Report

The system shall generate containment view reports showing the physical hierarchical structure of the model.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Containment Specification](../../Refinements.md#containment-specification)
  * [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * refinedBy: [ContainmentView](DesignDocuments/ContainmentView.md#containmentview)
  * refinedBy: [Containment View Report Refinement Specification](Specifications.md#containment-view-report-refinement-specification)
  * verifiedBy: [Containment View Design Documents Test](Verifications/ReportingVerifications.md#containment-view-design-documents-test)
---

### Model Diagram Output Formats

System shall support Markdown, pure Mermaid, and JSON output formats.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * [Diagram Relation Filtering Specification](Specifications.md#diagram-relation-filtering-specification)

#### Relations
  * derive: [Forward-Only Relation Traversal](#forward-only-relation-traversal)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * refinedBy: [Model Diagram Output Formats Refinement Specification](Specifications.md#model-diagram-output-formats-refinement-specification)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](Verifications/ReportingVerifications.md#model-command-verification)
---

### Forward-Only Relation Traversal

When filtering by root element, system shall traverse only forward relations down to leaf elements.

#### Details
Traversal behavior shall follow the associated behavior refinement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * refinedBy: [Forward-Only Relation Traversal Behavior](Behaviors.md#forward-only-relation-traversal-behavior)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](Verifications/ReportingVerifications.md#model-command-verification)
---

### Reverse Relation Traversal

The system shall support reverse relation traversal for model views, following defined rules in Reverse Relation Traversal Behavior.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * refinedBy: [Reverse Relation Traversal Behavior](Behaviors.md#reverse-relation-traversal-behavior)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * verifiedBy: [Reverse Model Traversal Test](Verifications/ReportingVerifications.md#reverse-model-traversal-test)
---

### Start Element Type Filtering

The system shall support filtering starting elements by type for model traversal, following defined rules in Start Element Type Filter Behavior.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * refinedBy: [Start Element Type Filter Behavior](Behaviors.md#start-element-type-filter-behavior)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * verifiedBy: [Start Type Filter Test](Verifications/ReportingVerifications.md#start-type-filter-test)
---

### Requirement Submodels Report

The system shall provide a submodels report that identifies independent capability-root subgraphs and cross-submodel requirement couplings.

#### Details
The report shall support:
- Full model view listing all discovered capability-rooted submodels and cross-submodel couplings
- Filtered view scoped to one capability or requirement subtree by element name
- Scope filtering follows transitive descendants via capability hierarchy, `specifiedBy`, and requirement hierarchy in downstream direction.
- When filtered from a capability, the selected capability is reported as the scoped capability submodel and requirement counts include requirements in that capability subtree.
- When filtered from a requirement, the selected requirement is a boundary and is not counted as a reported submodel entry; first-level child requirement branches are reported as scoped requirement submodels.
- When a selected requirement subtree has no child submodels, the filtered report contains zero scoped submodels.
- The report summary includes deterministic counts for total submodels, total requirements represented in scope, and total cross-submodel couplings; in scoped mode, counts are computed from the scoped submodels and couplings only.
- Summary content follows the report paragraph: `Submodels`, `Requirements`, and `Cross-Submodel Couplings`.

Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [RelationTypes](../Core/DesignDocuments/RelationTypes.md#relationtypes)

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * refinedBy: [Requirement Submodels Report Specification](Specifications.md#requirement-submodels-report-specification)
  * satisfiedBy: [report_submodels.rs](../../../core/src/report_submodels.rs)
  * verifiedBy: [Submodels Report Verification](Verifications/ReportingVerifications.md#submodels-report-verification)
---

### Ontology and Shapes Collection

The system shall collect ontology `#### Ontology` and semantic-contract `#### Shapes` RDF blocks from the graph registry into a reusable semantic export context, and shall optionally project Reqvire model context and generated ontology construct facts into the same RDF export.

#### Details
The default collection shall expose authored ontology RDF content and semantic-contract SHACL RDF content without changing the Markdown model as the source of truth.

When full semantic model export is requested, the collection shall also emit RDF triples for Reqvire model elements, element metadata, capability-to-ontology attachments, requirement-to-capability specification relations, requirement-to-semantic-contract refinement relations, ontology hierarchy relations, concept references, ontology term declarations, semantic-contract shape references, and generated ontology projection facts materialized from direct-authored OWL/RDFS/SHACL constructs.

Semantic-query-contract `#### Query` content and query metadata are not part of ontology collection, full semantic export, or `ontologies.ttl`. Generated ontology projection facts may cite the semantic-query-contract IRI that defines the intended construct pattern, but raw query text remains exposed through search JSON until a dedicated query-export command is specified and implemented.

The collection shall preserve source element identifiers, source file paths, section kind, and line numbers so CLI, HTML export, and downstream semantic tooling can cite the model source of each RDF block.

#### Metadata
  * type: requirement

#### Attachments
  * [Semantic Contract Structure Specification](../Core/Specifications.md#semantic-contract-structure-specification)

#### Relations
  * specify: [Semantic Model Export](../../Capabilities/ReportsAndQuery.md#semantic-model-export)
  * refinedBy: [Ontology Collection Output Specification](Specifications.md#ontology-collection-output-specification)
  * satisfiedBy: [semantic_contract.rs](../../../core/src/semantic_contract.rs)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * verifiedBy: [CLI Ontologies Command Verification](../../Interfaces/CLI/Verifications/CLIVerifications.md#cli-ontologies-command-verification)
---

### Ontology Projection Subgraph Materialization

The system shall materialize generated ontology construct facts as a subgraph of the existing in-memory RDF projection so semantic exports and HTML ontology exploration consume the same ontology construct facts.

#### Details
The ontology projection subgraph shall:
- Extend the existing in-memory RDF projection used by full semantic export; it is not a separate database, persistent store, or HTML-only model.
- Be attached to the reusable `SemanticIndex` as structured generated projection data so semantic export, JSON-LD export, and HTML rendering share one authoritative projection source.
- Be generated from authored ontology and semantic-contract RDF quads during semantic index processing or immediately after parsing.
- Materialize direct-authored OWL/RDFS/SHACL constructs into Reqvire ontology projection facts without changing authored Markdown ontology or semantic-contract blocks.
- Preserve source element identifier, source name, source file, source line, source block kind, construct subject, construct object, construct members, construct property, ordered sequence index when relevant, symbol code point, rendered symbol, and derivation mode.
- Derive normalized SHACL slot/facet projection records from node-shape target classes and property-shape paths so target class inspectors and named property inspectors can share the same source-backed semantic evidence.
- Include constructs for property domain/range, subclass or inclusion, class membership, disjointness, equivalence, inverse properties, property chains, property characteristics, restrictions, intersections, unions, complement or difference-style expressions when authored, and SHACL shape overlays when present.
- Use semantic-query-contract refinements as declarative pattern contracts for the generated projection facts. The implementation may use native Rust projection over the parsed RDF graph as long as the materialized facts satisfy the same pattern intent.
- Distinguish direct-authored projection from inferred projection. Direct-authored projection is in scope; OWL reasoning, SHACL-AF rule execution, and inferred materialization require separate inference requirements before they can contribute generated facts.
- Feed generated facts into `reqvire ontologies --full`, `reqvire ontologies --full --jsonld`, and the Ontologies HTML explorer through the same in-memory projection context. The default `reqvire ontologies` and exported `ontologies.ttl` artifact remain authored ontology/SHACL collection outputs unless full export is requested.

#### Metadata
  * type: requirement

#### Attachments
  * [Ontology Collection Output Specification](Specifications.md#ontology-collection-output-specification)
  * [Semantic Query Contract Structure Specification](../Core/Specifications.md#semantic-query-contract-structure-specification)

#### Relations
  * specify: [Semantic Model Export](../../Capabilities/ReportsAndQuery.md#semantic-model-export)
  * refinedBy: [Ontology Projection Subgraph Materialization Specification](Specifications.md#ontology-projection-subgraph-materialization-specification)
  * verifiedBy: [CLI Ontologies Command Verification](../../Interfaces/CLI/Verifications/CLIVerifications.md#cli-ontologies-command-verification)
---

### Search Report Generator

The system shall implement a search report generator with comprehensive filtering and element type tracking.

#### Details
The search report must include file-level, section-level, and element-level information.

Search JSON shall expose parsed semantic model fields when present. Ontology elements expose `ontology`; semantic-contract elements expose `semantic_contract`; semantic-query-contract elements expose `semantic_query_contract` with derived identity and Query fenced block details. Short mode may omit these parsed semantic fields.

The system shall define comprehensive search filtering capabilities:
- By file path patterns
- By element name patterns
- By element type
- By requirement governance metadata values
- By element content patterns
- By presence/absence of relations
- By presence/absence of attachments

Search result element evidence shall include effective requirement governance metadata when applicable.
Search result summaries shall include effective governance metadata counts for matched governance-bearing elements.

The system shall define custom element type tracking:
- Identify types not in standard categories
- Report custom types with counts

Search report kinds, search filter kinds, collect source types, coverage source types, and submodel concepts are defined by the Reqvire report ontology.

#### Metadata
  * type: requirement

#### Attachments
  * [Requirement Governance Metadata Specification](../Core/Specifications.md#requirement-governance-metadata-specification)
  * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * refinedBy: [Requirement Governance Metadata JSON Output Specification](Specifications.md#requirement-governance-metadata-json-output-specification)
  * refinedBy: [SearchFiltering](DesignDocuments/SearchFiltering.md#searchfiltering)
  * satisfiedBy: [filters.rs](../../../core/src/filters.rs)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
  * verifiedBy: [Search Command Tests](Verifications/ReportingVerifications.md#search-command-tests)
---

### Flexible Search Type Filtering

The system shall support filtering search results by multiple element types simultaneously to enable flexible querying across type categories.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Search Report Generator](#search-report-generator)
  * refinedBy: [Flexible Search Type Filtering Refinement Specification](Specifications.md#flexible-search-type-filtering-refinement-specification)
---

### Comma-Separated Type Filter Parsing

The system shall parse comma-separated element type values in the `--filter-type` flag, validating each type and applying OR logic to match elements.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Flexible Search Type Filtering](#flexible-search-type-filtering)
  * refinedBy: [Comma-Separated Type Filter Parsing Refinement Specification](Specifications.md#comma-separated-type-filter-parsing-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
---

### Provide Validation Reports

The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the System model structure.

#### Details
Validation shall be performed automatically when any command requires the parsed model, eliminating the need for a separate validation command. Commands that operate on raw files shall skip validation to allow operation on potentially invalid documents.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Validation Report Generator](#validation-report-generator)
  * derivedFrom: [Model Reports](#model-reports)
---

### Validation Report Generator

The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Provide Validation Reports](#provide-validation-reports)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

### Requirement Implementation Coverage Report

The system shall generate requirement implementation coverage reports that identify which requirements are implemented using direct `satisfiedBy` evidence and refinement-contract consumption evidence.

#### Details
The implementation coverage report shall provide:
- Total count of requirements in scope (`requirement` only; excludes direct capability rows)
- Count and percentage of implementation-covered requirements
- Count and percentage of implementation-uncovered requirements
- Coverage source classification for covered requirements:
  - direct `satisfiedBy` on the requirement
  - refinement-contract coverage through owned refinement elements attached by directly satisfied requirements
  - refinement-contract coverage when a requirement that owns refinement has directly satisfied derived descendants
- Detailed lists grouped by file and section, including coverage source and evidence references
- Output in both human-readable text and machine-readable JSON formats
- Coverage percentages shall be reported with at most 2 decimal places

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [Implementation Coverage Behavior](Behaviors.md#implementation-coverage-behavior)
  * refinedBy: [Implementation Coverage Output Structure Specification](Specifications.md#implementation-coverage-output-structure-specification)
  * refinedBy: [Requirement Implementation Coverage Logic Specification](Specifications.md#requirement-implementation-coverage-logic-specification)
  * satisfiedBy: [report_coverage.rs](../../../core/src/report_coverage.rs)
---

### Resources Report

The system shall provide a resources report showing all files referenced by the model through relations and attachments in text, JSON, and HTML formats.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [Resources Report Format Specification](Specifications.md#resources-report-format-specification)
  * verifiedBy: [Resources Report Verification](Verifications/ReportingVerifications.md#resources-report-verification)
---

### Verification Coverage Report

The system shall generate verification coverage reports focusing on leaf requirements, showing the percentage and details of verified and unverified requirements following clearly defined coverage philosophy.

#### Details
The verification coverage report shall provide:
- Total count of leaf requirements with breakdown by requirement type
- Count and percentage of verified leaf requirements (those with verifiedBy relations)
- Count and percentage of unverified leaf requirements
- Total count of verification artifacts with breakdown by verification type
- Count and percentage of satisfied test-verification artifacts
- Count and percentage of orphaned verification artifacts
- Detailed lists grouped by file and section
- Output in both human-readable text and machine-readable JSON formats

The report helps track verification completeness and identify gaps in requirement verification coverage.

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Roll-up Specification](../Processing/Specifications.md#verification-roll-up-specification)
  * [Verification Type Selection Guidelines](../Core/Specifications.md#verification-type-selection-guidelines)

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [Verification Coverage Philosophy Behavior](Behaviors.md#verification-coverage-philosophy-behavior)
  * refinedBy: [Verification Coverage Specification](../../Refinements.md#verification-coverage-specification)
---

### TraceFlow View Report Generation

The system shall generate a TraceFlow view page showing the verification traceability flow using an interactive D3.js Sankey diagram visualization. The view displays how capabilities are specified by requirements, how requirements flow to verifications, and how capabilities may be directly verified.

#### Metadata
  * type: requirement

#### Attachments
  * [Verification Trace Tree Construction](../Processing/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
  * refinedBy: [TraceFlowView](DesignDocuments/TraceFlowView.md#traceflowview)
  * refinedBy: [Verification Trace Diagram Specification](Specifications.md#verification-trace-diagram-specification)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * verifiedBy: [TraceFlow View Test](Verifications/ReportingVerifications.md#traceflow-view-test)
---

### Tracing Structural Changes

When tracing structural changes, the system shall analyze the System model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Metadata
  * type: requirement

#### Attachments
  * [Traceability Reporting Specification](../../Refinements.md#traceability-reporting-specification)

#### Relations
  * derive: [Change Impact Detection](../Processing/ChangeImpact.md#change-impact-detection)
  * derive: [Change Impact Semantic Contract](#change-impact-semantic-contract)
  * specify: [Trace Changes in System Model](../../Capabilities/RelationsAndImpact.md#trace-changes-in-system-model)
  * verifiedBy: [Structural Change Reports Verification](../Processing/Verifications/ChangeImpactVerifications.md#structural-change-reports-verification)
---

### Change Impact Semantic Contract

The system shall define SHACL constraints for change-impact analysis records, impact edges, semantic dependencies, and review-routing metadata.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Tracing Structural Changes](#tracing-structural-changes)
---
