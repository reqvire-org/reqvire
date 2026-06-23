# Elements

### Concept Relation Projection Materialization

The system shall materialize normalized SKOS concept-relation facts from native concept Markdown relations so concept exports, full semantic graph output, Project Store thesaurus data, Ontologies Concepts-layer rendering, and MCP concept tools consume the same concept-relation projection.

#### Details
The concept relation projection shall:
- Treat authored `broader` and `narrower` concept relations as inverse aliases for one taxonomy edge.
- Treat authored `related`, `exactMatch`, and `closeMatch` concept relations as symmetric concept association or mapping edges.
- Accept either direction in Markdown authoring, and accept consistent reciprocal authoring without reporting a duplicate relation error.
- Materialize both explicit SKOS directions for taxonomy: `skos:broader` from narrower concept to broader concept and `skos:narrower` from broader concept to narrower concept.
- Materialize reciprocal SKOS concept association or mapping facts where the SKOS predicate is symmetric, while consumers may canonicalize those reciprocal facts to one visual edge.
- Keep raw authored Markdown relation evidence separate from generated normalized concept-relation projection facts.
- Not write generated inverse or reciprocal SKOS facts back to authored Markdown.
- Feed generated concept-relation facts into `reqvire semantic concepts`, `reqvire semantic graph --full`, full JSON-LD output, served Project Store `thesaurus` projection, Ontologies Concepts-layer graph data, and MCP concept/thesaurus tools.
- Remain separate from ontology construct projection because SKOS concept taxonomy and mappings are conceptual thesaurus facts, not OWL/RDFS/SHACL construct classifications.

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Concept Relation Projection Specification](Specifications.md#concept-relation-projection-specification)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
---

### External Vocabulary Exposure Policy

The system shall expose only constructed used external vocabulary content through external-inclusive semantic output surfaces.

#### Details
Semantic output surfaces shall not expose raw full external ontology files. When `include_external` is requested, Reqvire shall expose only the used external vocabulary content selected and constructed from internal external dependency inputs. Unused external dependency facts shall remain internal and shall not appear in CLI ontology output, MCP semantic ontology output, MCP vocabulary output, MCP SPARQL graphs, or Explorer ontology views.

Default semantic export and MCP semantic metadata shall keep authored ontology, SHACL shape, SKOS concept, and combined graph concerns as separate surfaces. `reqvire semantic ontologies --include-external`, `reqvire semantic graph --include-external`, and MCP `include_external: true` shall include used external subset triples, used external declarations, and used external vocabulary metadata where that layer supports external materialization. `reqvire semantic graph --full --include-external` and MCP full semantic query with `include_external: true` shall include authored triples, the used external subset, Reqvire model context, and generated ontology projection facts.

No CLI, MCP, Explorer, website, or assistant-facing contract shall specify a public full third-party ontology dump mode.

#### Concept References
  * [Used external ontology subset](../../Thesaurus/Thesaurus.md#used-external-ontology-subset)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [External Vocabulary Exposure Policy Specification](Specifications.md#external-vocabulary-exposure-policy-specification)
  * derivedFrom: [External Vocabulary Description Construction](../../Semantics/SemanticModelRequirements.md#external-vocabulary-description-construction)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### Model Reports

When requested the system shall provide human readable and machine readable System model reports with deterministic output and consistent ordering.

#### Metadata
  * type: requirement

#### Relations
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
- Content collection rules for elements, definedBy targets, and contract_bindings
- Output format specifications for text and JSON modes
- Direction-based traversal over capability hierarchy, requirement hierarchy, and the `specify`/`specifiedBy` bridge where defined by the collect traversal specification

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Collect Content Specification](Specifications.md#collect-content-specification)
  * definedBy: [Collect Output Format Specification](Specifications.md#collect-output-format-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [report_collect.rs](../../../crates/reqvire-core/src/report_collect.rs)
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
  * satisfiedBy: [element.rs](../../../crates/reqvire-core/src/element.rs)
  * satisfiedBy: [report_model.rs](../../../crates/reqvire-core/src/report_model.rs)
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

#### Contract Bindings
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

#### Contract Bindings
  * [Mermaid Diagram Generation Specification](Specifications.md#mermaid-diagram-generation-specification)
  * [Diagram Relation Filtering Specification](Specifications.md#diagram-relation-filtering-specification)

#### Relations
  * definedBy: [Model Diagram Output Formats Contract Specification](Specifications.md#model-diagram-output-formats-contract-specification)
  * derive: [Forward-Only Relation Traversal](#forward-only-relation-traversal)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [report_model.rs](../../../crates/reqvire-core/src/report_model.rs)
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
  * satisfiedBy: [diagrams.rs](../../../crates/reqvire-core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#model-command-verification)
---

### Reverse Relation Traversal

The system shall support reverse relation traversal for model views, following defined rules in Reverse Relation Traversal Behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Reverse Relation Traversal Behavior](Behaviors.md#reverse-relation-traversal-behavior)
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * satisfiedBy: [report_model.rs](../../../crates/reqvire-core/src/report_model.rs)
  * verifiedBy: [Reverse Model Traversal Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#reverse-model-traversal-test)
---

### Start Element Type Filtering

The system shall support filtering starting elements by type for model traversal, following defined rules in Start Element Type Filter Behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Start Element Type Filter Behavior](Behaviors.md#start-element-type-filter-behavior)
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * satisfiedBy: [report_model.rs](../../../crates/reqvire-core/src/report_model.rs)
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
  * satisfiedBy: [report_submodels.rs](../../../crates/reqvire-core/src/report_submodels.rs)
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
- By presence/absence of contract_bindings

Search result element evidence shall include effective requirement governance metadata when applicable.
Search result summaries shall include effective governance metadata counts for matched governance-bearing elements.

The system shall define custom element type tracking:
- Identify types not in standard categories
- Report custom types with counts

Search filter tokens, collect source tokens, coverage source tokens, submodel report concepts, and report-command kind tokens are defined by the reporting, search, coverage, and interface contracts that emit or consume them.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)

#### Relations
  * definedBy: [SearchFiltering](SearchFiltering.md#searchfiltering)
  * definedBy: [Requirement Governance Metadata JSON Output Specification](Specifications.md#requirement-governance-metadata-json-output-specification)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [filters.rs](../../../crates/reqvire-core/src/filters.rs)
  * satisfiedBy: [search.rs](../../../crates/reqvire-core/src/search.rs)
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
  * satisfiedBy: [cli.rs](../../../crates/reqvire-cli/src/cli.rs)
  * satisfiedBy: [search.rs](../../../crates/reqvire-core/src/search.rs)
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
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/model.rs)
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
  * satisfiedBy: [report_coverage.rs](../../../crates/reqvire-core/src/report_coverage.rs)
---

### Resources Report

The system shall provide a resources report showing all files referenced by the model through relations and contract_bindings in text, JSON, and Explorer views.

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

#### Contract Bindings
  * [Verification Roll-up Specification](../../Verification/Traceability/Specifications.md#verification-roll-up-specification)
  * [Verification Type Selection Guidelines](../../ModelStructure/Specifications.md#verification-type-selection-guidelines)

#### Relations
  * definedBy: [Verification Coverage Philosophy Behavior](Behaviors.md#verification-coverage-philosophy-behavior)
  * definedBy: [Verification Coverage Specification](Specifications.md#verification-coverage-specification)
  * derivedFrom: [Model Reports](#model-reports)
---

### Traces View Report Generation

The system shall seed Explorer Traces SPA route data showing verification traceability from concrete verifications through verified requirements and owning capability context. The routed view displays grouped trace rows and per-verification Mermaid roll-up diagrams from served Project Store state.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
---

### Ontology Collection Output

The system shall expose the semantic model core context as CLI, MCP, serve-time, and Explorer ontology output without making reporting the owner of ontology or semantic-contract source semantics.

#### Details
Ontology collection output consumes the semantic context from [Ontology and Shapes Collection](../../Semantics/SemanticModelRequirements.md#ontology-and-shapes-collection), including generated ontology document declarations, generated term definition links, authored ontology RDF, semantic-contract SHACL RDF, optional full model context, generated projection facts, and optional used external vocabulary subset content.

The output contract owns serialization choices, command/API flags, Project Store payload shape, and Explorer artifact inclusion. It does not own source resolution, semantic-contract reachability, reserved vocabulary recognition, or built-in external source policy.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ontology Collection Output Specification](Specifications.md#ontology-collection-output-specification)
  * derivedFrom: [Ontology and Shapes Collection](../../Semantics/SemanticModelRequirements.md#ontology-and-shapes-collection)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### Ontology Projection Subgraph Materialization

The system shall materialize generated ontology construct facts as a subgraph of the existing in-memory RDF projection so semantic exports and the Ontologies Explorer consume the same ontology construct facts.

#### Details
The ontology projection subgraph shall:
- Extend the existing in-memory RDF projection used by full semantic export; it is not a separate database, persistent store, or route-local model.
- Be bound to the reusable `SemanticIndex` as structured generated projection data so semantic export, JSON-LD export, and Explorer rendering share one authoritative projection source.
- Be generated from authored ontology and semantic-contract RDF quads during semantic index processing or immediately after parsing.
- Materialize o-kernel construct classifications into Reqvire ontology projection facts without changing authored Markdown ontology or semantic-contract blocks.
- Preserve source element identifier, source name, source file, source line, source block kind, construct subject, construct object, construct members, construct property, ordered sequence index when relevant, symbol code point, rendered symbol, and derivation mode.
- Derive normalized SHACL slot/facet projection records from node-shape target classes and property-shape paths so WebInterface ontology views and semantic exports can share the same source-backed semantic evidence.
- Distinguish direct-authored projection from inferred projection. Direct-authored projection is in scope; OWL reasoning, SHACL-AF rule execution, and inferred materialization require separate inference requirements before they can contribute generated facts.
- Feed generated facts into `reqvire semantic graph --full`, `reqvire semantic graph --full --jsonld`, and the Ontologies Explorer through the same in-memory projection context. The served `ontologies.ttl` artifact includes generated ontology document declarations plus authored ontology/SHACL collection output, but does not include generated ontology projection facts unless full semantic output is requested.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Ontology Collection Output Specification](Specifications.md#ontology-collection-output-specification)
  * [Ontology Construct Classification Specification](../../Architecture/OntologyKernelSpecifications.md#ontology-construct-classification-specification)

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Ontology Projection Subgraph Materialization Specification](Specifications.md#ontology-projection-subgraph-materialization-specification)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### Semantic Relation Family Projection

The system shall materialize ontology-defined relation-family projection facts as part of full semantic model export so semantic search can query relation meaning independently of raw Markdown relation token direction.

#### Details
- Full semantic model export shall treat authored model relations and contract bindings edges as first-class semantic relation records.
- Full semantic model export shall emit deterministic `reqvire:ModelRelation` resources with source, target, relation type, and target identifier facts.
- Full semantic model export shall emit canonical forward and inverse normalized predicates for ontology-defined relation families without removing raw authored relation predicates.
- The relation-family projection shall be an in-memory semantic export projection, not a source Markdown mutation and not an MCP-owned materialization step.
- The projection shall follow the ontology-authored `reqvire:RelationRule` semantics and the relation-family construct-query contract.

#### Concept References
  * [Relation family construct query](../../Thesaurus/Thesaurus.md#relation-family-construct-query)
  * [Model relation](../../Thesaurus/Thesaurus.md#model-relation)

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Semantic Relation Family Projection Specification](Specifications.md#semantic-relation-family-projection-specification)
  * satisfiedBy: [semantic_contract.rs](../../../crates/reqvire-core/src/semantic_contract.rs)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Semantic Query Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-semantic-query-tools-verification)
---

### Tracing Structural Changes

When tracing structural changes, the system shall analyze the System model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Traceability Reporting Specification](Specifications.md#traceability-reporting-specification)

#### Relations
  * constrainedBy: [Change Impact Analysis Shape](../../Ontologies/RelationsAndImpact.md#change-impact-analysis-shape)
  * derive: [Change Impact Detection](../../Processing/ChangeImpact/ChangeImpactRequirements.md#change-impact-detection)
  * specify: [Trace Changes in System Model](../../Processing/RelationsAndImpactFeature.md#trace-changes-in-system-model)
  * verifiedBy: [Structural Change Reports Verification](../../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#structural-change-reports-verification)
---
