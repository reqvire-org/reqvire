# User Requirements

## Generate Diagrams
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  eedf6d6d3d2354d9["Interactive Mermaid Diagrams"];
  click eedf6d6d3d2354d9 "specifications/UserRequirements.md#interactive-mermaid-diagrams#interactive-mermaid-diagrams";
  class eedf6d6d3d2354d9 requirement;
  37a5b8e199a838f["UserStories.md/Generate Diagrams"];
  class 37a5b8e199a838f requirement;
  click 37a5b8e199a838f "specifications/UserStories.md#generate-diagrams#generate-diagrams";
  eedf6d6d3d2354d9 -.->|deriveReqT| 37a5b8e199a838f;
```

---

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

#### Relations
  * derivedFrom: [UserStories.md/Generate Diagrams](UserStories.md#generate-diagrams)

---

## Aligning Design with Code
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  398641ced77f628c["Suggest Code Refactoring"];
  click 398641ced77f628c "specifications/UserRequirements.md#suggest-code-refactoring#suggest-code-refactoring";
  class 398641ced77f628c requirement;
  855a1b3061c7bcdd["UserStories.md/Aligning Design with Code"];
  class 855a1b3061c7bcdd requirement;
  click 855a1b3061c7bcdd "specifications/UserStories.md#aligning-design-with-code#aligning-design-with-code";
  398641ced77f628c -->|refines| 855a1b3061c7bcdd;
  26ca72b617aff229["Code Traceability"];
  click 26ca72b617aff229 "specifications/UserRequirements.md#code-traceability#code-traceability";
  class 26ca72b617aff229 requirement;
  26ca72b617aff229 -->|refines| 855a1b3061c7bcdd;
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

---

## Validating Structures
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  586b073cd97908da["Validate Markdown Structure"];
  click 586b073cd97908da "specifications/UserRequirements.md#validate-markdown-structure#validate-markdown-structure";
  class 586b073cd97908da requirement;
  113748a94885138d["UserStories.md/Validating Structures"];
  class 113748a94885138d requirement;
  click 113748a94885138d "specifications/UserStories.md#validating-structures#validating-structures";
  586b073cd97908da -->|refines| 113748a94885138d;
  974ccf933675ef44["Format Consistency Enforcement"];
  click 974ccf933675ef44 "specifications/UserRequirements.md#format-consistency-enforcement#format-consistency-enforcement";
  class 974ccf933675ef44 requirement;
  7305c1d6f7f1e2b2["Model Linting"];
  class 7305c1d6f7f1e2b2 requirement;
  click 7305c1d6f7f1e2b2 "specifications/UserRequirements.md#model-linting#model-linting";
  974ccf933675ef44 --o|contains| 7305c1d6f7f1e2b2;
  c50887ce89be280a["Validate Internal Consistency"];
  click c50887ce89be280a "specifications/UserRequirements.md#validate-internal-consistency#validate-internal-consistency";
  class c50887ce89be280a requirement;
  c50887ce89be280a -->|refines| 113748a94885138d;
  ec10e748b5e9516e["Linting Command Output"];
  click ec10e748b5e9516e "specifications/UserRequirements.md#linting-command-output#linting-command-output";
  class ec10e748b5e9516e requirement;
  a51179cda67cf9f2["Linting Command"];
  class a51179cda67cf9f2 requirement;
  click a51179cda67cf9f2 "specifications/UserRequirements.md#linting-command#linting-command";
  ec10e748b5e9516e -->|refines| a51179cda67cf9f2;
  8dfe33c28555e80a["Replace Absolute Links with Relative Links"];
  click 8dfe33c28555e80a "specifications/UserRequirements.md#replace-absolute-links-with-relative-links#replace-absolute-links-with-relative-links";
  class 8dfe33c28555e80a requirement;
  8dfe33c28555e80a --o|contains| 7305c1d6f7f1e2b2;
  c2b6c74b77726ad9["Generate Documentation Index"];
  click c2b6c74b77726ad9 "specifications/UserRequirements.md#generate-documentation-index#generate-documentation-index";
  class c2b6c74b77726ad9 requirement;
  e61b7c1baa89bfc6["UserStories.md/Managing MBSE Models"];
  class e61b7c1baa89bfc6 requirement;
  click e61b7c1baa89bfc6 "specifications/UserStories.md#managing-mbse-models#managing-mbse-models";
  c2b6c74b77726ad9 -.->|deriveReqT| e61b7c1baa89bfc6;
  3b10b8811daaed67["Enhanced Validation Error Reporting"];
  click 3b10b8811daaed67 "specifications/UserRequirements.md#enhanced-validation-error-reporting#enhanced-validation-error-reporting";
  class 3b10b8811daaed67 requirement;
  3b10b8811daaed67 -->|refines| 113748a94885138d;
  7305c1d6f7f1e2b2 -->|refines| 113748a94885138d;
  a51179cda67cf9f2 --o|contains| 7305c1d6f7f1e2b2;
  3bd9d29239564eeb["Validate Cross-Component Dependencies"];
  click 3bd9d29239564eeb "specifications/UserRequirements.md#validate-cross-component-dependencies#validate-cross-component-dependencies";
  class 3bd9d29239564eeb requirement;
  3bd9d29239564eeb -->|refines| 113748a94885138d;
  84b3d0502132adb5["Documentation Index HTML Integration"];
  click 84b3d0502132adb5 "specifications/UserRequirements.md#documentation-index-html-integration#documentation-index-html-integration";
  class 84b3d0502132adb5 requirement;
  84b3d0502132adb5 -->|refines| c2b6c74b77726ad9;
  f25cbfbca6d6d92e["Validate Relation Types"];
  click f25cbfbca6d6d92e "specifications/UserRequirements.md#validate-relation-types#validate-relation-types";
  class f25cbfbca6d6d92e requirement;
  f25cbfbca6d6d92e -->|refines| 113748a94885138d;
  5c482dc763b4133a["Validate Filesystem Structure"];
  click 5c482dc763b4133a "specifications/UserRequirements.md#validate-filesystem-structure#validate-filesystem-structure";
  class 5c482dc763b4133a requirement;
  5c482dc763b4133a -->|refines| 113748a94885138d;
```

