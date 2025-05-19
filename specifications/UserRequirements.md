# User Requirements

## Generate Diagrams
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  eed0b020b6ddeae1["Visualize Model Relationships"];
  click eed0b020b6ddeae1 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#visualize-model-relationships";
  class eed0b020b6ddeae1 requirement;
  694c4bb24b10f0c7["UserStories.md/Generate Diagrams"];
  class 694c4bb24b10f0c7 requirement;
  click 694c4bb24b10f0c7 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#generate-diagrams";
  eed0b020b6ddeae1 -->|refines| 694c4bb24b10f0c7;
  a0274ca0625d8493["Export Diagrams in Standard Formats"];
  click a0274ca0625d8493 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  class a0274ca0625d8493 requirement;
  a0274ca0625d8493 --o|contains| 694c4bb24b10f0c7;
  aa6a73dcebdfbc0d["Select Custom Diagram Viewpoints"];
  click aa6a73dcebdfbc0d "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#select-custom-diagram-viewpoints";
  class aa6a73dcebdfbc0d requirement;
  aa6a73dcebdfbc0d -->|refines| 694c4bb24b10f0c7;
  ac914f743d73674e["Highlight Changes in Diagrams"];
  click ac914f743d73674e "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  class ac914f743d73674e requirement;
  ac914f743d73674e -->|refines| 694c4bb24b10f0c7;
  89097c1311055b72["Store Automated Diagrams in Designated Locations"];
  click 89097c1311055b72 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  class 89097c1311055b72 requirement;
  89097c1311055b72 -->|refines| 694c4bb24b10f0c7;
  c522cf4c404bdc24["Automate Diagram Generation"];
  click c522cf4c404bdc24 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#automate-diagram-generation";
  class c522cf4c404bdc24 requirement;
  c522cf4c404bdc24 -.->|deriveReqT| eed0b020b6ddeae1;
  66e9d8186acafd13["Filter Relationships by Type"];
  click 66e9d8186acafd13 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#filter-relationships-by-type";
  class 66e9d8186acafd13 requirement;
  66e9d8186acafd13 -->|refines| 694c4bb24b10f0c7;
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

