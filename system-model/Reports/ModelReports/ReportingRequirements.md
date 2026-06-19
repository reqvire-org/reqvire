# Elements

### Model Reports

When requested the system shall provide human readable and machine readable System model reports with deterministic output and consistent ordering.

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Report Output Vocabulary Shape](../../Ontologies/ReportsAndQuery.md#report-output-vocabulary-shape)
  * definedBy: [Deterministic Output Specification](Specifications.md#deterministic-output-specification)
  * definedBy: [JSON Output Structure](Specifications.md#json-output-structure)
  * definedBy: [Markdown Report Style Specification](Specifications.md#markdown-report-style-specification)
  * definedBy: [Report Command Catalog Specification](Specifications.md#report-command-catalog-specification)
  * definedBy: [Text Output Formatting](Specifications.md#text-output-formatting)
  * definedBy: [Traceability Reporting Specification](Specifications.md#traceability-reporting-specification)
  * derive: [Interactive Mermaid Diagrams](DiagramGeneration.md#interactive-mermaid-diagrams)
  * derive: [Collect Capability and Requirement Context](#collect-capability-and-requirement-context)
  * derive: [JSON Element Size Estimate Exposure](#json-element-size-estimate-exposure)
  * derive: [Model Structure and Summaries](#model-structure-and-summaries)
  * derive: [Provide Validation Reports](#provide-validation-reports)
  * derive: [Requirement Implementation Coverage Report](#requirement-implementation-coverage-report)
  * derive: [Resources Report](#resources-report)
  * derive: [Verification Coverage Report](#verification-coverage-report)
  * specify: [Provide Reports](../ReportsAndQueryFeature.md#provide-reports)
---

### Collect Capability and Requirement Context

The system shall collect and consolidate context from a capability or requirement element, including directional capability and requirement traversal, authored concept references, requirement-owned `definedBy` targets, reused requirement contract contents, and source citations in text or JSON format.

#### Details
The system shall define:
- Content collection rules for elements, definedBy targets, and reused_contract_context
- Output format specifications for text and JSON modes
- Direction-based traversal over capability hierarchy, requirement hierarchy, and the `specify`/`specifiedBy` bridge where defined by the collect traversal specification

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Collect Content Specification](Specifications.md#collect-content-specification)
  * definedBy: [Collect Output Format Specification](Specifications.md#collect-output-format-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [report_collect.rs](../../../core/src/report_collect.rs)
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
  * definedBy: [JSON Element Size Estimate Output Specification](Specifications.md#json-element-size-estimate-output-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * verifiedBy: [JSON Element Size Estimate Output Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#json-element-size-estimate-output-verification)
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
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Containment Specification](../../ModelStructure/Specifications.md#containment-specification)
  * [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)

#### Relations
  * definedBy: [Containment View Report Contract Specification](Specifications.md#containment-view-report-contract-specification)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * verifiedBy: [Containment View Design Documents Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#containment-view-design-documents-test)
---

### Model Diagram Output Formats

System shall support Markdown, pure Mermaid, and JSON output formats.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * [Diagram Relation Filtering Specification](Specifications.md#diagram-relation-filtering-specification)

#### Relations
  * definedBy: [Model Diagram Output Formats Contract Specification](Specifications.md#model-diagram-output-formats-contract-specification)
  * derive: [Forward-Only Relation Traversal](#forward-only-relation-traversal)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * verifiedBy: [Model Command Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#model-command-verification)
---

### Forward-Only Relation Traversal

When filtering by root element, system shall traverse only forward relations down to leaf elements.

#### Details
Traversal behavior shall follow the associated behavior contract.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Forward-Only Relation Traversal Behavior](Behaviors.md#forward-only-relation-traversal-behavior)
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#model-command-verification)
---

### Reverse Relation Traversal

The system shall support reverse relation traversal for model views, following defined rules in Reverse Relation Traversal Behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Reverse Relation Traversal Behavior](Behaviors.md#reverse-relation-traversal-behavior)
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * verifiedBy: [Reverse Model Traversal Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#reverse-model-traversal-test)
---

### Start Element Type Filtering

The system shall support filtering starting elements by type for model traversal, following defined rules in Start Element Type Filter Behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Start Element Type Filter Behavior](Behaviors.md#start-element-type-filter-behavior)
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * verifiedBy: [Start Type Filter Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#start-type-filter-test)
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

Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Requirement Submodels Report Specification](Specifications.md#requirement-submodels-report-specification)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [report_submodels.rs](../../../core/src/report_submodels.rs)
  * verifiedBy: [Submodels Report Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#submodels-report-verification)
---

### Search Report Generator

The system shall implement a search report generator with comprehensive filtering and element type tracking.

#### Details
The search report must include file-level, section-level, and element-level information.

Search JSON shall expose parsed semantic model fields when present. Ontology elements expose `ontology`; semantic-contract elements expose `semantic_contract`. Short mode may omit these parsed semantic fields.

The system shall define comprehensive search filtering capabilities:
- By file path patterns
- By element name patterns
- By element type
- By requirement governance metadata values
- By element content patterns
- By presence/absence of relations
- By presence/absence of reused_contract_context

Search result element evidence shall include effective requirement governance metadata when applicable.
Search result summaries shall include effective governance metadata counts for matched governance-bearing elements.

The system shall define custom element type tracking:
- Identify types not in standard categories
- Report custom types with counts

Search report kinds, search filter kinds, collect source types, coverage source types, and submodel concepts are defined by the Reqvire report ontology.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)

#### Relations
  * definedBy: [SearchFiltering](SearchFiltering.md#searchfiltering)
  * definedBy: [Requirement Governance Metadata JSON Output Specification](Specifications.md#requirement-governance-metadata-json-output-specification)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [filters.rs](../../../core/src/filters.rs)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
  * verifiedBy: [Search Command Tests](../../Verifications/Reports/ModelReports/ReportingVerifications.md#search-command-tests)
---

### Flexible Search Type Filtering

The system shall support filtering search results by multiple element types simultaneously to enable flexible querying across type categories.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Flexible Search Type Filtering Contract Specification](Specifications.md#flexible-search-type-filtering-contract-specification)
  * derivedFrom: [Search Report Generator](#search-report-generator)
---

### Comma-Separated Type Filter Parsing

The system shall parse comma-separated element type values in the `--filter-type` flag, validating each type and applying OR logic to match elements.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Comma-Separated Type Filter Parsing Contract Specification](Specifications.md#comma-separated-type-filter-parsing-contract-specification)
  * derivedFrom: [Flexible Search Type Filtering](#flexible-search-type-filtering)
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

The system shall generate requirement implementation coverage reports that identify which requirements are implemented using direct `satisfiedBy` evidence and contract consumption evidence.

#### Details
The implementation coverage report shall provide:
- Total count of requirements in scope (`requirement` only; excludes direct capability rows)
- Count and percentage of implementation-covered requirements
- Count and percentage of implementation-uncovered requirements
- Coverage source classification for covered requirements:
  - direct `satisfiedBy` on the requirement
  - contract coverage through owned contract elements reused by directly satisfied requirements
  - contract coverage when a requirement that owns contract has directly satisfied derived descendants
- Detailed lists grouped by file and section, including coverage source and evidence references
- Output in both human-readable text and machine-readable JSON formats
- Coverage percentages shall be reported with at most 2 decimal places

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Implementation Coverage Behavior](Behaviors.md#implementation-coverage-behavior)
  * definedBy: [Implementation Coverage Output Structure Specification](Specifications.md#implementation-coverage-output-structure-specification)
  * definedBy: [Requirement Implementation Coverage Logic Specification](Specifications.md#requirement-implementation-coverage-logic-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [report_coverage.rs](../../../core/src/report_coverage.rs)
---

### Resources Report

The system shall provide a resources report showing all files referenced by the model through relations and reused_contract_context in text, JSON, and Explorer views.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Resources Report Format Specification](Specifications.md#resources-report-format-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * verifiedBy: [Resources Report Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#resources-report-verification)
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
- Verification objectives shall be visible in model/search reports but excluded from verification coverage denominators because they organize verification intent rather than verifying requirements directly.
- Count and percentage of orphaned verification artifacts
- Detailed lists grouped by file and section
- Output in both human-readable text and machine-readable JSON formats

The report helps track verification completeness and identify gaps in requirement verification coverage.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Verification Roll-up Specification](../../Verification/Traceability/Specifications.md#verification-roll-up-specification)
  * [Verification Type Selection Guidelines](../../ModelStructure/Specifications.md#verification-type-selection-guidelines)

#### Relations
  * definedBy: [Verification Coverage Philosophy Behavior](Behaviors.md#verification-coverage-philosophy-behavior)
  * definedBy: [Verification Coverage Specification](Specifications.md#verification-coverage-specification)
  * derivedFrom: [Model Reports](#model-reports)
---

### TraceFlow View Report Generation

The system shall seed TraceFlow/Traces SPA route data showing verification traceability flow for an interactive D3.js Sankey diagram visualization. The routed view displays how capabilities are specified by requirements, how requirements flow to verifications, and how capabilities may be directly verified through served Explorer state.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
  * satisfiedBy: [store.rs](../../../core/src/html/store.rs)
---

### Ontology Projection Subgraph Materialization

The system shall materialize generated ontology construct facts as a subgraph of the existing in-memory RDF projection so semantic exports and the Ontologies Explorer consume the same ontology construct facts.

#### Details
The ontology projection subgraph shall:
- Extend the existing in-memory RDF projection used by full semantic export; it is not a separate database, persistent store, or route-local model.
- Be reused to the reusable `SemanticIndex` as structured generated projection data so semantic export, JSON-LD export, and Explorer rendering share one authoritative projection source.
- Be generated from authored ontology and semantic-contract RDF quads during semantic index processing or immediately after parsing.
- Materialize direct-authored OWL/RDFS/SHACL constructs into Reqvire ontology projection facts without changing authored Markdown ontology or semantic-contract blocks.
- Preserve source element identifier, source name, source file, source line, source block kind, construct subject, construct object, construct members, construct property, ordered sequence index when relevant, symbol code point, rendered symbol, and derivation mode.
- Derive normalized SHACL slot/facet projection records from node-shape target classes and property-shape paths so WebInterface ontology views and semantic exports can share the same source-backed semantic evidence.
- Include constructs for property domain/range, subclass or inclusion, class membership, disjointness, equivalence, inverse properties, property chains, property characteristics, restrictions, intersections, unions, complement or difference-style expressions when authored, and SHACL shape overlays when present.
- Distinguish direct-authored projection from inferred projection. Direct-authored projection is in scope; OWL reasoning, SHACL-AF rule execution, and inferred materialization require separate inference requirements before they can contribute generated facts.
- Feed generated facts into `reqvire ontologies --full`, `reqvire ontologies --full --jsonld`, and the Ontologies Explorer through the same in-memory projection context. The default `reqvire ontologies` and served `ontologies.ttl` artifact include generated ontology document declarations plus authored ontology/SHACL collection output, but do not include generated ontology projection facts unless full semantic output is requested.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Ontology Collection Output Specification](Specifications.md#ontology-collection-output-specification)

#### Relations
  * definedBy: [Ontology Projection Subgraph Materialization Specification](Specifications.md#ontology-projection-subgraph-materialization-specification)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### Ontology and Shapes Collection

The system shall collect ontology `#### Ontology` and semantic-contract `#### Shapes` RDF blocks from the graph registry into a reusable semantic export context, and shall optionally project Reqvire model context and generated ontology construct facts into the same RDF export.

#### Details
The default collection shall expose authored ontology RDF content and semantic-contract SHACL RDF content without changing the Markdown model as the source of truth.

When full semantic model export is requested, the collection shall also emit RDF triples for Reqvire model elements, element metadata, relation-family projection facts, requirement-to-capability specification relations, requirement-to-semantic-contract constraint relations, semantic-contract-to-ontology use relations, ontology hierarchy relations, concept references, ontology term declarations, semantic-contract shape references, and generated ontology projection facts materialized from direct-authored OWL/RDFS/SHACL constructs.

The collection shall preserve source element identifiers, source file paths, section kind, and line numbers so CLI, Explorer rendering, and downstream semantic tooling can cite the model source of each RDF block.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../../Ontologies/ReportsAndQuery.md#semantic-export-projection-shape)
  * definedBy: [Ontology Collection Output Specification](Specifications.md#ontology-collection-output-specification)
  * derive: [OWL Reserved Vocabulary Recognition](#owl-reserved-vocabulary-recognition)
  * satisfiedBy: [explorer_runtime.rs](../../../core/src/explorer_runtime.rs)
  * satisfiedBy: [semantic_contract.rs](../../../core/src/semantic_contract.rs)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### Local External Ontology Sources

The system shall allow ontology elements to declare local external ontology source files that provide imported vocabulary namespaces for validation and optional semantic export materialization.

#### Details
External ontology sources shall be declared with repeatable `#### External Ontology` sections on ontology elements. Each section shall define `prefix`, `namespace`, `resource`, `source`, and an optional `format` value. Supported source formats shall include Turtle/TTL, RDF/XML, and JSON-LD.

The `source` path shall be local and resolved like a model path; Reqvire shall not fetch network ontology sources during validation or export.

External ontology source triples shall be parsed into the semantic index before validating ontology and semantic-contract references. Terms declared by the local source shall be available to the declaring ontology element, its ontology descendants, and semantic contracts that use that ontology context. Imported terms shall remain marked as external declarations and shall not be promoted to authored Reqvire ontology terms.

Turtle blocks remain explicit. External source sections do not inject prefixes, ontology declarations, imports, or semantic triples into authored ontology or SHACL blocks.

Default semantic export and MCP semantic metadata shall include authored ontology and SHACL content only. `reqvire ontologies --include-external` and MCP `include_external: true` shall include parsed external source triples, external declarations, and external vocabulary metadata. `reqvire ontologies --full --include-external` and MCP full semantic query with `include_external: true` shall include authored triples, external source triples, Reqvire model context, and generated ontology projection facts.

Standard OWL reserved vocabulary and built-in datatype IRIs remain recognized by the fixed reserved vocabulary registry and do not require `#### External Ontology` declarations.

#### Concept References
  * External ontology source: https://www.reqvire.org/ontology#ExternalOntologySource
  * External ontology prefix: https://www.reqvire.org/ontology#externalOntologyPrefix
  * External ontology namespace: https://www.reqvire.org/ontology#externalOntologyNamespace
  * External ontology resource: https://www.reqvire.org/ontology#externalOntologyResource
  * External ontology source path: https://www.reqvire.org/ontology#externalOntologySourcePath
  * External ontology format: https://www.reqvire.org/ontology#externalOntologyFormat

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Local External Ontology Source Specification](Specifications.md#local-external-ontology-source-specification)
  * derivedFrom: [Ontology and Shapes Collection](#ontology-and-shapes-collection)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### OWL Reserved Vocabulary Recognition

The system shall recognize fixed OWL reserved vocabulary IRIs in ontology and semantic-contract RDF positions without requiring local external ontology source declarations for those IRIs.

#### Details
Reserved vocabulary recognition shall be fixed-list based over expanded IRIs, not prefix-name or namespace-prefix matching. Turtle prefix names are aliases: `xsd:string` and `xs:string` are equivalent when both expand to `http://www.w3.org/2001/XMLSchema#string`.

Reqvire shall treat known reserved vocabulary IRIs as model-valid references in positions where their OWL role is valid without requiring `#### External Ontology` sections for those namespaces.

Built-in datatype IRIs are one reserved vocabulary subset and shall be accepted in datatype positions such as ontology datatype property ranges and SHACL `sh:datatype` values. SHACL `sh:datatype` positions shall also accept the Reqvire-supported XML Schema datatype-position subset, including date/time datatypes such as `xsd:date`, without requiring authored ontology declarations for those standard IRIs.

Custom IRIs outside the reserved vocabulary registry remain subject to normal authored or external ontology resolution when term existence validation applies.

#### Concept References
  * OWL reserved vocabulary registry: https://www.reqvire.org/ontology#OwlReservedVocabularyRegistry
  * OWL reserved vocabulary term: https://www.reqvire.org/ontology#OwlReservedVocabularyTerm
  * OWL built-in datatype: https://www.reqvire.org/ontology#OwlBuiltInDatatype

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [OWL Reserved Vocabulary Recognition Specification](Specifications.md#owl-reserved-vocabulary-recognition-specification)
  * derivedFrom: [Ontology and Shapes Collection](#ontology-and-shapes-collection)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### Semantic Relation Family Projection

The system shall materialize ontology-defined relation-family projection facts as part of full semantic model export so semantic search can query relation meaning independently of raw Markdown relation token direction.

#### Details
- Full semantic model export shall treat authored model relations and reused contract context edges as first-class semantic relation records.
- Full semantic model export shall emit deterministic `reqvire:ModelRelation` resources with source, target, relation type, and target identifier facts.
- Full semantic model export shall emit canonical forward and inverse normalized predicates for ontology-defined relation families without removing raw authored relation predicates.
- The relation-family projection shall be an in-memory semantic export projection, not a source Markdown mutation and not an MCP-owned materialization step.
- The projection shall follow the ontology-authored `reqvire:RelationRule` semantics and the relation-family construct-query contract.

#### Concept References
  * Relation family construct query: https://www.reqvire.org/ontology#RelationFamilyConstructQuery
  * Model relation: https://www.reqvire.org/ontology#ModelRelation

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../../Ontologies/ReportsAndQuery.md#semantic-export-projection-shape)
  * definedBy: [Semantic Relation Family Projection Specification](Specifications.md#semantic-relation-family-projection-specification)
  * satisfiedBy: [semantic_contract.rs](../../../core/src/semantic_contract.rs)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Semantic Query Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-query-tools-verification)
---

### Tracing Structural Changes

When tracing structural changes, the system shall analyze the System model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Traceability Reporting Specification](Specifications.md#traceability-reporting-specification)

#### Relations
  * constrainedBy: [Change Impact Analysis Shape](../../Ontologies/RelationsAndImpact.md#change-impact-analysis-shape)
  * derive: [Change Impact Detection](../../Processing/ChangeImpact/ChangeImpactRequirements.md#change-impact-detection)
  * specify: [Trace Changes in System Model](../../Processing/RelationsAndImpactFeature.md#trace-changes-in-system-model)
  * verifiedBy: [Structural Change Reports Verification](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#structural-change-reports-verification)
---
