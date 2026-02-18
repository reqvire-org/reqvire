# Elements

### Model Reports

When requested the system shall provide human readable and machine readable System model reports with deterministic output and consistent ordering.

#### Metadata
  * type: user-requirement

#### Attachments
  * [Traceability Reporting Specification](../../Refinements.md#traceability-reporting-specification)

#### Relations
  * derive: [Interactive Mermaid Diagrams](DiagramGeneration.md#interactive-mermaid-diagrams)
  * derive: [Collect Content from Requirement Chain](#collect-content-from-requirement-chain)
  * derive: [Model Structure and Summaries](#model-structure-and-summaries)
  * derive: [Provide Validation Reports](#provide-validation-reports)
  * derive: [Resources Report](#resources-report)
  * derive: [Requirement Implementation Coverage Report](#requirement-implementation-coverage-report)
  * derive: [Verification Coverage Report](#verification-coverage-report)
  * derivedFrom: [Provide Reports](../../UserStories.md#provide-reports)
  * refinedBy: [Deterministic Output Specification](Specifications.md#deterministic-output-specification)
  * refinedBy: [Diff Output Format Specification](Specifications.md#diff-output-format-specification)
  * refinedBy: [Error Message Format Specification](Specifications.md#error-message-format-specification)
  * refinedBy: [JSON Output Structure](Specifications.md#json-output-structure)
  * refinedBy: [Markdown Report Style Specification](Specifications.md#markdown-report-style-specification)
  * refinedBy: [Text Output Formatting](Specifications.md#text-output-formatting)
---

### Collect Content from Requirement Chain

The system shall collect and consolidate all content from a requirement element and its related requirements via derivedFrom relations (upstream to ancestors) or derive relations (downstream to descendants), including refinedBy targets (refinement elements) and attachment contents, and output with source citations in text or JSON format.

#### Details
The system shall define:
- Content collection rules for elements, refinedBy targets, and attachments
- Output format specifications for text and JSON modes
- Direction-based traversal: upstream (ancestors via derivedFrom) or downstream (descendants via derive)

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [Collect Content Specification](Specifications.md#collect-content-specification)
  * refinedBy: [Collect Output Format Specification](Specifications.md#collect-output-format-specification)
  * satisfiedBy: [report_collect.rs](../../../core/src/report_collect.rs)
---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the System model, including counts and types of connections also supporting json output.

#### Metadata
  * type: user-requirement

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
  * type: user-requirement

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

System shall support markdown and JSON output formats.

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

### Search Report Generator

The system shall implement a search report generator with comprehensive filtering and element type tracking.

#### Details
The search report must include file-level, section-level, and element-level information.

The system shall define comprehensive search filtering capabilities:
- By file path patterns
- By element name patterns
- By element type
- By element content patterns
- By presence/absence of relations
- By presence/absence of attachments

The system shall define custom element type tracking:
- Identify types not in standard categories
- Report custom types with counts

#### Metadata
  * type: requirement

#### Attachments
  * [Supported Element Types Specification](../../Refinements.md#supported-element-types-specification)
  * [Resources Report Format Specification](Specifications.md#resources-report-format-specification)

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
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
  * type: user-requirement

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
  * type: user-requirement

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

### Resources Report

The system shall provide a resources report showing all files referenced by the model through relations and attachments in text, JSON, and HTML formats.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [Resources Report Format Specification](Specifications.md#resources-report-format-specification)
  * verifiedBy: [Resources Report Verification](Verifications/ReportingVerifications.md#resources-report-verification)
---

### Requirement Submodels Report

The system shall provide a submodels report that identifies independent requirement hierarchies (by top roots) and cross-submodel requirement couplings.

#### Details
The report shall support:
- Full model view listing all discovered submodels and cross-submodel couplings
- Filtered view scoped to one requirement subtree by requirement name
- In filtered view, the selected requirement is a scope boundary and SHALL NOT be counted as a reported submodel entry

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
  * type: user-requirement

#### Attachments
  * [Verification Coverage Specification](../../Refinements.md#verification-coverage-specification)
  * [Verification Roll-up Specification](../Processing/Specifications.md#verification-roll-up-specification)
  * [Verification Type Selection Guidelines](../Core/Specifications.md#verification-type-selection-guidelines)

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [Verification Coverage Philosophy Behavior](Behaviors.md#verification-coverage-philosophy-behavior)
---

### Requirement Implementation Coverage Report

The system shall generate requirement implementation coverage reports that identify which requirements are implemented using direct `satisfiedBy` evidence and refinement-contract consumption evidence.

#### Details
The implementation coverage report shall provide:
- Total count of requirements in scope (`requirement` only; excludes `user-requirement`)
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

#### Attachments

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * refinedBy: [Requirement Implementation Coverage Logic Specification](Specifications.md#requirement-implementation-coverage-logic-specification)
  * refinedBy: [Implementation Coverage Output Structure Specification](Specifications.md#implementation-coverage-output-structure-specification)
  * refinedBy: [Implementation Coverage Behavior](Behaviors.md#implementation-coverage-behavior)
  * satisfiedBy: [report_coverage.rs](../../../core/src/report_coverage.rs)
---

### TraceFlow View Report Generation

The system shall generate a TraceFlow view page showing the verification traceability flow using an interactive D3.js Sankey diagram visualization. The view displays how requirements flow from user requirements through system requirements to verifications.

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
  * type: user-requirement

#### Attachments
  * [Traceability Reporting Specification](../../Refinements.md#traceability-reporting-specification)

#### Relations
  * derive: [Change Impact Detection](../Processing/ChangeImpact.md#change-impact-detection)
  * derivedFrom: [Trace Changes in System Model](../../UserStories.md#trace-changes-in-system-model)
  * verifiedBy: [Structural Change Reports Verification](../Processing/Verifications/ChangeImpactVerifications.md#structural-change-reports-verification)
---
