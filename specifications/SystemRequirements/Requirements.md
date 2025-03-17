# System Requirements

## Linting    
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;
  3efb9595f2["Indentation Consistency Linting Implementation"];
  click 3efb9595f2 "Requirements.md#indentation-consistency-linting-implementation";
  class 3efb9595f2 requirement;
  81758bdb22["UserRequirements.md/Format Consistency Enforcement"];
  class 81758bdb22 requirement;
  click 81758bdb22 "../UserRequirements.md#format-consistency-enforcement";
  3efb9595f2 -.->|deriveReqT| 81758bdb22;
  5604a53ad3["linting/indentation.rs"];
  class 5604a53ad3 default;
  click 5604a53ad3 "../../src/linting/indentation.rs";
  5604a53ad3 -->|satisfies| 3efb9595f2;
  66aa36deca["Parallel Linting Processing"];
  click 66aa36deca "Requirements.md#parallel-linting-processing";
  class 66aa36deca requirement;
  103ddb8dc3["UserRequirements.md/Model Linting"];
  class 103ddb8dc3 requirement;
  click 103ddb8dc3 "../UserRequirements.md#model-linting";
  66aa36deca -.->|deriveReqT| 103ddb8dc3;
  3c61a8f8d3["linting/mod.rs"];
  class 3c61a8f8d3 default;
  click 3c61a8f8d3 "../../src/linting/mod.rs";
  3c61a8f8d3 -->|satisfies| 66aa36deca;
  74344124a8["Index Generation During Linting"];
  click 74344124a8 "Requirements.md#index-generation-during-linting";
  class 74344124a8 requirement;
  f5b5eaeb28["UserRequirements.md/Generate Documentation Index"];
  class f5b5eaeb28 requirement;
  click f5b5eaeb28 "../UserRequirements.md#generate-documentation-index";
  74344124a8 -.->|deriveReqT| f5b5eaeb28;
  58ac33706d["linting/index_generator.rs"];
  class 58ac33706d default;
  click 58ac33706d "../../src/linting/index_generator.rs";
  58ac33706d -->|satisfies| 74344124a8;
  8f2198f681["Dry Run Mode"];
  click 8f2198f681 "Requirements.md#dry-run-mode";
  class 8f2198f681 requirement;
  fffbb22796["CLI Lint Flag"];
  class fffbb22796 requirement;
  click fffbb22796 "Requirements.md#cli-lint-flag";
  8f2198f681 -.->|deriveReqT| fffbb22796;
  6b749da146["cli.rs"];
  class 6b749da146 default;
  click 6b749da146 "../../src/cli.rs";
  6b749da146 -->|satisfies| 8f2198f681;
  887a7d36ca["UserRequirements.md/Linting Command Behavior"];
  class 887a7d36ca requirement;
  click 887a7d36ca "../UserRequirements.md#linting-command";
  fffbb22796 -.->|deriveReqT| 887a7d36ca;
  6b749da146 -->|satisfies| fffbb22796;
  3f3d3f9ccb["Excess Whitespace Linting Implementation"];
  click 3f3d3f9ccb "Requirements.md#excess-whitespace-linting-implementation";
  class 3f3d3f9ccb requirement;
  3f3d3f9ccb -.->|deriveReqT| 81758bdb22;
  defc3a45e2["linting/whitespace.rs"];
  class defc3a45e2 default;
  click defc3a45e2 "../../src/linting/whitespace.rs";
  defc3a45e2 -->|satisfies| 3f3d3f9ccb;
  6e1435ad10["Inconsistent Newlines Linting Implementation"];
  click 6e1435ad10 "Requirements.md#inconsistent-newlines-linting-implementation";
  class 6e1435ad10 requirement;
  6e1435ad10 -.->|deriveReqT| 81758bdb22;
  adb880afe2["linting/newlines.rs"];
  class adb880afe2 default;
  click adb880afe2 "../../src/linting/newlines.rs";
  adb880afe2 -->|satisfies| 6e1435ad10;
  63f1cd4974["Git-Style Diff Output for Linting"];
  click 63f1cd4974 "Requirements.md#git-style-diff-output-for-linting";
  class 63f1cd4974 requirement;
  808b1863c8["UserRequirements.md/Linting Command Output"];
  class 808b1863c8 requirement;
  click 808b1863c8 "../UserRequirements.md#linting-command-output";
  63f1cd4974 -.->|deriveReqT| 808b1863c8;
  3c61a8f8d3 -->|satisfies| 63f1cd4974;
  45770e9b31["File Pattern Exclusion for Linting"];
  click 45770e9b31 "Requirements.md#file-pattern-exclusion-for-linting";
  class 45770e9b31 requirement;
  cc8128cae3["Configurable Filename Exclusion Patterns"];
  class cc8128cae3 requirement;
  click cc8128cae3 "Requirements.md#configurable-filename-exclusion-patterns";
  45770e9b31 ==>|refines| cc8128cae3;
  5962484230["utils.rs"];
  class 5962484230 default;
  click 5962484230 "../../src/utils.rs";
  5962484230 -->|satisfies| 45770e9b31;
  ec14dc67ea["Automated Multiple Linting Passes"];
  click ec14dc67ea "Requirements.md#automated-multiple-linting-passes";
  class ec14dc67ea requirement;
  feb2e14e3b["Multi-Pass Linting Capability"];
  class feb2e14e3b requirement;
  click feb2e14e3b "Requirements.md#multi-pass-linting-capability";
  ec14dc67ea ==>|refines| feb2e14e3b;
  3c61a8f8d3 -->|satisfies| ec14dc67ea;
  dd0846393d["Missing Separators Linting Implementation"];
  click dd0846393d "Requirements.md#missing-separators-linting-implementation";
  class dd0846393d requirement;
  dd0846393d -.->|deriveReqT| 81758bdb22;
  b90e4bcdd6["linting/separators.rs"];
  class b90e4bcdd6 default;
  click b90e4bcdd6 "../../src/linting/separators.rs";
  b90e4bcdd6 -->|satisfies| dd0846393d;
  d93eda81e3["Directory Structure Processing"];
  click d93eda81e3 "Requirements.md#directory-structure-processing";
  class d93eda81e3 requirement;
  ec70b05609["Index Generator Implementation"];
  class ec70b05609 requirement;
  click ec70b05609 "Requirements.md#index-generator-implementation";
  d93eda81e3 ==>|refines| ec70b05609;
  58ac33706d -->|satisfies| d93eda81e3;