When requested, the system shall automatically generate diagrams and save them to the required locations of the model, so that the diagrams are always accessible and up-to-date.

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

  43dbd0667720212c["Suggest Code Refactoring"];
  click 43dbd0667720212c "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#suggest-code-refactoring";
  class 43dbd0667720212c requirement;
  a9bc74070fc71e6d["UserStories.md/Aligning Design with Code"];
  class a9bc74070fc71e6d requirement;
  click a9bc74070fc71e6d "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#aligning-design-with-code";
  43dbd0667720212c -->|refines| a9bc74070fc71e6d;
  8c02a4fb740392b["Code Traceability"];
  click 8c02a4fb740392b "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#code-traceability";
  class 8c02a4fb740392b requirement;
  8c02a4fb740392b -->|refines| a9bc74070fc71e6d;
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

  1ddbeea0cf8eaad5["Format Consistency Enforcement"];
  click 1ddbeea0cf8eaad5 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#format-consistency-enforcement";
  class 1ddbeea0cf8eaad5 requirement;
  84c4dc11e82e8638["Model Linting"];
  class 84c4dc11e82e8638 requirement;
  click 84c4dc11e82e8638 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#model-linting";
  1ddbeea0cf8eaad5 --o|contains| 84c4dc11e82e8638;
  229bb675bac39dc9["Replace Absolute Links with Relative Links"];
  click 229bb675bac39dc9 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#replace-absolute-links-with-relative-links";
  class 229bb675bac39dc9 requirement;
  229bb675bac39dc9 --o|contains| 84c4dc11e82e8638;
  62c066a5aad4dafe["Linting Command Output"];
  click 62c066a5aad4dafe "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#linting-command-output";
  class 62c066a5aad4dafe requirement;
  28b0f9fa78937e61["Linting Command"];
  class 28b0f9fa78937e61 requirement;
  click 28b0f9fa78937e61 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#linting-command";
  62c066a5aad4dafe -->|refines| 28b0f9fa78937e61;
  7ec3cb7f400a2e8d["Validate Markdown Structure"];
  click 7ec3cb7f400a2e8d "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#validate-markdown-structure";
  class 7ec3cb7f400a2e8d requirement;
  a0f9571f6563d9d3["UserStories.md/Validating Structures"];
  class a0f9571f6563d9d3 requirement;
  click a0f9571f6563d9d3 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#validating-structures";
  7ec3cb7f400a2e8d -->|refines| a0f9571f6563d9d3;
  b5146db7aedfd66["Documentation Index HTML Integration"];
  click b5146db7aedfd66 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#documentation-index-html-integration";
  class b5146db7aedfd66 requirement;
  9019be8bfdc22b35["Generate Documentation Index"];
  class 9019be8bfdc22b35 requirement;
  click 9019be8bfdc22b35 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#generate-documentation-index";
  b5146db7aedfd66 -->|refines| 9019be8bfdc22b35;
  c75ac8fa29479ca5["UserStories.md/Managing MBSE Models"];
  class c75ac8fa29479ca5 requirement;
  click c75ac8fa29479ca5 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#managing-mbse-models";
  9019be8bfdc22b35 -.->|deriveReqT| c75ac8fa29479ca5;
  c390b990a63def2a["Validate Filesystem Structure"];
  click c390b990a63def2a "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#validate-filesystem-structure";
  class c390b990a63def2a requirement;
  c390b990a63def2a -->|refines| a0f9571f6563d9d3;
  28b0f9fa78937e61 --o|contains| 84c4dc11e82e8638;
  ee05a46627b568b7["Validate Cross-Component Dependencies"];
  click ee05a46627b568b7 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#validate-cross-component-dependencies";
  class ee05a46627b568b7 requirement;
  ee05a46627b568b7 -->|refines| a0f9571f6563d9d3;
  8a3ca9461643d887["Validate Relation Types"];
  click 8a3ca9461643d887 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#validate-relation-types";
  class 8a3ca9461643d887 requirement;
  8a3ca9461643d887 -->|refines| a0f9571f6563d9d3;
  3d2fe0b05ff9c8e3["Enhanced Validation Error Reporting"];
  click 3d2fe0b05ff9c8e3 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  class 3d2fe0b05ff9c8e3 requirement;
  3d2fe0b05ff9c8e3 -->|refines| a0f9571f6563d9d3;
  84c4dc11e82e8638 -->|refines| a0f9571f6563d9d3;
  f9182ad2999d989c["Validate Internal Consistency"];
  click f9182ad2999d989c "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#validate-internal-consistency";
  class f9182ad2999d989c requirement;
  f9182ad2999d989c -->|refines| a0f9571f6563d9d3;
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

  efa1cab60b058344["Generate Change Logs for Pull Requests"];
  click efa1cab60b058344 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#generate-change-logs-for-pull-requests";
  class efa1cab60b058344 requirement;
  1563f8454019c887["UserStories.md/Integrate with GitHub Workflows"];
  class 1563f8454019c887 requirement;
  click 1563f8454019c887 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#integrate-with-github-workflows";
  efa1cab60b058344 -->|refines| 1563f8454019c887;
  b41f362e18fb2449["Automate Pull Request Validations"];
  click b41f362e18fb2449 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#automate-pull-request-validations";
  class b41f362e18fb2449 requirement;
  b41f362e18fb2449 -->|refines| 1563f8454019c887;
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