---

### Enhanced Validation Error Reporting
The system shall provide comprehensive validation messages that include file paths and line numbers when available, to help users quickly locate and fix model integrity and structure issues in their MBSE specifications.

#### Relations
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)

---

### Model Linting
The ssystem shall provide linting capabilities to identify and fix stylistic, formatting, and non-critical issues in MBSE models that don't affect functional integrity.

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
The system shall generate a SpecificationIndex.md file in the repository root that contains a structured summary of all specification documents and folders.

#### Relations
  * derivedFrom: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Documentation Index HTML Integration
The SpecificationIndex.md file shall be converted to index.html when HTML output is generated, serving as the primary entry point for HTML documentation.

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

---

### Validate Relation Types

The system shall validate relation types and allow only supported types.

#### Relations
  * refine: [UserStories.md/Validating Structures](UserStories.md#validating-structures)

---

## Integrate with GitHub Workflows
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  98a581084d5542fa["Automate Diagram Generation"];
  click 98a581084d5542fa "specifications/UserRequirements.md#automate-diagram-generation#automate-diagram-generation";
  class 98a581084d5542fa requirement;
  98eaeddc27f99e11["UserStories.md/Integrate with GitHub Workflows"];
  class 98eaeddc27f99e11 requirement;
  click 98eaeddc27f99e11 "specifications/UserStories.md#integrate-with-github-workflows#integrate-with-github-workflows";
  98a581084d5542fa -->|refines| 98eaeddc27f99e11;
  672b444a568468b8["Generate Change Logs for Pull Requests"];
  click 672b444a568468b8 "specifications/UserRequirements.md#generate-change-logs-for-pull-requests#generate-change-logs-for-pull-requests";
  class 672b444a568468b8 requirement;
  672b444a568468b8 -->|refines| 98eaeddc27f99e11;
  15f2f511b2399406["Automate Pull Request Validations"];
  click 15f2f511b2399406 "specifications/UserRequirements.md#automate-pull-request-validations#automate-pull-request-validations";
  class 15f2f511b2399406 requirement;
  15f2f511b2399406 -->|refines| 98eaeddc27f99e11;
```

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

---

### Automate Diagram Generation

The system shall automate generation of diagrams in the GitHub workflow on PR merge event, so that the diagrams are always accessible and up-to-date.

#### Relations
  * refine: [UserStories.md/Integrate with GitHub Workflows](UserStories.md#integrate-with-github-workflows)

---

## Provide Reports
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  b5c9876f83e7d0ce["Generate Summary Reports"];
  click b5c9876f83e7d0ce "specifications/UserRequirements.md#generate-summary-reports#generate-summary-reports";
  class b5c9876f83e7d0ce requirement;
  91a0adb0b4d959c4["UserStories.md/Provide Reports"];
  class 91a0adb0b4d959c4 requirement;
  click 91a0adb0b4d959c4 "specifications/UserStories.md#provide-reports#provide-reports";
  b5c9876f83e7d0ce -->|refines| 91a0adb0b4d959c4;
  1908501a80db5c46["Structural Change Reports"];
  click 1908501a80db5c46 "specifications/UserRequirements.md#structural-change-reports#structural-change-reports";
  class 1908501a80db5c46 requirement;
  70701096d332c0b2["Model Reports"];
  class 70701096d332c0b2 requirement;
  click 70701096d332c0b2 "specifications/UserRequirements.md#model-reports#model-reports";
  1908501a80db5c46 -.->|deriveReqT| 70701096d332c0b2;
  70701096d332c0b2 -->|refines| 91a0adb0b4d959c4;
  ad6f7a2d41d80a38["Model Structure and Summaries"];
  click ad6f7a2d41d80a38 "specifications/UserRequirements.md#model-structure-and-summaries#model-structure-and-summaries";
  class ad6f7a2d41d80a38 requirement;
  ad6f7a2d41d80a38 -.->|deriveReqT| 70701096d332c0b2;
  ed31b6bed1cde2f8["Provide Validation Reports"];
  click ed31b6bed1cde2f8 "specifications/UserRequirements.md#provide-validation-reports#provide-validation-reports";
  class ed31b6bed1cde2f8 requirement;
  ed31b6bed1cde2f8 -->|refines| 91a0adb0b4d959c4;
```

---

### Model Reports
When requested the system shall provide human readable MBSE model reports.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)