```

### CLI Lint Flag
The system shall provide a linting function, activated by the (--lint flag), which shall execute the linting process upon user request.

#### Relations
  * derivedFrom: [UserRequirements.md/Linting Command Behavior](../UserRequirements.md#linting-command)
  * satisfiedBy: [cli.rs](../../src/cli.rs)    
---
  
### Dry Run Mode
The system shall provide a dry run mode (--dry-run flag) for linting that shows the suggested changes without applying them, allowing users to review modifications before committing to them.

#### Details
--dry-run flag works in tandem with the main lint command flag and cannot be used standalone.

#### Relations
  * derivedFrom: [CLI Lint Flag](#cli-lint-flag)
  * satisfiedBy: [cli.rs](../../src/cli.rs)    
---
  
### Excess Whitespace Linting Implementation
The system shall detect and fix excess whitespace after element headers, subsection headers, and relation identifiers to maintain consistent formatting across all requirements documents.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/whitespace.rs](../../src/linting/whitespace.rs)
---

### Excess Newlines Linting Implementation
The system shall detect and fix excess newlines after element headers, subsection headers to maintain consistent formatting across all requirements documents.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/whitespace.rs](../../src/linting/newlines.rs)
---

### Inconsistent Newlines Linting Implementation
The system shall identify instances where subsection headers lack proper spacing (a blank line before them) and add the necessary spacing to ensure consistent document structure.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/newlines.rs](../../src/linting/newlines.rs)
---

### Missing Separators Linting Implementation
The system shall detect consecutive element sections that lack a separator line (---) between them and insert the separator to maintain consistent visual separation in the documentation.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/separators.rs](../../src/linting/separators.rs)
---

### Indentation Consistency Linting Implementation
The system shall identify and fix inconsistent indentation and bullet types in relation lists, standardizing to a consistent format across all requirements documents.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/indentation.rs](../../src/linting/indentation.rs)
---

### Git-Style Diff Output for Linting
The system shall display linting change suggestions in a git-style diff format, color-coded when possible, to clearly show what modifications will be or have been made to the documents.

#### Relations
  * derivedFrom: [UserRequirements.md/Linting Command Output](../UserRequirements.md#linting-command-output)
  * satisfiedBy: [linting/mod.rs](../../src/linting/mod.rs)
---

### Automated Multiple Linting Passes 
The system shall support automatic multiple linting passes with a configurable iteration limit to ensure all interdependent formatting issues are resolved without requiring multiple manual invocations.

#### Relations
  * refine: [Multi-Pass Linting Capability](#multi-pass-linting-capability)
  * satisfiedBy: [linting/mod.rs](../../src/linting/mod.rs)
---

### Parallel Linting Processing
The system shall implement parallel processing for linting operations when possible, leveraging multi-core capabilities to improve performance on large documentation sets.

#### Relations
  * derivedFrom: [UserRequirements.md/Model Linting](../UserRequirements.md#model-linting)
  * satisfiedBy: [linting/mod.rs](../../src/linting/mod.rs)
---

### File Pattern Exclusion for Linting
The system shall respect configured excluded filename patterns when performing linting operations, ensuring that files intentionally excluded from processing do not receive inappropriate linting suggestions.

#### Relations
  * refine: [Configurable Filename Exclusion Patterns](#configurable-filename-exclusion-patterns)
  * satisfiedBy: [utils.rs](../../src/utils.rs)
---

### Index Generation During Linting
The system shall generate or update an index.md file in the specifications root directory when linting is performed, creating a structured table of contents that links to all documentation files.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Documentation Index](../UserRequirements.md#generate-documentation-index)
  * satisfiedBy: [linting/index_generator.rs](../../src/linting/index_generator.rs)
---

### Directory Structure Processing
The system shall parse the 'specifications' and 'external folders' directory structure using the configured paths from reqflow.yaml to identify documentation files and their hierarchical relationships.

#### Details
TODO: this needs to be more clear on what folders to walk

#### Relations
  * refine: [Index Generator Implementation](#index-generator-implementation)
  * satisfiedBy: [linting/index_generator.rs](../../src/linting/index_generator.rs)
---

## Configuration
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;
  d38ab4ad13["External Folders Support"];
  click d38ab4ad13 "Requirements.md#external-folders-support";
  class d38ab4ad13 requirement;
  fc49f77257["UserRequirements.md/Support for Distributed Requirements"];
  class fc49f77257 requirement;
  click fc49f77257 "../UserRequirements.md#support-for-distributed-requirements";
  d38ab4ad13 -.->|deriveReqT| fc49f77257;
  5adb6eb451["config.rs"];
  class 5adb6eb451 default;
  click 5adb6eb451 "../../src/config.rs";
  5adb6eb451 -->|satisfies| d38ab4ad13;
```
---

