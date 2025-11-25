# Elements

### Containment View Report Generation

The system shall generate containment view reports showing the physical hierarchical structure of the model (folders → files → elements) in multiple output formats including Mermaid diagrams, JSON, and HTML export integration.

#### Attachments
  * [ContainmentView.md](DesignDocuments/ContainmentView.md)

#### Relations
  * derivedFrom: [Containment View Report](#containment-view-report)
---

### Containment View Design Documents

The system shall include design documents (files in DesignDocuments folders) in the containment view, grouped by their containing folder and displayed alongside specification elements.

#### Details
- Design documents are non-specification markdown files in DesignDocuments folders
- They shall be shown in the containment hierarchy under their parent folder
- Each design document shall display its filename
- Design documents shall be visually distinguished from specification elements
- Clicking a design document in diagrams shall navigate to the file

#### Relations
  * derivedFrom: [Containment View Report Generation](#containment-view-report-generation)
  * satisfiedBy: [containment.rs](../../../core/src/containment.rs)
  * verifiedBy: [Containment View Design Documents Test](Verifications/ReportingVerifications.md#containment-view-design-documents-test)
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
  * derivedFrom: [CLI Model Diagram Command](../../Interfaces/CLI.md#cli-model-diagram-command)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](Verifications/ReportingVerifications.md#model-command-verification)
---

### Search Report Generator

The system shall implement a search report generator that produces comprehensive element searches with filtering, supporting both full and abbreviated output modes.

#### Details
The search report must include:

**File-level Information:**
- File path and name
- Number of sections per file
- Number of elements per file
- Page content (frontmatter content before first section header) - omitted in short mode

**Section-level Information:**
- Section name and hierarchy
- Number of elements per section - omitted in short mode
- Section content (content between section header and first element, excluding generated diagrams) - omitted in short mode

**Element Information:**
- Element identifier, name, type, and section
- Element content - omitted in short mode
- Complete list of relations with targets and types
- Complete list of attachments as strings (file paths and element identifiers) - omitted in short mode
  - File path attachments displayed as relative paths (e.g., `"path/to/file.pdf"`)
  - Element identifier attachments displayed as full identifiers (e.g., `"specifications/File.md#refinement-element"`)

**Global Counts** (omitted in short mode):
- Total files and elements counts
- Requirements by type: map of requirement types to counts (e.g., `{"user-requirement": 5, "system-requirement": 10}`)
- Verifications by type: map of verification types to counts (e.g., `{"test-verification": 8, "analysis-verification": 2}`)
- Refinements by type: map of refinement types to counts (e.g., `{"behavior": 3, "constraint": 1}`)
- Other types: map of any custom element types not in standard categories

**Output Formats:**
- Human-readable text format with hierarchical display
- Human-readable abbreviated text format (with --short flag): one-line per element showing `[type] identifier - name`
- JSON format for programmatic processing
- JSON abbreviated format (with --short flag): omits content, page_content, attachments, element_count, total_elements, global_counters

The system must support comprehensive filtering by file path, element name, type, element content, page content, and relation presence. All filters are applied conjunctively.

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
---

### Verification Coverage Report Generator

The system shall provide a verification coverage report generator that analyzes leaf requirements verification status, test-verification satisfaction status, and orphaned verifications to produce coverage metrics and detailed reports.

#### Details
The coverage report generator must:
- Identify all leaf requirements (requirements without forward relations to other requirements) in the model
- Determine leaf requirement verification status based on presence of verifiedBy relations
- Identify all verification elements in the model with breakdown by verification type
- Determine test-verification satisfaction status based on presence of satisfiedBy relations
- Identify orphaned verifications (verification elements without any verify relations to requirements)
- Calculate coverage percentages: (verified_leaf_requirements/total_leaf_requirements * 100), (satisfied_test_verifications/total_test_verifications * 100), and (orphaned_verifications/total_verifications * 100)
- Group results by file and section for organization
- Support both human-readable text and machine-readable JSON output formats

The report structure shall include:
- Summary section with leaf requirements, test-verification, and orphaned verification counts and percentages
- Verified leaf requirements section grouped by file and section
- Unverified leaf requirements section with details (flagged for attention)
- Satisfied test-verification elements section grouped by file and section
- Unsatisfied test-verification elements section with details (flagged for attention)
- Orphaned verifications section with details (flagged for attention as they may be redundant or incorrectly configured)
- Analysis, inspection, and demonstration verification elements are considered satisfied by default

#### Relations
  * satisfiedBy: [report_coverage.rs](../../../core/src/report_coverage.rs)
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
  * derivedFrom: [Search Report Generator](#search-report-generator)
---

### Custom Element Type Tracking

The system SHALL track and display custom element types (any type not in the standard categories) in model summary reports, providing counts for each custom type in both text and JSON output formats.

#### Details
The custom element type tracking feature must:

**Custom Type Definition:**
- Identify any element type that is not one of the standard types: requirement, user-requirement, verification, test-verification, analysis-verification, inspection-verification, or demonstration-verification
- Track custom types separately from standard element type counters
- Store custom types in a HashMap with type name as key and count as value

**Text Output:**
- Display custom element types in the "Element Types" section of the summary
- Format: `Custom (type-name): count`
- Sort custom types alphabetically by type name
- Only display custom types section when at least one custom type exists

**JSON Output:**
- Include custom element types in global_counters as `custom_element_types` field
- Structure: `{"custom_element_types": {"type-name": count, ...}}`
- Skip serialization of the field when no custom types exist (using `skip_serializing_if`)
- Maintain alphabetical sorting of type names in JSON output

**Exclusions:**
- Standard requirement types (requirement, user-requirement) SHALL NOT be counted as custom
- Standard verification types (verification, test-verification, analysis-verification, inspection-verification, demonstration-verification) SHALL NOT be counted as custom
- File-type elements SHALL NOT be counted as custom types

#### Relations
  * derivedFrom: [Search Report Generator](#search-report-generator)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
---

### Model Diagram Output Formats

System shall support markdown and JSON output formats.

#### Details
- Markdown format shall include embedded Mermaid diagram with model structure
- Markdown shall show hierarchical structure (folders > files > sections > elements)
- JSON format shall use structured data with folders, files, sections, elements, relations, and attachments
- Both formats shall represent the same filtered or complete model data
- Element attachments shall be included as an array of strings in both formats (file paths and element identifiers)

#### Relations
  * derivedFrom: [CLI Model Diagram Command](../../Interfaces/CLI.md#cli-model-diagram-command)
  * satisfiedBy: [diagrams.rs](../../../core/src/diagrams.rs)
  * verifiedBy: [Model Command Verification](Verifications/ReportingVerifications.md#model-command-verification)
---

### Search Fine Grained Filtering

The system shall implement comprehensive fine-grained filtering for the search report generator following the specifications below.

#### Attachments
  * [SearchFiltering.md](DesignDocuments/SearchFiltering.md)

#### Relations
  * satisfiedBy: [filters.rs](../../../core/src/filters.rs)
  * satisfiedBy: [search.rs](../../../core/src/search.rs)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
  * derivedFrom: [Search Report Generator](#search-report-generator)
  * verifiedBy: [Search Command Tests](Verifications/ReportingVerifications.md#search-command-tests)
---

### Tracing Structural Changes

When tracing structural changes, the system shall analyze the System model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Trace Changes in System Model](../../UserStories.md#trace-changes-in-system-model)
---

### Validation Report Generator

The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Relations
  * derivedFrom: [Provide Validation Reports](#provide-validation-reports)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

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
  * derivedFrom: [Provide Reports](../../UserStories.md#provide-reports)
---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the System model, including counts and types of connections also supporting json and cypher output.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
---

### Containment View Report

The system shall provide a containment view report that displays the physical hierarchical structure of the model showing the containment relationships between folders, files, and elements.

#### Details
The containment view shows the **physical organization** of the model, complementing the existing relation-centric model view that shows derivedFrom/verifiedBy relationships.

The containment hierarchy represents:
- **Root folder** → **Subfolders** → **Files** → **Elements**
- Sections are skipped in this view (elements are shown directly under files)

This view helps users:
- Understand model organization and file structure
- Navigate the physical layout of specifications
- Identify where elements are located
- Visualize the containment structure in diagrams

The containment view must support multiple output formats (text, JSON, Mermaid diagram) and be included in HTML export.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Structure and Summaries](#model-structure-and-summaries)
---

### Provide Validation Reports

The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the System model structure.

#### Details
Validation shall be performed automatically when any command requires the parsed model, eliminating the need for a separate validation command. Commands that operate on raw files shall skip validation to allow operation on potentially invalid documents.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Align with Industry Standards](../../UserStories.md#align-with-industry-standards)
  * derivedFrom: [Model Reports](#model-reports)
---

### Verification Coverage Report

The system shall generate verification coverage reports focusing on leaf requirements (requirements that do not have forward relations to any other requirement), showing the percentage and details of verified and unverified leaf requirements, including breakdowns by file, section, and verification type.

#### Details
The verification coverage report shall provide:
- Total count of leaf requirements with breakdown by requirement type
- Count and percentage of verified leaf requirements (those with verifiedBy relations pointing to existing verification artifacts)
- Count and percentage of unverified leaf requirements
- Total count of verification artifacts with breakdown by verification type (test, analysis, inspection, demonstration)
- Count and percentage of satisfied test-verification artifacts (those with satisfiedBy relations pointing to existing test implementations)
- Count and percentage of orphaned verification artifacts (those without any verify relations to requirements)
- Detailed list of verified leaf requirements grouped by file and section
- Detailed list of unverified leaf requirements with impact analysis
- Detailed list of orphaned verifications (flagged for attention as they may be redundant or incorrectly configured)
- Output in both human-readable text and machine-readable JSON formats

The report helps track verification completeness and identify gaps in requirement verification coverage, supporting quality assurance and compliance activities.

**Coverage Philosophy:**
- **Leaf requirements** (requirements that don't derive other requirements) MUST be verified
- **Parent/intermediate requirements** MAY be verified but it's not a hard requirement as they might be covered in verification of leaf requirements
- One verification may verify multiple leaf requirements (N:1 relationship)
- The change impact analysis system propagates changes from parent requirements down to leaf requirements and their verifications
- System engineers/architects are responsible for ensuring verification scopes are broad enough to cover parent requirements when there's no dedicated parent verification
- AI systems can help create comprehensive verification scopes and prevent verification overlap

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
---

### Verification Upward Traceability

The system shall provide upward traceability visualization from verifications to root requirements, showing the complete requirement hierarchy and indicating which requirements are directly verified.

#### Details
**Used for identifying redundant verifications**: When a verification directly verifies both a leaf requirement and its parent requirement, this creates a redundant relation that adds noise into the model and may be removed from the parent - verifying the leaf requirement is sufficient since it traces upward to the parent. This keeps verification placement at the most specific level.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Verification Coverage Report](#verification-coverage-report)
---