---

### Model Structure and Summaries

When requested the system shall generate reports summarizing the structure and relationships in the MBSE model, including counts and types of connections also supporting json and cypher output.

#### Relations
  * derivedFrom: [Model Reports](#model-reports)

---

### Structural Change Reports

The system shall generate detailed reports summarizing the impact of structural changes, including affected relationships and components.

#### Relations
  * derivedFrom: [Model Reports](#model-reports)

---

### Provide Validation Reports
The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the MBSE model structure.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)

---

### Generate Summary Reports

The system shall allow users to generate summary reports highlighting key metrics and statuses within the MBSE model.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)

---

## Trace Changes in MBSE Model
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  d7b7b13a5b8d96e1["Tracing Structural Changes"];
  click d7b7b13a5b8d96e1 "specifications/UserRequirements.md#tracing-structural-changes#tracing-structural-changes";
  class d7b7b13a5b8d96e1 requirement;
  5ef9c8ae19a9f55a["UserStories.md/Trace Changes in MBSE Model"];
  class 5ef9c8ae19a9f55a requirement;
  click 5ef9c8ae19a9f55a "specifications/UserStories.md#trace-changes-in-mbse-model#trace-changes-in-mbse-model";
  d7b7b13a5b8d96e1 -.->|deriveReqT| 5ef9c8ae19a9f55a;
  7de9a55d6102af23["Export Traceability Matrix"];
  click 7de9a55d6102af23 "specifications/UserRequirements.md#export-traceability-matrix#export-traceability-matrix";
  class 7de9a55d6102af23 requirement;
  4e30ea0930dc9c26["Traceability Matrix"];
  class 4e30ea0930dc9c26 requirement;
  click 4e30ea0930dc9c26 "specifications/UserRequirements.md#traceability-matrix#traceability-matrix";
  7de9a55d6102af23 -.->|deriveReqT| 4e30ea0930dc9c26;
  6d32b919c82784b7["Include Verification Checkboxes"];
  click 6d32b919c82784b7 "specifications/UserRequirements.md#include-verification-checkboxes#include-verification-checkboxes";
  class 6d32b919c82784b7 requirement;
  6d32b919c82784b7 -->|refines| 4e30ea0930dc9c26;
  4e30ea0930dc9c26 --o|contains| 5ef9c8ae19a9f55a;
