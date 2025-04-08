# System Requirements

## Linting    
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  3f3d3f9ccb["Excess Whitespace Linting Implementation"];
  click 3f3d3f9ccb "Requirements.md#excess-whitespace-linting-implementation";
  class 3f3d3f9ccb requirement;
  81758bdb22["UserRequirements.md/Format Consistency Enforcement"];
  class 81758bdb22 requirement;
  click 81758bdb22 "../UserRequirements.md#format-consistency-enforcement";
  3f3d3f9ccb -.->|deriveReqT| 81758bdb22;
  bb06efd0b8["linting/whitespace.rs"];
  class bb06efd0b8 default;
  click bb06efd0b8 "../../core/src/linting/whitespace.rs";
  bb06efd0b8 -->|satisfies| 3f3d3f9ccb;
  4cd0fe733f["Excess Whitespace Detection and Correction"];
  class 4cd0fe733f verification;
  click 4cd0fe733f "../Verifications/LintingTests.md#excess-whitespace-detection-and-correction";
  4cd0fe733f -->|verifies| 3f3d3f9ccb;
  63f1cd4974["Git-Style Diff Output for Linting"];
  click 63f1cd4974 "Requirements.md#git-style-diff-output-for-linting";
  class 63f1cd4974 requirement;
  808b1863c8["UserRequirements.md/Linting Command Output"];
  class 808b1863c8 requirement;
  click 808b1863c8 "../UserRequirements.md#linting-command-output";
  63f1cd4974 -.->|deriveReqT| 808b1863c8;
  8b47697e4e["linting/mod.rs"];
  class 8b47697e4e default;
  click 8b47697e4e "../../core/src/linting/mod.rs";
  8b47697e4e -->|satisfies| 63f1cd4974;
  66aa36deca["Parallel Linting Processing"];
  click 66aa36deca "Requirements.md#parallel-linting-processing";
  class 66aa36deca requirement;
  103ddb8dc3["UserRequirements.md/Model Linting"];
  class 103ddb8dc3 requirement;
  click 103ddb8dc3 "../UserRequirements.md#model-linting";
  66aa36deca -.->|deriveReqT| 103ddb8dc3;
  8b47697e4e -->|satisfies| 66aa36deca;
  fffbb22796["CLI Lint Flag"];
  click fffbb22796 "Requirements.md#cli-lint-flag";
  class fffbb22796 requirement;
  887a7d36ca["UserRequirements.md/Linting Command Behavior"];
  class 887a7d36ca requirement;
  click 887a7d36ca "../UserRequirements.md#linting-command";
  fffbb22796 -.->|deriveReqT| 887a7d36ca;
  11ffc4632a["cli.rs"];
  class 11ffc4632a default;
  click 11ffc4632a "../../cli/src/cli.rs";
  11ffc4632a -->|satisfies| fffbb22796;
  8f2198f681["Dry Run Mode"];
  class 8f2198f681 requirement;
  click 8f2198f681 "Requirements.md#dry-run-mode";
  8f2198f681 -.->|deriveReqT| fffbb22796;
  dd0846393d["Missing Separators Linting Implementation"];
  click dd0846393d "Requirements.md#missing-separators-linting-implementation";
  class dd0846393d requirement;
  dd0846393d -.->|deriveReqT| 81758bdb22;
  9abe404e42["linting/separators.rs"];
  class 9abe404e42 default;
  click 9abe404e42 "../../core/src/linting/separators.rs";
  9abe404e42 -->|satisfies| dd0846393d;
  193cd64638["Reserved Subsections Linting Implementation"];
  click 193cd64638 "Requirements.md#reserved-subsections-linting-implementation";
  class 193cd64638 requirement;
  193cd64638 -.->|deriveReqT| 81758bdb22;
  3aaafb0eaf["linting/reserved_subsections.rs"];
  class 3aaafb0eaf default;
  click 3aaafb0eaf "../../core/src/linting/indentation.rs";
  3aaafb0eaf -->|satisfies| 193cd64638;
  8f2198f681 -.->|deriveReqT| fffbb22796;
  11ffc4632a -->|satisfies| 8f2198f681;
  4cd0fe733f -->|verifies| 8f2198f681;
  45770e9b31["File Pattern Exclusion for Linting"];
  click 45770e9b31 "Requirements.md#file-pattern-exclusion-for-linting";
  class 45770e9b31 requirement;
  cc8128cae3["Configurable Filename Exclusion Patterns"];
  class cc8128cae3 requirement;
  click cc8128cae3 "Requirements.md#configurable-filename-exclusion-patterns";
  45770e9b31 ==>|refines| cc8128cae3;
  3189923c3f["utils.rs"];
  class 3189923c3f default;
  click 3189923c3f "../../core/src/utils.rs";
  3189923c3f -->|satisfies| 45770e9b31;
  c5c85bedd1["Excluded File Relation Validation"];
  class c5c85bedd1 requirement;
  click c5c85bedd1 "Requirements.md#excluded-file-relation-validation";
  45770e9b31 -->|relates to| c5c85bedd1;
  f98ae394db["Incosistent Newlines Linting Implementation"];
  click f98ae394db "Requirements.md#incosistent-newlines-linting-implementation";
  class f98ae394db requirement;
  f98ae394db -.->|deriveReqT| 81758bdb22;
  b52472a7cc["linting/newlines.rs"];
  class b52472a7cc default;
  click b52472a7cc "../../core/src/linting/newlines.rs";
  b52472a7cc -->|satisfies| f98ae394db;
