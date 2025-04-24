# System Requirements

## Linting    
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  f98ae394dbbde42c["Incosistent Newlines Linting Implementation"];
  click f98ae394dbbde42c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#incosistent-newlines-linting-implementation";
  class f98ae394dbbde42c requirement;
  81758bdb22a3329d["UserRequirements.md/Format Consistency Enforcement"];
  class 81758bdb22a3329d requirement;
  click 81758bdb22a3329d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#format-consistency-enforcement";
  f98ae394dbbde42c -.->|deriveReqT| 81758bdb22a3329d;
  b52472a7ccc9c395["linting/newlines.rs"];
  class b52472a7ccc9c395 default;
  click b52472a7ccc9c395 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/linting/newlines.rs";
  b52472a7ccc9c395 -->|satisfies| f98ae394dbbde42c;
  193cd6463891423e["Reserved Subsections Linting Implementation"];
  click 193cd6463891423e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#reserved-subsections-linting-implementation";
  class 193cd6463891423e requirement;
  193cd6463891423e -.->|deriveReqT| 81758bdb22a3329d;
  3aaafb0eaf60961c["linting/reserved_subsections.rs"];
  class 3aaafb0eaf60961c default;
  click 3aaafb0eaf60961c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/linting/indentation.rs";
  3aaafb0eaf60961c -->|satisfies| 193cd6463891423e;
  63f1cd4974cbf62["Git-Style Diff Output for Linting"];
  click 63f1cd4974cbf62 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#git-style-diff-output-for-linting";
  class 63f1cd4974cbf62 requirement;
  808b1863c88b26c4["UserRequirements.md/Linting Command Output"];
  class 808b1863c88b26c4 requirement;
  click 808b1863c88b26c4 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#linting-command-output";
  63f1cd4974cbf62 -.->|deriveReqT| 808b1863c88b26c4;
  8b47697e4e9fce62["linting/mod.rs"];
  class 8b47697e4e9fce62 default;
  click 8b47697e4e9fce62 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/linting/mod.rs";
  8b47697e4e9fce62 -->|satisfies| 63f1cd4974cbf62;
  8f2198f681f63fea["Dry Run Mode"];
  click 8f2198f681f63fea "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#dry-run-mode";
  class 8f2198f681f63fea requirement;
  fffbb227966be4f2["CLI Lint Flag"];
  class fffbb227966be4f2 requirement;
  click fffbb227966be4f2 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cli-lint-flag";
  8f2198f681f63fea -.->|deriveReqT| fffbb227966be4f2;
  11ffc4632afd7c4c["cli.rs"];
  class 11ffc4632afd7c4c default;
  click 11ffc4632afd7c4c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/cli/src/cli.rs";
  11ffc4632afd7c4c -->|satisfies| 8f2198f681f63fea;
  45770e9b319d3819["File Pattern Exclusion for Linting"];
  click 45770e9b319d3819 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  class 45770e9b319d3819 requirement;
  cc8128cae305b29d["Configurable Filename Exclusion Patterns"];
  class cc8128cae305b29d requirement;
  click cc8128cae305b29d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  45770e9b319d3819 -->|refines| cc8128cae305b29d;
  3189923c3f5e2124["utils.rs"];
  class 3189923c3f5e2124 default;
  click 3189923c3f5e2124 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/utils.rs";
  3189923c3f5e2124 -->|satisfies| 45770e9b319d3819;
  66aa36deca421516["Parallel Linting Processing"];
  click 66aa36deca421516 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#parallel-linting-processing";
  class 66aa36deca421516 requirement;
  103ddb8dc3242215["UserRequirements.md/Model Linting"];
  class 103ddb8dc3242215 requirement;
  click 103ddb8dc3242215 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#model-linting";
  66aa36deca421516 -.->|deriveReqT| 103ddb8dc3242215;
  8b47697e4e9fce62 -->|satisfies| 66aa36deca421516;
  3f3d3f9ccb15029a["Excess Whitespace Linting Implementation"];
  click 3f3d3f9ccb15029a "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  class 3f3d3f9ccb15029a requirement;
  3f3d3f9ccb15029a -.->|deriveReqT| 81758bdb22a3329d;
  bb06efd0b85636ec["linting/whitespace.rs"];
  class bb06efd0b85636ec default;
  click bb06efd0b85636ec "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/linting/whitespace.rs";
  bb06efd0b85636ec -->|satisfies| 3f3d3f9ccb15029a;
  887a7d36caacab2b["UserRequirements.md/Linting Command Behavior"];
  class 887a7d36caacab2b requirement;
  click 887a7d36caacab2b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#linting-command";
  fffbb227966be4f2 -.->|deriveReqT| 887a7d36caacab2b;
  11ffc4632afd7c4c -->|satisfies| fffbb227966be4f2;
  dd0846393d037337["Missing Separators Linting Implementation"];
  click dd0846393d037337 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#missing-separators-linting-implementation";
  class dd0846393d037337 requirement;
  dd0846393d037337 -.->|deriveReqT| 81758bdb22a3329d;
  9abe404e42ed3086["linting/separators.rs"];
  class 9abe404e42ed3086 default;
  click 9abe404e42ed3086 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/linting/separators.rs";
  9abe404e42ed3086 -->|satisfies| dd0846393d037337;
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

  d38ab4ad139183d3["External Folders Support"];
  click d38ab4ad139183d3 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#external-folders-support";
  class d38ab4ad139183d3 requirement;
  e7286123b1b97862["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class e7286123b1b97862 requirement;
  click e7286123b1b97862 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  d38ab4ad139183d3 -.->|deriveReqT| e7286123b1b97862;
  c87fb8d32afb10bb["config.rs"];
  class c87fb8d32afb10bb default;
  click c87fb8d32afb10bb "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/cli/src/config.rs";
  c87fb8d32afb10bb -->|satisfies| d38ab4ad139183d3;
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

  5ec6a2668bddf0e["JSON Output Format"];
  click 5ec6a2668bddf0e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#json-output-format";
  class 5ec6a2668bddf0e requirement;
  7cf5cf9900076be6["UserRequirements.md/Enhanced Validation Error Reporting"];
  class 7cf5cf9900076be6 requirement;
  click 7cf5cf9900076be6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  5ec6a2668bddf0e -.->|deriveReqT| 7cf5cf9900076be6;
  11ffc4632afd7c4c["cli.rs"];
  class 11ffc4632afd7c4c default;
  click 11ffc4632afd7c4c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/cli/src/cli.rs";
  11ffc4632afd7c4c -->|satisfies| 5ec6a2668bddf0e;
  98bd2bd6bdf5bf97["File Content Caching for Performance"];
  click 98bd2bd6bdf5bf97 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#file-content-caching-for-performance";
  class 98bd2bd6bdf5bf97 requirement;
  21e4eb87cb55ea5e["../ManagingMbseModelsRequirements.md#Efficient Processing"];
  class 21e4eb87cb55ea5e requirement;
  click 21e4eb87cb55ea5e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  98bd2bd6bdf5bf97 -.->|deriveReqT| 21e4eb87cb55ea5e;
  6136c216c9f80165["model.rs"];
  class 6136c216c9f80165 default;
  click 6136c216c9f80165 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/model.rs";
  6136c216c9f80165 -->|satisfies| 98bd2bd6bdf5bf97;
  bdfd9d65e46117e7["Detailed Error Handling and Logging"];
  click bdfd9d65e46117e7 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  class bdfd9d65e46117e7 requirement;
  bdfd9d65e46117e7 -.->|deriveReqT| 7cf5cf9900076be6;
  8cd8a561b4c440e3["src/error.rs"];
  class 8cd8a561b4c440e3 default;
  click 8cd8a561b4c440e3 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/error.rs";
  8cd8a561b4c440e3 -->|satisfies| bdfd9d65e46117e7;
  5a1719a2649b9922["Unstructured Documents"];
  click 5a1719a2649b9922 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#unstructured-documents";
  class 5a1719a2649b9922 requirement;
  daadd8e583647e4f["ManagingMbseModelsRequirements.md#Coexistence of Structured and Unstructured Documents"];
  class daadd8e583647e4f requirement;
  click daadd8e583647e4f "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  5a1719a2649b9922 -.->|deriveReqT| daadd8e583647e4f;
  8acd24c7c228637b["Index Generation"];
  click 8acd24c7c228637b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#index-generation";
  class 8acd24c7c228637b requirement;
  f5b5eaeb28c5a7b1["UserRequirements.md/Generate Documentation Index"];
  class f5b5eaeb28c5a7b1 requirement;
  click f5b5eaeb28c5a7b1 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#generate-documentation-index";
  8acd24c7c228637b -.->|deriveReqT| f5b5eaeb28c5a7b1;
  3ef6afafe29becbc["index_generator.rs"];
  class 3ef6afafe29becbc default;
  click 3ef6afafe29becbc "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/index_generator.rs";
  3ef6afafe29becbc -->|satisfies| 8acd24c7c228637b;
  c8b6ccc187b0a27e["HTML Export"];
  click c8b6ccc187b0a27e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#html-export";
  class c8b6ccc187b0a27e requirement;
  72c7eda6183f0893["../UserRequirements.md/Export HTML specifications"];
  class 72c7eda6183f0893 requirement;
  click 72c7eda6183f0893 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#export-html-specifications";
  c8b6ccc187b0a27e -.->|deriveReqT| 72c7eda6183f0893;
  8cb11d152f3b9d2d["html_export.rs"];
  class 8cb11d152f3b9d2d default;
  click 8cb11d152f3b9d2d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/html_export.rs";
  8cb11d152f3b9d2d -->|satisfies| c8b6ccc187b0a27e;
  86e0701b6ce7de0b["HTML Navigation Enhancement"];
  click 86e0701b6ce7de0b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#html-navigation-enhancement";
  class 86e0701b6ce7de0b requirement;
  e1c89b5d94837122["UserRequirements.md/Documentation Index HTML Integration"];
  class e1c89b5d94837122 requirement;
  click e1c89b5d94837122 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#documentation-index-html-integration";
  86e0701b6ce7de0b -.->|deriveReqT| e1c89b5d94837122;
  c788e5b84faa2ae3["html.rs"];
  class c788e5b84faa2ae3 default;
  click c788e5b84faa2ae3 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/html.rs";
  c788e5b84faa2ae3 -->|satisfies| 86e0701b6ce7de0b;
  8cb11d152f3b9d2d -->|satisfies| 86e0701b6ce7de0b;
  8ba9c7e059799274["LLM Context Command"];
  click 8ba9c7e059799274 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#llm-context-command";
  class 8ba9c7e059799274 requirement;
  5a6fc253c51845b6["UserRequirements.md/AI Agent Context"];
  class 5a6fc253c51845b6 requirement;
  click 5a6fc253c51845b6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#ai-agent-context";
  8ba9c7e059799274 -.->|deriveReqT| 5a6fc253c51845b6;
  4a5edefd6e054300["main.rs"];
  class 4a5edefd6e054300 default;
  click 4a5edefd6e054300 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/cli/src/main.rs";
  4a5edefd6e054300 -->|satisfies| 8ba9c7e059799274;
  7a8da8dfee910932["Interactive Mermaid Diagram Node Behavior"];
  click 7a8da8dfee910932 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#interactive-mermaid-diagram-node-behavior";
  class 7a8da8dfee910932 requirement;
  e867499409ae347a["UserRequirements.md/Interactive Mermaid Diagrams"];
  class e867499409ae347a requirement;
  click e867499409ae347a "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#interactive-mermaid-diagrams";
  7a8da8dfee910932 -.->|deriveReqT| e867499409ae347a;
  d4240e29691d4b88["html.rs"];
  class d4240e29691d4b88 default;
  click d4240e29691d4b88 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/diagrams.rs";
  d4240e29691d4b88 -->|satisfies| 7a8da8dfee910932;
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

  2737f2d770aa0757["Requirements Files Search and Detection"];
  click 2737f2d770aa0757 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#requirements-files-search-and-detection";
  class 2737f2d770aa0757 requirement;
  21e4eb87cb55ea5e["ManagingMbseModelsRequirements.md/Efficient Processing"];
  class 21e4eb87cb55ea5e requirement;
  click 21e4eb87cb55ea5e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  2737f2d770aa0757 -.->|deriveReqT| 21e4eb87cb55ea5e;
  6136c216c9f80165["model.rs"];
  class 6136c216c9f80165 default;
  click 6136c216c9f80165 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/model.rs";
  6136c216c9f80165 -->|satisfies| 2737f2d770aa0757;
  99bed90a0d96a1d2["Requirements Processing"];
  click 99bed90a0d96a1d2 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#requirements-processing";
  class 99bed90a0d96a1d2 requirement;
  e7286123b1b97862["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class e7286123b1b97862 requirement;
  click e7286123b1b97862 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  99bed90a0d96a1d2 -.->|deriveReqT| e7286123b1b97862;
  6136c216c9f80165 -->|satisfies| 99bed90a0d96a1d2;
  3b1e87396caf6495["parser.rs"];
  class 3b1e87396caf6495 default;
  click 3b1e87396caf6495 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/parser.rs";
  3b1e87396caf6495 -->|satisfies| 99bed90a0d96a1d2;
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

  cc8128cae305b29d["Configurable Filename Exclusion Patterns"];
  click cc8128cae305b29d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  class cc8128cae305b29d requirement;
  ce24dbacb9646f3b["ManagingMbseModelsRequirements.md/Project Configuration with YAML"];
  class ce24dbacb9646f3b requirement;
  click ce24dbacb9646f3b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  cc8128cae305b29d -.->|deriveReqT| ce24dbacb9646f3b;
  c87fb8d32afb10bb["config.rs"];
  class c87fb8d32afb10bb default;
  click c87fb8d32afb10bb "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/cli/src/config.rs";
  c87fb8d32afb10bb -->|satisfies| cc8128cae305b29d;
  c826c1ee7c93bf6b["SysML-Compatible Relationship Rendering"];
  click c826c1ee7c93bf6b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#sysml-compatible-relationship-rendering";
  class c826c1ee7c93bf6b requirement;
  a6a836283607bf45["UserRequirements.md/Visualize Model Relationships"];
  class a6a836283607bf45 requirement;
  click a6a836283607bf45 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#visualize-model-relationships";
  c826c1ee7c93bf6b -.->|deriveReqT| a6a836283607bf45;
  d4240e29691d4b88["diagrams.rs"];
  class d4240e29691d4b88 default;
  click d4240e29691d4b88 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/diagrams.rs";
  d4240e29691d4b88 -->|satisfies| c826c1ee7c93bf6b;
  a7ec66314a62423b["Mermaid Diagram Format Conversion"];
  click a7ec66314a62423b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#mermaid-diagram-format-conversion";
  class a7ec66314a62423b requirement;
  10c00a1bd12fefa5["UserRequirements.md/Export Diagrams in Standard Formats"];
  class 10c00a1bd12fefa5 requirement;
  click 10c00a1bd12fefa5 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  a7ec66314a62423b -.->|deriveReqT| 10c00a1bd12fefa5;
  d4240e29691d4b88 -->|satisfies| a7ec66314a62423b;
  191d27287e362df2["Diagram Storage Path Configuration"];
  click 191d27287e362df2 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#diagram-storage-path-configuration";
  class 191d27287e362df2 requirement;
  aee397f35b867556["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class aee397f35b867556 requirement;
  click aee397f35b867556 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  191d27287e362df2 -.->|deriveReqT| aee397f35b867556;
  6136c216c9f80165["model.rs"];
  class 6136c216c9f80165 default;
  click 6136c216c9f80165 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/model.rs";
  6136c216c9f80165 -->|satisfies| 191d27287e362df2;
  9860815d52cb0fbb["Visual Differential Rendering"];
  click 9860815d52cb0fbb "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#visual-differential-rendering";
  class 9860815d52cb0fbb requirement;
  fd7388e379372d7b["UserRequirements.md/Highlight Changes in Diagrams"];
  class fd7388e379372d7b requirement;
  click fd7388e379372d7b "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  9860815d52cb0fbb -.->|deriveReqT| fd7388e379372d7b;
  1414e7f8890f2939["Relationship Type Filter Implementation"];
  click 1414e7f8890f2939 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#relationship-type-filter-implementation";
  class 1414e7f8890f2939 requirement;
  30d97803eba68a13["UserRequirements.md/Filter Relationships by Type"];
  class 30d97803eba68a13 requirement;
  click 30d97803eba68a13 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#filter-relationships-by-type";
  1414e7f8890f2939 -.->|deriveReqT| 30d97803eba68a13;
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

  665b7456c9b61ded["CLI Change Impact Report Flag"];
  click 665b7456c9b61ded "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cli-change-impact-report-flag";
  class 665b7456c9b61ded requirement;
  918cc4a26d642f30["Structural Change Analyzer"];
  class 918cc4a26d642f30 requirement;
  click 918cc4a26d642f30 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#structural-change-analyzer";
  665b7456c9b61ded -.->|deriveReqT| 918cc4a26d642f30;
  11ffc4632afd7c4c["cli.rs"];
  class 11ffc4632afd7c4c default;
  click 11ffc4632afd7c4c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/cli/src/cli.rs";
  11ffc4632afd7c4c -->|satisfies| 665b7456c9b61ded;
  5eca866a0338e0fe["Traceability Matrix Builder Implementation"];
  click 5eca866a0338e0fe "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#traceability-matrix-builder-implementation";
  class 5eca866a0338e0fe requirement;
  ba40352f8e72c125["UserRequirements.md/Traceability Matrix"];
  class ba40352f8e72c125 requirement;
  click ba40352f8e72c125 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#traceability-matrix";
  5eca866a0338e0fe -.->|deriveReqT| ba40352f8e72c125;
  48ddeb303b98f1b4["matrix_generator.rs"];
  class 48ddeb303b98f1b4 default;
  click 48ddeb303b98f1b4 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/matrix_generator.rs";
  48ddeb303b98f1b4 -->|satisfies| 5eca866a0338e0fe;
  1ffba11189b9b4c8["CLI Traces SVG Flag"];
  click 1ffba11189b9b4c8 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cli-traces-svg-flag";
  class 1ffba11189b9b4c8 requirement;
  438a74e06c98c101["CLI Traces Flag"];
  class 438a74e06c98c101 requirement;
  click 438a74e06c98c101 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cli-traces-flag";
  1ffba11189b9b4c8 -.->|deriveReqT| 438a74e06c98c101;
  11ffc4632afd7c4c -->|satisfies| 1ffba11189b9b4c8;
  91ebf7e73d5ac081["UserRequirements.md/Tracing Structural Changes"];
  class 91ebf7e73d5ac081 requirement;
  click 91ebf7e73d5ac081 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#tracing-structural-changes";
  918cc4a26d642f30 -.->|deriveReqT| 91ebf7e73d5ac081;
  cc976e6bcded86e7["model.rs"];
  class cc976e6bcded86e7 default;
  click cc976e6bcded86e7 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/change_impact.rs";
  cc976e6bcded86e7 -->|satisfies| 918cc4a26d642f30;
  d7e5fbf806d650d9["Markdown Matrix Formatter"];
  click d7e5fbf806d650d9 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#markdown-matrix-formatter";
  class d7e5fbf806d650d9 requirement;
  d7e5fbf806d650d9 -.->|deriveReqT| ba40352f8e72c125;
  48ddeb303b98f1b4 -->|satisfies| d7e5fbf806d650d9;
  b8b40184613535c1["CLI Git Commit Hash Flag"];
  click b8b40184613535c1 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cli-git-commit-hash-flag";
  class b8b40184613535c1 requirement;
  b8b40184613535c1 -.->|deriveReqT| 665b7456c9b61ded;
  11ffc4632afd7c4c -->|satisfies| b8b40184613535c1;
  438a74e06c98c101 -.->|deriveReqT| ba40352f8e72c125;
  11ffc4632afd7c4c -->|satisfies| 438a74e06c98c101;
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

The system shall implement a markdown formatter for traceability matrices that produces well-structured, readable markdown tables conforming to the ReqFlow markdown-first methodology.

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

  887db62e0fb2818d["Markdown Structure Validator"];
  click 887db62e0fb2818d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#markdown-structure-validator";
  class 887db62e0fb2818d requirement;
  7b1772417b3ad5e["UserRequirements.md/Validate Markdown Structure"];
  class 7b1772417b3ad5e requirement;
  click 7b1772417b3ad5e "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-markdown-structure";
  887db62e0fb2818d -.->|deriveReqT| 7b1772417b3ad5e;
  6136c216c9f80165["model.rs"];
  class 6136c216c9f80165 default;
  click 6136c216c9f80165 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/model.rs";
  6136c216c9f80165 -->|satisfies| 887db62e0fb2818d;
  3b1e87396caf6495["parser.rs"];
  class 3b1e87396caf6495 default;
  click 3b1e87396caf6495 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/parser.rs";
  3b1e87396caf6495 -->|satisfies| 887db62e0fb2818d;
  5870488e00ee4f36["Relation Type Validation"];
  click 5870488e00ee4f36 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#relation-type-validation";
  class 5870488e00ee4f36 requirement;
  7cf5cf9900076be6["UserRequirements.md/Enhanced Validation Error Reporting"];
  class 7cf5cf9900076be6 requirement;
  click 7cf5cf9900076be6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  5870488e00ee4f36 -.->|deriveReqT| 7cf5cf9900076be6;
  ed4f94bde93b51b2["src/relation.rs"];
  class ed4f94bde93b51b2 default;
  click ed4f94bde93b51b2 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/relation.rs";
  ed4f94bde93b51b2 -->|satisfies| 5870488e00ee4f36;
  c5c85bedd1cf11e6["Excluded File Relation Validation"];
  click c5c85bedd1cf11e6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#excluded-file-relation-validation";
  class c5c85bedd1cf11e6 requirement;
  45770e9b319d3819["File Pattern Exclusion for Linting"];
  class 45770e9b319d3819 requirement;
  click 45770e9b319d3819 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  c5c85bedd1cf11e6 -->|refines| 45770e9b319d3819;
  3b1e87396caf6495 -->|satisfies| c5c85bedd1cf11e6;
  c7d88aff4ec97281["Cross-Component Dependency Validator"];
  click c7d88aff4ec97281 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cross-component-dependency-validator";
  class c7d88aff4ec97281 requirement;
  6e40bf9f83a718fa["UserRequirements.md/Validate Cross-Component Dependencies"];
  class 6e40bf9f83a718fa requirement;
  click 6e40bf9f83a718fa "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-cross-component-dependencies";
  c7d88aff4ec97281 -.->|deriveReqT| 6e40bf9f83a718fa;
  6136c216c9f80165 -->|satisfies| c7d88aff4ec97281;
  3b1e87396caf6495 -->|satisfies| c7d88aff4ec97281;
  212d1317cd2b25fc["Relation Element Type Validator"];
  click 212d1317cd2b25fc "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#relation-element-type-validator";
  class 212d1317cd2b25fc requirement;
  e587d63764466914["../UserRequirements.md#Validate Relation Types"];
  class e587d63764466914 requirement;
  click e587d63764466914 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-relation-types";
  212d1317cd2b25fc -.->|deriveReqT| e587d63764466914;
  a34d9281a02c6839["../SpecificationsRequirements.md#Relation Types Specifications"];
  class a34d9281a02c6839 requirement;
  click a34d9281a02c6839 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SpecificationsRequirements.md#relation-types-specifications";
  212d1317cd2b25fc -.->|deriveReqT| a34d9281a02c6839;
  6136c216c9f80165 -->|satisfies| 212d1317cd2b25fc;
  3b1e87396caf6495 -->|satisfies| 212d1317cd2b25fc;
  ec201a112c9de469["Filesystem Structure Validator"];
  click ec201a112c9de469 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#filesystem-structure-validator";
  class ec201a112c9de469 requirement;
  d834cc4bc9dbb07c["UserRequirements.md/Validate Filesystem Structure"];
  class d834cc4bc9dbb07c requirement;
  click d834cc4bc9dbb07c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-filesystem-structure";
  ec201a112c9de469 -.->|deriveReqT| d834cc4bc9dbb07c;
  6136c216c9f80165 -->|satisfies| ec201a112c9de469;
  1c2e7c81f9f01e10["Internal Consistency Validator"];
  click 1c2e7c81f9f01e10 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#internal-consistency-validator";
  class 1c2e7c81f9f01e10 requirement;
  9e524ac696c43a26["UserRequirements.md/Validate Internal Consistency"];
  class 9e524ac696c43a26 requirement;
  click 9e524ac696c43a26 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#validate-internal-consistency";
  1c2e7c81f9f01e10 -.->|deriveReqT| 9e524ac696c43a26;
  6136c216c9f80165 -->|satisfies| 1c2e7c81f9f01e10;
  3b1e87396caf6495 -->|satisfies| 1c2e7c81f9f01e10;
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
  * derivedFrom: [../UserRequirements.md#Validate Relation Types](../UserRequirements.md#validate-relation-types)
  * derivedFrom: [../SpecificationsRequirements.md#Relation Types Specifications](../SpecificationsRequirements.md#relation-types-specifications)  
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

  cccd4e46e2b3239a["Model Summary Report Generator"];
  click cccd4e46e2b3239a "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#model-summary-report-generator";
  class cccd4e46e2b3239a requirement;
  b342220e0dc8751d["UserRequirements.md/Model Structure and Summaries"];
  class b342220e0dc8751d requirement;
  click b342220e0dc8751d "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#model-structure-and-summaries";
  cccd4e46e2b3239a -.->|deriveReqT| b342220e0dc8751d;
  6292255058880ee6["model.rs"];
  class 6292255058880ee6 default;
  click 6292255058880ee6 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/reports.rs";
  6292255058880ee6 -->|satisfies| cccd4e46e2b3239a;
  6f86272134897867["CLI Summary Report Flag"];
  click 6f86272134897867 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#cli-summary-report-flag";
  class 6f86272134897867 requirement;
  6f86272134897867 -->|refines| cccd4e46e2b3239a;
  11ffc4632afd7c4c["cli.rs"];
  class 11ffc4632afd7c4c default;
  click 11ffc4632afd7c4c "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/cli/src/cli.rs";
  11ffc4632afd7c4c -->|satisfies| 6f86272134897867;
  143766be8cc41384["Validation Report Generator"];
  click 143766be8cc41384 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/SystemRequirements/Requirements.md#validation-report-generator";
  class 143766be8cc41384 requirement;
  482c757913204fb8["UserRequirements.md/Provide Validation Reports"];
  class 482c757913204fb8 requirement;
  click 482c757913204fb8 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/specifications/UserRequirements.md#provide-validation-reports";
  143766be8cc41384 -.->|deriveReqT| 482c757913204fb8;
  6136c216c9f80165["model.rs"];
  class 6136c216c9f80165 default;
  click 6136c216c9f80165 "https://github.com/ilijaljubicic/ReqFlow/blob/a94231473c9a4a56fe21608ebd13787e0cc3f2dc/core/src/model.rs";
  6136c216c9f80165 -->|satisfies| 143766be8cc41384;
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
