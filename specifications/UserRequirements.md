# User Requirements

## Generate Diagrams
### Interactive Mermaid Diagrams

The system shall produce visual representations of relationships within the MBSE model in the form of Mermaid diagrams, enabling users to explore relations and understand dependencies and their impact.

#### Details
Diagrams must be broken into several diagrams using following logic:
 * requirements_file_name/'## section name'
   * all requirements inside are 1 diagram
   * if requirements documents doesn't have '##' paragraphs then requirements file name is used only
   * external related resources box must be a link to actual resource

Color code for rendering diagrams:
 * red for requirement
 * yellow for resources which satisfies requirement
 * green for verifiction which verifies requirement
 * light blue for box representing another diagram/category with requirments where linked requirement or resource exist.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Generate Diagrams](UserStories.md#generate-diagrams)
---

### Remove Generated Diagrams

The system shall provide functionality to remove all generated Mermaid diagrams from the model, allowing users to clean up generated artifacts when needed.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Generate Diagrams](UserStories.md#generate-diagrams)
---

### Complete Model Structure Visualization

The system shall provide visualization of the complete model structure showing the containment hierarchy of folders, files, sections, and elements with their relationships.

#### Details
The visualization shall:
- Show the complete model structure using nested containment boxes (folders > files > sections > elements)
- Display elements with their relations connecting them inside their containing sections
- Use consistent visual styling similar to existing mermaid diagrams
- Support interactive pan and zoom for large models
- Be generated alongside other model artifacts during export

The visualization helps users understand the overall model organization, navigate complex structures, and identify structural patterns across the entire model.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Generate Diagrams](UserStories.md#generate-diagrams)
---

### Model Visualization and Exploration

Users shall be able to generate and view model structure diagrams from any starting point of the model.

#### Details
- Users shall be able to generate complete model structure diagrams showing all elements and relations
- Users shall be able to focus on specific parts of the model by filtering from any element as a root of the tree
- Output shall be available in human-readable (markdown) and machine-processable (JSON) formats

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Generate Diagrams](UserStories.md#generate-diagrams)
---

## Aligning Design with Code
### Code Traceability

The system shall support code traceability by using structured comments to link code implementations to corresponding requirements in the MBSE model.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Aligning Design with Code](UserStories.md#aligning-design-with-code)
---

### Suggest Code Refactoring

The system shall suggest code refactoring opportunities to better align with the structure and relationships in the MBSE model.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Aligning Design with Code](UserStories.md#aligning-design-with-code)
---

## Validating Structures
### Enhanced Validation Error Reporting

The system shall provide comprehensive validation messages that include file paths and line numbers when available, to help users quickly locate and fix model integrity and structure issues in their MBSE specifications.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
---

### Model Formatting

The system shall provide formatting capabilities to normalize and standardize MBSE models for consistency and readability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
---

### Model Linting

The system shall provide model linting capabilities to analyze model quality and detect issues in requirements relations.

#### Details
The linting capability should:
- Identify issues with model relations that may need attention
- Distinguish between issues that can be automatically fixed and those requiring human judgment
- Provide clear categorization of findings to help users understand what actions are needed
- Support both reporting mode and automatic fixing mode
- Allow filtering to focus on specific categories of issues

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
---

### Formatting Output

The system shall display formatting changes suggestion in similar manner as git diffs.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
---

### Replace Absolute Links with Relative Links

The system shall replace absolute links with relative links, where applicable and contextually appropriate, to conform to repository standards and enhance portability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * verifiedBy: [Format Command Requirements Verification](Verifications/Misc.md#format-command-requirements-verification)
---

### Format Consistency Enforcement

The system shall provide formatting capability to ensure consistent formatting in requirements documents.

#### Details
  * Trimming excess whitespace after element names and relation identifiers
  * Normalizing to exactly two newlines before subsections (e.g., "#### Details")
  * Automatically inserting separator lines ("---") between elements if not already present
  * Normalizing consecutive separators to single separators
  * Ensuring consistent indentation in relation lists (2-space format)
  * Normalizing relation entries to proper 2-space indentation format
  * Displaying changes with sequential line numbering that reflects final file positions
  * Providing context lines with proper line number continuity

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * derivedFrom: [Align with Industry Standards](UserRequirements.md#align-with-industry-standards)
---

### Documentation Index for HTML Export

The system shall automatically generate an index document during HTML export that contains a structured summary of all specification documents and folders, serving as the primary entry point (index.html) for HTML documentation.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing MBSE Models](UserStories.md#managing-mbse-models)
---

### Validate Markdown Structure

The system shall validate the Markdown structure of MBSE documentation to ensure compliance with formatting standards.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](UserRequirements.md#align-with-industry-standards)
---

### Validate Filesystem Structure

The system shall validate the organization of files and folders in the repository to ensure consistency with the MBSE methodology.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
---

### Validate Internal Consistency

The system shall check the internal consistency of the MBSE model, ensuring that relationships and elements align correctly.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](UserRequirements.md#align-with-industry-standards)
---

### Validate Cross-Component Dependencies

The system shall validate dependencies across different components of the MBSE model to identify mismatches or gaps.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](UserRequirements.md#align-with-industry-standards)
---

### Validate Relation Types

The system shall validate relation types and allow only supported types.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](UserRequirements.md#align-with-industry-standards)
---

## Integrate with GitHub Workflows
### Automate Pull Request Validations

The system shall automate validations of pull requests in the GitHub workflow to ensure model consistency before merging.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Integrate with GitHub Workflows](UserStories.md#integrate-with-github-workflows)
---

### Generate Change Logs for Pull Requests

The system shall generate detailed change logs for pull requests, summarizing modifications to the MBSE model and related components.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Integrate with GitHub Workflows](UserStories.md#integrate-with-github-workflows)
---

### Automate Diagram Generation

The system shall automate generation of diagrams in the GitHub workflow on PR merge event, so that the diagrams are always accessible and up-to-date.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Integrate with GitHub Workflows](UserStories.md#integrate-with-github-workflows)
---

## Provide Reports
### Model Reports

When requested the system shall provide human readable MBSE model reports.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Provide Reports](UserStories.md#provide-reports)
---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the MBSE model, including counts and types of connections also supporting json and cypher output.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
---

### Structural Change Reports

The system shall generate detailed reports summarizing the impact of structural changes, including affected relationships and components.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)
---

### Provide Validation Reports

The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the MBSE model structure.

#### Details
Validation shall be performed automatically when any command requires the parsed model, eliminating the need for a separate validation command. Commands that operate on raw files shall skip validation to allow operation on potentially invalid documents.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Provide Reports](UserStories.md#provide-reports)
  * derivedFrom: [Align with Industry Standards](UserRequirements.md#align-with-industry-standards)
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
  * derivedFrom: [Model Reports](#model-reports)
---

## Trace Changes in MBSE Model
### Tracing Structural Changes

When tracing structural changes, the system shall analyze the MBSE model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)
---

### Traceability Matrix

When requested the system shall generate traceability matrices, in Markdown format by default, and supporting different views.

#### Details
A matrix is a textual representation which is the most efficient way to convey numerous relationships within a compact space.

Requirements-to-Verification View:
  * Maps each requirement to one or more verification activities, tests, or validation procedures. This helps track which requirements are verified and how.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)
---

### Include Verification Checkboxes

The system shall include checkboxes in the traceability matrix for each verification entry, allowing users to manually mark verification as completed.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Traceability Matrix](#traceability-matrix)
---

### Export Traceability Matrix

The system shall provide an option to export the traceability matrix in formats such as Excel or PDF for external sharing and analysis.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Traceability Matrix](#traceability-matrix)
---

## Exporting Specifications
### Export HTML specifications

The system shall export specifications into HTML format and save in designated output location.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Export Specifications](UserStories.md#export-specifications)
---

### Browse Model via Web Interface

The system SHALL provide a web-based interface to browse the MBSE model documentation, including all generated artifacts such as diagrams, reports, verification traces, and traceability matrix.

#### Details
The browse interface allows users to:
- View HTML-rendered specifications and requirements
- Navigate through diagrams and visualizations
- Access verification traces and coverage reports
- Review traceability matrices
- Explore the complete model structure through an integrated web interface

This capability enables both human users (via browser) and AI agents (via MCP server) to efficiently explore and understand the MBSE model without manually navigating file structures.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing MBSE Models](UserStories.md#managing-mbse-models)
  * derivedFrom: [Export Specifications](UserStories.md#export-specifications)
---

### HTML Navigation Bar

The system SHALL provide a fixed navigation bar in all HTML pages with links to key model artifacts for easy access.

#### Details
The navigation bar must include:
- Home: Link to index.html (model structure overview)
- Traces: Link to traces.html (verification upward traceability report)
- Coverage: Link to coverage.html (verification coverage report)
- Matrix: Link to matrix.svg (traceability matrix visualization)

The navigation bar must be:
- Always visible (fixed position) while scrolling
- Consistent across all HTML pages
- Clearly visible and accessible

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Browse Model via Web Interface](#browse-model-via-web-interface)
---

## Trace Changes
### Change Impact Analysis

When requested the system shall generate change impact report, in Markdown format by default and also supporting json output.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Reports](#model-reports)
  * derivedFrom: [Structural Change Reports](#structural-change-reports)
---

## Interfaces
### CLI interface

The system shall provide command line interface (CLI) to faciliate model management.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing MBSE Models](UserStories.md#managing-mbse-models)
---

## Mission requirements
Mission requirements represent the high-level mission / enterprise  objectives, needs and measures of effectiveness, that a system must fulfill to align with the strategic goals of the organization and satisfy stakeholder expectations.


### Align with Industry Standards

The system must adhere to widely recognized industry standards, such as ISO/IEC/IEEE 15288, to ensure compatibility and relevance in systems engineering practices.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_CE](MOEs.md#moe_ce)
---

### Promote Automation and Efficiency

The system must significantly reduce manual effort in managing requirements, models, and traceability by automating routine tasks.

#### Metadata
  * type: user-requirement

#### Relations
  * trace: [MOE_UA](MOEs.md#moe_ua)
---

## Managing MBSE Models
### Coexistence of Structured and Unstructured Documents

The system shall allow structured markdown and unstructured. (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same MBSE model.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing MBSE Models](UserStories.md#managing-mbse-models)
---

### Efficient Processing

The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing MBSE Models](UserStories.md#managing-mbse-models)
  * derivedFrom: [Promote Automation and Efficiency](UserRequirements.md#promote-automation-and-efficiency)
---

### Git Repository as Project Root

The system shall treat the **root directory of the Git repository as the project's base** for all file and folder references, streamlining configuration and promoting a self-contained project structure.

#### Details
All paths specified in the Reqvire configuration will be resolved relative to the Git repository root.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing MBSE Models](UserStories.md#managing-mbse-models)
---

### Default Requirement Type Assignment

The system shall automatically assign the **default type `requirement`** to all elements if not explicitly specified in their `metadata` subsection.

#### Details
<details>
<summary>Type Assignment Rules</summary>

When an element does not have a `#### Metadata` subsection with a `type` property, the system assigns the default type `requirement`.

**This behavior is location-independent:** All elements default to type `requirement` regardless of their folder location within the Git repository.

**To use other element types**, users must explicitly specify the type in the element's Metadata subsection:
```markdown
#### Metadata
  * type: user-requirement
```

**Supported element types:**
- `requirement` (default)
- `user-requirement`
- `verification` / `test-verification`
- `analysis-verification`
- `inspection-verification`
- `demonstration-verification`
- `other`

</details>

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing MBSE Models](UserStories.md#managing-mbse-models)
---