```

---

### Tracing Structural Changes

When tracing structural changes, the system shall analyze the MBSE model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

#### Relations
  * derivedFrom: [UserStories.md/Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)

---

### Traceability Matrix

When requested the system shall generate traceability matrices, in Markdown format by default, and supporting different views.

#### Details

Requirements-to-Verification View:
  * Maps each requirement to one or more verification activities, tests, or validation procedures. This helps track which requirements are verified and how.

#### Info

A matrix is a textual representation which is the most efficient way to convey numerous relationships within a compact space.

#### Relations
  * containedBy: [UserStories.md/Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)

---

### Include Verification Checkboxes

The system shall include checkboxes in the traceability matrix for each verification entry, allowing users to manually mark verification as completed.

#### Relations
  * refine: [Traceability Matrix](#traceability-matrix)

---

### Export Traceability Matrix

The system shall provide an option to export the traceability matrix in formats such as Excel or PDF for external sharing and analysis.

#### Relations
  * derivedFrom: [Traceability Matrix](#traceability-matrix)

---

## Exporting Specifications
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  a4c40962cac85d0c["Export HTML specifications"];
  click a4c40962cac85d0c "specifications/UserRequirements.md#export-html-specifications#export-html-specifications";
  class a4c40962cac85d0c requirement;
  b3b899678f557ee9["UserStories.md/Export Specifications"];
  class b3b899678f557ee9 requirement;
  click b3b899678f557ee9 "specifications/UserStories.md#export-specifications#export-specifications";
  a4c40962cac85d0c -.->|deriveReqT| b3b899678f557ee9;
```

---

### Export HTML specifications

The system shall export specifications into HTML format and save in designated output location.

#### Relations
  * derivedFrom: [UserStories.md/Export Specifications](UserStories.md#export-specifications)

---

## Trace Changes
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  9933cac5853a8584["Change Impact Analysis"];
  click 9933cac5853a8584 "specifications/UserRequirements.md#change-impact-analysis#change-impact-analysis";
  class 9933cac5853a8584 requirement;
  5ef9c8ae19a9f55a["UserStories.md/Trace Changes in MBSE Model"];
  class 5ef9c8ae19a9f55a requirement;
  click 5ef9c8ae19a9f55a "specifications/UserStories.md#trace-changes-in-mbse-model#trace-changes-in-mbse-model";
  9933cac5853a8584 --o|contains| 5ef9c8ae19a9f55a;
```

---

### Change Impact Analysis

When requested the system shall generate change impact report, in Markdown format by default and also supporting json output.

#### Details

Change Report:
 * Overview of all the changes in the model and impact to related requirements and other system elements.

Change Impact Analysis Report:
  * When a requirement changes, the traceability helps identify:
    * Which related requirements are affected by a change.
    * Which verification procedures or test cases are impacted and potentially invalidated.
    * Which other model elements might be affected.

#### Relations
  * containedBy: [UserStories.md/Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)

---