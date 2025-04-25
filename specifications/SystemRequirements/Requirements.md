# System Requirements

## Linting    
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  75640c8e8822d1e7["Parallel Linting Processing"];
  click 75640c8e8822d1e7 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#parallel-linting-processing";
  class 75640c8e8822d1e7 requirement;
  36d76b90ace3a564["UserRequirements.md/Model Linting"];
  class 36d76b90ace3a564 requirement;
  click 36d76b90ace3a564 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#model-linting";
  75640c8e8822d1e7 -.->|deriveReqT| 36d76b90ace3a564;
  2d48fe5f748f81e4["linting/mod.rs"];
  class 2d48fe5f748f81e4 default;
  click 2d48fe5f748f81e4 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/linting/mod.rs";
  2d48fe5f748f81e4 -->|satisfies| 75640c8e8822d1e7;
  4efdafe79ef3057["Reserved Subsections Linting Implementation"];
  click 4efdafe79ef3057 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#reserved-subsections-linting-implementation";
  class 4efdafe79ef3057 requirement;
  43bbec4721cc6a68["UserRequirements.md/Format Consistency Enforcement"];
  class 43bbec4721cc6a68 requirement;
  click 43bbec4721cc6a68 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#format-consistency-enforcement";
  4efdafe79ef3057 -.->|deriveReqT| 43bbec4721cc6a68;
  f3496fe5f8d186a4["linting/reserved_subsections.rs"];
  class f3496fe5f8d186a4 default;
  click f3496fe5f8d186a4 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/linting/indentation.rs";
  f3496fe5f8d186a4 -->|satisfies| 4efdafe79ef3057;
  470a0057ca1c964["File Pattern Exclusion for Linting"];
  click 470a0057ca1c964 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  class 470a0057ca1c964 requirement;
  6f822bdd8c71c7c8["Configurable Filename Exclusion Patterns"];
  class 6f822bdd8c71c7c8 requirement;
  click 6f822bdd8c71c7c8 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  470a0057ca1c964 -->|refines| 6f822bdd8c71c7c8;
  6cc937ff9e3a6b59["utils.rs"];
  class 6cc937ff9e3a6b59 default;
  click 6cc937ff9e3a6b59 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/utils.rs";
  6cc937ff9e3a6b59 -->|satisfies| 470a0057ca1c964;
  c080457c84fad54c["Dry Run Mode"];
  click c080457c84fad54c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#dry-run-mode";
  class c080457c84fad54c requirement;
  cb01ebe750839cf7["CLI Lint Flag"];
  class cb01ebe750839cf7 requirement;
  click cb01ebe750839cf7 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cli-lint-flag";
  c080457c84fad54c -.->|deriveReqT| cb01ebe750839cf7;
  5fe26772336463bc["cli.rs"];
  class 5fe26772336463bc default;
  click 5fe26772336463bc "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/cli/src/cli.rs";
  5fe26772336463bc -->|satisfies| c080457c84fad54c;
  f1de164cd9ee3fd["UserRequirements.md/Linting Command Behavior"];
  class f1de164cd9ee3fd requirement;
  click f1de164cd9ee3fd "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#linting-command";
  cb01ebe750839cf7 -.->|deriveReqT| f1de164cd9ee3fd;
  5fe26772336463bc -->|satisfies| cb01ebe750839cf7;
  db7b9709bced7f7e["Missing Separators Linting Implementation"];
  click db7b9709bced7f7e "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#missing-separators-linting-implementation";
  class db7b9709bced7f7e requirement;
  db7b9709bced7f7e -.->|deriveReqT| 43bbec4721cc6a68;
  277f2d41101b4374["linting/separators.rs"];
  class 277f2d41101b4374 default;
  click 277f2d41101b4374 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/linting/separators.rs";
  277f2d41101b4374 -->|satisfies| db7b9709bced7f7e;
  5833165793ba0498["Git-Style Diff Output for Linting"];
  click 5833165793ba0498 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#git-style-diff-output-for-linting";
  class 5833165793ba0498 requirement;
  5862305273e75e10["UserRequirements.md/Linting Command Output"];
  class 5862305273e75e10 requirement;
  click 5862305273e75e10 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#linting-command-output";
  5833165793ba0498 -.->|deriveReqT| 5862305273e75e10;
  2d48fe5f748f81e4 -->|satisfies| 5833165793ba0498;
  acb338a79cfa8b2d["Incosistent Newlines Linting Implementation"];
  click acb338a79cfa8b2d "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#incosistent-newlines-linting-implementation";
  class acb338a79cfa8b2d requirement;
  acb338a79cfa8b2d -.->|deriveReqT| 43bbec4721cc6a68;
  96c9e48a25e4de33["linting/newlines.rs"];
  class 96c9e48a25e4de33 default;
  click 96c9e48a25e4de33 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/linting/newlines.rs";
  96c9e48a25e4de33 -->|satisfies| acb338a79cfa8b2d;
  87548b2599ea86c2["Excess Whitespace Linting Implementation"];
  click 87548b2599ea86c2 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  class 87548b2599ea86c2 requirement;
  87548b2599ea86c2 -.->|deriveReqT| 43bbec4721cc6a68;
  839419873727313d["linting/whitespace.rs"];
  class 839419873727313d default;
  click 839419873727313d "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/linting/whitespace.rs";
  839419873727313d -->|satisfies| 87548b2599ea86c2;
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

  13158cbf874bcf08["External Folders Support"];
  click 13158cbf874bcf08 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#external-folders-support";
  class 13158cbf874bcf08 requirement;
  1355222227e76540["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class 1355222227e76540 requirement;
  click 1355222227e76540 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  13158cbf874bcf08 -.->|deriveReqT| 1355222227e76540;
  72abc04ae358293a["config.rs"];
  class 72abc04ae358293a default;
  click 72abc04ae358293a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/cli/src/config.rs";
  72abc04ae358293a -->|satisfies| 13158cbf874bcf08;
```

---

### External Folders Support
The system shall implement configuration parameter to support processing requirements stored in external folders outside the main specifications directory structure, treating them as system requirements in diagram generation and validation.

#### Details

'paths.external_folders' parameter of type  Vec<String> defines additional external folders that contain system requirements and other files.
These can be absolute paths or paths relative to the 'specifications' folder but must not be subfolders of 'specifications' folder.
Empty list is allowed.
All markdown files in these folders are considered **system requirements** (except those matching exclusion patterns).
```reqvire.yaml
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

  34955d64b2f2498a["JSON Output Format"];
  click 34955d64b2f2498a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#json-output-format";
  class 34955d64b2f2498a requirement;
  20193d4c951bb5d8["UserRequirements.md/Enhanced Validation Error Reporting"];
  class 20193d4c951bb5d8 requirement;
  click 20193d4c951bb5d8 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  34955d64b2f2498a -.->|deriveReqT| 20193d4c951bb5d8;
  5fe26772336463bc["cli.rs"];
  class 5fe26772336463bc default;
  click 5fe26772336463bc "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/cli/src/cli.rs";
  5fe26772336463bc -->|satisfies| 34955d64b2f2498a;
  d0ab992050899c65["Detailed Error Handling and Logging"];
  click d0ab992050899c65 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  class d0ab992050899c65 requirement;
  d0ab992050899c65 -.->|deriveReqT| 20193d4c951bb5d8;
  7d8c02914ff6562f["src/error.rs"];
  class 7d8c02914ff6562f default;
  click 7d8c02914ff6562f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/error.rs";
  7d8c02914ff6562f -->|satisfies| d0ab992050899c65;
  6fda52e049427373["File Content Caching for Performance"];
  click 6fda52e049427373 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#file-content-caching-for-performance";
  class 6fda52e049427373 requirement;
  b6c8faf2d138c14a["../ManagingMbseModelsRequirements.md#Efficient Processing"];
  class b6c8faf2d138c14a requirement;
  click b6c8faf2d138c14a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  6fda52e049427373 -.->|deriveReqT| b6c8faf2d138c14a;
  2671722513a08b18["model.rs"];
  class 2671722513a08b18 default;
  click 2671722513a08b18 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/model.rs";
  2671722513a08b18 -->|satisfies| 6fda52e049427373;
  9bd5bfd010597a61["LLM Context Command"];
  click 9bd5bfd010597a61 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#llm-context-command";
  class 9bd5bfd010597a61 requirement;
  c5fffc3cd9f22134["UserRequirements.md/AI Agent Context"];
  class c5fffc3cd9f22134 requirement;
  click c5fffc3cd9f22134 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#ai-agent-context";
  9bd5bfd010597a61 -.->|deriveReqT| c5fffc3cd9f22134;
  f6af4e23e0321ea9["main.rs"];
  class f6af4e23e0321ea9 default;
  click f6af4e23e0321ea9 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/cli/src/main.rs";
  f6af4e23e0321ea9 -->|satisfies| 9bd5bfd010597a61;
  e0125fd451efef8b["HTML Export"];
  click e0125fd451efef8b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#html-export";
  class e0125fd451efef8b requirement;
  6424b4fd0b132482["../UserRequirements.md/Export HTML specifications"];
  class 6424b4fd0b132482 requirement;
  click 6424b4fd0b132482 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#export-html-specifications";
  e0125fd451efef8b -.->|deriveReqT| 6424b4fd0b132482;
  263d4f4f6c5b0e15["html_export.rs"];
  class 263d4f4f6c5b0e15 default;
  click 263d4f4f6c5b0e15 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/html_export.rs";
  263d4f4f6c5b0e15 -->|satisfies| e0125fd451efef8b;
  d0f717482d447c04["Interactive Mermaid Diagram Node Behavior"];
  click d0f717482d447c04 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#interactive-mermaid-diagram-node-behavior";
  class d0f717482d447c04 requirement;
  33a6290c04810f44["UserRequirements.md/Interactive Mermaid Diagrams"];
  class 33a6290c04810f44 requirement;
  click 33a6290c04810f44 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#interactive-mermaid-diagrams";
  d0f717482d447c04 -.->|deriveReqT| 33a6290c04810f44;
  e29a97d68903e7bb["html.rs"];
  class e29a97d68903e7bb default;
  click e29a97d68903e7bb "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/diagrams.rs";
  e29a97d68903e7bb -->|satisfies| d0f717482d447c04;
  eae22c5060352d6f["HTML Navigation Enhancement"];
  click eae22c5060352d6f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#html-navigation-enhancement";
  class eae22c5060352d6f requirement;
  3197771fa6f58185["UserRequirements.md/Documentation Index HTML Integration"];
  class 3197771fa6f58185 requirement;
  click 3197771fa6f58185 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#documentation-index-html-integration";
  eae22c5060352d6f -.->|deriveReqT| 3197771fa6f58185;
  49d2748b5e210381["html.rs"];
  class 49d2748b5e210381 default;
  click 49d2748b5e210381 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/html.rs";
  49d2748b5e210381 -->|satisfies| eae22c5060352d6f;
  263d4f4f6c5b0e15 -->|satisfies| eae22c5060352d6f;
  d6a1fbde09883ed0["Unstructured Documents"];
  click d6a1fbde09883ed0 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#unstructured-documents";
  class d6a1fbde09883ed0 requirement;
  a67f325ec6416b1d["ManagingMbseModelsRequirements.md#Coexistence of Structured and Unstructured Documents"];
  class a67f325ec6416b1d requirement;
  click a67f325ec6416b1d "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  d6a1fbde09883ed0 -.->|deriveReqT| a67f325ec6416b1d;
  ca07c1345af51729["Index Generation"];
  click ca07c1345af51729 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#index-generation";
  class ca07c1345af51729 requirement;
  a0f45824211bff87["UserRequirements.md/Generate Documentation Index"];
  class a0f45824211bff87 requirement;
  click a0f45824211bff87 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#generate-documentation-index";
  ca07c1345af51729 -.->|deriveReqT| a0f45824211bff87;
  770285ae87065aa3["index_generator.rs"];
  class 770285ae87065aa3 default;
  click 770285ae87065aa3 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/index_generator.rs";
  770285ae87065aa3 -->|satisfies| ca07c1345af51729;
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

The system shall provide a command-line option `--llm-context` that outputs comprehensive contextual information about Reqvire methodology, document structure, relation types, and CLI usage to help Large Language Models understand and work with Reqvire-based projects.

#### Relations
  * derivedFrom: [UserRequirements.md/AI Agent Context](../UserRequirements.md#ai-agent-context)
  * satisfiedBy: [main.rs](../../cli/src/main.rs)

---

### Interactive Mermaid Diagram Node Behavior

The system shall implement interactive click behavior for Mermaid diagram nodes that redirects to the referenced element when clicked, using stable git repository URLs with commit hashes when available.

#### Details

When generating diagram node links, the system shall:
- Use stable git repository links (`{repository-url}/blob/{commit-hash}/{file-path}`) when git repository information is available
- Fallback to relative HTML links when git repository information is not available
- Use the current commit hash to ensure links remain stable even as the repository evolves
- Match the same link format used in traceability matrices and change impact reports
- Preserve interactive behavior across all generated diagrams

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

  c6d19363284e9125["Requirements Processing"];
  click c6d19363284e9125 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#requirements-processing";
  class c6d19363284e9125 requirement;
  1355222227e76540["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class 1355222227e76540 requirement;
  click 1355222227e76540 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  c6d19363284e9125 -.->|deriveReqT| 1355222227e76540;
  2671722513a08b18["model.rs"];
  class 2671722513a08b18 default;
  click 2671722513a08b18 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/model.rs";
  2671722513a08b18 -->|satisfies| c6d19363284e9125;
  8af498811b4b1f17["parser.rs"];
  class 8af498811b4b1f17 default;
  click 8af498811b4b1f17 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/parser.rs";
  8af498811b4b1f17 -->|satisfies| c6d19363284e9125;
  734ff30870a0bde0["Requirements Files Search and Detection"];
  click 734ff30870a0bde0 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#requirements-files-search-and-detection";
  class 734ff30870a0bde0 requirement;
  b6c8faf2d138c14a["ManagingMbseModelsRequirements.md/Efficient Processing"];
  class b6c8faf2d138c14a requirement;
  click b6c8faf2d138c14a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  734ff30870a0bde0 -.->|deriveReqT| b6c8faf2d138c14a;
  2671722513a08b18 -->|satisfies| 734ff30870a0bde0;
```

---

### Requirements Processing

The system shall parse the 'specifications' and 'external folders' directory structure using the configured paths from reqvire.yaml to identify system elements files and their hierarchical relationships.

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

  c35d39c7f9e6808b["Diagram Storage Path Configuration"];
  click c35d39c7f9e6808b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#diagram-storage-path-configuration";
  class c35d39c7f9e6808b requirement;
  9276544d5ee17790["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class 9276544d5ee17790 requirement;
  click 9276544d5ee17790 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  c35d39c7f9e6808b -.->|deriveReqT| 9276544d5ee17790;
  2671722513a08b18["model.rs"];
  class 2671722513a08b18 default;
  click 2671722513a08b18 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/model.rs";
  2671722513a08b18 -->|satisfies| c35d39c7f9e6808b;
  5c774c80f071101f["Relationship Type Filter Implementation"];
  click 5c774c80f071101f "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#relationship-type-filter-implementation";
  class 5c774c80f071101f requirement;
  f3450185979ff229["UserRequirements.md/Filter Relationships by Type"];
  class f3450185979ff229 requirement;
  click f3450185979ff229 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#filter-relationships-by-type";
  5c774c80f071101f -.->|deriveReqT| f3450185979ff229;
  6f822bdd8c71c7c8["Configurable Filename Exclusion Patterns"];
  click 6f822bdd8c71c7c8 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  class 6f822bdd8c71c7c8 requirement;
  fbe7e0d3b82611ef["ManagingMbseModelsRequirements.md/Project Configuration with YAML"];
  class fbe7e0d3b82611ef requirement;
  click fbe7e0d3b82611ef "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  6f822bdd8c71c7c8 -.->|deriveReqT| fbe7e0d3b82611ef;
  72abc04ae358293a["config.rs"];
  class 72abc04ae358293a default;
  click 72abc04ae358293a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/cli/src/config.rs";
  72abc04ae358293a -->|satisfies| 6f822bdd8c71c7c8;
  1d5464f20f10765e["Mermaid Diagram Format Conversion"];
  click 1d5464f20f10765e "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#mermaid-diagram-format-conversion";
  class 1d5464f20f10765e requirement;
  ff7932724ee600f1["UserRequirements.md/Export Diagrams in Standard Formats"];
  class ff7932724ee600f1 requirement;
  click ff7932724ee600f1 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  1d5464f20f10765e -.->|deriveReqT| ff7932724ee600f1;
  e29a97d68903e7bb["diagrams.rs"];
  class e29a97d68903e7bb default;
  click e29a97d68903e7bb "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/diagrams.rs";
  e29a97d68903e7bb -->|satisfies| 1d5464f20f10765e;
  273cce972e603178["Automated Diagram Generation on PR Merge"];
  click 273cce972e603178 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge";
  class 273cce972e603178 requirement;
  4d4dad9ce307fade["UserRequirements.md/Automate Diagram Generation"];
  class 4d4dad9ce307fade requirement;
  click 4d4dad9ce307fade "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#automate-diagram-generation";
  273cce972e603178 -.->|deriveReqT| 4d4dad9ce307fade;
  323a6281f4383168["UserRequirements.md/Automate Pull Request Validations"];
  class 323a6281f4383168 requirement;
  click 323a6281f4383168 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#automate-pull-request-validations";
  273cce972e603178 -.->|deriveReqT| 323a6281f4383168;
  6699836c9fa45af6["generate_diagrams.yml"];
  class 6699836c9fa45af6 default;
  click 6699836c9fa45af6 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/.github/workflows/generate_diagrams.yml";
  6699836c9fa45af6 -->|satisfies| 273cce972e603178;
  4512593c8e0cc8ae["Visual Differential Rendering"];
  click 4512593c8e0cc8ae "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#visual-differential-rendering";
  class 4512593c8e0cc8ae requirement;
  efa8bb8c8484bb40["UserRequirements.md/Highlight Changes in Diagrams"];
  class efa8bb8c8484bb40 requirement;
  click efa8bb8c8484bb40 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  4512593c8e0cc8ae -.->|deriveReqT| efa8bb8c8484bb40;
  b7e7ff82d313249["SysML-Compatible Relationship Rendering"];
  click b7e7ff82d313249 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#sysml-compatible-relationship-rendering";
  class b7e7ff82d313249 requirement;
  e98d18ae3a41815a["UserRequirements.md/Visualize Model Relationships"];
  class e98d18ae3a41815a requirement;
  click e98d18ae3a41815a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#visualize-model-relationships";
  b7e7ff82d313249 -.->|deriveReqT| e98d18ae3a41815a;
  e29a97d68903e7bb -->|satisfies| b7e7ff82d313249;
```
- **Stakeholder Needs Requirements File**: Files located in the root of the `specifications` folder and meeting all conditions.
- **System Requirements File**: Files found in external folders or subfolders within `specifications`.
- **Not a Requirements File**: Files that match exclusion patterns, do not have a `.md` extension, or are in the design folder.

---

### Automated Diagram Generation on PR Merge

The system shall implement a GitHub workflow that automatically generates and commits updated diagrams when pull requests are merged to the main branch.

#### Details

The GitHub workflow shall:
- Be triggered only when a pull request is merged to the main branch (not on PR creation or updates)
- Check out the latest code from the main branch post-merge
- Build the Reqvire tool from source
- Run the diagram generation process using the `--generate-diagrams` flag
- Generate a traceability matrix SVG using the `--traces --svg` flags
- Check if any diagrams or matrix files have been added or modified
- Commit any updated files with a standardized commit message
- Push the updates back to the main branch

This ensures that the Mermaid diagrams in the repository are always up-to-date after changes are merged to the main branch, providing accurate visual representations of the latest model state without requiring manual intervention.

#### Relations
  * derivedFrom: [UserRequirements.md/Automate Diagram Generation](../UserRequirements.md#automate-diagram-generation)
  * derivedFrom: [UserRequirements.md/Automate Pull Request Validations](../UserRequirements.md#automate-pull-request-validations)
  * satisfiedBy: [generate_diagrams.yml](../../.github/workflows/generate_diagrams.yml)

---

### Configurable Filename Exclusion Patterns
The system shall support configurable glob patterns to exclude specific files from requirement processing, regardless if they are located in specifications or external folders.

#### Details
```reqvire.yaml
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
| Relation        | Stereotype                  | Line style            | Arrowhead               | Semantic direction                          |
|-----------------|-----------------------------|-----------------------|-------------------------|-------------------------------------------- |
| **deriveReqT**  | «deriveReqt»                | dashed dependency     | open (hollow) arrowhead | Derived Requirement → Source Requirement   |
| **satisfy**     | «satisfy»                   | solid dependency      | open (hollow) arrowhead | Design/Model Element → Requirement          |
| **verify**      | «verify»                    | dashed dependency     | open (hollow) arrowhead | Verification Element → Requirement          |
| **refine**      | «refine»                    | solid dependency      | open (hollow) arrowhead | Refining Element → Refined Element          |
| **trace**       | «trace»                     | dashed dependency     | open (hollow) arrowhead | Dependent Element → Source Element          |
| **containment** | «contain» / «containedBy»¹  | composite association | filled (black) diamond  | Container Element → Contained Element       |

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

  7a56a2d0b94cbc94["CLI Traces Flag"];
  click 7a56a2d0b94cbc94 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cli-traces-flag";
  class 7a56a2d0b94cbc94 requirement;
  681cda683cd3fa2a["UserRequirements.md/Traceability Matrix"];
  class 681cda683cd3fa2a requirement;
  click 681cda683cd3fa2a "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#traceability-matrix";
  7a56a2d0b94cbc94 -.->|deriveReqT| 681cda683cd3fa2a;
  5fe26772336463bc["cli.rs"];
  class 5fe26772336463bc default;
  click 5fe26772336463bc "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/cli/src/cli.rs";
  5fe26772336463bc -->|satisfies| 7a56a2d0b94cbc94;
  dae7b02b70487825["CLI Traces SVG Flag"];
  click dae7b02b70487825 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cli-traces-svg-flag";
  class dae7b02b70487825 requirement;
  dae7b02b70487825 -.->|deriveReqT| 7a56a2d0b94cbc94;
  5fe26772336463bc -->|satisfies| dae7b02b70487825;
  9a465de9fd86cbf3["CLI Change Impact Report Flag"];
  click 9a465de9fd86cbf3 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cli-change-impact-report-flag";
  class 9a465de9fd86cbf3 requirement;
  8911202d5ac22fa9["Structural Change Analyzer"];
  class 8911202d5ac22fa9 requirement;
  click 8911202d5ac22fa9 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#structural-change-analyzer";
  9a465de9fd86cbf3 -.->|deriveReqT| 8911202d5ac22fa9;
  5fe26772336463bc -->|satisfies| 9a465de9fd86cbf3;
  7d76667e6a943402["CLI Git Commit Hash Flag"];
  click 7d76667e6a943402 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cli-git-commit-hash-flag";
  class 7d76667e6a943402 requirement;
  7d76667e6a943402 -.->|deriveReqT| 9a465de9fd86cbf3;
  5fe26772336463bc -->|satisfies| 7d76667e6a943402;
  4dc854c91f0e4c8d["UserRequirements.md/Tracing Structural Changes"];
  class 4dc854c91f0e4c8d requirement;
  click 4dc854c91f0e4c8d "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#tracing-structural-changes";
  8911202d5ac22fa9 -.->|deriveReqT| 4dc854c91f0e4c8d;
  3a36b9f74d4d889b["model.rs"];
  class 3a36b9f74d4d889b default;
  click 3a36b9f74d4d889b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/change_impact.rs";
  3a36b9f74d4d889b -->|satisfies| 8911202d5ac22fa9;
  c82329bbc603aed3["Traceability Matrix Builder Implementation"];
  click c82329bbc603aed3 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#traceability-matrix-builder-implementation";
  class c82329bbc603aed3 requirement;
  c82329bbc603aed3 -.->|deriveReqT| 681cda683cd3fa2a;
  9f97df7988362b9["matrix_generator.rs"];
  class 9f97df7988362b9 default;
  click 9f97df7988362b9 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/matrix_generator.rs";
  9f97df7988362b9 -->|satisfies| c82329bbc603aed3;
  a06aa9bb2311c8b3["Markdown Matrix Formatter"];
  click a06aa9bb2311c8b3 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#markdown-matrix-formatter";
  class a06aa9bb2311c8b3 requirement;
  a06aa9bb2311c8b3 -.->|deriveReqT| 681cda683cd3fa2a;
  9f97df7988362b9 -->|satisfies| a06aa9bb2311c8b3;
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

### CLI Traces Flag

The system shall provide a traceability matrix generation function, activated by the (--traces flag), which shall generate a traceability matrix showing the relationships between requirements and verification elements.

#### Relations
  * derivedFrom: [UserRequirements.md/Traceability Matrix](../UserRequirements.md#traceability-matrix)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

### Traceability Matrix Builder Implementation

The system shall implement a traceability matrix builder component that extracts relationship data from the model, processes it according to configured parameters, and generates structured matrix representations showing connections between requirements and other elements.

#### Details

The traceability matrix shall be organized into multiple tables, with one table per root requirement (requirements without parents). This organization improves readability by grouping related requirements together.

Each table shall have the following structure:
- The first column shows the requirement name as a markdown link to its location in the git repository using the current commit hash
- Requirements shall be displayed in a hierarchical structure with parent-child relationships clearly indicated
- Child requirements shall be indented to show their relationship to parent requirements using arrow and underscore characters:
  - Level 1 (direct children): "↳ " (right arrow followed by a space)
  - Level 2 (grandchildren): "__↳ " (two underscores, then arrow and space)
  - Level 3 (great-grandchildren): "____↳ " (four underscores, then arrow and space)
  - Deeper levels: "______↳ " (six underscores, then arrow and space)
- The second column shows the verification status with a green checkmark "✅" if verified by at least one verification element, or "❌" if not verified
- The subsequent columns represent individual verification elements that verify requirements in this group, with each element name displayed as a markdown link to its location in the git repository
- Each row represents a requirement within the group
- Cell intersections show the relationship between requirements and verifications with a green checkmark "✅" where a relationship exists or empty where no relationship exists
- The matrix shall be rendered as a markdown table for human readability
- The JSON format shall be available for machine processing with all identifiers relative to the repository root

The links in the matrix shall use the git repository URL with the current commit hash to ensure that links remain stable even as the repository evolves. The format shall be similar to that used in the change impact report.

#### Relations
  * derivedFrom: [UserRequirements.md/Traceability Matrix](../UserRequirements.md#traceability-matrix)
  * satisfiedBy: [matrix_generator.rs](../../core/src/matrix_generator.rs)

---

### Markdown Matrix Formatter

The system shall implement a markdown formatter for traceability matrices that produces well-structured, readable markdown tables conforming to the Reqvire markdown-first methodology.

#### Relations
  * derivedFrom: [UserRequirements.md/Traceability Matrix](../UserRequirements.md#traceability-matrix)
  * satisfiedBy: [matrix_generator.rs](../../core/src/matrix_generator.rs)

---

### CLI Traces SVG Flag

The system shall provide an SVG output option for traceability matrices, activated by the (--svg flag), which shall generate a simplified SVG representation of the matrix that can be viewed directly or embedded in documents.

#### Details

The SVG output of the matrix shall have the following characteristics:
- It shall only be available when the --traces flag is used
- It cannot be used together with the --json flag (they are mutually exclusive)
- It shall display full element names instead of truncated names with ellipses
- It shall dynamically adjust column widths based on the maximum element name length to ensure all text is readable
- It shall not include hyperlinks to elements in the git repository
- It shall maintain the same hierarchical structure as the markdown version
- It shall use the same visual indicators for verification status and relationships
- The output shall be in a self-contained SVG format suitable for embedding in other documents

#### Relations
  * derivedFrom: [CLI Traces Flag](#cli-traces-flag)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

## Validation Capabilities
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  5c3a18f061fe0b18["Relation Type Validation"];
  click 5c3a18f061fe0b18 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#relation-type-validation";
  class 5c3a18f061fe0b18 requirement;
  20193d4c951bb5d8["UserRequirements.md/Enhanced Validation Error Reporting"];
  class 20193d4c951bb5d8 requirement;
  click 20193d4c951bb5d8 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  5c3a18f061fe0b18 -.->|deriveReqT| 20193d4c951bb5d8;
  a5d081e55d4f6501["src/relation.rs"];
  class a5d081e55d4f6501 default;
  click a5d081e55d4f6501 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/relation.rs";
  a5d081e55d4f6501 -->|satisfies| 5c3a18f061fe0b18;
  7a9ea38f16ddc722["Internal Consistency Validator"];
  click 7a9ea38f16ddc722 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#internal-consistency-validator";
  class 7a9ea38f16ddc722 requirement;
  944bd4459db32d65["UserRequirements.md/Validate Internal Consistency"];
  class 944bd4459db32d65 requirement;
  click 944bd4459db32d65 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-internal-consistency";
  7a9ea38f16ddc722 -.->|deriveReqT| 944bd4459db32d65;
  2671722513a08b18["model.rs"];
  class 2671722513a08b18 default;
  click 2671722513a08b18 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/model.rs";
  2671722513a08b18 -->|satisfies| 7a9ea38f16ddc722;
  8af498811b4b1f17["parser.rs"];
  class 8af498811b4b1f17 default;
  click 8af498811b4b1f17 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/parser.rs";
  8af498811b4b1f17 -->|satisfies| 7a9ea38f16ddc722;
  44b70517dea81a26["Excluded File Relation Validation"];
  click 44b70517dea81a26 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#excluded-file-relation-validation";
  class 44b70517dea81a26 requirement;
  470a0057ca1c964["File Pattern Exclusion for Linting"];
  class 470a0057ca1c964 requirement;
  click 470a0057ca1c964 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  44b70517dea81a26 -->|refines| 470a0057ca1c964;
  8af498811b4b1f17 -->|satisfies| 44b70517dea81a26;
  89f96731acd18db6["Filesystem Structure Validator"];
  click 89f96731acd18db6 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#filesystem-structure-validator";
  class 89f96731acd18db6 requirement;
  b92914a9715d4d36["UserRequirements.md/Validate Filesystem Structure"];
  class b92914a9715d4d36 requirement;
  click b92914a9715d4d36 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-filesystem-structure";
  89f96731acd18db6 -.->|deriveReqT| b92914a9715d4d36;
  2671722513a08b18 -->|satisfies| 89f96731acd18db6;
  fcd840339c79fec2["Markdown Structure Validator"];
  click fcd840339c79fec2 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#markdown-structure-validator";
  class fcd840339c79fec2 requirement;
  b6ee889a6a1ac979["UserRequirements.md/Validate Markdown Structure"];
  class b6ee889a6a1ac979 requirement;
  click b6ee889a6a1ac979 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-markdown-structure";
  fcd840339c79fec2 -.->|deriveReqT| b6ee889a6a1ac979;
  2671722513a08b18 -->|satisfies| fcd840339c79fec2;
  8af498811b4b1f17 -->|satisfies| fcd840339c79fec2;
  52720669fca1a6fd["Cross-Component Dependency Validator"];
  click 52720669fca1a6fd "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cross-component-dependency-validator";
  class 52720669fca1a6fd requirement;
  a830c3c9ac9cf1a9["UserRequirements.md/Validate Cross-Component Dependencies"];
  class a830c3c9ac9cf1a9 requirement;
  click a830c3c9ac9cf1a9 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-cross-component-dependencies";
  52720669fca1a6fd -.->|deriveReqT| a830c3c9ac9cf1a9;
  2671722513a08b18 -->|satisfies| 52720669fca1a6fd;
  8af498811b4b1f17 -->|satisfies| 52720669fca1a6fd;
  27a77338e4f9304c["Relation Element Type Validator"];
  click 27a77338e4f9304c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#relation-element-type-validator";
  class 27a77338e4f9304c requirement;
  46eca65b8a17dbc5["../UserRequirements.md#Validate Relation Types"];
  class 46eca65b8a17dbc5 requirement;
  click 46eca65b8a17dbc5 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#validate-relation-types";
  27a77338e4f9304c -.->|deriveReqT| 46eca65b8a17dbc5;
  b2e7c7fada8a1e2d["../SpecificationsRequirements.md#Relation Types And Behaviors"];
  class b2e7c7fada8a1e2d requirement;
  click b2e7c7fada8a1e2d "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SpecificationsRequirements.md#relation-types-and-behaviors";
  27a77338e4f9304c -.->|deriveReqT| b2e7c7fada8a1e2d;
  2671722513a08b18 -->|satisfies| 27a77338e4f9304c;
  8af498811b4b1f17 -->|satisfies| 27a77338e4f9304c;
```

---

### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces Reqvire's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)
  * satisfiedBy: [model.rs](../../core/src/model.rs)    
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)

---

### Filesystem Structure Validator
The system shall implement a validator that checks the organization of files and directories against the expected Reqvire project structure, verifying required folders exist and files are appropriately placed according to configuration settings.

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
  * derivedFrom: [../UserRequirements.md#Validate Relation Types](../UserRequirements.md#validate-relation-types)
  * derivedFrom: [../SpecificationsRequirements.md#Relation Types And Behaviors](../SpecificationsRequirements.md#relation-types-and-behaviors)  
  * satisfiedBy: [model.rs](../../core/src/model.rs)    
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)

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

## Reporting Features
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  a3d8f2cd99b2d548["CLI Summary Report Flag"];
  click a3d8f2cd99b2d548 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#cli-summary-report-flag";
  class a3d8f2cd99b2d548 requirement;
  a11c0dbb7acf27ea["Model Summary Report Generator"];
  class a11c0dbb7acf27ea requirement;
  click a11c0dbb7acf27ea "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#model-summary-report-generator";
  a3d8f2cd99b2d548 -->|refines| a11c0dbb7acf27ea;
  5fe26772336463bc["cli.rs"];
  class 5fe26772336463bc default;
  click 5fe26772336463bc "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/cli/src/cli.rs";
  5fe26772336463bc -->|satisfies| a3d8f2cd99b2d548;
  bbe607150e40021c["Validation Report Generator"];
  click bbe607150e40021c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/SystemRequirements/Requirements.md#validation-report-generator";
  class bbe607150e40021c requirement;
  e1f6859d4a4ea65b["UserRequirements.md/Provide Validation Reports"];
  class e1f6859d4a4ea65b requirement;
  click e1f6859d4a4ea65b "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#provide-validation-reports";
  bbe607150e40021c -.->|deriveReqT| e1f6859d4a4ea65b;
  2671722513a08b18["model.rs"];
  class 2671722513a08b18 default;
  click 2671722513a08b18 "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/model.rs";
  2671722513a08b18 -->|satisfies| bbe607150e40021c;
  9774a66acb48024c["UserRequirements.md/Model Structure and Summaries"];
  class 9774a66acb48024c requirement;
  click 9774a66acb48024c "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/specifications/UserRequirements.md#model-structure-and-summaries";
  a11c0dbb7acf27ea -.->|deriveReqT| 9774a66acb48024c;
  e07a26f0a7c1e1db["model.rs"];
  class e07a26f0a7c1e1db default;
  click e07a26f0a7c1e1db "https://github.com/ilijaljubicic/Reqvire/blob/8a68ca1eab81bac7964085e47777a0a92447c6a5/core/src/reports.rs";
  e07a26f0a7c1e1db -->|satisfies| a11c0dbb7acf27ea;
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