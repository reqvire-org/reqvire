# User Requirements

## Generate Diagrams
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  9276544d5ee17790["Store Automated Diagrams in Designated Locations"];
  click 9276544d5ee17790 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  class 9276544d5ee17790 requirement;
  ab3c35050243abb3["UserStories.md/Generate Diagrams"];
  class ab3c35050243abb3 requirement;
  click ab3c35050243abb3 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#generate-diagrams";
  9276544d5ee17790 -->|refines| ab3c35050243abb3;
  f3450185979ff229["Filter Relationships by Type"];
  click f3450185979ff229 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#filter-relationships-by-type";
  class f3450185979ff229 requirement;
  f3450185979ff229 -->|refines| ab3c35050243abb3;
  efa8bb8c8484bb40["Highlight Changes in Diagrams"];
  click efa8bb8c8484bb40 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  class efa8bb8c8484bb40 requirement;
  efa8bb8c8484bb40 -->|refines| ab3c35050243abb3;
  4d4dad9ce307fade["Automate Diagram Generation"];
  click 4d4dad9ce307fade "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#automate-diagram-generation";
  class 4d4dad9ce307fade requirement;
  e98d18ae3a41815a["Visualize Model Relationships"];
  class e98d18ae3a41815a requirement;
  click e98d18ae3a41815a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#visualize-model-relationships";
  4d4dad9ce307fade -.->|deriveReqT| e98d18ae3a41815a;
  e98d18ae3a41815a -->|refines| ab3c35050243abb3;
  979760c5530f9260["Select Custom Diagram Viewpoints"];
  click 979760c5530f9260 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#select-custom-diagram-viewpoints";
  class 979760c5530f9260 requirement;
  979760c5530f9260 -->|refines| ab3c35050243abb3;
  ff7932724ee600f1["Export Diagrams in Standard Formats"];
  click ff7932724ee600f1 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  class ff7932724ee600f1 requirement;
  ff7932724ee600f1 --o|contains| ab3c35050243abb3;
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

  47469d8977723944["Code Traceability"];
  click 47469d8977723944 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#code-traceability";
  class 47469d8977723944 requirement;
  3aa9c1e4906c1b45["UserStories.md/Aligning Design with Code"];
  class 3aa9c1e4906c1b45 requirement;
  click 3aa9c1e4906c1b45 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#aligning-design-with-code";
  47469d8977723944 -->|refines| 3aa9c1e4906c1b45;
  ab1a0f0272cfb90b["Suggest Code Refactoring"];
  click ab1a0f0272cfb90b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#suggest-code-refactoring";
  class ab1a0f0272cfb90b requirement;
  ab1a0f0272cfb90b -->|refines| 3aa9c1e4906c1b45;
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

  b6ee889a6a1ac979["Validate Markdown Structure"];
  click b6ee889a6a1ac979 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-markdown-structure";
  class b6ee889a6a1ac979 requirement;
  e411816051b86f1c["UserStories.md/Validating Structures"];
  class e411816051b86f1c requirement;
  click e411816051b86f1c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#validating-structures";
  b6ee889a6a1ac979 -->|refines| e411816051b86f1c;
  3197771fa6f58185["Documentation Index HTML Integration"];
  click 3197771fa6f58185 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#documentation-index-html-integration";
  class 3197771fa6f58185 requirement;
  a0f45824211bff87["Generate Documentation Index"];
  class a0f45824211bff87 requirement;
  click a0f45824211bff87 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#generate-documentation-index";
  3197771fa6f58185 -->|refines| a0f45824211bff87;
  a830c3c9ac9cf1a9["Validate Cross-Component Dependencies"];
  click a830c3c9ac9cf1a9 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-cross-component-dependencies";
  class a830c3c9ac9cf1a9 requirement;
  a830c3c9ac9cf1a9 -->|refines| e411816051b86f1c;
  944bd4459db32d65["Validate Internal Consistency"];
  click 944bd4459db32d65 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-internal-consistency";
  class 944bd4459db32d65 requirement;
  944bd4459db32d65 -->|refines| e411816051b86f1c;
  b92914a9715d4d36["Validate Filesystem Structure"];
  click b92914a9715d4d36 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-filesystem-structure";
  class b92914a9715d4d36 requirement;
  b92914a9715d4d36 -->|refines| e411816051b86f1c;
  36d76b90ace3a564["Model Linting"];
  click 36d76b90ace3a564 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#model-linting";
  class 36d76b90ace3a564 requirement;
  36d76b90ace3a564 -->|refines| e411816051b86f1c;
  20193d4c951bb5d8["Enhanced Validation Error Reporting"];
  click 20193d4c951bb5d8 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  class 20193d4c951bb5d8 requirement;
  20193d4c951bb5d8 -->|refines| e411816051b86f1c;
  5862305273e75e10["Linting Command Output"];
  click 5862305273e75e10 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#linting-command-output";
  class 5862305273e75e10 requirement;
  f1de164cd9ee3fd["Linting Command"];
  class f1de164cd9ee3fd requirement;
  click f1de164cd9ee3fd "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#linting-command";
  5862305273e75e10 -->|refines| f1de164cd9ee3fd;
  f1de164cd9ee3fd --o|contains| 36d76b90ace3a564;
  43bbec4721cc6a68["Format Consistency Enforcement"];
  click 43bbec4721cc6a68 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#format-consistency-enforcement";
  class 43bbec4721cc6a68 requirement;
  43bbec4721cc6a68 --o|contains| 36d76b90ace3a564;
  46eca65b8a17dbc5["Validate Relation Types"];
  click 46eca65b8a17dbc5 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-relation-types";
  class 46eca65b8a17dbc5 requirement;
  46eca65b8a17dbc5 -->|refines| e411816051b86f1c;
  f95a63db50140538["Replace Absolute Links with Relative Links"];
  click f95a63db50140538 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#replace-absolute-links-with-relative-links";
  class f95a63db50140538 requirement;
  f95a63db50140538 --o|contains| 36d76b90ace3a564;
  1902659ed14c4615["UserStories.md/Managing MBSE Models"];
  class 1902659ed14c4615 requirement;
  click 1902659ed14c4615 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#managing-mbse-models";
  a0f45824211bff87 -.->|deriveReqT| 1902659ed14c4615;
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

  323a6281f4383168["Automate Pull Request Validations"];
  click 323a6281f4383168 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#automate-pull-request-validations";
  class 323a6281f4383168 requirement;
  dfb5ca2ca89e0152["UserStories.md/Integrate with GitHub Workflows"];
  class dfb5ca2ca89e0152 requirement;
  click dfb5ca2ca89e0152 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#integrate-with-github-workflows";
  323a6281f4383168 -->|refines| dfb5ca2ca89e0152;
  da2937c66e26572c["Generate Change Logs for Pull Requests"];
  click da2937c66e26572c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#generate-change-logs-for-pull-requests";
  class da2937c66e26572c requirement;
  da2937c66e26572c -->|refines| dfb5ca2ca89e0152;
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

  30313c48ad55b65e["Generate Summary Reports"];
  click 30313c48ad55b65e "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#generate-summary-reports";
  class 30313c48ad55b65e requirement;
  69208127f6580b16["UserStories.md/Provide Reports"];
  class 69208127f6580b16 requirement;
  click 69208127f6580b16 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#provide-reports";
  30313c48ad55b65e -->|refines| 69208127f6580b16;
  9774a66acb48024c["Model Structure and Summaries"];
  click 9774a66acb48024c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#model-structure-and-summaries";
  class 9774a66acb48024c requirement;
  b7cb385d30a22dd2["Model Reports"];
  class b7cb385d30a22dd2 requirement;
  click b7cb385d30a22dd2 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#model-reports";
  9774a66acb48024c -.->|deriveReqT| b7cb385d30a22dd2;
  e1f6859d4a4ea65b["Provide Validation Reports"];
  click e1f6859d4a4ea65b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#provide-validation-reports";
  class e1f6859d4a4ea65b requirement;
  e1f6859d4a4ea65b -->|refines| 69208127f6580b16;
  a0dc2f1f06233058["Structural Change Reports"];
  click a0dc2f1f06233058 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#structural-change-reports";
  class a0dc2f1f06233058 requirement;
  a0dc2f1f06233058 -.->|deriveReqT| b7cb385d30a22dd2;
  b7cb385d30a22dd2 -->|refines| 69208127f6580b16;
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

  33a6290c04810f44["Interactive Mermaid Diagrams"];
  click 33a6290c04810f44 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#interactive-mermaid-diagrams";
  class 33a6290c04810f44 requirement;
  681cda683cd3fa2a["Traceability Matrix"];
  class 681cda683cd3fa2a requirement;
  click 681cda683cd3fa2a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#traceability-matrix";
  33a6290c04810f44 -.->|deriveReqT| 681cda683cd3fa2a;
  c99ffeebd04e23f["UserStories.md/Trace Changes in MBSE Model"];
  class c99ffeebd04e23f requirement;
  click c99ffeebd04e23f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#trace-changes-in-mbse-model";
  681cda683cd3fa2a --o|contains| c99ffeebd04e23f;
  4dc854c91f0e4c8d["Tracing Structural Changes"];
  click 4dc854c91f0e4c8d "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#tracing-structural-changes";
  class 4dc854c91f0e4c8d requirement;
  4dc854c91f0e4c8d -.->|deriveReqT| c99ffeebd04e23f;
  ef3da3123b583d8["Export Traceability Matrix"];
  click ef3da3123b583d8 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#export-traceability-matrix";
  class ef3da3123b583d8 requirement;
  ef3da3123b583d8 -.->|deriveReqT| 681cda683cd3fa2a;
  c2864bc79bdbe487["Include Verification Checkboxes"];
  click c2864bc79bdbe487 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#include-verification-checkboxes";
  class c2864bc79bdbe487 requirement;
  c2864bc79bdbe487 -->|refines| 681cda683cd3fa2a;
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

  6424b4fd0b132482["Export HTML specifications"];
  click 6424b4fd0b132482 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#export-html-specifications";
  class 6424b4fd0b132482 requirement;
  53d8b873d743e94["UserStories.md/Export Specifications"];
  class 53d8b873d743e94 requirement;
  click 53d8b873d743e94 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#export-specifications";
  6424b4fd0b132482 -.->|deriveReqT| 53d8b873d743e94;
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

  c5fffc3cd9f22134["AI Agent Context"];
  click c5fffc3cd9f22134 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#ai-agent-context";
  class c5fffc3cd9f22134 requirement;
  86153e8e478c5ae5["UserStories.md#AI-Assisted MBSE Model Management"];
  class 86153e8e478c5ae5 requirement;
  click 86153e8e478c5ae5 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#ai-assisted-mbse-model-management";
  c5fffc3cd9f22134 -->|refines| 86153e8e478c5ae5;
```

---

### AI Agent Context

The system shall provide needed context for AI agents to understand how to use reqvire and methodology.

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

  6f4efc192ae34938["Change Impact Analysis"];
  click 6f4efc192ae34938 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#change-impact-analysis";
  class 6f4efc192ae34938 requirement;
  c99ffeebd04e23f["UserStories.md/Trace Changes in MBSE Model"];
  class c99ffeebd04e23f requirement;
  click c99ffeebd04e23f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserStories.md#trace-changes-in-mbse-model";
  6f4efc192ae34938 --o|contains| c99ffeebd04e23f;
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