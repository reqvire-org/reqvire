# User Requirements

## Managing MBSE Models
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  fc49f77257["Support for Distributed Requirements"];
  click fc49f77257 "UserRequirements.md#support-for-distributed-requirements";
  class fc49f77257 requirement;
  7430e12113["Project Configuration with YAML"];
  class 7430e12113 requirement;
  click 7430e12113 "UserRequirements.md#project-configuration-with-yaml";
  fc49f77257 -.->|deriveReqT| 7430e12113;
  f43076b79a["Configurable External Folders"];
  class f43076b79a requirement;
  click f43076b79a "UserRequirements.md#configurable-external-folders";
  fc49f77257 -->|relates to| f43076b79a;
  f07cf1cbfa["Efficient Processing"];
  click f07cf1cbfa "UserRequirements.md#efficient-processing";
  class f07cf1cbfa requirement;
  852dea6cfe["UserStories.md/Managing MBSE Models"];
  class 852dea6cfe requirement;
  click 852dea6cfe "UserStories.md#managing-mbse-models";
  f07cf1cbfa ==>|refines| 852dea6cfe;
  5011baf9fd["Configurable Specifications Folder"];
  click 5011baf9fd "UserRequirements.md#configurable-specifications-folder";
  class 5011baf9fd requirement;
  5011baf9fd -.->|deriveReqT| 7430e12113;
  7430e12113 ==>|refines| 852dea6cfe;
  fc49f77257 -.->|deriveReqT| 7430e12113;
  5011baf9fd -.->|deriveReqT| 7430e12113;
  a2398f7051["Bootstrap model struture"];
  click a2398f7051 "UserRequirements.md#bootstrap-model-struture";
  class a2398f7051 requirement;
  a2398f7051 ==>|refines| 852dea6cfe;
  6849361af5["Coexistence of Structured and Unstructured Documents"];
  click 6849361af5 "UserRequirements.md#coexistence-of-structured-and-unstructured-documents";
  class 6849361af5 requirement;
  6849361af5 ==>|refines| 852dea6cfe;
  f43076b79a ==>|refines| fc49f77257;
```

---

### Coexistence of Structured and Unstructured Documents
The system shall allow structured markdown and unstructured. (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same MBSE model.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Efficient Processing
The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Project Configuration with YAML
The system shall support a YAML-based configuration file that defines folder names and structures to be used by the ReqFlow tool when processing model artifacts.

#### Relations
  * refine: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

---

### Configurable Specifications Folder

The system shall allow users to configure the main specification folder through the configuration file, supporting flexible project organization.

#### Relations
  * derivedFrom: [Project Configuration with YAML](#project-configuration-with-yaml)

---

### Support for Distributed Requirements
The system shall support referencing folders that may exist in different repositories through configuration, allowing for distributed requirements management across multiple repositories.

#### Relations
  * derivedFrom: [Project Configuration with YAML](#project-configuration-with-yaml)

---

### Configurable External Folders
The system shall allow users to configure the External folders through the configuration file.

#### Relations
  * refine: [Support for Distributed Requirements](#support-for-distributed-requirements)

---

## Generate Diagrams
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  aee397f35b["Store Automated Diagrams in Designated Locations"];
  click aee397f35b "UserRequirements.md#store-automated-diagrams-in-designated-locations";
  class aee397f35b requirement;
  cf0026c2fe["UserStories.md/Generate Diagrams"];
  class cf0026c2fe requirement;
  click cf0026c2fe "UserStories.md#generate-diagrams";
  aee397f35b ==>|refines| cf0026c2fe;
  10c00a1bd1["Export Diagrams in Standard Formats"];
  click 10c00a1bd1 "UserRequirements.md#export-diagrams-in-standard-formats";
  class 10c00a1bd1 requirement;
  cf0026c2fe --o|contains| 10c00a1bd1;
  a6a8362836["Visualize Model Relationships"];
  click a6a8362836 "UserRequirements.md#visualize-model-relationships";
  class a6a8362836 requirement;
  a6a8362836 ==>|refines| cf0026c2fe;
  81f9235ded["Automate Diagram Generation"];
  class 81f9235ded requirement;
  click 81f9235ded "UserRequirements.md#automate-diagram-generation";
  81f9235ded -.->|deriveReqT| a6a8362836;
  30053341d8["Select Custom Diagram Viewpoints"];
  click 30053341d8 "UserRequirements.md#select-custom-diagram-viewpoints";
  class 30053341d8 requirement;
  30053341d8 ==>|refines| cf0026c2fe;
  30d97803eb["Filter Relationships by Type"];
  click 30d97803eb "UserRequirements.md#filter-relationships-by-type";
  class 30d97803eb requirement;
  30d97803eb ==>|refines| cf0026c2fe;
  81f9235ded -.->|deriveReqT| a6a8362836;
  fd7388e379["Highlight Changes in Diagrams"];
  click fd7388e379 "UserRequirements.md#highlight-changes-in-diagrams";
  class fd7388e379 requirement;
  fd7388e379 ==>|refines| cf0026c2fe;
```

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