## Provide Reports
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  40de7485b25294["Model Structure and Summaries"];
  click 40de7485b25294 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#model-structure-and-summaries";
  class 40de7485b25294 requirement;
  98d3f01fb666c41f["Model Reports"];
  class 98d3f01fb666c41f requirement;
  click 98d3f01fb666c41f "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#model-reports";
  40de7485b25294 -.->|deriveReqT| 98d3f01fb666c41f;
  db41436c9a771e21["Generate Summary Reports"];
  click db41436c9a771e21 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#generate-summary-reports";
  class db41436c9a771e21 requirement;
  9cb6618c17d19a11["UserStories.md/Provide Reports"];
  class 9cb6618c17d19a11 requirement;
  click 9cb6618c17d19a11 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#provide-reports";
  db41436c9a771e21 -->|refines| 9cb6618c17d19a11;
  98d3f01fb666c41f -->|refines| 9cb6618c17d19a11;
  2d3cfde19fc6bb79["Provide Validation Reports"];
  click 2d3cfde19fc6bb79 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#provide-validation-reports";
  class 2d3cfde19fc6bb79 requirement;
  2d3cfde19fc6bb79 -->|refines| 9cb6618c17d19a11;
  6572e9cc4d78415a["Structural Change Reports"];
  click 6572e9cc4d78415a "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#structural-change-reports";
  class 6572e9cc4d78415a requirement;
  6572e9cc4d78415a -.->|deriveReqT| 98d3f01fb666c41f;
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

  92ab92840dee10a5["Export Traceability Matrix"];
  click 92ab92840dee10a5 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#export-traceability-matrix";
  class 92ab92840dee10a5 requirement;
  c5b8a7944b6943e2["Traceability Matrix"];
  class c5b8a7944b6943e2 requirement;
  click c5b8a7944b6943e2 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#traceability-matrix";
  92ab92840dee10a5 -.->|deriveReqT| c5b8a7944b6943e2;
  b8997351b6f34048["Interactive Mermaid Diagrams"];
  click b8997351b6f34048 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#interactive-mermaid-diagrams";
  class b8997351b6f34048 requirement;
  b8997351b6f34048 -.->|deriveReqT| c5b8a7944b6943e2;
  2591b90e1fb90daa["UserStories.md/Trace Changes in MBSE Model"];
  class 2591b90e1fb90daa requirement;
  click 2591b90e1fb90daa "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#trace-changes-in-mbse-model";
  c5b8a7944b6943e2 --o|contains| 2591b90e1fb90daa;
  4bedae4112e254b["Include Verification Checkboxes"];
  click 4bedae4112e254b "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#include-verification-checkboxes";
  class 4bedae4112e254b requirement;
  4bedae4112e254b -->|refines| c5b8a7944b6943e2;
  9b9c33c7182d6eeb["Tracing Structural Changes"];
  click 9b9c33c7182d6eeb "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#tracing-structural-changes";
  class 9b9c33c7182d6eeb requirement;
  9b9c33c7182d6eeb -.->|deriveReqT| 2591b90e1fb90daa;
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

## Exporting Specifications
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  d9686a154fe87b2["Export HTML specifications"];
  click d9686a154fe87b2 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#export-html-specifications";
  class d9686a154fe87b2 requirement;
  3bb2096c4648e3ff["UserStories.md/Export Specifications"];
  class 3bb2096c4648e3ff requirement;
  click 3bb2096c4648e3ff "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#export-specifications";
  d9686a154fe87b2 -.->|deriveReqT| 3bb2096c4648e3ff;
```

---

### Export HTML specifications

The system shall export specifications into HTML format and save in designated output location.

#### Relations
  * derivedFrom: [UserStories.md/Export Specifications](UserStories.md#export-specifications)

---

## AI-Driven Model Suggestions
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  7fd9156eac77c270["AI Agent Context"];
  click 7fd9156eac77c270 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#ai-agent-context";
  class 7fd9156eac77c270 requirement;
  8fc677b7413bb247["UserStories.md#AI-Assisted MBSE Model Management"];
  class 8fc677b7413bb247 requirement;
  click 8fc677b7413bb247 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#ai-assisted-mbse-model-management";
  7fd9156eac77c270 -->|refines| 8fc677b7413bb247;
```

---

### AI Agent Context

The system shall provide needed context for AI agents to understand how to use reqvire and help with model suggestions.

#### Relations
  * refine: [UserStories.md#AI-Assisted MBSE Model Management](UserStories.md#ai-assisted-mbse-model-management)

---

## Trace Changes
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  c699ef8d6d1f99d1["Change Impact Analysis"];
  click c699ef8d6d1f99d1 "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserRequirements.md#change-impact-analysis";
  class c699ef8d6d1f99d1 requirement;
  2591b90e1fb90daa["UserStories.md/Trace Changes in MBSE Model"];
  class 2591b90e1fb90daa requirement;
  click 2591b90e1fb90daa "https://github.com/Reqvire/reqvire/blob/ad88ba6b828e94c93382866fefd058c011c1ac60/specifications/UserStories.md#trace-changes-in-mbse-model";
  c699ef8d6d1f99d1 --o|contains| 2591b90e1fb90daa;
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