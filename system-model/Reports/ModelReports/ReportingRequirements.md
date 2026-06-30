# Elements

### Concept Relation Projection Materialization

The system shall materialize normalized SKOS concept-relation facts from native concept Markdown relations so semantic exports and downstream concept consumers use the same concept-relation projection.

#### Details
Detailed inverse, symmetric, reciprocal, non-mutation, consumer, and ontology-projection separation rules shall follow the associated specification.

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
Detailed raw-source exclusion, used-subset exposure, layer behavior, API visibility, metadata, and no-full-dump rules shall follow the associated specification.

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
  * derive: [Trace Diagram Projection Data](DiagramGeneration.md#trace-diagram-projection-data)
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
  * satisfiedBy: [collect.rs](../../../crates/reqvire-core/src/report/collect.rs)
---

### JSON Element Size Estimate Exposure

The system shall expose element-level `size_estimate` records in JSON model evidence outputs when the model was built with size estimates enabled.

#### Details
Detailed JSON-only inclusion, enabled-only behavior, nested element target handling, and aggregate-summary exclusion rules shall follow the associated output specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [JSON Element Size Estimate Output Specification](Specifications.md#json-element-size-estimate-output-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [element.rs](../../../crates/reqvire-core/src/element.rs)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
  * verifiedBy: [JSON Element Size Estimate Output Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#json-element-size-estimate-output-verification)
---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the System model, including counts and types of connections, ontology-root and capability-root starting contexts, and JSON output.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Containment View Report](#containment-view-report)
  * derive: [Model JSON Output Format](#model-json-output-format)
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
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)

#### Relations
  * definedBy: [Containment View Report Contract Specification](Specifications.md#containment-view-report-contract-specification)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * verifiedBy: [Containment View Design Documents Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#containment-view-design-documents-test)
---

### Model JSON Output Format

System shall support JSON model output as the canonical CLI and operation result format.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Model JSON Output Format Contract Specification](Specifications.md#model-json-output-format-contract-specification)
  * derive: [Forward-Only Relation Traversal](#forward-only-relation-traversal)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
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
  * derivedFrom: [Model JSON Output Format](#model-json-output-format)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
  * verifiedBy: [Model Command Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#model-command-verification)
---

### Reverse Relation Traversal

The system shall support reverse relation traversal for model views, following defined rules in Reverse Relation Traversal Behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Reverse Relation Traversal Behavior](Behaviors.md#reverse-relation-traversal-behavior)
  * derivedFrom: [Model JSON Output Format](#model-json-output-format)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
  * verifiedBy: [Reverse Model Traversal Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#reverse-model-traversal-test)
---

### Start Element Type Filtering

The system shall support filtering starting elements by type for model traversal, following defined rules in Start Element Type Filter Behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Start Element Type Filter Behavior](Behaviors.md#start-element-type-filter-behavior)
  * derivedFrom: [Model JSON Output Format](#model-json-output-format)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
  * verifiedBy: [Start Type Filter Test](../../Verifications/Reports/ModelReports/ReportingVerifications.md#start-type-filter-test)
---

### Requirement Submodels Report

The system shall provide a submodels report that identifies independent capability-root subgraphs and cross-submodel requirement couplings.

#### Details
Detailed scope resolution, filtered capability/requirement behavior, empty-submodel behavior, coupling detection, and deterministic summary rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Requirement Submodels Report Specification](Specifications.md#requirement-submodels-report-specification)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [submodels.rs](../../../crates/reqvire-core/src/report/submodels.rs)
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
Detailed scope, source classification, evidence, output, grouping, and percentage formatting rules shall follow the associated behavior and specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Implementation Coverage Output Structure Specification](Specifications.md#implementation-coverage-output-structure-specification)
  * definedBy: [Requirement Implementation Coverage Logic Specification](Specifications.md#requirement-implementation-coverage-logic-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [coverage.rs](../../../crates/reqvire-core/src/report/coverage.rs)
---

### Resources Report

The system shall provide a resources report inventorying InternalPath relation targets that resolve to existing workspace-root-relative eligible Git-worktree files and contract_bindings targets that resolve to model element identifiers.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Resources Report Format Specification](Specifications.md#resources-report-format-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [resources.rs](../../../crates/reqvire-core/src/report/resources.rs)
  * verifiedBy: [Resources Report Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#resources-report-verification)
---

### Verification Coverage Report

The system shall generate verification coverage reports focusing on leaf requirements, showing the percentage and details of verified and unverified requirements following clearly defined coverage philosophy.

#### Details
Detailed leaf-requirement scope, verification artifact handling, objective exclusion, grouping, output, and percentage rules shall follow the associated behavior and specification.

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

### Trace Projection Data Generation

The system shall materialize trace projection data showing verification traceability from concrete verifications through verified requirements and owning capability context, including grouped trace rows and per-verification roll-up diagram data for downstream report consumers.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
---

### Ontology Collection Output

The system shall expose semantic model core context as reusable ontology output for command, API, served-artifact, and interactive consumers without making reporting the owner of ontology or semantic-contract source semantics.

#### Details
Detailed semantic context consumption, serialization choices, command/API flags, consumer payload shape, artifact inclusion, and source-semantics boundaries shall follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ontology Collection Output Specification](Specifications.md#ontology-collection-output-specification)
  * derivedFrom: [Prefixed Turtle Semantic Export](../../Semantics/SemanticModelRequirements.md#prefixed-turtle-semantic-export)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### Ontology Projection Subgraph Materialization

The system shall materialize generated ontology construct facts as a subgraph of the existing in-memory RDF projection so semantic exports and downstream ontology consumers use the same ontology construct facts.

#### Details
Detailed projection storage, generation timing, source/provenance fields, SHACL slot/facet records, direct-authored scope, identifier namespace ownership, and export/consumer rules shall follow the associated specification.

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
Detailed relation-record, canonicalization, construct-query, implementation-boundary, non-mutation, and contract_bindings projection rules shall follow the associated specification.

#### Concept References
  * [Relation family construct query](../../Thesaurus/Thesaurus.md#relation-family-construct-query)
  * [Model relation](../../Thesaurus/Thesaurus.md#model-relation)

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Semantic Relation Family Projection Specification](Specifications.md#semantic-relation-family-projection-specification)
  * satisfiedBy: [export.rs](../../../crates/reqvire-core/src/semantic_contract/export.rs)
  * satisfiedBy: [vocabulary.rs](../../../crates/reqvire-core/src/semantic_contract/vocabulary.rs)
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