```

---

### CLI Lint Flag
The system shall provide a linting function, activated by the (--lint flag), which shall execute the linting process upon user request.

#### Relations
  * derivedFrom: [UserRequirements.md/Linting Command Behavior](../UserRequirements.md#linting-command)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)    

---

### Dry Run Mode
The system shall provide a dry run mode (--dry-run flag) for linting that shows the suggested changes without applying them, allowing users to review modifications before committing to them.

#### Details
--dry-run flag works in tandem with the main lint command flag and cannot be used standalone.

#### Relations
  * derivedFrom: [CLI Lint Flag](#cli-lint-flag)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)    

---

### Excess Whitespace Linting Implementation
The system shall detect and fix excess whitespace after element headers, subsection headers, and relation identifiers to maintain consistent formatting across all requirements documents.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/whitespace.rs](../../core/src/linting/whitespace.rs)

---

### Incosistent Newlines Linting Implementation

The system shall detect and fix excess or missing newlines before element headers, subsection headers to maintain consistent formatting across all requirements documents.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/newlines.rs](../../core/src/linting/newlines.rs)

---

### Missing Separators Linting Implementation

The system shall detect consecutive element sections that lack a separator line (---) between them and insert the separator to maintain consistent visual separation in the documentation.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/separators.rs](../../core/src/linting/separators.rs)

---

### Reserved Subsections Linting Implementation

The system shall identify and fix inconsistent indentation and bullet types in relation lists and other reserved subsections, standardizing to a consistent format across all requirements documents.

#### Relations
  * derivedFrom: [UserRequirements.md/Format Consistency Enforcement](../UserRequirements.md#format-consistency-enforcement)
  * satisfiedBy: [linting/reserved_subsections.rs](../../core/src/linting/indentation.rs)

---

### Git-Style Diff Output for Linting
The system shall display linting change suggestions in a git-style diff format, color-coded when possible, to clearly show what modifications will be or have been made to the documents.

#### Relations
  * derivedFrom: [UserRequirements.md/Linting Command Output](../UserRequirements.md#linting-command-output)
  * satisfiedBy: [linting/mod.rs](../../core/src/linting/mod.rs)

---

### Parallel Linting Processing
The system shall implement parallel processing for linting operations when possible, leveraging multi-core capabilities to improve performance on large documentation sets.

#### Relations
  * derivedFrom: [UserRequirements.md/Model Linting](../UserRequirements.md#model-linting)
  * satisfiedBy: [linting/mod.rs](../../core/src/linting/mod.rs)

---

### File Pattern Exclusion for Linting
The system shall respect configured excluded filename patterns when performing linting operations, ensuring that files intentionally excluded from processing do not receive inappropriate linting suggestions.

#### Relations
  * refine: [Configurable Filename Exclusion Patterns](#configurable-filename-exclusion-patterns)
  * satisfiedBy: [utils.rs](../../core/src/utils.rs)

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
  e7286123b1["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class e7286123b1 requirement;
  click e7286123b1 "../ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  d38ab4ad13 -.->|deriveReqT| e7286123b1;
  c87fb8d32a["config.rs"];
  class c87fb8d32a default;
  click c87fb8d32a "../../cli/src/config.rs";
  c87fb8d32a -->|satisfies| d38ab4ad13;
  97e9fdec98["External Folders Support Verification"];
  class 97e9fdec98 verification;
  click 97e9fdec98 "../Verifications/LintingTests.md#external-folders-support-verification";
  97e9fdec98 -->|verifies| d38ab4ad13;
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
  * derivedFrom: [ManagingMbseModelsRequirements.md/Support for Distributed Requirements](../ManagingMbseModelsRequirements.md#support-for-distributed-requirements)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)

---

## CLI
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  bdfd9d65e4["Detailed Error Handling and Logging"];
  click bdfd9d65e4 "Requirements.md#detailed-error-handling-and-logging";
  class bdfd9d65e4 requirement;
  7cf5cf9900["../UserRequirements.md#Enhanced Validation Error Reporting"];
  class 7cf5cf9900 requirement;
  click 7cf5cf9900 "../UserRequirements.md#enhanced-validation-error-reporting";
  bdfd9d65e4 -.->|deriveReqT| 7cf5cf9900;
  8cd8a561b4["src/error.rs"];
  class 8cd8a561b4 default;
  click 8cd8a561b4 "../../core/src/error.rs";
  8cd8a561b4 -->|satisfies| bdfd9d65e4;
  fbf9362574["Invalid Relations Test"];
  class fbf9362574 verification;
  click fbf9362574 "../Verifications/ValidationTests.md#invalid-relations-test";
  fbf9362574 -->|verifies| bdfd9d65e4;
  86e0701b6c["HTML Navigation Enhancement"];
  click 86e0701b6c "Requirements.md#html-navigation-enhancement";
  class 86e0701b6c requirement;
  e1c89b5d94["UserRequirements.md/Documentation Index HTML Integration"];
  class e1c89b5d94 requirement;
  click e1c89b5d94 "../UserRequirements.md#documentation-index-html-integration";
  86e0701b6c -.->|deriveReqT| e1c89b5d94;
  c788e5b84f["html.rs"];
  class c788e5b84f default;
  click c788e5b84f "../../core/src/html.rs";
  c788e5b84f -->|satisfies| 86e0701b6c;
  8cb11d152f["html_export.rs"];
  class 8cb11d152f default;
  click 8cb11d152f "../../core/src/html_export.rs";
  8cb11d152f -->|satisfies| 86e0701b6c;
  5870488e00["Relation Type Validation"];
  click 5870488e00 "Requirements.md#relation-type-validation";
  class 5870488e00 requirement;
  5870488e00 -.->|deriveReqT| 7cf5cf9900;
  ed4f94bde9["src/relation.rs"];
  class ed4f94bde9 default;
  click ed4f94bde9 "../../core/src/relation.rs";
  ed4f94bde9 -->|satisfies| 5870488e00;
  fbf9362574 -->|verifies| 5870488e00;
  c5c85bedd1["Excluded File Relation Validation"];
  click c5c85bedd1 "Requirements.md#excluded-file-relation-validation";
  class c5c85bedd1 requirement;
  45770e9b31["File Pattern Exclusion for Linting"];
  class 45770e9b31 requirement;
  click 45770e9b31 "Requirements.md#file-pattern-exclusion-for-linting";
  c5c85bedd1 ==>|refines| 45770e9b31;
  3b1e87396c["src/parser.rs"];
  class 3b1e87396c default;
  click 3b1e87396c "../../core/src/parser.rs";
  3b1e87396c -->|satisfies| c5c85bedd1;
  5ec6a2668b["JSON Output Format"];
  click 5ec6a2668b "Requirements.md#json-output-format";
  class 5ec6a2668b requirement;
  5ec6a2668b -.->|deriveReqT| 7cf5cf9900;
  11ffc4632a["cli.rs"];
  class 11ffc4632a default;
  click 11ffc4632a "../../cli/src/cli.rs";
  11ffc4632a -->|satisfies| 5ec6a2668b;
  8acd24c7c2["Index Generation"];
  click 8acd24c7c2 "Requirements.md#index-generation";
  class 8acd24c7c2 requirement;
  f5b5eaeb28["UserRequirements.md/Generate Documentation Index"];
  class f5b5eaeb28 requirement;
  click f5b5eaeb28 "../UserRequirements.md#generate-documentation-index";
  8acd24c7c2 -.->|deriveReqT| f5b5eaeb28;
  3ef6afafe2["index_generator.rs"];
  class 3ef6afafe2 default;
  click 3ef6afafe2 "../../core/src/index_generator.rs";
  3ef6afafe2 -->|satisfies| 8acd24c7c2;
  5a1719a264["Unstructured Documents"];
  click 5a1719a264 "Requirements.md#unstructured-documents";
  class 5a1719a264 requirement;
  daadd8e583["ManagingMbseModelsRequirements.md#Coexistence of Structured and Unstructured Documents"];
  class daadd8e583 requirement;
  click daadd8e583 "../ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  5a1719a264 -.->|deriveReqT| daadd8e583;
  c8b6ccc187["HTML Export"];
  click c8b6ccc187 "Requirements.md#html-export";
  class c8b6ccc187 requirement;
  72c7eda618["../UserRequirements.md/Export HTML specifications"];
  class 72c7eda618 requirement;
  click 72c7eda618 "../UserRequirements.md#export-html-specifications";
  c8b6ccc187 -.->|deriveReqT| 72c7eda618;
  8cb11d152f -->|satisfies| c8b6ccc187;
  7a8da8dfee["Interactive Mermaid Diagram Node Behavior"];
  click 7a8da8dfee "Requirements.md#interactive-mermaid-diagram-node-behavior";
  class 7a8da8dfee requirement;
  e867499409["UserRequirements.md/Interactive Mermaid Diagrams"];
  class e867499409 requirement;
  click e867499409 "../UserRequirements.md#interactive-mermaid-diagrams";
  7a8da8dfee -.->|deriveReqT| e867499409;
  d4240e2969["html.rs"];
  class d4240e2969 default;
  click d4240e2969 "../../core/src/diagrams.rs";
  d4240e2969 -->|satisfies| 7a8da8dfee;
  8ba9c7e059["LLM Context Command"];
  click 8ba9c7e059 "Requirements.md#llm-context-command";
  class 8ba9c7e059 requirement;
  ec56dd665a["UserRequirements.md/Provide Actionable Model Improvement Suggestions"];
  class ec56dd665a requirement;
  click ec56dd665a "../UserRequirements.md#provide-actionable-model-improvement-suggestions";
  8ba9c7e059 -.->|deriveReqT| ec56dd665a;
  4a5edefd6e["main.rs"];
  class 4a5edefd6e default;
  click 4a5edefd6e "../../cli/src/main.rs";
  4a5edefd6e -->|satisfies| 8ba9c7e059;
  98bd2bd6bd["File Content Caching for Performance"];
  click 98bd2bd6bd "Requirements.md#file-content-caching-for-performance";
  class 98bd2bd6bd requirement;
  21e4eb87cb["../ManagingMbseModelsRequirements.md#Efficient Processing"];
  class 21e4eb87cb requirement;
  click 21e4eb87cb "../ManagingMbseModelsRequirements.md#efficient-processing";
  98bd2bd6bd -.->|deriveReqT| 21e4eb87cb;
  6136c216c9["model.rs"];
  class 6136c216c9 default;
  click 6136c216c9 "../../core/src/model.rs";
  6136c216c9 -->|satisfies| 98bd2bd6bd;
```