### External Folders Support
The system shall implement configuration parameter to support processing requirements stored in external folders outside the main specifications directory structure, treating them as system requirements in diagram generation and validation.

#### Details
'paths.external_folders' parameter of type  Vec<String> defines additional external folders that contain system requirements and other files.
These can be absolute paths or paths relative to the 'specifications' folder but must not be subfolders of 'specifications' folder.
Empty list is allowed.
All markdown files in these folders are considered **system requirements** (except those matching exclusion patterns).
```reqflow.yaml
paths:
  external_folders:
    - /path/to/folder1
    - ../../folder2 
```

#### Relations
  * derivedFrom: [UserRequirements.md/Support for Distributed Requirements](../UserRequirements.md#support-for-distributed-requirements)
  * satisfiedBy: [config.rs](../../src/config.rs)
---

## CLI
---

### Index Generator Implementation
The system shall implement an IndexGenerator component that traverses the specifications directory structure and creates a hierarchical index.md file with links and summaries.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Documentation Index](../UserRequirements.md#generate-documentation-index)
  * satisfiedBy: [linting/index_generator.rs](../../src/linting/index_generator.rs)
---

### Markdown Content Summary Extraction
The system shall extract summaries from the first heading and paragraph of each document to include meaningful descriptions in the generated index.

#### Relations
  * containedBy: [Index Generator Implementation](#index-generator-implementation)
  * satisfiedBy: [linting/index_generator.rs](../../src/linting/index_generator.rs)
---

### Proper Link URL Generation
The system shall generate URLs in the index file with both Markdown (.md) and HTML (.md) extensions, ensuring documentation navigation works in both formats.

#### Relations
  * containedBy: [Index Generator Implementation](#index-generator-implementation)
  * satisfiedBy: [linting/index_generator.rs](../../src/linting/index_generator.rs)
---

### HTML Navigation Enhancement 
The system shall enhance the HTML generator to process index.md as a special file, adding navigation elements and ensuring it serves as the primary entry point.

#### Relations
  * derivedFrom: [UserRequirements.md/Documentation Index HTML Integration](../UserRequirements.md#documentation-index-html-integration)
  * satisfiedBy: [html.rs](../../src/html.rs)
  * satisfiedBy: [html_export.rs](../../src/html_export.rs)
---

### LLM Context Command
The system shall provide a command-line option `--llm-context` that outputs comprehensive contextual information about ReqFlow methodology, document structure, relation types, and CLI usage to help Large Language Models understand and work with ReqFlow-based projects.

#### Relations
  * derivedFrom: [UserRequirements.md/Provide Actionable Model Improvement Suggestions](../UserRequirements.md#provide-actionable-model-improvement-suggestions)
  * satisfiedBy: [main.rs](../../src/main.rs)
---

### JSON Validation Output Format
The system shall provide validation results in machine-readable JSON format to facilitate integration with CI/CD pipelines and automated reporting tools.

#### Relations
  * derivedFrom: [UserRequirements.md/Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [parser.rs](../../src/parser.rs)
---

### Multiple Validation Modes Support
The system shall support different validation modes (validate_markdown, validate_relations, validate_all) with configurable behaviors to accommodate different use cases.

#### Relations
  * derivedFrom: [UserRequirements.md/Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [parser.rs](../../src/parser.rs)
---

### Interactive Mermaid Diagram Node Behavior
The system shall implement interactive click behavior for Mermaid diagram nodes that redirects to the referenced element when clicked.

#### Relations
  * derivedFrom: [UserRequirements.md/Interactive Mermaid Diagrams](../UserRequirements.md#interactive-mermaid-diagrams)
  * satisfiedBy: [html.rs](../../src/html.rs)
  * satisfiedBy: [html_export.rs](../../src/html_export.rs)
---

### Command Line Configuration Overrides
The system shall allow command line arguments to override YAML configuration settings to provide flexibility without modifying configuration files.

#### Relations
  * derivedFrom: [UserRequirements.md/Project Configuration with YAML](../UserRequirements.md#project-configuration-with-yaml)
  * satisfiedBy: [main.rs](../../src/main.rs)
---

### Unstructured Documents
The system shall allow unstructured documents to be ignored during processing.
TODO: add requirment that defines a config filter out patterns.

#### Relations
  * derivedFrom: [UserRequirements.md#Unstructured Documents](../UserRequirements.md#unstructured-documents)
---

### Relation Type Validation
The system shall validate relation types against a defined vocabulary and provide clear error messages for unsupported relation types, including suggestions for the correct relation types.

#### Relations
  * derivedFrom: [UserRequirements.md/Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [src/relation.rs](../../src/relation.rs)
---

### Excluded File Relation Validation
The system shall properly validate relations targeting files matching excluded filename patterns, enabling references to excluded files while still respecting their exclusion from processing and linting operations.

#### Details
The validation process for excluded files:
1. Files matching excluded patterns are registered in the element registry for relation validation only
2. Internal elements within excluded files are not processed or validated

#### Todo
  * derivedFrom: [Configurable Filename Exclusion Patterns](#configurable-filename-exclusion-patterns) 
  * refine: [File Pattern Exclusion for Linting](#file-pattern-exclusion-for-linting)
  
#### Relations
  * refine: [File Pattern Exclusion for Linting](#file-pattern-exclusion-for-linting)
  * satisfiedBy: [src/parser.rs](../../src/parser.rs)
---

### Multi-Pass Linting Capability
The system shall support multi-pass linting with a configurable iteration limit to ensure all interdependent formatting issues are resolved.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/mod.rs](../../src/linting/mod.rs)
---

### HTML Export
The system shall generate HTML output for all markdown files, not just requirements documents, to provide consistent representation of the entire model.

#### Relations
  * derivedFrom: [../UserRequirements.md/Export HTML specifications](../UserRequirements.md#export-html-specifications)
  * satisfiedBy: [html_export.rs](../../src/html_export.rs)
---

### Detailed Error Handling and Logging
The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Relations
  * derivedFrom: [../UserRequirements.md#Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [src/error.rs](../../src/error.rs)
---

### File Content Caching for Performance
The system shall cache file contents during processing to optimize performance for operations that require multiple passes through the same files.

#### Relations
  * derivedFrom: [../UserRequirements.md#Efficient Processing](../UserRequirements.md#efficient-processing)
  * satisfiedBy: [model.rs](../../src/model.rs)
  
  
---

## Logic
---

### Requirements Processing
The system shall find and process all requirements in  'external_folders' and it's subfolders' consistently and 'system requirements'.

#### Relations
  * derivedFrom: [UserRequirements.md/Support for Distributed Requirements](../UserRequirements.md#support-for-distributed-requirements)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Requirements Files Search and Detection
The system shall identify and categorize files in the `specifications` and `external_folders` directories as **Stakeholder Needs Requirements**, **System Requirements**, or **Not a Requirements File** based on predefined rules.

#### Details
Identification Process:
The process follows these steps to determine whether a file contains requirements and, if so, whether it falls under **Stakeholder Needs Requirements** or **System Requirements**:
1. **File Selection**: The process scans all files in the `specifications` folder (including subfolders) and `external_folders`.
2. **Excluded Patterns Check**: If a file matches any excluded patterns, it is marked as **not a requirements file**.
3. **File Extension Check**: If the file does not have a `.md` extension, it is marked as **not a requirements file**.
4. **Design Folder Check**: If the file is located in a folder defined by the `design_specifications_folder` configuration, it is marked as **not a requirements file**.
5. **External Folder Check**: If the file is found in an external folder (or any of its subfolders), it is categorized as a **System Requirements file**.
6. **Root Folder Check**: If the file is located at the root of the `specifications` folder, it is categorized as a **Stakeholder Needs Requirements file**. Otherwise, it is categorized as a **System Requirements file**.

#### Relations
  * satisfiedBy: [model.rs](../../src/model.rs)

## Categorization Outcomes
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;
  1414e7f889["Relationship Type Filter Implementation"];
  click 1414e7f889 "Requirements.md#relationship-type-filter-implementation";
  class 1414e7f889 requirement;
  30d97803eb["UserRequirements.md/Filter Relationships by Type"];
  class 30d97803eb requirement;
  click 30d97803eb "../UserRequirements.md#filter-relationships-by-type";
  1414e7f889 -.->|deriveReqT| 30d97803eb;
  191d27287e["Diagram Storage Path Configuration"];
  click 191d27287e "Requirements.md#diagram-storage-path-configuration";
  class 191d27287e requirement;
  aee397f35b["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class aee397f35b requirement;
  click aee397f35b "../UserRequirements.md#store-automated-diagrams-in-designated-locations";
  191d27287e -.->|deriveReqT| aee397f35b;
  74b4dd6a32["model.rs"];
  class 74b4dd6a32 default;
  click 74b4dd6a32 "../../src/model.rs";
  74b4dd6a32 -->|satisfies| 191d27287e;
  a7ec66314a["Mermaid Diagram Format Conversion"];
  click a7ec66314a "Requirements.md#mermaid-diagram-format-conversion";
  class a7ec66314a requirement;
  10c00a1bd1["UserRequirements.md/Export Diagrams in Standard Formats"];
  class 10c00a1bd1 requirement;
  click 10c00a1bd1 "../UserRequirements.md#export-diagrams-in-standard-formats";
  a7ec66314a -.->|deriveReqT| 10c00a1bd1;
  d9e753e4d8["diagrams.rs"];
  class d9e753e4d8 default;
  click d9e753e4d8 "../../src/diagrams.rs";
  d9e753e4d8 -->|satisfies| a7ec66314a;
  9860815d52["Visual Differential Rendering"];
  click 9860815d52 "Requirements.md#visual-differential-rendering";
  class 9860815d52 requirement;
  fd7388e379["UserRequirements.md/Highlight Changes in Diagrams"];
  class fd7388e379 requirement;
  click fd7388e379 "../UserRequirements.md#highlight-changes-in-diagrams";
  9860815d52 -.->|deriveReqT| fd7388e379;
  cc8128cae3["Configurable Filename Exclusion Patterns"];
  click cc8128cae3 "Requirements.md#configurable-filename-exclusion-patterns";
  class cc8128cae3 requirement;
  7430e12113["UserRequirements.md/Project Configuration with YAML"];
  class 7430e12113 requirement;
  click 7430e12113 "../UserRequirements.md#project-configuration-with-yaml";
  cc8128cae3 -.->|deriveReqT| 7430e12113;
  5adb6eb451["config.rs"];
  class 5adb6eb451 default;
  click 5adb6eb451 "../../src/config.rs";
  5adb6eb451 -->|satisfies| cc8128cae3;
  c826c1ee7c["SysML-Compatible Relationship Rendering"];
  click c826c1ee7c "Requirements.md#sysml-compatible-relationship-rendering";
  class c826c1ee7c requirement;
  a6a8362836["UserRequirements.md/Visualize Model Relationships"];
  class a6a8362836 requirement;
  click a6a8362836 "../UserRequirements.md#visualize-model-relationships";
  c826c1ee7c -.->|deriveReqT| a6a8362836;
  d9e753e4d8 -->|satisfies| c826c1ee7c;
```
- **Stakeholder Needs Requirements File**: Files located in the root of the `specifications` folder and meeting all conditions.
- **System Requirements File**: Files found in external folders or subfolders within `specifications`.
- **Not a Requirements File**: Files that match exclusion patterns, do not have a `.md` extension, or are in the design folder.
---

### Configurable Filename Exclusion Patterns
The system shall support configurable glob patterns to exclude specific files from requirement processing, regardless if they are located in specifications or external folders.

#### Details
```reqflow.yaml
paths:

  # Glob patterns to exclude from requirements processing

  # These are patterns that shouldn't be considered requirements even if they're in specifications or external folders
  excluded_filename_patterns:
    - "**/Logical*.md"
    - "**/Physical*.md"    
```

#### Relations
  * derivedFrom: [UserRequirements.md/Project Configuration with YAML](../UserRequirements.md#project-configuration-with-yaml)
  * satisfiedBy: [config.rs](../../src/config.rs)
---

### Mermaid Diagram Format Conversion
The system shall implement an export functionality that converts Mermaid diagram syntax to standard image formats (PNG, SVG) using external rendering tools or APIs, with configurable resolution and quality settings.

#### Relations
  * derivedFrom: [UserRequirements.md/Export Diagrams in Standard Formats](../UserRequirements.md#export-diagrams-in-standard-formats)
  * satisfiedBy: [diagrams.rs](../../src/diagrams.rs)
---

### Visual Differential Rendering
The system shall implement a visual differential rendering algorithm that compares the current and previous versions of a diagram, visually highlighting elements that have been added, modified, or removed using distinct color coding and graphical indicators.

#### Relations
  * derivedFrom: [UserRequirements.md/Highlight Changes in Diagrams](../UserRequirements.md#highlight-changes-in-diagrams)
---

### SysML-Compatible Relationship Rendering
The system shall implement a relationship rendering engine that adheres to SysML notation standards, defining specific arrow styles, line types, and visual properties for each relationship type to ensure diagram consistency and standards compliance.

#### Details
The visual representation and direction of relationships in diagrams aligns with the SysML specification. 
Each relationship is represented using SysML standard notation with a specified arrow direction.
derive (deriveReqT):
- Definition: Indicates that a requirement is derived from a higher-level requirement.  
- Notation: Dashed arrow with an open arrowhead.  
- Direction:  
  - Derived Requirement → Source Requirement  
contains:
- Definition: Represents an element containing another element.  
- Notation: Solid arrow with no arrowhead.  
- Direction:  
  - Container Element → Contained Element  
refine:
- Definition: Indicates that an element provides further detail or enhancement of another element.  
- Notation: Dashed arrow with an open arrowhead.  
- Direction:  
  - Refining Element → Refined Element  
verify:
- Definition: Represents verification of a requirement by a test case or other verification element.  
- Notation: Dashed arrow with an open arrowhead.  
- Direction:  
  - Verification Element (Test Case, Analysis, Inspection, Demonstration) → Requirement  
trace:
- Definition: Shows a general traceability relationship between elements without implying derivation or refinement.  
- Notation: Dashed arrow with an open arrowhead.  
- Direction:  
  - Dependent Element → Source Element  
satisfy:
- Definition: Indicates that a design or model element satisfies a requirement.  
- Notation: Solid arrow with an open arrowhead.  
- Direction:  
  - Design / Model Element → Requirement  
**Summary Table**
| Relationship | Notation | Arrow Direction |
|-------------|---------|----------------|
| deriveReqT | Dashed, open arrowhead | Derived Requirement → Source Requirement |
| contain | Solid, no arrowhead | Container Element → Contained Element |
| refine | Dashed, open arrowhead | Refining Element → Refined Element |
| verify | Dashed, open arrowhead | Verification Element → Requirement |
| trace | Dashed, open arrowhead | Dependent Element → Source Element |
| satisfy | Solid, open arrowhead | Design/Model Element → Requirement |

#### Relations
  * derivedFrom: [UserRequirements.md/Visualize Model Relationships](../UserRequirements.md#visualize-model-relationships)
  * satisfiedBy: [diagrams.rs](../../src/diagrams.rs)
---

### Relationship Type Filter Implementation
The system shall implement filtering capabilities in the diagram generation logic that allow selective inclusion or exclusion of relationship types through command-line arguments and configuration settings, supporting regex patterns for complex filtering rules.

#### Relations
  * derivedFrom: [UserRequirements.md/Filter Relationships by Type](../UserRequirements.md#filter-relationships-by-type)
---

### Diagram Storage Path Configuration
The system shall implement a configurable storage mechanism for generated diagrams that uses a combination of YAML configuration settings and command-line overrides to determine storage paths, file naming patterns, and versioning strategies.

#### Relations
  * derivedFrom: [UserRequirements.md/Store Automated Diagrams in Designated Locations](../UserRequirements.md#store-automated-diagrams-in-designated-locations)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

## Traceability Matrix Generation
---

### Traceability Matrix Builder Implementation
The system shall implement a traceability matrix builder component that extracts relationship data from the model, processes it according to configured parameters, and generates structured matrix representations showing connections between requirements and other elements.

#### Relations
  * derivedFrom: [UserRequirements.md/Create Traceability Matrices](../UserRequirements.md#create-traceability-matrices)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Relation-Based Matrix View Generator
The system shall implement specialized view generators for different relationship types (VerifiedBy, SatisfiedBy, TracedFrom), each producing a focused matrix view that filters and organizes data according to the specific relationship semantics.

#### Relations
  * derivedFrom: [UserRequirements.md/Support Relation-Based Views](../UserRequirements.md#support-relation-based-views)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Markdown Matrix Formatter
The system shall implement a markdown formatter for traceability matrices that produces well-structured, readable markdown tables and diagrams conforming to the ReqFlow markdown-first methodology.

#### Relations
  * derivedFrom: [UserRequirements.md/Markdown-Based Default Format](../UserRequirements.md#markdown-based-default-format)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Matrix File Output Handler
The system shall implement a file output handler for traceability matrices that saves generated content to designated locations with appropriate naming conventions, handles file conflicts, and maintains content consistency.

#### Relations
  * derivedFrom: [UserRequirements.md/Save matrices to designated files](../UserRequirements.md#save-matrices-to-designated-files)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Verification Checkbox Implementation
The system shall implement an interactive checkbox mechanism in HTML output that allows users to mark verification statuses, storing this data in a format that persists across report regenerations.

#### Relations
  * derivedFrom: [UserRequirements.md/Include Verification Checkboxes](../UserRequirements.md#include-verification-checkboxes)
---

### Matrix Export Format Handler
The system shall implement export handlers for traceability matrices that convert the internal matrix representation to various external formats including Excel-compatible CSV/XLSX and PDF, preserving structural relationships and formatting.

#### Relations
  * derivedFrom: [UserRequirements.md/Export Traceability Matrix](../UserRequirements.md#export-traceability-matrix)
---

### CI/CD Pipeline Integration Interface
The system shall implement an interface compatible with common CI/CD platforms that allows automated generation and verification of traceability matrices as part of build and deployment processes.

#### Relations
  * derivedFrom: [UserRequirements.md/Support CI/CD Integration](../UserRequirements.md#support-ci/cd-integration)
---

## Validation Capabilities
---

### Markdown Structure Validator
The system shall implement a markdown structure validator that enforces ReqFlow's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)
  * satisfiedBy: [parser.rs](../../src/model.rs)  
  * satisfiedBy: [parser.rs](../../src/parser.rs)
---

### Filesystem Structure Validator
The system shall implement a validator that checks the organization of files and directories against the expected ReqFlow project structure, verifying required folders exist and files are appropriately placed according to configuration settings.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Filesystem Structure](../UserRequirements.md#validate-filesystem-structure)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Internal Consistency Validator
The system shall implement a consistency validator that verifies logical coherence within the model, including checking for circular dependencies, orphaned elements, and inconsistent relationship patterns, with detailed error reporting.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Internal Consistency](../UserRequirements.md#validate-internal-consistency)
  * satisfiedBy: [parser.rs](../../src/model.rs)    
  * satisfiedBy: [parser.rs](../../src/parser.rs)
---

### Cross-Component Dependency Validator
The system shall implement a specialized validator that analyzes dependencies across different model components, ensuring proper alignment between architectural layers, requirement levels, and verification elements.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Cross-Component Dependencies](../UserRequirements.md#validate-cross-component-dependencies)
  * satisfiedBy: [parser.rs](../../src/model.rs)    
  * satisfiedBy: [model.rs](../../src/model.rs)
---

## Reporting Features
---

### Relationship Report Generator
The system shall implement a relationship report generator that produces comprehensive summaries of model relationships, including statistics on relationship types, connection patterns, and outlier detection with visualizations.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Relationship Reports](../UserRequirements.md#generate-relationship-reports)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Structural Change Analysis Engine
The system shall implement a change analysis engine that compares model versions to identify structural modifications, analyzes impact chains, and generates reports highlighting affected elements with severity classifications.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Structural Change Reports](../UserRequirements.md#generate-structural-change-reports)
---

### Validation Report Generator
The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Relations
  * derivedFrom: [UserRequirements.md/Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Verification Gap Analyzer
The system shall implement a verification gap analyzer that identifies requirements lacking verification relationships, assesses verification coverage across the model, and produces reports highlighting verification deficiencies.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Verifications Reports](../UserRequirements.md#generate-verifications-reports)
---

### Model Summary Report Generator
The system shall implement a summary report generator that produces high-level overviews of the model state, including key metrics, element counts by type, relationship densities, and change statistics with trend analysis.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Summary Reports](../UserRequirements.md#generate-summary-reports)
  * satisfiedBy: [model.rs](../../src/model.rs)
---

### Dependency Report Generator
The system shall implement a dependency report generator that analyzes and visualizes complex dependency chains within the model, highlighting critical paths, dependency clusters, and potential bottlenecks with impact assessments.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Dependency Reports](../UserRequirements.md#generate-dependency-reports)
---

### Report Export Formatter
The system shall implement format conversion engines for reports that transform internal report representations to standardized external formats including PDF, Excel, and HTML, preserving structural information and visual elements.

#### Relations
  * derivedFrom: [UserRequirements.md/Export Reports to Standard Formats](../UserRequirements.md#export-reports-to-standard-formats)
---

## Change Tracing
---

### Structural Change Analyzer
The system shall implement a model change analyzer that identifies structural modifications between model versions, determines affected elements through relationship traversal, and categorizes impacts according to change propagation rules.

#### Relations
  * derivedFrom: [UserRequirements.md/Tracing Structural Changes](../UserRequirements.md#tracing-structural-changes)
---

### Structural Update Recommender
The system shall implement a recommendation engine that analyzes model changes, identifies inconsistencies or incomplete modifications, and generates specific suggestions for structural updates to maintain model integrity.

#### Relations
  * derivedFrom: [UserRequirements.md/Suggest Structural Updates](../UserRequirements.md#suggest-structural-updates)
---
