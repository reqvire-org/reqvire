# Elements

### Model Reports

When requested the system shall provide human readable and machine readable System model reports.

#### Details
All generated reports shall produce deterministic output with consistent ordering to enable reliable testing, version control, and reproducible builds.

The system shall ensure deterministic output by:

1. **Element Ordering**: Elements shall be sorted by identifier before iteration to ensure consistent processing order across all operations
2. **Relation Ordering**: Relations within each element shall be sorted by relation type name and then by target identifier before rendering
3. **Section Ordering**: Sections within files shall be sorted alphabetically when order is not semantically significant
4. **File Ordering**: Files within folders shall be sorted alphabetically

This determinism ensures that:
- Running the same operation multiple times produces byte-identical output
- Automated tests can reliably compare expected and actual outputs using simple diff tools without special normalization
- Version control diffs are meaningful and reflect actual changes rather than random ordering variations
- Continuous integration pipelines produce consistent, reproducible results

This requirement applies to all report operations, including:
- Model summary reporting - Summary reports in text and JSON formats
- Verification tracing - Upward traceability trees from verifications to requirements
- Coverage reporting - Verification coverage analysis
- Change impact analysis - Reports showing propagation of changes
- Validation reporting - Model validation error reports
- Linting - Model quality issue reports

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Complete Model Structure Visualization](DiagramGeneration.md#complete-model-structure-visualization)
  * derive: [Interactive Mermaid Diagrams](DiagramGeneration.md#interactive-mermaid-diagrams)
  * derive: [Model Visualization and Exploration](DiagramGeneration.md#model-visualization-and-exploration)
  * derive: [Collect Content from Requirement Chain](#collect-content-from-requirement-chain)
  * derive: [Model Structure and Summaries](#model-structure-and-summaries)
  * derive: [Provide Validation Reports](#provide-validation-reports)
  * derive: [Resources Report](#resources-report)
  * derive: [Verification Coverage Report](#verification-coverage-report)
  * derivedFrom: [Provide Reports](../../UserStories.md#provide-reports)
---

### Collect Content from Requirement Chain

The system shall collect and consolidate all content from a requirement element and its ancestors via derivedFrom relations, including attachment contents, and output with source citations in text or JSON format.

#### Details
The system shall define:
- Content collection rules for elements and attachments
- Output format specifications for text and JSON modes

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [report_collect.rs](../../../core/src/report_collect.rs)
  * satisfiedBy: [Collect Content Specification](Specifications.md#collect-content-specification)
  * satisfiedBy: [Collect Output Format Specification](Specifications.md#collect-output-format-specification)
---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the System model, including counts and types of connections also supporting json and cypher output.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Containment View Report](#containment-view-report)
  * derive: [Model Diagram Output Formats](#model-diagram-output-formats)
  * derive: [Search Report Generator](#search-report-generator)
  * derivedFrom: [Model Reports](#model-reports)
---

### Containment View Report

The system shall generate containment view reports showing the physical hierarchical structure of the model.

#### Details
The containment view shows the physical organization of the model:
- Root folder → Subfolders → Files → Elements
- Sections skipped (elements shown directly under files)

The system shall generate reports in multiple formats:
- Mermaid diagrams for visualization
- JSON for programmatic access
- HTML export integration

The system shall include design documents:
- Files in DesignDocuments folders displayed alongside specifications
- Design documents visually distinguished from specification elements
- Clickable navigation to document files

#### Metadata
  * type: user-requirement

#### Attachments
  * [ContainmentView.md](DesignDocuments/ContainmentView.md)

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * verifiedBy: [Containment View Design Documents Test](Verifications/ReportingVerifications.md#containment-view-design-documents-test)
---

### Model Diagram Output Formats

System shall support markdown and JSON output formats.

#### Details
- Markdown format shall include embedded Mermaid diagram with model structure
- Markdown shall show hierarchical structure using containment subgraphs (folders > files > elements)
- Mermaid diagrams shall use folder and file subgraphs to visually group elements by their physical location
- JSON format shall use structured data with folders, files, sections, elements, relations, and attachments
- Both formats shall represent the same filtered or complete model data
- Element attachments shall be included as an array of strings in both formats (file paths and element identifiers)

#### Relations
  * derive: [Forward-Only Relation Traversal](#forward-only-relation-traversal)
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](Verifications/ReportingVerifications.md#model-command-verification)
---

### Forward-Only Relation Traversal

When filtering by root element, system shall traverse only forward relations down to leaf elements.

#### Details
- Shall follow only forward relations (derive, satisfiedBy, verifiedBy, trace)
- Shall start from specified root element (looked up by name)
- Shall recursively traverse outgoing relations to leaf elements
- Shall NOT traverse backward (no bidirectional traversal)
- Unfiltered diagrams (no --from) shall show complete model with all elements

#### Relations
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](Verifications/ReportingVerifications.md#model-command-verification)
---

### Reverse Relation Traversal

The system shall support reverse relation traversal for model views, following defined rules in Reverse Relation Traversal Behavior.

#### Relations
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * satisfiedBy: [Reverse Relation Traversal Behavior](Behaviors.md#reverse-relation-traversal-behavior)
  * verifiedBy: [Reverse Model Traversal Test](Verifications/ReportingVerifications.md#reverse-model-traversal-test)
---

### Start Element Type Filtering

The system shall support filtering starting elements by type for model traversal, following defined rules in Start Element Type Filter Behavior.

#### Relations
  * derivedFrom: [Model Diagram Output Formats](#model-diagram-output-formats)
  * satisfiedBy: [report_model.rs](../../../core/src/report_model.rs)
  * satisfiedBy: [Start Element Type Filter Behavior](Behaviors.md#start-element-type-filter-behavior)
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

#### Attachments
  * [SearchFiltering.md](DesignDocuments/SearchFiltering.md)
  * [Supported Element Types Specification](../Core/Specifications.md#supported-element-types-specification)
  * [JSON Output Structure](Specifications.md#json-output-structure)
  * [Text Output Formatting](Specifications.md#text-output-formatting)

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [filters.rs](../../../core/src/filters.rs)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
  * verifiedBy: [Search Command Tests](Verifications/ReportingVerifications.md#search-command-tests)
---

### Provide Validation Reports

The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the System model structure.

#### Details
Validation shall be performed automatically when any command requires the parsed model, eliminating the need for a separate validation command. Commands that operate on raw files shall skip validation to allow operation on potentially invalid documents.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Integrated Validation](../Core/Validation.md#integrated-validation)
  * derive: [Validation Report Generator](#validation-report-generator)
  * derivedFrom: [Model Reports](#model-reports)
  * derivedFrom: [Align with Industry Standards](../../UserStories.md#align-with-industry-standards)
---

### Validation Report Generator

The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Relations
  * derive: [Validation Error Handling](../Core/Validation.md#validation-error-handling)
  * derivedFrom: [Provide Validation Reports](#provide-validation-reports)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

### Resources Report

The system shall provide a resources report showing all files referenced by the model through relations and attachments.

#### Details
**Report Structure:**
- Two sections: Relations and Attachments
- Each section lists files alphabetically by path
- Each file shows referencing elements with links

**Relations Section:**
- Files from InternalPath relation targets (satisfiedBy, trace, etc.)
- Shows relation type and source element for each reference
- Sorted by relation type, then by element identifier

**Attachments Section:**
- Files from FilePath attachment targets
- Shows source element for each reference
- Sorted by element identifier

**Output Formats:**
- Text/Markdown: Human-readable with markdown links
- JSON: Structured data for programmatic use

**HTML Export:**
- Resources view available in HTML export with navigation link
- Shows complete list of referenced files with element traceability

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [CLI Resources Command](../../Interfaces/CLI.md#cli-resources-command)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [report_resources.rs](../../../core/src/report_resources.rs)
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
  * type: user-requirement

#### Attachments
  * [Verification Type Categories Specification](../Core/Specifications.md#verification-type-categories-specification)
  * [JSON Output Structure](Specifications.md#json-output-structure)
  * [Text Output Formatting](Specifications.md#text-output-formatting)

#### Relations
  * derive: [Verification Upward Traceability](#verification-upward-traceability)
  * derivedFrom: [Model Reports](#model-reports)
  * satisfiedBy: [report_coverage.rs](../../../core/src/report_coverage.rs)
  * satisfiedBy: [Verification Coverage Philosophy Behavior](Behaviors.md#verification-coverage-philosophy-behavior)
---

### Verification Upward Traceability

The system shall provide upward traceability visualization from verifications to root requirements, showing the complete requirement hierarchy and indicating which requirements are directly verified.

#### Details
**Used for identifying redundant verifications**: When a verification directly verifies both a leaf requirement and its parent requirement, this creates a redundant relation that adds noise into the model and may be removed from the parent - verifying the leaf requirement is sufficient since it traces upward to the parent. This keeps verification placement at the most specific level.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [TraceFlow View Report Generation](#traceflow-view-report-generation)
  * derive: [Verification Roll-up Strategy](../Processing/VerificationTraces.md#verification-roll-up-strategy)
  * derive: [Verification Trace Builder](../Processing/VerificationTraces.md#verification-trace-builder)
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
---

### TraceFlow View Report Generation

The system shall generate a TraceFlow view page showing the verification traceability flow using an interactive D3.js Sankey diagram visualization. The view displays how requirements flow from user requirements through system requirements to verifications.

#### Attachments
  * [TraceFlowView.md](DesignDocuments/TraceFlowView.md)

#### Relations
  * derivedFrom: [Verification Upward Traceability](#verification-upward-traceability)
  * satisfiedBy: [export.rs](../../../core/src/export.rs)
  * verifiedBy: [TraceFlow View Test](Verifications/ReportingVerifications.md#traceflow-view-test)
---

### Tracing Structural Changes

When tracing structural changes, the system shall analyze the System model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Change Impact Detection](../Processing/ChangeImpact.md#change-impact-detection)
  * derivedFrom: [Trace Changes in System Model](../../UserStories.md#trace-changes-in-system-model)
  * verifiedBy: [Structural Change Reports Verification](../Processing/Verifications/ChangeImpactVerifications.md#structural-change-reports-verification)
---