### Automate Diagram Generation
When requested the system shall automatically generate diagrams and save them to the required locations of the model, so that the diagrams are always accessible and up-to-date.

#### Relations
  * derivedFrom: [Visualize Model Relationships](#visualize-model-relationships)

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

---

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

---

## Validating Structures
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  7cf5cf9900["Enhanced Validation Error Reporting"];
  click 7cf5cf9900 "UserRequirements.md#enhanced-validation-error-reporting";
  class 7cf5cf9900 requirement;
  a60d88b6e2["UserStories.md/Validating Structures"];
  class a60d88b6e2 requirement;
  click a60d88b6e2 "UserStories.md#validating-structures";
  7cf5cf9900 ==>|refines| a60d88b6e2;
  f5b5eaeb28["Generate Documentation Index"];
  click f5b5eaeb28 "UserRequirements.md#generate-documentation-index";
  class f5b5eaeb28 requirement;
  852dea6cfe["UserStories.md/Managing MBSE Models"];
  class 852dea6cfe requirement;
  click 852dea6cfe "UserStories.md#managing-mbse-models";
  f5b5eaeb28 ==>|refines| 852dea6cfe;
  e1c89b5d94["Documentation Index HTML Integration"];
  class e1c89b5d94 requirement;
  click e1c89b5d94 "UserRequirements.md#documentation-index-html-integration";
  f5b5eaeb28 -->|relates to| e1c89b5d94;
  6e40bf9f83["Validate Cross-Component Dependencies"];
  click 6e40bf9f83 "UserRequirements.md#validate-cross-component-dependencies";
  class 6e40bf9f83 requirement;
  6e40bf9f83 ==>|refines| a60d88b6e2;
  808b1863c8["Linting Command Output"];
  click 808b1863c8 "UserRequirements.md#linting-command-output";
  class 808b1863c8 requirement;
  887a7d36ca["Linting Command"];
  class 887a7d36ca requirement;
  click 887a7d36ca "UserRequirements.md#linting-command";
  808b1863c8 ==>|refines| 887a7d36ca;
  d834cc4bc9["Validate Filesystem Structure"];
  click d834cc4bc9 "UserRequirements.md#validate-filesystem-structure";
  class d834cc4bc9 requirement;
  d834cc4bc9 ==>|refines| a60d88b6e2;
  e1c89b5d94 ==>|refines| f5b5eaeb28;
  103ddb8dc3["Model Linting"];
  click 103ddb8dc3 "UserRequirements.md#model-linting";
  class 103ddb8dc3 requirement;
  103ddb8dc3 ==>|refines| a60d88b6e2;
  81758bdb22["Format Consistency Enforcement"];
  class 81758bdb22 requirement;
  click 81758bdb22 "UserRequirements.md#format-consistency-enforcement";
  103ddb8dc3 --o|contains| 81758bdb22;
  103ddb8dc3 --o|contains| 887a7d36ca;
  85f6854a46["Replace Absolute Links with Relative Links"];
  class 85f6854a46 requirement;
  click 85f6854a46 "UserRequirements.md#replace-absolute-links-with-relative-links";
  103ddb8dc3 --o|contains| 85f6854a46;
  103ddb8dc3 --o|contains| 81758bdb22;
  9e524ac696["Validate Internal Consistency"];
  click 9e524ac696 "UserRequirements.md#validate-internal-consistency";
  class 9e524ac696 requirement;
  9e524ac696 ==>|refines| a60d88b6e2;
  103ddb8dc3 --o|contains| 887a7d36ca;
  887a7d36ca -->|relates to| 808b1863c8;
  103ddb8dc3 --o|contains| 85f6854a46;
  7b1772417b["Validate Markdown Structure"];
  click 7b1772417b "UserRequirements.md#validate-markdown-structure";
  class 7b1772417b requirement;
  7b1772417b ==>|refines| a60d88b6e2;
```

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
  * derivedFrom: [UserStories.md/Managing MBSE Models](UserStories.md#managing-mbse-models)

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

---

## Integrate with GitHub Workflows
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  b2a387077d["Generate Change Logs for Pull Requests"];
  click b2a387077d "UserRequirements.md#generate-change-logs-for-pull-requests";
  class b2a387077d requirement;
  ba120b7caf["UserStories.md/Integrate with GitHub Workflows"];
  class ba120b7caf requirement;
  click ba120b7caf "UserStories.md#integrate-with-github-workflows";
  b2a387077d ==>|refines| ba120b7caf;
  68454e6166["Automate Pull Request Validations"];
  click 68454e6166 "UserRequirements.md#automate-pull-request-validations";
  class 68454e6166 requirement;
  68454e6166 ==>|refines| ba120b7caf;
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
  71ba0e325e["Suggest Refactoring for MBSE Consistency  ---> Needs more work"];
  click 71ba0e325e "UserRequirements.md#suggest-refactoring-for-mbse-consistency----->-needs-more-work";
  class 71ba0e325e requirement;
  71ba0e325e ==>|refines| a4657fca5a;
  c6d300e51a["Highlight Potential Code-Model Conflicts --> also too advanced for now"];
  click c6d300e51a "UserRequirements.md#highlight-potential-code-model-conflicts--->-also-too-advanced-for-now";
  class c6d300e51a requirement;
  c6d300e51a ==>|refines| a4657fca5a;
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

---

## AI-Driven Model Suggestions
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  a8f00c5619["Recommend Missing Components"];
  click a8f00c5619 "UserRequirements.md#recommend-missing-components";
  class a8f00c5619 requirement;
  ec56dd665a["Provide Actionable Model Improvement Suggestions"];
  class ec56dd665a requirement;
  click ec56dd665a "UserRequirements.md#provide-actionable-model-improvement-suggestions";
  a8f00c5619 ==>|refines| ec56dd665a;
  6912d98267["UserStories.md/AI-Driven Model Suggestions"];
  class 6912d98267 requirement;
  click 6912d98267 "UserStories.md#ai-driven-model-suggestions";
  ec56dd665a ==>|refines| 6912d98267;
  ec56dd665a -->|relates to| a8f00c5619;
  44c87c93a4["Propose Validation Fixes"];
  class 44c87c93a4 requirement;
  click 44c87c93a4 "UserRequirements.md#propose-validation-fixes";
  ec56dd665a -->|relates to| 44c87c93a4;
  72e13dc549["Suggest Refinements to Model Relationships"];
  class 72e13dc549 requirement;
  click 72e13dc549 "UserRequirements.md#suggest-refinements-to-model-relationships";
  ec56dd665a -->|relates to| 72e13dc549;
  44c87c93a4 ==>|refines| ec56dd665a;
  72e13dc549 ==>|refines| ec56dd665a;
```

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

---

## Provide Reports
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  d0e9e8d143["Generate Verifications Reports"];
  click d0e9e8d143 "UserRequirements.md#generate-verifications-reports";
  class d0e9e8d143 requirement;
  fe32882ee2["UserStories.md/Provide Reports"];
  class fe32882ee2 requirement;
  click fe32882ee2 "UserStories.md#provide-reports";
  d0e9e8d143 ==>|refines| fe32882ee2;
  482c757913["Provide Validation Reports"];
  click 482c757913 "UserRequirements.md#provide-validation-reports";
  class 482c757913 requirement;
  482c757913 ==>|refines| fe32882ee2;
  596c459d31["Generate Summary Reports"];
  click 596c459d31 "UserRequirements.md#generate-summary-reports";
  class 596c459d31 requirement;
  596c459d31 ==>|refines| fe32882ee2;
  6a2d21600e["Generate Structural Change Reports"];
  click 6a2d21600e "UserRequirements.md#generate-structural-change-reports";
  class 6a2d21600e requirement;
  f8e7625d29["Model Reports"];
  class f8e7625d29 requirement;
  click f8e7625d29 "UserRequirements.md#model-reports";
  6a2d21600e -.->|deriveReqT| f8e7625d29;
  2afa7f3a20["Export Reports to Standard Formats"];
  click 2afa7f3a20 "UserRequirements.md#export-reports-to-standard-formats";
  class 2afa7f3a20 requirement;
  2afa7f3a20 ==>|refines| fe32882ee2;
  f8e7625d29 ==>|refines| fe32882ee2;
  6a2d21600e -.->|deriveReqT| f8e7625d29;
  b342220e0d["Model Structure and Summaries"];
  class b342220e0d requirement;
  click b342220e0d "UserRequirements.md#model-structure-and-summaries";
  b342220e0d -.->|deriveReqT| f8e7625d29;
  b342220e0d -.->|deriveReqT| f8e7625d29;
  812d42f453["Generate Dependency Reports"];
  click 812d42f453 "UserRequirements.md#generate-dependency-reports";
  class 812d42f453 requirement;
  812d42f453 ==>|refines| fe32882ee2;
```

---

### Model Reports
When requested the system shall provide human readable MBSE model reports.

#### Relations
  * refine: [UserStories.md/Provide Reports](UserStories.md#provide-reports)

---

### Model Structure and Summaries
When requested the system shall generate reports summarizing the structure and relationships in the MBSE model, including counts and types of connections.

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

---

## Trace Changes in MBSE Model

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

Traceability matrices are commonly used in Model-Based Systems Engineering (MBSE) to track relationships between various system elements, 
including requirements, designs, verifications, and tests, allowing to efficiently maintain alignment between requirements and their associated system elements. 

One of their key uses is to manage the impact of changes, including identifying which verifications may be invalidated due to changes in requirements or system components.

#### Relations
  * containedBy: [UserStories.md/Trace Changes in MBSE Model](UserStories.md#trace-changes-in-mbse-model)

---

### Save matrices to designated files

The system shall automatically save generated traceability matrices as a Markdown documents.

#### Relations
  * refine: [Traceability Matrix](#traceability-matrix)

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
  * derivedFrom: [Traceability Matrix](#traceability-matrix)

---

## Change Tracing

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

### Specification Design Document for Requirements Change Propagation

The system **shall provide a Specification Design Document (DSD)** that defines how changes in requirements affect child requirements and verifications, ensuring traceability and controlled impact analysis.

#### Relations
  * refine: [Change Impact Analysis](#change-impact-analysis)
  * satisfiedBy: [DesignSpecifications/RequirementsChangePropagation.md](DesignSpecifications/RequirementsChangePropagation.md)

---

## Exporting Specifications
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  33733d2f42["Support CI/CD Integration"];
  click 33733d2f42 "UserRequirements.md#support-ci/cd-integration";
  class 33733d2f42 requirement;
  e4eb5bf7e5["UserStories.md/Automate Traceability Matrix"];
  class e4eb5bf7e5 requirement;
  click e4eb5bf7e5 "UserStories.md#generate-traceability-matrix";
  33733d2f42 ==>|refines| e4eb5bf7e5;
  72c7eda618["Export HTML specifications"];
  click 72c7eda618 "UserRequirements.md#export-html-specifications";
  class 72c7eda618 requirement;
  100197ce81["UserStories.md/Export Specifications"];
  class 100197ce81 requirement;
  click 100197ce81 "UserStories.md#export-specifications";
  72c7eda618 -.->|deriveReqT| 100197ce81;
```

---

### Export HTML specifications

The system shall export specifications into HTML format and save in designated output location.

#### Relations
  * derivedFrom: [UserStories.md/Export Specifications](UserStories.md#export-specifications)

---