---

### JSON Output Format

The system shall provide different output results in machine-readable JSON format to facilitate integration with CI/CD pipelines and automated reporting tools.

#### Relations
  * derivedFrom: [UserRequirements.md/Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### Index Generation

The system shall implement an IndexGenerator component that traverses the specifications directory structure and creates a hierarchical README.md file with links to documents and elements.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Documentation Index](../UserRequirements.md#generate-documentation-index)
  * satisfiedBy: [index_generator.rs](../../core/src/index_generator.rs)

---

### HTML Navigation Enhancement 

The system shall enhance the HTML generator to process README.md as a special file, adding navigation elements and ensuring it serves as the primary entry point.

#### Details

README.md file must be saved as index.html file when exported.

#### Relations
  * derivedFrom: [UserRequirements.md/Documentation Index HTML Integration](../UserRequirements.md#documentation-index-html-integration)
  * satisfiedBy: [html.rs](../../core/src/html.rs)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)

---

### LLM Context Command

The system shall provide a command-line option `--llm-context` that outputs comprehensive contextual information about ReqFlow methodology, document structure, relation types, and CLI usage to help Large Language Models understand and work with ReqFlow-based projects.

#### Relations
  * derivedFrom: [UserRequirements.md/Provide Actionable Model Improvement Suggestions](../UserRequirements.md#provide-actionable-model-improvement-suggestions)
  * satisfiedBy: [main.rs](../../cli/src/main.rs)

---

### Interactive Mermaid Diagram Node Behavior

The system shall implement interactive click behavior for Mermaid diagram nodes that redirects to the referenced element when clicked.

#### Relations
  * derivedFrom: [UserRequirements.md/Interactive Mermaid Diagrams](../UserRequirements.md#interactive-mermaid-diagrams)
  * satisfiedBy: [html.rs](../../core/src/diagrams.rs)

---

### Unstructured Documents

The system shall allow unstructured documents to be ignored during processing.
TODO: add requirment that defines a config filter out patterns.

#### Relations
  * derivedFrom: [ManagingMbseModelsRequirements.md#Coexistence of Structured and Unstructured Documents](../ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents)

---

### Relation Type Validation

The system shall validate relation types against a defined vocabulary and provide clear error messages for unsupported relation types, including suggestions for the correct relation types.

#### Relations
  * derivedFrom: [UserRequirements.md/Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [src/relation.rs](../../core/src/relation.rs)

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
  * satisfiedBy: [src/parser.rs](../../core/src/parser.rs)

---

### HTML Export

The system shall generate HTML output for all markdown files, not just requirements documents, to provide consistent representation of the entire model.

#### Relations
  * derivedFrom: [../UserRequirements.md/Export HTML specifications](../UserRequirements.md#export-html-specifications)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)

---

### Detailed Error Handling and Logging

The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Relations
  * derivedFrom: [../UserRequirements.md#Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [src/error.rs](../../core/src/error.rs)

---

### File Content Caching for Performance

The system shall cache file contents during processing to optimize performance for operations that require multiple passes through the same files.

#### Relations
  * derivedFrom: [../ManagingMbseModelsRequirements.md#Efficient Processing](../ManagingMbseModelsRequirements.md#efficient-processing)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---

## Logic
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  2737f2d770["Requirements Files Search and Detection"];
  click 2737f2d770 "Requirements.md#requirements-files-search-and-detection";
  class 2737f2d770 requirement;
  21e4eb87cb["ManagingMbseModelsRequirements.md/Efficient Processing"];
  class 21e4eb87cb requirement;
  click 21e4eb87cb "../ManagingMbseModelsRequirements.md#efficient-processing";
  2737f2d770 -.->|deriveReqT| 21e4eb87cb;
  6136c216c9["model.rs"];
  class 6136c216c9 default;
  click 6136c216c9 "../../core/src/model.rs";
  6136c216c9 -->|satisfies| 2737f2d770;
  99bed90a0d["Requirements Processing"];
  click 99bed90a0d "Requirements.md#requirements-processing";
  class 99bed90a0d requirement;
  e7286123b1["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class e7286123b1 requirement;
  click e7286123b1 "../ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  99bed90a0d -.->|deriveReqT| e7286123b1;
  6136c216c9 -->|satisfies| 99bed90a0d;
  3b1e87396c["parser.rs"];
  class 3b1e87396c default;
  click 3b1e87396c "../../core/src/parser.rs";
  3b1e87396c -->|satisfies| 99bed90a0d;
  97e9fdec98["External Folders Support Verification"];
  class 97e9fdec98 verification;
  click 97e9fdec98 "../Verifications/LintingTests.md#external-folders-support-verification";
  97e9fdec98 -->|verifies| 99bed90a0d;
```

---

### Requirements Processing

The system shall parse the 'specifications' and 'external folders' directory structure using the configured paths from reqflow.yaml to identify system elements files and their hierarchical relationships.

#### Relations
  * derivedFrom: [ManagingMbseModelsRequirements.md/Support for Distributed Requirements](../ManagingMbseModelsRequirements.md#support-for-distributed-requirements)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)  

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
  * derivedFrom: [ManagingMbseModelsRequirements.md/Efficient Processing](../ManagingMbseModelsRequirements.md#efficient-processing)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---

## Categorization Outcomes
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  191d27287e["Diagram Storage Path Configuration"];
  click 191d27287e "Requirements.md#diagram-storage-path-configuration";
  class 191d27287e requirement;
  aee397f35b["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class aee397f35b requirement;
  click aee397f35b "../UserRequirements.md#store-automated-diagrams-in-designated-locations";
  191d27287e -.->|deriveReqT| aee397f35b;
  6136c216c9["model.rs"];
  class 6136c216c9 default;
  click 6136c216c9 "../../core/src/model.rs";
  6136c216c9 -->|satisfies| 191d27287e;
  9860815d52["Visual Differential Rendering"];
  click 9860815d52 "Requirements.md#visual-differential-rendering";
  class 9860815d52 requirement;
  fd7388e379["UserRequirements.md/Highlight Changes in Diagrams"];
  class fd7388e379 requirement;
  click fd7388e379 "../UserRequirements.md#highlight-changes-in-diagrams";
  9860815d52 -.->|deriveReqT| fd7388e379;
  c826c1ee7c["SysML-Compatible Relationship Rendering"];
  click c826c1ee7c "Requirements.md#sysml-compatible-relationship-rendering";
  class c826c1ee7c requirement;
  a6a8362836["UserRequirements.md/Visualize Model Relationships"];
  class a6a8362836 requirement;
  click a6a8362836 "../UserRequirements.md#visualize-model-relationships";
  c826c1ee7c -.->|deriveReqT| a6a8362836;
  d4240e2969["diagrams.rs"];
  class d4240e2969 default;
  click d4240e2969 "../../core/src/diagrams.rs";
  d4240e2969 -->|satisfies| c826c1ee7c;
  cc8128cae3["Configurable Filename Exclusion Patterns"];
  click cc8128cae3 "Requirements.md#configurable-filename-exclusion-patterns";
  class cc8128cae3 requirement;
  ce24dbacb9["ManagingMbseModelsRequirements.md/Project Configuration with YAML"];
  class ce24dbacb9 requirement;
  click ce24dbacb9 "../ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  cc8128cae3 -.->|deriveReqT| ce24dbacb9;
  c87fb8d32a["config.rs"];
  class c87fb8d32a default;
  click c87fb8d32a "../../cli/src/config.rs";
  c87fb8d32a -->|satisfies| cc8128cae3;
  45770e9b31["File Pattern Exclusion for Linting"];
  class 45770e9b31 requirement;
  click 45770e9b31 "Requirements.md#file-pattern-exclusion-for-linting";
  cc8128cae3 -->|relates to| 45770e9b31;
  1414e7f889["Relationship Type Filter Implementation"];
  click 1414e7f889 "Requirements.md#relationship-type-filter-implementation";
  class 1414e7f889 requirement;
  30d97803eb["UserRequirements.md/Filter Relationships by Type"];
  class 30d97803eb requirement;
  click 30d97803eb "../UserRequirements.md#filter-relationships-by-type";
  1414e7f889 -.->|deriveReqT| 30d97803eb;
  a7ec66314a["Mermaid Diagram Format Conversion"];
  click a7ec66314a "Requirements.md#mermaid-diagram-format-conversion";
  class a7ec66314a requirement;
  10c00a1bd1["UserRequirements.md/Export Diagrams in Standard Formats"];
  class 10c00a1bd1 requirement;
  click 10c00a1bd1 "../UserRequirements.md#export-diagrams-in-standard-formats";
  a7ec66314a -.->|deriveReqT| 10c00a1bd1;
  d4240e2969 -->|satisfies| a7ec66314a;
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
  * derivedFrom: [ManagingMbseModelsRequirements.md/Project Configuration with YAML](../ManagingMbseModelsRequirements.md#project-configuration-with-yaml)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)

---

### Mermaid Diagram Format Conversion
The system shall implement an export functionality that converts Mermaid diagram syntax to standard image formats (PNG, SVG) using external rendering tools or APIs, with configurable resolution and quality settings.

#### Relations
  * derivedFrom: [UserRequirements.md/Export Diagrams in Standard Formats](../UserRequirements.md#export-diagrams-in-standard-formats)
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)

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
  * satisfiedBy: [diagrams.rs](../../core/src/diagrams.rs)

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
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---

## Change Tracing
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  5eca866a03["Traceability Matrix Builder Implementation"];
  click 5eca866a03 "Requirements.md#traceability-matrix-builder-implementation";
  class 5eca866a03 requirement;
  ba40352f8e["UserRequirements.md/Traceability Matrix"];
  class ba40352f8e requirement;
  click ba40352f8e "../UserRequirements.md#traceability-matrix";
  5eca866a03 -.->|deriveReqT| ba40352f8e;
  d7e5fbf806["Markdown Matrix Formatter"];
  click d7e5fbf806 "Requirements.md#markdown-matrix-formatter";
  class d7e5fbf806 requirement;
  d7e5fbf806 -.->|deriveReqT| ba40352f8e;
  b8b4018461["CLI Git Commit Hash Flag"];
  click b8b4018461 "Requirements.md#cli-git-commit-hash-flag";
  class b8b4018461 requirement;
  665b7456c9["CLI Change Impact Report Flag"];
  class 665b7456c9 requirement;
  click 665b7456c9 "Requirements.md#cli-change-impact-report-flag";
  b8b4018461 -.->|deriveReqT| 665b7456c9;
  11ffc4632a["cli.rs"];
  class 11ffc4632a default;
  click 11ffc4632a "../../cli/src/cli.rs";
  11ffc4632a -->|satisfies| b8b4018461;
  83a2343e97["Matrix File Output Handler"];
  click 83a2343e97 "Requirements.md#matrix-file-output-handler";
  class 83a2343e97 requirement;
  c4b025f6ac["UserRequirements.md/Save matrices to designated files"];
  class c4b025f6ac requirement;
  click c4b025f6ac "../UserRequirements.md#save-matrices-to-designated-files";
  83a2343e97 -.->|deriveReqT| c4b025f6ac;
  8897111f9b["Matrix Export Format Handler"];
  click 8897111f9b "Requirements.md#matrix-export-format-handler";
  class 8897111f9b requirement;
  4b7b432817["UserRequirements.md/Export Traceability Matrix"];
  class 4b7b432817 requirement;
  click 4b7b432817 "../UserRequirements.md#export-traceability-matrix";
  8897111f9b -.->|deriveReqT| 4b7b432817;
  918cc4a26d["Structural Change Analyzer"];
  class 918cc4a26d requirement;
  click 918cc4a26d "Requirements.md#structural-change-analyzer";
  665b7456c9 -.->|deriveReqT| 918cc4a26d;
  11ffc4632a -->|satisfies| 665b7456c9;
  b8b4018461 -.->|deriveReqT| 665b7456c9;
  9d6f79f601["Change Impact Command Line Interface"];
  class 9d6f79f601 requirement;
  click 9d6f79f601 "ChangeImpactRequirements.md#change-impact-command-line-interface";
  9d6f79f601 -.->|deriveReqT| 665b7456c9;
  91ebf7e73d["UserRequirements.md/Tracing Structural Changes"];
  class 91ebf7e73d requirement;
  click 91ebf7e73d "../UserRequirements.md#tracing-structural-changes";
  918cc4a26d -.->|deriveReqT| 91ebf7e73d;
  cc976e6bcd["model.rs"];
  class cc976e6bcd default;
  click cc976e6bcd "../../core/src/change_impact.rs";
  cc976e6bcd -->|satisfies| 918cc4a26d;
  665b7456c9 -.->|deriveReqT| 918cc4a26d;
```

---

### Structural Change Analyzer

The system shall implement a model change analyzer that identifies structural modifications between model versions, determines affected elements through relationship traversal, and categorizes impacts according to change propagation rules.

#### Relations
  * derivedFrom: [UserRequirements.md/Tracing Structural Changes](../UserRequirements.md#tracing-structural-changes)
  * satisfiedBy: [model.rs](../../core/src/change_impact.rs)  

---

### CLI Change Impact Report Flag

The system shall provide a change and impact report function, activated by the (--change_impact flag), which shall generate change impact report

#### Relations
  * derivedFrom: [Structural Change Analyzer](#structural-change-analyzer)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)    

---

### CLI Git Commit Hash Flag

The system shall provide a git commit hash flag  (--git_commit flag), to be used with ** CLI Change Impact Report Flag**.

#### Relations
  * derivedFrom: [CLI Change Impact Report Flag](#cli-change-impact-report-flag)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)    

---

### Traceability Matrix Builder Implementation

The system shall implement a traceability matrix builder component that extracts relationship data from the model, processes it according to configured parameters, and generates structured matrix representations showing connections between requirements and other elements.

#### Relations
  * derivedFrom: [UserRequirements.md/Traceability Matrix](../UserRequirements.md#traceability-matrix)

---

### Markdown Matrix Formatter

The system shall implement a markdown formatter for traceability matrices that produces well-structured, readable markdown tables and diagrams conforming to the ReqFlow markdown-first methodology.

#### Relations
  * derivedFrom: [UserRequirements.md/Traceability Matrix](../UserRequirements.md#traceability-matrix)

---

### Matrix File Output Handler

The system shall implement a file output handler for traceability matrices that saves generated content to designated locations with appropriate naming conventions, handles file conflicts, and maintains content consistency.

#### Relations
  * derivedFrom: [UserRequirements.md/Save matrices to designated files](../UserRequirements.md#save-matrices-to-designated-files)

---

### Matrix Export Format Handler

The system shall implement export handlers for traceability matrices that convert the internal matrix representation to various external formats including Excel-compatible CSV/XLSX and PDF, preserving structural relationships and formatting.

#### Relations
  * derivedFrom: [UserRequirements.md/Export Traceability Matrix](../UserRequirements.md#export-traceability-matrix)

---

## Validation Capabilities
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  212d1317cd["Relation Element Type Validator"];
  click 212d1317cd "Requirements.md#relation-element-type-validator";
  class 212d1317cd requirement;
  9890e1ca49["DesignSpecifications/RelationTypesRegistry.md"];
  class 9890e1ca49 default;
  click 9890e1ca49 "../DesignSpecifications/RelationTypesRegistry.md";
  212d1317cd -.->|deriveReqT| 9890e1ca49;
  fbf9362574["Invalid Relations Test"];
  class fbf9362574 verification;
  click fbf9362574 "../Verifications/ValidationTests.md#invalid-relations-test";
  fbf9362574 -->|verifies| 212d1317cd;
  887db62e0f["Markdown Structure Validator"];
  click 887db62e0f "Requirements.md#markdown-structure-validator";
  class 887db62e0f requirement;
  7b1772417b["UserRequirements.md/Validate Markdown Structure"];
  class 7b1772417b requirement;
  click 7b1772417b "../UserRequirements.md#validate-markdown-structure";
  887db62e0f -.->|deriveReqT| 7b1772417b;
  6136c216c9["model.rs"];
  class 6136c216c9 default;
  click 6136c216c9 "../../core/src/model.rs";
  6136c216c9 -->|satisfies| 887db62e0f;
  3b1e87396c["parser.rs"];
  class 3b1e87396c default;
  click 3b1e87396c "../../core/src/parser.rs";
  3b1e87396c -->|satisfies| 887db62e0f;
  c7d88aff4e["Cross-Component Dependency Validator"];
  click c7d88aff4e "Requirements.md#cross-component-dependency-validator";
  class c7d88aff4e requirement;
  6e40bf9f83["UserRequirements.md/Validate Cross-Component Dependencies"];
  class 6e40bf9f83 requirement;
  click 6e40bf9f83 "../UserRequirements.md#validate-cross-component-dependencies";
  c7d88aff4e -.->|deriveReqT| 6e40bf9f83;
  6136c216c9 -->|satisfies| c7d88aff4e;
  3b1e87396c -->|satisfies| c7d88aff4e;
  ec201a112c["Filesystem Structure Validator"];
  click ec201a112c "Requirements.md#filesystem-structure-validator";
  class ec201a112c requirement;
  d834cc4bc9["UserRequirements.md/Validate Filesystem Structure"];
  class d834cc4bc9 requirement;
  click d834cc4bc9 "../UserRequirements.md#validate-filesystem-structure";
  ec201a112c -.->|deriveReqT| d834cc4bc9;
  6136c216c9 -->|satisfies| ec201a112c;
  1c2e7c81f9["Internal Consistency Validator"];
  click 1c2e7c81f9 "Requirements.md#internal-consistency-validator";
  class 1c2e7c81f9 requirement;
  9e524ac696["UserRequirements.md/Validate Internal Consistency"];
  class 9e524ac696 requirement;
  click 9e524ac696 "../UserRequirements.md#validate-internal-consistency";
  1c2e7c81f9 -.->|deriveReqT| 9e524ac696;
  6136c216c9 -->|satisfies| 1c2e7c81f9;
  3b1e87396c -->|satisfies| 1c2e7c81f9;
```

---

### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces ReqFlow's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)
  * satisfiedBy: [model.rs](../../core/src/model.rs)    
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)

---

### Filesystem Structure Validator
The system shall implement a validator that checks the organization of files and directories against the expected ReqFlow project structure, verifying required folders exist and files are appropriately placed according to configuration settings.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Filesystem Structure](../UserRequirements.md#validate-filesystem-structure)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---

### Internal Consistency Validator
The system shall implement a consistency validator that verifies logical coherence within the model, including checking for circular dependencies, orphaned elements, and inconsistent relationship patterns, with detailed error reporting.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Internal Consistency](../UserRequirements.md#validate-internal-consistency)
  * satisfiedBy: [model.rs](../../core/src/model.rs)    
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)

---

### Cross-Component Dependency Validator
The system shall implement a specialized validator that analyzes dependencies across different model components, ensuring proper alignment between architectural layers, requirement levels, and verification elements.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Cross-Component Dependencies](../UserRequirements.md#validate-cross-component-dependencies)
  * satisfiedBy: [model.rs](../../core/src/model.rs)    
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)

---

### Relation Element Type Validator
The system shall implement validation that verifies relation endpoints have appropriate element types based on the relation type.

#### Details
- For `verifiedBy`/`verify` relations, validate that one endpoint is a requirement element and the other is a verification element
- For `satisfiedBy`/`satisfy` relations, validate that one endpoint is a requirement element and the other is an implementation element
- Relations should only connect elements of appropriate types based on the RelationTypesRegistry definition
- Warnings should be issued when relation endpoints have incompatible element types

#### Relations
  * derivedFrom: [DesignSpecifications/RelationTypesRegistry.md](../../specifications/DesignSpecifications/RelationTypesRegistry.md)

---

## Reporting Features
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  57cbef16c5["Dependency Report Generator"];
  click 57cbef16c5 "Requirements.md#dependency-report-generator";
  class 57cbef16c5 requirement;
  812d42f453["UserRequirements.md/Generate Dependency Reports"];
  class 812d42f453 requirement;
  click 812d42f453 "../UserRequirements.md#generate-dependency-reports";
  57cbef16c5 -.->|deriveReqT| 812d42f453;
  d842bc0e30["Verification Gap Analyzer"];
  click d842bc0e30 "Requirements.md#verification-gap-analyzer";
  class d842bc0e30 requirement;
  d0e9e8d143["UserRequirements.md/Generate Verifications Reports"];
  class d0e9e8d143 requirement;
  click d0e9e8d143 "../UserRequirements.md#generate-verifications-reports";
  d842bc0e30 -.->|deriveReqT| d0e9e8d143;
  6f86272134["CLI Summary Report Flag"];
  click 6f86272134 "Requirements.md#cli-summary-report-flag";
  class 6f86272134 requirement;
  cccd4e46e2["Model Summary Report Generator"];
  class cccd4e46e2 requirement;
  click cccd4e46e2 "Requirements.md#model-summary-report-generator";
  6f86272134 ==>|refines| cccd4e46e2;
  11ffc4632a["cli.rs"];
  class 11ffc4632a default;
  click 11ffc4632a "../../cli/src/cli.rs";
  11ffc4632a -->|satisfies| 6f86272134;
  b342220e0d["UserRequirements.md/Model Structure and Summaries"];
  class b342220e0d requirement;
  click b342220e0d "../UserRequirements.md#model-structure-and-summaries";
  cccd4e46e2 -.->|deriveReqT| b342220e0d;
  6292255058["model.rs"];
  class 6292255058 default;
  click 6292255058 "../../core/src/reports.rs";
  6292255058 -->|satisfies| cccd4e46e2;
  cccd4e46e2 -->|relates to| 6f86272134;
  143766be8c["Validation Report Generator"];
  click 143766be8c "Requirements.md#validation-report-generator";
  class 143766be8c requirement;
  482c757913["UserRequirements.md/Provide Validation Reports"];
  class 482c757913 requirement;
  click 482c757913 "../UserRequirements.md#provide-validation-reports";
  143766be8c -.->|deriveReqT| 482c757913;
  6136c216c9["model.rs"];
  class 6136c216c9 default;
  click 6136c216c9 "../../core/src/model.rs";
  6136c216c9 -->|satisfies| 143766be8c;
  2ccb7e5510["Report Export Formatter"];
  click 2ccb7e5510 "Requirements.md#report-export-formatter";
  class 2ccb7e5510 requirement;
  2afa7f3a20["UserRequirements.md/Export Reports to Standard Formats"];
  class 2afa7f3a20 requirement;
  click 2afa7f3a20 "../UserRequirements.md#export-reports-to-standard-formats";
  2ccb7e5510 -.->|deriveReqT| 2afa7f3a20;
```

---

### Model Summary Report Generator

The system shall implement a summary report generator that  produces comprehensive summaries of model relationships, including key metrics, element counts by type and counts.

#### Relations
  * derivedFrom: [UserRequirements.md/Model Structure and Summaries](../UserRequirements.md#model-structure-and-summaries)
  * satisfiedBy: [model.rs](../../core/src/reports.rs)

---

### CLI Summary Report Flag

The system shall provide a model summary report function, activated by the (--model-summary flag), which shall generate model summary report

#### Relations
  * refine: [Model Summary Report Generator](#model-summary-report-generator)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)    

---

### Validation Report Generator

The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Relations
  * derivedFrom: [UserRequirements.md/Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---

### Verification Gap Analyzer
The system shall implement a verification gap analyzer that identifies requirements lacking verification relationships, assesses verification coverage across the model, and produces reports highlighting verification deficiencies.

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Verifications Reports](../UserRequirements.md#generate-verifications-reports)

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