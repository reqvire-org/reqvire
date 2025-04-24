# User Requirements

## Generate Diagrams
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  64be2a98bd80a653["Automate Diagram Generation"];
  click 64be2a98bd80a653 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#automate-diagram-generation";
  class 64be2a98bd80a653 requirement;
  37611ee8059e0f03["Visualize Model Relationships"];
  class 37611ee8059e0f03 requirement;
  click 37611ee8059e0f03 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#visualize-model-relationships";
  64be2a98bd80a653 -.->|deriveReqT| 37611ee8059e0f03;
  aaa09eb94d160979["Filter Relationships by Type"];
  click aaa09eb94d160979 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#filter-relationships-by-type";
  class aaa09eb94d160979 requirement;
  3ca57dc5dc6a2dae["UserStories.md/Generate Diagrams"];
  class 3ca57dc5dc6a2dae requirement;
  click 3ca57dc5dc6a2dae "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#generate-diagrams";
  aaa09eb94d160979 -->|refines| 3ca57dc5dc6a2dae;
  e5f8f9f127a22da["Store Automated Diagrams in Designated Locations"];
  click e5f8f9f127a22da "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  class e5f8f9f127a22da requirement;
  e5f8f9f127a22da -->|refines| 3ca57dc5dc6a2dae;
  a5bfec80a94dcd8b["Select Custom Diagram Viewpoints"];
  click a5bfec80a94dcd8b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#select-custom-diagram-viewpoints";
  class a5bfec80a94dcd8b requirement;
  a5bfec80a94dcd8b -->|refines| 3ca57dc5dc6a2dae;
  3e72f83cabd0bad8["Export Diagrams in Standard Formats"];
  click 3e72f83cabd0bad8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  class 3e72f83cabd0bad8 requirement;
  3e72f83cabd0bad8 --o|contains| 3ca57dc5dc6a2dae;
  d3a1b6b68298a744["Highlight Changes in Diagrams"];
  click d3a1b6b68298a744 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  class d3a1b6b68298a744 requirement;
  d3a1b6b68298a744 -->|refines| 3ca57dc5dc6a2dae;
  37611ee8059e0f03 -->|refines| 3ca57dc5dc6a2dae;
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

  3e1922f7db183ab0["Suggest Code Refactoring"];
  click 3e1922f7db183ab0 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#suggest-code-refactoring";
  class 3e1922f7db183ab0 requirement;
  8bcbfc64008114c9["UserStories.md/Aligning Design with Code"];
  class 8bcbfc64008114c9 requirement;
  click 8bcbfc64008114c9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#aligning-design-with-code";
  3e1922f7db183ab0 -->|refines| 8bcbfc64008114c9;
  ee212ffe5248cf1a["Code Traceability"];
  click ee212ffe5248cf1a "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#code-traceability";
  class ee212ffe5248cf1a requirement;
  ee212ffe5248cf1a -->|refines| 8bcbfc64008114c9;
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

  d5b992a94f4d6669["Validate Relation Types"];
  click d5b992a94f4d6669 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-relation-types";
  class d5b992a94f4d6669 requirement;
  725d90f6f42e9407["UserStories.md/Validating Structures"];
  class 725d90f6f42e9407 requirement;
  click 725d90f6f42e9407 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#validating-structures";
  d5b992a94f4d6669 -->|refines| 725d90f6f42e9407;
  d7e3e4aadbe49925["Replace Absolute Links with Relative Links"];
  click d7e3e4aadbe49925 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#replace-absolute-links-with-relative-links";
  class d7e3e4aadbe49925 requirement;
  a479ae0b8d8c4fce["Model Linting"];
  class a479ae0b8d8c4fce requirement;
  click a479ae0b8d8c4fce "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#model-linting";
  d7e3e4aadbe49925 --o|contains| a479ae0b8d8c4fce;
  b692557f47cee0f7["Format Consistency Enforcement"];
  click b692557f47cee0f7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#format-consistency-enforcement";
  class b692557f47cee0f7 requirement;
  b692557f47cee0f7 --o|contains| a479ae0b8d8c4fce;
  a479ae0b8d8c4fce -->|refines| 725d90f6f42e9407;
  fe8919c53f8115d7["Linting Command Output"];
  click fe8919c53f8115d7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#linting-command-output";
  class fe8919c53f8115d7 requirement;
  51a11693af2e41fb["Linting Command"];
  class 51a11693af2e41fb requirement;
  click 51a11693af2e41fb "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#linting-command";
  fe8919c53f8115d7 -->|refines| 51a11693af2e41fb;
  9f579ceba1c84b17["Documentation Index HTML Integration"];
  click 9f579ceba1c84b17 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#documentation-index-html-integration";
  class 9f579ceba1c84b17 requirement;
  c7c34d508e89ee3c["Generate Documentation Index"];
  class c7c34d508e89ee3c requirement;
  click c7c34d508e89ee3c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#generate-documentation-index";
  9f579ceba1c84b17 -->|refines| c7c34d508e89ee3c;
  a469d82e490f4e4["UserStories.md/Managing MBSE Models"];
  class a469d82e490f4e4 requirement;
  click a469d82e490f4e4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#managing-mbse-models";
  c7c34d508e89ee3c -.->|deriveReqT| a469d82e490f4e4;
  51a11693af2e41fb --o|contains| a479ae0b8d8c4fce;
  ad1bea65795cf377["Enhanced Validation Error Reporting"];
  click ad1bea65795cf377 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  class ad1bea65795cf377 requirement;
  ad1bea65795cf377 -->|refines| 725d90f6f42e9407;
  a6c1d4d1f6866aa8["Validate Cross-Component Dependencies"];
  click a6c1d4d1f6866aa8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-cross-component-dependencies";
  class a6c1d4d1f6866aa8 requirement;
  a6c1d4d1f6866aa8 -->|refines| 725d90f6f42e9407;
  cc431fdb7d8cadde["Validate Markdown Structure"];
  click cc431fdb7d8cadde "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-markdown-structure";
  class cc431fdb7d8cadde requirement;
  cc431fdb7d8cadde -->|refines| 725d90f6f42e9407;
  c95011b2518dcd9d["Validate Filesystem Structure"];
  click c95011b2518dcd9d "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-filesystem-structure";
  class c95011b2518dcd9d requirement;
  c95011b2518dcd9d -->|refines| 725d90f6f42e9407;
  38ec9e189e6980d7["Validate Internal Consistency"];
  click 38ec9e189e6980d7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-internal-consistency";
  class 38ec9e189e6980d7 requirement;
  38ec9e189e6980d7 -->|refines| 725d90f6f42e9407;
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

  c09147d42d82485["Generate Change Logs for Pull Requests"];
  click c09147d42d82485 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#generate-change-logs-for-pull-requests";
  class c09147d42d82485 requirement;
  8edc9736fd751490["UserStories.md/Integrate with GitHub Workflows"];
  class 8edc9736fd751490 requirement;
  click 8edc9736fd751490 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#integrate-with-github-workflows";
  c09147d42d82485 -->|refines| 8edc9736fd751490;
  9f72b91320c287ce["Automate Pull Request Validations"];
  click 9f72b91320c287ce "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#automate-pull-request-validations";
  class 9f72b91320c287ce requirement;
  9f72b91320c287ce -->|refines| 8edc9736fd751490;
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

  83cf8dc3700bef91["Generate Summary Reports"];
  click 83cf8dc3700bef91 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#generate-summary-reports";
  class 83cf8dc3700bef91 requirement;
  a8ca33c2b9a1e9de["UserStories.md/Provide Reports"];
  class a8ca33c2b9a1e9de requirement;
  click a8ca33c2b9a1e9de "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#provide-reports";
  83cf8dc3700bef91 -->|refines| a8ca33c2b9a1e9de;
  7aea0316c5a26622["Structural Change Reports"];
  click 7aea0316c5a26622 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#structural-change-reports";
  class 7aea0316c5a26622 requirement;
  14b6d75a44a408f8["Model Reports"];
  class 14b6d75a44a408f8 requirement;
  click 14b6d75a44a408f8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#model-reports";
  7aea0316c5a26622 -.->|deriveReqT| 14b6d75a44a408f8;
  738524091202242b["Model Structure and Summaries"];
  click 738524091202242b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#model-structure-and-summaries";
  class 738524091202242b requirement;
  738524091202242b -.->|deriveReqT| 14b6d75a44a408f8;
  14b6d75a44a408f8 -->|refines| a8ca33c2b9a1e9de;
  a4b1fa740dda1d5["Provide Validation Reports"];
  click a4b1fa740dda1d5 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#provide-validation-reports";
  class a4b1fa740dda1d5 requirement;
  a4b1fa740dda1d5 -->|refines| a8ca33c2b9a1e9de;
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

  e42698fdbbf344aa["Tracing Structural Changes"];
  click e42698fdbbf344aa "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#tracing-structural-changes";
  class e42698fdbbf344aa requirement;
  1e57ec134f31779c["UserStories.md/Trace Changes in MBSE Model"];
  class 1e57ec134f31779c requirement;
  click 1e57ec134f31779c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#trace-changes-in-mbse-model";
  e42698fdbbf344aa -.->|deriveReqT| 1e57ec134f31779c;
  a2555e9553031f1c["Export Traceability Matrix"];
  click a2555e9553031f1c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#export-traceability-matrix";
  class a2555e9553031f1c requirement;
  25ad41b0b912092b["Traceability Matrix"];
  class 25ad41b0b912092b requirement;
  click 25ad41b0b912092b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#traceability-matrix";
  a2555e9553031f1c -.->|deriveReqT| 25ad41b0b912092b;
  27cb448230f8b6b0["Interactive Mermaid Diagrams"];
  click 27cb448230f8b6b0 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#interactive-mermaid-diagrams";
  class 27cb448230f8b6b0 requirement;
  27cb448230f8b6b0 -.->|deriveReqT| 25ad41b0b912092b;
  2ec6b6707fa20e10["Include Verification Checkboxes"];
  click 2ec6b6707fa20e10 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#include-verification-checkboxes";
  class 2ec6b6707fa20e10 requirement;
  2ec6b6707fa20e10 -->|refines| 25ad41b0b912092b;
  25ad41b0b912092b --o|contains| 1e57ec134f31779c;
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

  6a4b7e96941effda["Export HTML specifications"];
  click 6a4b7e96941effda "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#export-html-specifications";
  class 6a4b7e96941effda requirement;
  625ef9aa59da5635["UserStories.md/Export Specifications"];
  class 625ef9aa59da5635 requirement;
  click 625ef9aa59da5635 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#export-specifications";
  6a4b7e96941effda -.->|deriveReqT| 625ef9aa59da5635;
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

  b1d0919d0418c0f1["AI Agent Context"];
  click b1d0919d0418c0f1 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#ai-agent-context";
  class b1d0919d0418c0f1 requirement;
  63454899a7184acc["UserStories.md#AI-Assisted MBSE Model Management"];
  class 63454899a7184acc requirement;
  click 63454899a7184acc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#ai-assisted-mbse-model-management";
  b1d0919d0418c0f1 -->|refines| 63454899a7184acc;
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

  52aac80b9a806080["Change Impact Analysis"];
  click 52aac80b9a806080 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#change-impact-analysis";
  class 52aac80b9a806080 requirement;
  1e57ec134f31779c["UserStories.md/Trace Changes in MBSE Model"];
  class 1e57ec134f31779c requirement;
  click 1e57ec134f31779c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserStories.md#trace-changes-in-mbse-model";
  52aac80b9a806080 --o|contains| 1e57ec134f31779c;
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