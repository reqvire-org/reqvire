# User Requirements

## Managing MBSE Models

```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;
  a2398f7051["Bootstrap model struture"];
  click a2398f7051 "UserRequirements.md#bootstrap-model-struture";
  class a2398f7051 requirement;
  852dea6cfe["UserStories.md/Managing MBSE Models"];
  class 852dea6cfe requirement;
  click 852dea6cfe "UserStories.md#managing-mbse-models";
  a2398f7051 ==>|refines| 852dea6cfe;
  fc49f77257["Support for Distributed Requirements"];
  click fc49f77257 "UserRequirements.md#support-for-distributed-requirements";
  class fc49f77257 requirement;
  7430e12113["Project Configuration with YAML"];
  class 7430e12113 requirement;
  click 7430e12113 "UserRequirements.md#project-configuration-with-yaml";
  fc49f77257 -.->|deriveReqT| 7430e12113;
  7430e12113 ==>|refines| 852dea6cfe;
  438060bb46["Configurable SystemRequirements Folder Name"];
  click 438060bb46 "UserRequirements.md#configurable-systemrequirements-folder-name";
  class 438060bb46 requirement;
  438060bb46 -.->|deriveReqT| 7430e12113;
  f07cf1cbfa["Efficient Processing"];
  click f07cf1cbfa "UserRequirements.md#efficient-processing";
  class f07cf1cbfa requirement;
  f07cf1cbfa ==>|refines| 852dea6cfe;
  3745496893["Configurable DesignSpecifications Folder Name"];
  click 3745496893 "UserRequirements.md#configurable-designspecifications-folder-name";
  class 3745496893 requirement;
  3745496893 -.->|deriveReqT| 7430e12113;
```

### Coexistence of Structured and Unstructured Documents

The system shall allow structured markdown and unstructured (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same MBSE model.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)
---

### Efficient Processing

The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)
---

### Bootstrap model struture

The system shall provide a command to automatically bootstrap a predefined directory and file structure for an MBSE model, ensuring compliance with ReqFlow methodology.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)
---

### Project Configuration with YAML

The system shall support a YAML-based configuration file that defines folder names and structures to be used by the ReqFlow tool when processing model artifacts.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)
---

### Configurable SystemRequirements Folder Name

The system shall allow users to configure the name of the SystemRequirements folder through the configuration file, supporting flexible project organization.

