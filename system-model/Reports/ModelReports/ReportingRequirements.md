# Elements

### Concept Relation Projection Materialization

The system shall materialize normalized SKOS concept-relation facts from native concept Markdown relations so semantic exports and downstream concept consumers use the same concept-relation projection.

#### Details
Detailed inverse, symmetric, reciprocal, non-mutation, consumer, and ontology-projection separation rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [index.rs](../../../crates/reqvire-core/src/semantic_contract/index.rs)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
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
  * satisfiedBy: [semantic_store.rs](../../../crates/reqvire-core/src/semantic_store.rs)
  * satisfiedBy: [export.rs](../../../crates/reqvire-core/src/semantic_contract/export.rs)
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
  * derive: [Resources Report](#resources-report)
  * specify: [Provide Reports](../ReportsAndQueryFeature.md#provide-reports)
---

### Collect Capability and Requirement Context

The system SHALL collect capability and requirement context with source citations in text or JSON format.

#### Details
WHEN a caller selects a traversal direction, the system SHALL collect the applicable capability and requirement hierarchy according to the owned collection contract.

WHEN collected elements own or consume contracts, the system SHALL include their contract content and distinguish ownership, Contract Bindings, and Contract References according to the owned and bound output contracts.

WHEN collected elements author concept references, the system SHALL include their concept context.

WHEN a referenced contract is reached through multiple collection paths, the system SHALL include its content and source citation once.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)
  * [Contract Reference Evidence Projection Specification](Specifications.md#contract-reference-evidence-projection-specification)

#### Relations
  * definedBy: [Collect Content Specification](Specifications.md#collect-content-specification)
  * definedBy: [Collect Output Format Specification](Specifications.md#collect-output-format-specification)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [collect.rs](../../../crates/reqvire-core/src/report/collect.rs)
---

### Coverage Reports

When coverage is requested, the system SHALL report implementation coverage and verification coverage as distinct assessments of modeled requirements.

#### Details
The system SHALL report capability implementation and verification coverage through the corresponding requirement and capability roll-up rules.

When a capability scope is selected, the system SHALL apply that scope consistently to both coverage assessments.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
---

### Requirement Implementation Coverage Report

When implementation coverage is requested, the system SHALL report requirement implementation coverage according to the referenced implementation coverage logic specification.

#### Details
The system SHALL present implementation coverage scope, classifications, supporting evidence, grouping, and percentages according to its output specification.

When reporting an individual requirement, the system SHALL include its aggregate verification-leaf and implementation-terminal counts evaluated over the full model.

#### Metadata
  * type: requirement

#### Contract References
  * [Verification Coverage Specification](Specifications.md#verification-coverage-specification)
  * [Requirement Implementation Coverage Logic Specification](../../Implementation/Traceability/Specifications.md#requirement-implementation-coverage-logic-specification)

#### Relations
  * definedBy: [Implementation Coverage Output Structure Specification](Specifications.md#implementation-coverage-output-structure-specification)
  * derivedFrom: [Coverage Reports](#coverage-reports)
  * satisfiedBy: [coverage.rs](../../../crates/reqvire-core/src/report/coverage.rs)
---

### Scoped Coverage Reporting

When a capability scope is selected, the system SHALL report verification and implementation coverage for that capability subtree while preserving each included requirement's whole-model coverage classification and supporting evidence for the same validated model snapshot.

#### Metadata
  * type: requirement

#### Contract References
  * [Capability Model Structure Specification](../../ModelStructure/Specifications.md#capability-model-structure-specification)
  * [Requirement Submodels Report Specification](Specifications.md#requirement-submodels-report-specification)
  * [Requirement Implementation Coverage Logic Specification](../../Implementation/Traceability/Specifications.md#requirement-implementation-coverage-logic-specification)
  * [Verification Coverage Specification](Specifications.md#verification-coverage-specification)
  * [Implementation Coverage Output Structure Specification](Specifications.md#implementation-coverage-output-structure-specification)

#### Relations
  * derivedFrom: [Coverage Reports](#coverage-reports)
  * satisfiedBy: [mod.rs](../../../crates/reqvire-core/src/operations/mod.rs)
  * satisfiedBy: [coverage.rs](../../../crates/reqvire-core/src/report/coverage.rs)
  * satisfiedBy: [submodels.rs](../../../crates/reqvire-core/src/report/submodels.rs)
---

### Verification Coverage Report

The system shall generate verification coverage reports focusing on leaf requirements, showing the percentage and details of verified and unverified requirements following clearly defined coverage philosophy.

#### Details
Detailed leaf-requirement scope, verification artifact handling, objective exclusion, grouping, output, and percentage rules shall follow the associated behavior and specification.

#### Metadata
  * type: requirement

#### Contract References
  * [Verification Roll-up Specification](../../Verification/Traceability/Specifications.md#verification-roll-up-specification)
  * [Verification Type Selection Guidelines](../../ModelStructure/Specifications.md#verification-type-selection-guidelines)

#### Relations
  * satisfiedBy: [coverage.rs](../../../crates/reqvire-core/src/report/coverage.rs)
  * definedBy: [Verification Coverage Philosophy Behavior](Behaviors.md#verification-coverage-philosophy-behavior)
  * definedBy: [Verification Coverage Specification](Specifications.md#verification-coverage-specification)
  * derivedFrom: [Coverage Reports](#coverage-reports)
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

#### Contract References
  * [Containment Specification](../../ModelStructure/Specifications.md#containment-specification)
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)

#### Relations
  * satisfiedBy: [containment.rs](../../../crates/reqvire-core/src/containment.rs)
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

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)

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

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)

#### Relations
  * definedBy: [Requirement Submodels Report Specification](Specifications.md#requirement-submodels-report-specification)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [submodels.rs](../../../crates/reqvire-core/src/report/submodels.rs)
  * verifiedBy: [Submodels Report Verification](../../Verifications/Reports/ModelReports/ReportingVerifications.md#submodels-report-verification)
---

### Search Report Generator

The system SHALL implement a search report generator with comprehensive filtering and element type tracking.

#### Details
The system SHALL include file-level, section-level, and element-level information in search reports.

WHEN full search results contain ontology or semantic-contract elements, the system SHALL expose their parsed semantic model fields.

WHEN callers supply file, name, type, governance metadata, content, relation-presence, or contract-dependency filters, the system SHALL select elements according to all active search filters.

WHEN a caller supplies contract-reference presence or target filters, the system SHALL select requirements by their authored Contract References and combine these filters with the other active search filters.

WHEN a caller supplies an invalid contract-reference target pattern, the system SHALL report the invalid filter.

WHEN matched elements carry governance metadata, the system SHALL include their effective metadata and corresponding summary counts.

WHEN matched elements have custom types, the system SHALL identify those types and report their counts.

The system SHALL use the filtering and output vocabulary defined by its owned and referenced contracts.

#### Metadata
  * type: requirement

#### Contract References
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [Supported Element Types Specification](../../ModelStructure/Specifications.md#supported-element-types-specification)
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)

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

### Trace Projection Data Generation

The system shall materialize trace projection data showing verification traceability from concrete verifications through verified requirements and owning capability context, including grouped trace rows and per-verification roll-up diagram data for downstream report consumers.

#### Metadata
  * type: requirement

#### Contract References
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Relations
  * satisfiedBy: [verification_trace.rs](../../../crates/reqvire-core/src/verification_trace.rs)
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
  * derivedFrom: [Model Reports](#model-reports)
---

### Ontology Collection Output

The system shall expose semantic model core context as reusable ontology output for command, API, served-artifact, and interactive consumers without making reporting the owner of ontology or semantic-contract source semantics.

#### Details
Detailed semantic context consumption, serialization choices, command/API flags, consumer payload shape, artifact inclusion, and source-semantics boundaries shall follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [export.rs](../../../crates/reqvire-core/src/semantic_contract/export.rs)
  * satisfiedBy: [semantic_tools.rs](../../../crates/reqvire-core/src/tool_interface/semantic_tools.rs)
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

#### Contract References
  * [Ontology Collection Output Specification](Specifications.md#ontology-collection-output-specification)
  * [Ontology Construct Classification Specification](../../Architecture/OntologyKernelSpecifications.md#ontology-construct-classification-specification)

#### Relations
  * satisfiedBy: [export.rs](../../../crates/reqvire-core/src/semantic_contract/export.rs)
  * satisfiedBy: [semantic_store.rs](../../../crates/reqvire-core/src/semantic_store.rs)
  * constrainedBy: [Semantic Export Projection Shape](../../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Ontology Projection Subgraph Materialization Specification](Specifications.md#ontology-projection-subgraph-materialization-specification)
  * specify: [Semantic Model Export](../ReportsAndQueryFeature.md#semantic-model-export)
  * verifiedBy: [CLI Ontologies Command Verification](../../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### Semantic Relation Family Projection

The system SHALL materialize ontology-defined relation-family projection facts as part of semantic model export.

#### Details
WHEN authored relations or contract dependencies are exported, the system SHALL preserve their source-target pairing and apply the canonical direction and predicates defined by the relation-family contracts.

WHEN Contract References are exported, the system SHALL expose their distinct content-dependency facts according to the bound evidence projection contract.

#### Concept References
  * [Relation family construct query](../../Thesaurus/Thesaurus.md#relation-family-construct-query)
  * [Model relation](../../Thesaurus/Thesaurus.md#model-relation)

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Contract Reference Evidence Projection Specification](Specifications.md#contract-reference-evidence-projection-specification)

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

### Contract Reference Evidence Projection

When publishing element evidence, the system SHALL identify Contract References separately from Contract Bindings.

#### Details
When collecting requirement context, the system SHALL include the content of explicitly referenced contracts.

When exporting semantic model facts, the system SHALL expose Contract References with their distinct forward and inverse predicates.

When rendering element details, the system SHALL present referenced contracts as navigable dependencies for change-impact review.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)

#### Relations
  * satisfiedBy: [store.rs](../../../crates/reqvire-core/src/html/store.rs)
  * satisfiedBy: [search.rs](../../../crates/reqvire-core/src/search.rs)
  * satisfiedBy: [containment.rs](../../../crates/reqvire-core/src/containment.rs)
  * derivedFrom: [Model Reports](#model-reports)
  * definedBy: [Contract Reference Evidence Projection Specification](Specifications.md#contract-reference-evidence-projection-specification)
  * satisfiedBy: [model.rs](../../../crates/reqvire-core/src/report/model.rs)
  * satisfiedBy: [collect.rs](../../../crates/reqvire-core/src/report/collect.rs)
  * satisfiedBy: [export.rs](../../../crates/reqvire-core/src/semantic_contract/export.rs)
---

### Coverage Element Scope Selection

When a coverage operation receives an explicit capability selector, the system SHALL resolve its exact name or canonical identifier through the shared element-selection contract before projecting the report.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Existing Element Selection Specification](../../ModelStructure/ElementSelection.md#existing-element-selection-specification)

#### Relations
  * satisfiedBy: [coverage.rs](../../../crates/reqvire-core/src/report/coverage.rs)
  * derivedFrom: [Scoped Coverage Reporting](#scoped-coverage-reporting)
  * verifiedBy: [Existing Element Selection Verification](../../Verifications/ModelStructure/ElementSelectionVerifications.md#existing-element-selection-verification)
---
