# User Requirements

## Generate Diagrams
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  81f9235dedaea6a9["Automate Diagram Generation"];
  click 81f9235dedaea6a9 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#automate-diagram-generation";
  class 81f9235dedaea6a9 requirement;
  a6a836283607bf45["Visualize Model Relationships"];
  class a6a836283607bf45 requirement;
  click a6a836283607bf45 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#visualize-model-relationships";
  81f9235dedaea6a9 -.->|deriveReqT| a6a836283607bf45;
  cf0026c2feeb1e0f["UserStories.md/Generate Diagrams"];
  class cf0026c2feeb1e0f requirement;
  click cf0026c2feeb1e0f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#generate-diagrams";
  a6a836283607bf45 -->|refines| cf0026c2feeb1e0f;
  30d97803eba68a13["Filter Relationships by Type"];
  click 30d97803eba68a13 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#filter-relationships-by-type";
  class 30d97803eba68a13 requirement;
  30d97803eba68a13 -->|refines| cf0026c2feeb1e0f;
  aee397f35b867556["Store Automated Diagrams in Designated Locations"];
  click aee397f35b867556 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  class aee397f35b867556 requirement;
  aee397f35b867556 -->|refines| cf0026c2feeb1e0f;
  30053341d874971a["Select Custom Diagram Viewpoints"];
  click 30053341d874971a "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#select-custom-diagram-viewpoints";
  class 30053341d874971a requirement;
  30053341d874971a -->|refines| cf0026c2feeb1e0f;
  10c00a1bd12fefa5["Export Diagrams in Standard Formats"];
  click 10c00a1bd12fefa5 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  class 10c00a1bd12fefa5 requirement;
  10c00a1bd12fefa5 --o|contains| cf0026c2feeb1e0f;
  fd7388e379372d7b["Highlight Changes in Diagrams"];
  click fd7388e379372d7b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  class fd7388e379372d7b requirement;
  fd7388e379372d7b -->|refines| cf0026c2feeb1e0f;
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

  1fc4e44d5fd988a6["Code Traceability"];
  click 1fc4e44d5fd988a6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#code-traceability";
  class 1fc4e44d5fd988a6 requirement;
  de2d3516cd5ef91d["UserStories.md/Aligning Design with Code"];
  class de2d3516cd5ef91d requirement;
  click de2d3516cd5ef91d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#aligning-design-with-code";
  1fc4e44d5fd988a6 -->|refines| de2d3516cd5ef91d;
  5922f3ef031b5fea["Suggest Code Refactoring"];
  click 5922f3ef031b5fea "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#suggest-code-refactoring";
  class 5922f3ef031b5fea requirement;
  5922f3ef031b5fea -->|refines| de2d3516cd5ef91d;
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

  f5b5eaeb28c5a7b1["Generate Documentation Index"];
  click f5b5eaeb28c5a7b1 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#generate-documentation-index";
  class f5b5eaeb28c5a7b1 requirement;
  852dea6cfecb47f5["UserStories.md/Managing MBSE Models"];
  class 852dea6cfecb47f5 requirement;
  click 852dea6cfecb47f5 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#managing-mbse-models";
  f5b5eaeb28c5a7b1 -.->|deriveReqT| 852dea6cfecb47f5;
  e1c89b5d94837122["Documentation Index HTML Integration"];
  click e1c89b5d94837122 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#documentation-index-html-integration";
  class e1c89b5d94837122 requirement;
  e1c89b5d94837122 -->|refines| f5b5eaeb28c5a7b1;
  e587d63764466914["Validate Relation Types"];
  click e587d63764466914 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-relation-types";
  class e587d63764466914 requirement;
  a60d88b6e2cb3842["UserStories.md/Validating Structures"];
  class a60d88b6e2cb3842 requirement;
  click a60d88b6e2cb3842 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#validating-structures";
  e587d63764466914 -->|refines| a60d88b6e2cb3842;
  103ddb8dc3242215["Model Linting"];
  click 103ddb8dc3242215 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#model-linting";
  class 103ddb8dc3242215 requirement;
  103ddb8dc3242215 -->|refines| a60d88b6e2cb3842;
  7b1772417b3ad5e["Validate Markdown Structure"];
  click 7b1772417b3ad5e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-markdown-structure";
  class 7b1772417b3ad5e requirement;
  7b1772417b3ad5e -->|refines| a60d88b6e2cb3842;
  7cf5cf9900076be6["Enhanced Validation Error Reporting"];
  click 7cf5cf9900076be6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  class 7cf5cf9900076be6 requirement;
  7cf5cf9900076be6 -->|refines| a60d88b6e2cb3842;
  9e524ac696c43a26["Validate Internal Consistency"];
  click 9e524ac696c43a26 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-internal-consistency";
  class 9e524ac696c43a26 requirement;
  9e524ac696c43a26 -->|refines| a60d88b6e2cb3842;
  6e40bf9f83a718fa["Validate Cross-Component Dependencies"];
  click 6e40bf9f83a718fa "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-cross-component-dependencies";
  class 6e40bf9f83a718fa requirement;
  6e40bf9f83a718fa -->|refines| a60d88b6e2cb3842;
  85f6854a4607b76b["Replace Absolute Links with Relative Links"];
  click 85f6854a4607b76b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#replace-absolute-links-with-relative-links";
  class 85f6854a4607b76b requirement;
  85f6854a4607b76b --o|contains| 103ddb8dc3242215;
  d834cc4bc9dbb07c["Validate Filesystem Structure"];
  click d834cc4bc9dbb07c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-filesystem-structure";
  class d834cc4bc9dbb07c requirement;
  d834cc4bc9dbb07c -->|refines| a60d88b6e2cb3842;
  808b1863c88b26c4["Linting Command Output"];
  click 808b1863c88b26c4 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#linting-command-output";
  class 808b1863c88b26c4 requirement;
  887a7d36caacab2b["Linting Command"];
  class 887a7d36caacab2b requirement;
  click 887a7d36caacab2b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#linting-command";
  808b1863c88b26c4 -->|refines| 887a7d36caacab2b;
  887a7d36caacab2b --o|contains| 103ddb8dc3242215;
  81758bdb22a3329d["Format Consistency Enforcement"];
  click 81758bdb22a3329d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#format-consistency-enforcement";
  class 81758bdb22a3329d requirement;
  81758bdb22a3329d --o|contains| 103ddb8dc3242215;
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

  b2a387077d97cf24["Generate Change Logs for Pull Requests"];
  click b2a387077d97cf24 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#generate-change-logs-for-pull-requests";
  class b2a387077d97cf24 requirement;
  ba120b7cafc5435f["UserStories.md/Integrate with GitHub Workflows"];
  class ba120b7cafc5435f requirement;
  click ba120b7cafc5435f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#integrate-with-github-workflows";
  b2a387077d97cf24 -->|refines| ba120b7cafc5435f;
  68454e6166703319["Automate Pull Request Validations"];
  click 68454e6166703319 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#automate-pull-request-validations";
  class 68454e6166703319 requirement;
  68454e6166703319 -->|refines| ba120b7cafc5435f;
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

  482c757913204fb8["Provide Validation Reports"];
  click 482c757913204fb8 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#provide-validation-reports";
  class 482c757913204fb8 requirement;
  fe32882ee273e24d["UserStories.md/Provide Reports"];
  class fe32882ee273e24d requirement;
  click fe32882ee273e24d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#provide-reports";
  482c757913204fb8 -->|refines| fe32882ee273e24d;
  32850509a034b0ee["Structural Change Reports"];
  click 32850509a034b0ee "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#structural-change-reports";
  class 32850509a034b0ee requirement;
  f8e7625d29ff6e01["Model Reports"];
  class f8e7625d29ff6e01 requirement;
  click f8e7625d29ff6e01 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#model-reports";
  32850509a034b0ee -.->|deriveReqT| f8e7625d29ff6e01;
  596c459d31db2976["Generate Summary Reports"];
  click 596c459d31db2976 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#generate-summary-reports";
  class 596c459d31db2976 requirement;
  596c459d31db2976 -->|refines| fe32882ee273e24d;
  b342220e0dc8751d["Model Structure and Summaries"];
  click b342220e0dc8751d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#model-structure-and-summaries";
  class b342220e0dc8751d requirement;
  b342220e0dc8751d -.->|deriveReqT| f8e7625d29ff6e01;
  f8e7625d29ff6e01 -->|refines| fe32882ee273e24d;
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

  e867499409ae347a["Interactive Mermaid Diagrams"];
  click e867499409ae347a "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#interactive-mermaid-diagrams";
  class e867499409ae347a requirement;
  ba40352f8e72c125["Traceability Matrix"];
  class ba40352f8e72c125 requirement;
  click ba40352f8e72c125 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#traceability-matrix";
  e867499409ae347a -.->|deriveReqT| ba40352f8e72c125;
  273dcdad7a4c7b9a["Include Verification Checkboxes"];
  click 273dcdad7a4c7b9a "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#include-verification-checkboxes";
  class 273dcdad7a4c7b9a requirement;
  273dcdad7a4c7b9a -->|refines| ba40352f8e72c125;
  4b7b4328173caada["Export Traceability Matrix"];
  click 4b7b4328173caada "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#export-traceability-matrix";
  class 4b7b4328173caada requirement;
  4b7b4328173caada -.->|deriveReqT| ba40352f8e72c125;
  36d8b2eb16113a7f["UserStories.md/Trace Changes in MBSE Model"];
  class 36d8b2eb16113a7f requirement;
  click 36d8b2eb16113a7f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#trace-changes-in-mbse-model";
  ba40352f8e72c125 --o|contains| 36d8b2eb16113a7f;
  91ebf7e73d5ac081["Tracing Structural Changes"];
  click 91ebf7e73d5ac081 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#tracing-structural-changes";
  class 91ebf7e73d5ac081 requirement;
  91ebf7e73d5ac081 -.->|deriveReqT| 36d8b2eb16113a7f;
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

  72c7eda6183f0893["Export HTML specifications"];
  click 72c7eda6183f0893 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#export-html-specifications";
  class 72c7eda6183f0893 requirement;
  100197ce818804["UserStories.md/Export Specifications"];
  class 100197ce818804 requirement;
  click 100197ce818804 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#export-specifications";
  72c7eda6183f0893 -.->|deriveReqT| 100197ce818804;
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

  5a6fc253c51845b6["AI Agent Context"];
  click 5a6fc253c51845b6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#ai-agent-context";
  class 5a6fc253c51845b6 requirement;
  2b35b7b37d52d4e6["UserStories.md#AI-Assisted MBSE Model Management"];
  class 2b35b7b37d52d4e6 requirement;
  click 2b35b7b37d52d4e6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#ai-assisted-mbse-model-management";
  5a6fc253c51845b6 -->|refines| 2b35b7b37d52d4e6;
```

---

### AI Agent Context

The system shall provide needed context for AI agents to understand how to use reqflow and methodology.

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

  bae5edae940ba590["Change Impact Analysis"];
  click bae5edae940ba590 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#change-impact-analysis";
  class bae5edae940ba590 requirement;
  36d8b2eb16113a7f["UserStories.md/Trace Changes in MBSE Model"];
  class 36d8b2eb16113a7f requirement;
  click 36d8b2eb16113a7f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserStories.md#trace-changes-in-mbse-model";
  bae5edae940ba590 --o|contains| 36d8b2eb16113a7f;
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