#### Relations
  * derivedFrom: [Project Configuration with YAML](#project-configuration-with-yaml)
---

### Configurable DesignSpecifications Folder Name

The system shall allow users to configure the DesignSpecifications folder name through the configuration file, with this name consistently applied across all model levels.

#### Relations
  * derivedFrom: [Project Configuration with YAML](#project-configuration-with-yaml)
---

### Support for Distributed Requirements

The system shall support referencing folders that may exist in different repositories through configuration, allowing for distributed requirements management across multiple repositories.

#### Relations
  * derivedFrom: [Project Configuration with YAML](#project-configuration-with-yaml)

## Generate Diagrams

---

### Automate Diagram Generation

When requested the system shall automatically generate diagrams and save them to the required locations of the model, so that the diagrams are always accessible and up-to-date.

#### Relations
  * refine: [UserStories.md#Generate Diagrams](UserStories.md#generate-diagrams)
---

### Select Custom Diagram Viewpoints

The system shall allow users to select custom viewpoints for diagrams, tailored to specific stakeholder needs.

#### Relations
  * refine: [UserStories.md/Generate Diagrams](UserStories.md#generate-diagrams)
---

### Export Diagrams in Standard Formats

The system shall allow users to export generated diagrams in standard formats (e.g., PNG, SVG, PDF) for easy sharing and presentation.

#### Relations
  * containedBy: [UserStories.md/Generate Diagrams](UserStories.md#generate-diagrams)
---

### Highlight Changes in Diagrams

The system shall provide an option to highlight changes made to the model in the generated diagrams for better traceability.

#### Relations
  * refine: [UserStories.md/Generate Diagrams](UserStories.md#generate-diagrams)
---

### Visualize Model Relationships

The system shall provide visual representations of relationships within the MBSE model in the diagrams, enabling users to understand dependencies and their impact.

#### Relations
  * refine: [UserStories.md/Generate Diagrams](UserStories.md#generate-diagrams)
---

### Filter Relationships by Type

The system shall allow users to filter relationships in the MBSE model by type, such as dependency, refinement, or verification when generating diagrams.

#### Relations
  * refine: [UserStories.md/Generate Diagrams](UserStories.md#generate-diagrams)
---

### Store Automated Diagrams in Designated Locations

The system shall store automatically generated diagrams in pre-configured locations in the model repository.

#### Relations
  * refine: [UserStories.md/Generate Diagrams](UserStories.md#generate-diagrams)

## Aligning Design with Code

```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;
  1fc4e44d5f["Code Traceability"];
  click 1fc4e44d5f "UserRequirements.md#code-traceability";
  class 1fc4e44d5f requirement;
  de2d3516cd["UserStories.md/Aligning Design with Code"];
  class de2d3516cd requirement;
  click de2d3516cd "UserStories.md#aligning-design-with-code";
  1fc4e44d5f ==>|refines| de2d3516cd;
  5922f3ef03["Suggest Code Refactoring"];
  click 5922f3ef03 "UserRequirements.md#suggest-code-refactoring";
  class 5922f3ef03 requirement;
  5922f3ef03 ==>|refines| de2d3516cd;
```
---

### Code Traceability

The system shall support code traceability by using structured comments to link code implementations to corresponding requirements in the MBSE model.

#### Relations
  * refine: [UserStories.md/Aligning Design with Code](UserStories.md#aligning-design-with-code)
---

### Suggest Code Refactoring

The system shall suggest code refactoring opportunities to better align with the structure and relationships in the MBSE model.

#### Relations
  * refine: [UserStories.md/Aligning Design with Code](UserStories.md#aligning-design-with-code)

## Validating Structures

---

### Enhanced Validation Error Reporting

The system shall provide comprehensive validation messages that include file paths and line numbers when available, to help users quickly locate and fix model integrity and structure issues in their MBSE specifications.

#### Relations
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)
---

### Model Linting

The system shall provide linting capabilities to identify and fix stylistic, formatting, and non-critical issues in MBSE models that don't affect functional integrity.

#### Relations
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)
---

### Linting Command

The system shall provide a linting command that by default automatically applies fixes to stylistic and non-critical formatting issues, while offering option to preview changes without applying them.

#### Relations
  * containedBy: [Model Linting](#model-linting)
---

### Linting Command Output

The system shall display linting changes suggestion in similar manner as git diffs.

#### Relations
  * refine: [Linting Command](#linting-command)
---

### Replace Absolute Links with Relative Links

The system shall replace absolute links with relative links, where applicable and contextually appropriate, to conform to repository standards and enhance portability.

#### Relations
  * containedBy: [Model Linting](#model-linting)
---

### Format Consistency Enforcement

The system shall provide linting capability to ensure consistent formatting in requirements documents.

#### Details

  * Trimming excess whitespace after element names and relation identifiers
  * Normalizing to exactly two newlines before subsections (e.g., "#### Details")
  * Automatically inserting separator lines ("---") between elements if not already present
  * Ensuring consistent indentation in relation lists
 
#### Relations
  * containedBy: [Model Linting](#model-linting)
---

### Generate Documentation Index

The system shall generate an index.md file in the specifications root folder during linting that contains a structured summary of all specification documents and folders.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)
---

### Documentation Index HTML Integration

The index.md file shall be converted to index.md when HTML output is generated, serving as the primary entry point for HTML documentation.

#### Relations
  * refine: [Generate Documentation Index](#generate-documentation-index)
---

### Validate Markdown Structure

The system shall validate the Markdown structure of MBSE documentation to ensure compliance with formatting standards.

#### Relations
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)
---

### Validate Filesystem Structure

The system shall validate the organization of files and folders in the repository to ensure consistency with the MBSE methodology.

#### Relations
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)
---

### Validate Internal Consistency

The system shall check the internal consistency of the MBSE model, ensuring that relationships and elements align correctly.

#### Relations
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)
---

### Validate Cross-Component Dependencies

The system shall validate dependencies across different components of the MBSE model to identify mismatches or gaps.

#### Relations
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)

## Integrate with GitHub Workflows

---

### Automate Pull Request Validations

The system shall automate validations of pull requests in the GitHub workflow to ensure model consistency before merging.

#### Relations
  * refine: [UserStories.md/Integrate with GitHub Workflows](UserStories.md#integrate-with-github-workflows)
---

### Generate Change Logs for Pull Requests

The system shall generate detailed change logs for pull requests, summarizing modifications to the MBSE model and related components.

#### Relations
  * refine: [UserStories.md/Integrate with GitHub Workflows](UserStories.md#integrate-with-github-workflows)

## AI-Driven Code Suggestions

```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;
  e19f1380d9["Analyze Code for Alignment ---> Needs more work"];
  click e19f1380d9 "UserRequirements.md#analyze-code-for-alignment---->-needs-more-work";
  class e19f1380d9 requirement;
  a4657fca5a["UserStories.md/AI-Driven Code Suggestions"];
  class a4657fca5a requirement;
  click a4657fca5a "UserStories.md#ai-driven-code-suggestions";
  e19f1380d9 ==>|refines| a4657fca5a;
  c6d300e51a["Highlight Potential Code-Model Conflicts --> also too advanced for now"];
  click c6d300e51a "UserRequirements.md#highlight-potential-code-model-conflicts--->-also-too-advanced-for-now";
  class c6d300e51a requirement;
  c6d300e51a ==>|refines| a4657fca5a;
  71ba0e325e["Suggest Refactoring for MBSE Consistency  ---> Needs more work"];
  click 71ba0e325e "UserRequirements.md#suggest-refactoring-for-mbse-consistency----->-needs-more-work";
  class 71ba0e325e requirement;
  71ba0e325e ==>|refines| a4657fca5a;
```
---

### Analyze Code for Alignment ---> Needs more work

The system shall allow AI agents to analyze code and identify deviations from the MBSE model.

#### Relations
  * refine: [UserStories.md/AI-Driven Code Suggestions](UserStories.md#ai-driven-code-suggestions)
---

### Suggest Refactoring for MBSE Consistency  ---> Needs more work

The system shall enable AI agents to suggest refactoring opportunities to ensure code consistency with the MBSE model.

#### Relations
  * refine: [UserStories.md/AI-Driven Code Suggestions](UserStories.md#ai-driven-code-suggestions)
---

### Highlight Potential Code-Model Conflicts --> also too advanced for now

The system shall allow AI agents to highlight potential conflicts between code and the MBSE model, providing recommendations for resolution.

#### Relations
  * refine: [UserStories.md/AI-Driven Code Suggestions](UserStories.md#ai-driven-code-suggestions)

## AI-Driven Model Suggestions

---

### Provide Actionable Model Improvement Suggestions

The system shall enable AI agents to provide actionable suggestions for improving the MBSE model based on system performance data, design inconsistencies, and project requirements.

#### Relations
  * refine: [UserStories.md/AI-Driven Model Suggestions](UserStories.md#ai-driven-model-suggestions)
---

### Suggest Refinements to Model Relationships

The system shall enable AI agents to suggest refinements to relationships within the MBSE model to improve consistency and traceability.

#### Relations
  * refine: [Provide Actionable Model Improvement Suggestions](#provide-actionable-model-improvement-suggestions)
---

### Recommend Missing Components

The system shall allow AI agents to recommend missing components or elements based on gaps in the MBSE model.

#### Relations
  * refine: [Provide Actionable Model Improvement Suggestions](#provide-actionable-model-improvement-suggestions)
---

### Propose Validation Fixes

The system shall enable AI agents to propose fixes for validation errors in the MBSE model.

#### Relations
  * refine: [Provide Actionable Model Improvement Suggestions](#provide-actionable-model-improvement-suggestions)

## Provide Reports

---

### Generate Relationship Reports

The system shall generate reports summarizing the relationships in the MBSE model, including counts and types of connections.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)
---

### Generate Structural Change Reports

The system shall generate detailed reports summarizing the impact of structural changes, including affected relationships and components.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)
---

### Provide Validation Reports

The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the MBSE model structure.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)
---

### Generate Verifications Reports

The system shall produce reports identifying User and Mission requirements that lack a verifiedBy relationship.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)
---

### Generate Summary Reports

The system shall allow users to generate summary reports highlighting key metrics and statuses within the MBSE model.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)
---

### Generate Dependency Reports

The system shall generate reports summarizing dependencies between requirements, components, and test cases in the MBSE model.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)
---

### Export Reports to Standard Formats

The system shall allow users to export generated reports in standard formats (e.g., PDF, Excel) for external sharing.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)

## Trace Changes in MBSE Model

---

### Tracing Structural Changes

When tracing structural changes, the system shall analyze the MBSE model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Relations
  * refine: [UserStories.md/Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)
---

### Suggest Structural Updates

The system shall suggest updates to maintain structural consistency when changes are introduced to the MBSE model.

#### Relations
  * refine: [UserStories.md/Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)
---

### AI Feedback on Structural Changes

When a report of impacted elements is fed into the AI agents' context, the system shall provide suggestions for propagating changes across the MBSE model and allow the user to approve or reject each suggestion, so that updates can be applied consistently and committed to the model after user validation.

#### Relations
  * derivedFrom: [Suggest Structural Updates](#suggest-structural-updates)

## Generate Traceability Matrix			

---

### Create Traceability Matrices

The system shall create a traceability matrices when requested by a user or as part of CI/CD actions.

#### Relations
  * refine: [UserStories.md/Generate Traceability Matrix](UserStories.md#generate-traceability-matrix)
---

### Support Relation-Based Views

The system shall generate traceability matrix views based on relations to requirements, such as:
1. **VerifiedBy**: Mapping requirements to their verification methods.
2. **SatisfiedBy**: Mapping system components to the requirements they satisfy.
3. **TracedFrom**: Mapping requirements to their parent or related elements.

#### Relations
  * containedBy: [Create Traceability Matrices](#create-traceability-matrices)
 
---

### Interactive Mermaid Diagrams

The system shall include Mermaid diagrams in the traceability matrix that provide interactive links to related elements in other documents, enabling navigation and exploration of dependencies.

#### Details

Diagrams must be broken into several diagrams using following logic:
 * requirements_file_name/'## paragraph name'
   * all requirements inside are 1 diagram
   * if requirements documents doesn't have '##' paragraphs then requirements file name is used only
   * external related resources box must be a link to actual resource
Color code for rendering diagrams:
 * red for requirement
 * yellow for resources which satisfies requirement
 * green for verifiction which verifies requirement
 * light blue for box representing another diagram/category with requirments where linked requirement or resource exist.

#### Relations
  * containedBy: [Create Traceability Matrices](#create-traceability-matrices)
---

### Markdown-Based Default Format

The system shall generate the traceability matrix in Markdown format by default, adhering to ReqFlow's markdown-first methodology.

#### Relations
  * containedBy: [Create Traceability Matrices](#create-traceability-matrices)
---

### Save matrices to designated files

The system shall save the generated traceability matrices as a Markdown documents with Mermaid diagrams.

#### Relations
  * containedBy: [Create Traceability Matrices](#create-traceability-matrices)
---

### Include Verification Checkboxes

The system shall include checkboxes in the traceability matrix for each verification entry, allowing users to manually mark verification as completed and commit the updated status.

#### Relations
  * refine: [UserStories.md/Generate Traceability Matrix](UserStories.md#generate-traceability-matrix)
---

### Handle Affected Verifications on Model Changes

The system shall uncheck verification checkboxes in the traceability matrix and save updates if a diff affects the related requirements or components, ensuring re-validation is required.

#### Relations
  * refine: [UserStories.md/Generate Traceability Matrix](UserStories.md#generate-traceability-matrix)
---

### Specification Design Document for Requirements Change Propagation

The system **shall provide a Specification Design Document (DSD)** that defines how changes in requirements affect child requirements and verifications, ensuring traceability and controlled impact analysis.

#### Relations
  * refine: [UserStories.md/Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)
  * satisfiedBy: [DesignSpecifications/RequirementsChangePropagation.md](DesignSpecifications/RequirementsChangePropagation.md)
---

### Export Traceability Matrix

The system shall provide an option to export the traceability matrix in formats such as Excel or PDF for external sharing and analysis.

#### Relations
  * refine: [UserStories.md/Generate Traceability Matrix](UserStories.md#generate-traceability-matrix)
---

## Exporting Specifications

---

### Export HTML specifications

The system shall export specifications into HTML format and save in designated output location.

#### Relations
  * derivedFrom: [UserStories.md/Export Specifications](UserStories.md#export-specifications)
---

### Support CI/CD Integration

The system shall integrate with CI/CD pipelines to generate or update traceability matrices as part of automated workflows.

#### Relations
  * refine: [UserStories.md/Automate Traceability Matrix](UserStories.md#generate-traceability-matrix)
