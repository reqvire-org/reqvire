# System Requiremeame rets

## Linting    
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  ab8dfb01e717d34["Excess Whitespace Linting Implementation"];
  click ab8dfb01e717d34 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  class ab8dfb01e717d34 requirement;
  1ddbeea0cf8eaad5["UserRequirements.md/Format Consistency Enforcement"];
  class 1ddbeea0cf8eaad5 requirement;
  click 1ddbeea0cf8eaad5 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#format-consistency-enforcement";
  ab8dfb01e717d34 -.->|deriveReqT| 1ddbeea0cf8eaad5;
  fd5178bf78bae55c["linting/whitespace.rs"];
  class fd5178bf78bae55c default;
  click fd5178bf78bae55c "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/linting/whitespace.rs";
  fd5178bf78bae55c -->|satisfies| ab8dfb01e717d34;
  f540b21f7eced636["Missing Separators Linting Implementation"];
  click f540b21f7eced636 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#missing-separators-linting-implementation";
  class f540b21f7eced636 requirement;
  f540b21f7eced636 -.->|deriveReqT| 1ddbeea0cf8eaad5;
  45e2af09e602182a["linting/separators.rs"];
  class 45e2af09e602182a default;
  click 45e2af09e602182a "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/linting/separators.rs";
  45e2af09e602182a -->|satisfies| f540b21f7eced636;
  56174a2a92eb637["Reserved Subsections Linting Implementation"];
  click 56174a2a92eb637 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#reserved-subsections-linting-implementation";
  class 56174a2a92eb637 requirement;
  56174a2a92eb637 -.->|deriveReqT| 1ddbeea0cf8eaad5;
  8c7c2481d50c628c["linting/reserved_subsections.rs"];
  class 8c7c2481d50c628c default;
  click 8c7c2481d50c628c "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/linting/indentation.rs";
  8c7c2481d50c628c -->|satisfies| 56174a2a92eb637;
  3f235c1000d5347f["CLI Lint Flag"];
  click 3f235c1000d5347f "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#cli-lint-flag";
  class 3f235c1000d5347f requirement;
  28b0f9fa78937e61["UserRequirements.md/Linting Command Behavior"];
  class 28b0f9fa78937e61 requirement;
  click 28b0f9fa78937e61 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#linting-command";
  3f235c1000d5347f -.->|deriveReqT| 28b0f9fa78937e61;
  2f21d2133dbfd205["cli.rs"];
  class 2f21d2133dbfd205 default;
  click 2f21d2133dbfd205 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/cli/src/cli.rs";
  2f21d2133dbfd205 -->|satisfies| 3f235c1000d5347f;
  48e8a0b4b18111c4["File Pattern Exclusion for Linting"];
  click 48e8a0b4b18111c4 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  class 48e8a0b4b18111c4 requirement;
  4bf74699f40f4a76["Configurable Filename Exclusion Patterns"];
  class 4bf74699f40f4a76 requirement;
  click 4bf74699f40f4a76 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  48e8a0b4b18111c4 -->|refines| 4bf74699f40f4a76;
  85989fd7bb727157["utils.rs"];
  class 85989fd7bb727157 default;
  click 85989fd7bb727157 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/utils.rs";
  85989fd7bb727157 -->|satisfies| 48e8a0b4b18111c4;
  e42a0715e74ccd66["Incosistent Newlines Linting Implementation"];
  click e42a0715e74ccd66 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#incosistent-newlines-linting-implementation";
  class e42a0715e74ccd66 requirement;
  e42a0715e74ccd66 -.->|deriveReqT| 1ddbeea0cf8eaad5;
  411d20097517505a["linting/newlines.rs"];
  class 411d20097517505a default;
  click 411d20097517505a "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/linting/newlines.rs";
  411d20097517505a -->|satisfies| e42a0715e74ccd66;
  d21b16b30de7350d["Dry Run Mode"];
  click d21b16b30de7350d "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#dry-run-mode";
  class d21b16b30de7350d requirement;
  d21b16b30de7350d -.->|deriveReqT| 3f235c1000d5347f;
  2f21d2133dbfd205 -->|satisfies| d21b16b30de7350d;
  c87ad8ce58149089["Parallel Linting Processing"];
  click c87ad8ce58149089 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#parallel-linting-processing";
  class c87ad8ce58149089 requirement;
  84c4dc11e82e8638["UserRequirements.md/Model Linting"];
  class 84c4dc11e82e8638 requirement;
  click 84c4dc11e82e8638 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#model-linting";
  c87ad8ce58149089 -.->|deriveReqT| 84c4dc11e82e8638;
  3e39cdcf485c5250["linting/mod.rs"];
  class 3e39cdcf485c5250 default;
  click 3e39cdcf485c5250 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/linting/mod.rs";
  3e39cdcf485c5250 -->|satisfies| c87ad8ce58149089;
  7c2ad2603d27c318["Git-Style Diff Output for Linting"];
  click 7c2ad2603d27c318 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#git-style-diff-output-for-linting";
  class 7c2ad2603d27c318 requirement;
  62c066a5aad4dafe["UserRequirements.md/Linting Command Output"];
  class 62c066a5aad4dafe requirement;
  click 62c066a5aad4dafe "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#linting-command-output";
  7c2ad2603d27c318 -.->|deriveReqT| 62c066a5aad4dafe;
  3e39cdcf485c5250 -->|satisfies| 7c2ad2603d27c318;
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

  66080aef4185b07d["External Folders Support"];
  click 66080aef4185b07d "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#external-folders-support";
  class 66080aef4185b07d requirement;
  c4d5865187c53ce6["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class c4d5865187c53ce6 requirement;
  click c4d5865187c53ce6 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  66080aef4185b07d -.->|deriveReqT| c4d5865187c53ce6;
  d89f4b0d2edaac20["config.rs"];
  class d89f4b0d2edaac20 default;
  click d89f4b0d2edaac20 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/cli/src/config.rs";
  d89f4b0d2edaac20 -->|satisfies| 66080aef4185b07d;
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

  3b4bfa0725509a0e["Index Generation"];
  click 3b4bfa0725509a0e "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#index-generation";
  class 3b4bfa0725509a0e requirement;
  9019be8bfdc22b35["UserRequirements.md/Generate Documentation Index"];
  class 9019be8bfdc22b35 requirement;
  click 9019be8bfdc22b35 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#generate-documentation-index";
  3b4bfa0725509a0e -.->|deriveReqT| 9019be8bfdc22b35;
  3461f4a59371fd45["index_generator.rs"];
  class 3461f4a59371fd45 default;
  click 3461f4a59371fd45 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/index_generator.rs";
  3461f4a59371fd45 -->|satisfies| 3b4bfa0725509a0e;
  d177913fffd1ed11["LLM Context Command"];
  click d177913fffd1ed11 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#llm-context-command";
  class d177913fffd1ed11 requirement;
  7fd9156eac77c270["UserRequirements.md/AI Agent Context"];
  class 7fd9156eac77c270 requirement;
  click 7fd9156eac77c270 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#ai-agent-context";
  d177913fffd1ed11 -.->|deriveReqT| 7fd9156eac77c270;
  615a0f4ad2620118["main.rs"];
  class 615a0f4ad2620118 default;
  click 615a0f4ad2620118 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/cli/src/main.rs";
  615a0f4ad2620118 -->|satisfies| d177913fffd1ed11;
  a2b1d4ec64cbd441["HTML Export"];
  click a2b1d4ec64cbd441 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#html-export";
  class a2b1d4ec64cbd441 requirement;
  d9686a154fe87b2["../UserRequirements.md/Export HTML specifications"];
  class d9686a154fe87b2 requirement;
  click d9686a154fe87b2 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#export-html-specifications";
  a2b1d4ec64cbd441 -.->|deriveReqT| d9686a154fe87b2;
  5dbf1a3141d3defb["html_export.rs"];
  class 5dbf1a3141d3defb default;
  click 5dbf1a3141d3defb "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/html_export.rs";
  5dbf1a3141d3defb -->|satisfies| a2b1d4ec64cbd441;
  3f1abb8fd2d7c9c2["Unstructured Documents"];
  click 3f1abb8fd2d7c9c2 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#unstructured-documents";
  class 3f1abb8fd2d7c9c2 requirement;
  da5831ca85881025["ManagingMbseModelsRequirements.md#Coexistence of Structured and Unstructured Documents"];
  class da5831ca85881025 requirement;
  click da5831ca85881025 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  3f1abb8fd2d7c9c2 -.->|deriveReqT| da5831ca85881025;
  d72f6096b9a5dd8e["Detailed Error Handling and Logging"];
  click d72f6096b9a5dd8e "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  class d72f6096b9a5dd8e requirement;
  3d2fe0b05ff9c8e3["../UserRequirements.md#Enhanced Validation Error Reporting"];
  class 3d2fe0b05ff9c8e3 requirement;
  click 3d2fe0b05ff9c8e3 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  d72f6096b9a5dd8e -.->|deriveReqT| 3d2fe0b05ff9c8e3;
  af1374ae222fbd72["src/error.rs"];
  class af1374ae222fbd72 default;
  click af1374ae222fbd72 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/error.rs";
  af1374ae222fbd72 -->|satisfies| d72f6096b9a5dd8e;
  ea45d7ea45c8ee7e["File Content Caching for Performance"];
  click ea45d7ea45c8ee7e "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#file-content-caching-for-performance";
  class ea45d7ea45c8ee7e requirement;
  386d7b145d008870["../ManagingMbseModelsRequirements.md#Efficient Processing"];
  class 386d7b145d008870 requirement;
  click 386d7b145d008870 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  ea45d7ea45c8ee7e -.->|deriveReqT| 386d7b145d008870;
  36fd2b1e82621caf["model.rs"];
  class 36fd2b1e82621caf default;
  click 36fd2b1e82621caf "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/model.rs";
  36fd2b1e82621caf -->|satisfies| ea45d7ea45c8ee7e;
  1550bb73b8029cb["HTML Navigation Enhancement"];
  click 1550bb73b8029cb "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#html-navigation-enhancement";
  class 1550bb73b8029cb requirement;
  b5146db7aedfd66["UserRequirements.md/Documentation Index HTML Integration"];
  class b5146db7aedfd66 requirement;
  click b5146db7aedfd66 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#documentation-index-html-integration";
  1550bb73b8029cb -.->|deriveReqT| b5146db7aedfd66;
  59e3b5b3087497da["html.rs"];
  class 59e3b5b3087497da default;
  click 59e3b5b3087497da "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/html.rs";
  59e3b5b3087497da -->|satisfies| 1550bb73b8029cb;
  5dbf1a3141d3defb -->|satisfies| 1550bb73b8029cb;
  bf6c9ff8abbc637b["JSON Output Format"];
  click bf6c9ff8abbc637b "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#json-output-format";
  class bf6c9ff8abbc637b requirement;
  bf6c9ff8abbc637b -.->|deriveReqT| 3d2fe0b05ff9c8e3;
  2f21d2133dbfd205["cli.rs"];
  class 2f21d2133dbfd205 default;
  click 2f21d2133dbfd205 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/cli/src/cli.rs";
  2f21d2133dbfd205 -->|satisfies| bf6c9ff8abbc637b;
  a17b8052345712d7["Interactive Mermaid Diagram Node Behavior"];
  click a17b8052345712d7 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#interactive-mermaid-diagram-node-behavior";
  class a17b8052345712d7 requirement;
  b8997351b6f34048["UserRequirements.md/Interactive Mermaid Diagrams"];
  class b8997351b6f34048 requirement;
  click b8997351b6f34048 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#interactive-mermaid-diagrams";
  a17b8052345712d7 -.->|deriveReqT| b8997351b6f34048;
  95f7dce6aff47396["html.rs"];
  class 95f7dce6aff47396 default;
  click 95f7dce6aff47396 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/diagrams.rs";
  95f7dce6aff47396 -->|satisfies| a17b8052345712d7;
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

  33b15634cbc8d029["Requirements Files Search and Detection"];
  click 33b15634cbc8d029 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#requirements-files-search-and-detection";
  class 33b15634cbc8d029 requirement;
  386d7b145d008870["ManagingMbseModelsRequirements.md/Efficient Processing"];
  class 386d7b145d008870 requirement;
  click 386d7b145d008870 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  33b15634cbc8d029 -.->|deriveReqT| 386d7b145d008870;
  36fd2b1e82621caf["model.rs"];
  class 36fd2b1e82621caf default;
  click 36fd2b1e82621caf "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/model.rs";
  36fd2b1e82621caf -->|satisfies| 33b15634cbc8d029;
  f24f11691f55af62["Requirements Processing"];
  click f24f11691f55af62 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#requirements-processing";
  class f24f11691f55af62 requirement;
  c4d5865187c53ce6["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class c4d5865187c53ce6 requirement;
  click c4d5865187c53ce6 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  f24f11691f55af62 -.->|deriveReqT| c4d5865187c53ce6;
  36fd2b1e82621caf -->|satisfies| f24f11691f55af62;
  dfd639447d711d62["parser.rs"];
  class dfd639447d711d62 default;
  click dfd639447d711d62 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/parser.rs";
  dfd639447d711d62 -->|satisfies| f24f11691f55af62;
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

  b8208c9b1d6a312a["Mermaid Diagram Format Conversion"];
  click b8208c9b1d6a312a "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#mermaid-diagram-format-conversion";
  class b8208c9b1d6a312a requirement;
  a0274ca0625d8493["UserRequirements.md/Export Diagrams in Standard Formats"];
  class a0274ca0625d8493 requirement;
  click a0274ca0625d8493 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  b8208c9b1d6a312a -.->|deriveReqT| a0274ca0625d8493;
  95f7dce6aff47396["diagrams.rs"];
  class 95f7dce6aff47396 default;
  click 95f7dce6aff47396 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/diagrams.rs";
  95f7dce6aff47396 -->|satisfies| b8208c9b1d6a312a;
  cb1ad39c7552a3cd["Diagram Storage Path Configuration"];
  click cb1ad39c7552a3cd "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#diagram-storage-path-configuration";
  class cb1ad39c7552a3cd requirement;
  89097c1311055b72["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class 89097c1311055b72 requirement;
  click 89097c1311055b72 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  cb1ad39c7552a3cd -.->|deriveReqT| 89097c1311055b72;
  36fd2b1e82621caf["model.rs"];
  class 36fd2b1e82621caf default;
  click 36fd2b1e82621caf "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/model.rs";
  36fd2b1e82621caf -->|satisfies| cb1ad39c7552a3cd;
  d812c13722abb71d["Visual Differential Rendering"];
  click d812c13722abb71d "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#visual-differential-rendering";
  class d812c13722abb71d requirement;
  ac914f743d73674e["UserRequirements.md/Highlight Changes in Diagrams"];
  class ac914f743d73674e requirement;
  click ac914f743d73674e "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  d812c13722abb71d -.->|deriveReqT| ac914f743d73674e;
  793154acc336992c["Automated Diagram Generation on PR Merge"];
  click 793154acc336992c "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge";
  class 793154acc336992c requirement;
  c522cf4c404bdc24["UserRequirements.md/Automate Diagram Generation"];
  class c522cf4c404bdc24 requirement;
  click c522cf4c404bdc24 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#automate-diagram-generation";
  793154acc336992c -.->|deriveReqT| c522cf4c404bdc24;
  b41f362e18fb2449["UserRequirements.md/Automate Pull Request Validations"];
  class b41f362e18fb2449 requirement;
  click b41f362e18fb2449 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#automate-pull-request-validations";
  793154acc336992c -.->|deriveReqT| b41f362e18fb2449;
  fdd29f919065644d["generate_diagrams.yml"];
  class fdd29f919065644d default;
  click fdd29f919065644d "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/.github/workflows/generate_diagrams.yml";
  fdd29f919065644d -->|satisfies| 793154acc336992c;
  4bf74699f40f4a76["Configurable Filename Exclusion Patterns"];
  click 4bf74699f40f4a76 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  class 4bf74699f40f4a76 requirement;
  b3ef0fb91572bcf0["ManagingMbseModelsRequirements.md/Project Configuration with YAML"];
  class b3ef0fb91572bcf0 requirement;
  click b3ef0fb91572bcf0 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  4bf74699f40f4a76 -.->|deriveReqT| b3ef0fb91572bcf0;
  d89f4b0d2edaac20["config.rs"];
  class d89f4b0d2edaac20 default;
  click d89f4b0d2edaac20 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/cli/src/config.rs";
  d89f4b0d2edaac20 -->|satisfies| 4bf74699f40f4a76;
  5071808e4276f33a["SysML-Compatible Relationship Rendering"];
  click 5071808e4276f33a "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#sysml-compatible-relationship-rendering";
  class 5071808e4276f33a requirement;
  eed0b020b6ddeae1["UserRequirements.md/Visualize Model Relationships"];
  class eed0b020b6ddeae1 requirement;
  click eed0b020b6ddeae1 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#visualize-model-relationships";
  5071808e4276f33a -.->|deriveReqT| eed0b020b6ddeae1;
  95f7dce6aff47396 -->|satisfies| 5071808e4276f33a;
  bd72564894314bf6["Relationship Type Filter Implementation"];
  click bd72564894314bf6 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#relationship-type-filter-implementation";
  class bd72564894314bf6 requirement;
  66e9d8186acafd13["UserRequirements.md/Filter Relationships by Type"];
  class 66e9d8186acafd13 requirement;
  click 66e9d8186acafd13 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#filter-relationships-by-type";
  bd72564894314bf6 -.->|deriveReqT| 66e9d8186acafd13;
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

  cdab2d3174ce86a9["CLI Traces Flag"];
  click cdab2d3174ce86a9 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#cli-traces-flag";
  class cdab2d3174ce86a9 requirement;
  c5b8a7944b6943e2["UserRequirements.md/Traceability Matrix"];
  class c5b8a7944b6943e2 requirement;
  click c5b8a7944b6943e2 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#traceability-matrix";
  cdab2d3174ce86a9 -.->|deriveReqT| c5b8a7944b6943e2;
  2f21d2133dbfd205["cli.rs"];
  class 2f21d2133dbfd205 default;
  click 2f21d2133dbfd205 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/cli/src/cli.rs";
  2f21d2133dbfd205 -->|satisfies| cdab2d3174ce86a9;
  a8066f495e5ed5dd["Traceability Matrix Builder Implementation"];
  click a8066f495e5ed5dd "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#traceability-matrix-builder-implementation";
  class a8066f495e5ed5dd requirement;
  a8066f495e5ed5dd -.->|deriveReqT| c5b8a7944b6943e2;
  4966ff561112ec56["matrix_generator.rs"];
  class 4966ff561112ec56 default;
  click 4966ff561112ec56 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/matrix_generator.rs";
  4966ff561112ec56 -->|satisfies| a8066f495e5ed5dd;
  a0943a440707d910["CLI Traces SVG Flag"];
  click a0943a440707d910 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#cli-traces-svg-flag";
  class a0943a440707d910 requirement;
  a0943a440707d910 -.->|deriveReqT| cdab2d3174ce86a9;
  2f21d2133dbfd205 -->|satisfies| a0943a440707d910;
  3b2f98c43f1ed3bb["Markdown Matrix Formatter"];
  click 3b2f98c43f1ed3bb "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#markdown-matrix-formatter";
  class 3b2f98c43f1ed3bb requirement;
  3b2f98c43f1ed3bb -.->|deriveReqT| c5b8a7944b6943e2;
  4966ff561112ec56 -->|satisfies| 3b2f98c43f1ed3bb;
  c020978403254f09["CLI Change Impact Report Flag"];
  click c020978403254f09 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#cli-change-impact-report-flag";
  class c020978403254f09 requirement;
  48e08ff33bc860c5["Structural Change Analyzer"];
  class 48e08ff33bc860c5 requirement;
  click 48e08ff33bc860c5 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#structural-change-analyzer";
  c020978403254f09 -.->|deriveReqT| 48e08ff33bc860c5;
  2f21d2133dbfd205 -->|satisfies| c020978403254f09;
  79259d512a5c44a6["CLI Git Commit Hash Flag"];
  click 79259d512a5c44a6 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#cli-git-commit-hash-flag";
  class 79259d512a5c44a6 requirement;
  79259d512a5c44a6 -.->|deriveReqT| c020978403254f09;
  2f21d2133dbfd205 -->|satisfies| 79259d512a5c44a6;
  9b9c33c7182d6eeb["UserRequirements.md/Tracing Structural Changes"];
  class 9b9c33c7182d6eeb requirement;
  click 9b9c33c7182d6eeb "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#tracing-structural-changes";
  48e08ff33bc860c5 -.->|deriveReqT| 9b9c33c7182d6eeb;
  76acbdfdc0fba01e["model.rs"];
  class 76acbdfdc0fba01e default;
  click 76acbdfdc0fba01e "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/change_impact.rs";
  76acbdfdc0fba01e -->|satisfies| 48e08ff33bc860c5;
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

  eb3ba1b7474d0e60["Internal Consistency Validator"];
  click eb3ba1b7474d0e60 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#internal-consistency-validator";
  class eb3ba1b7474d0e60 requirement;
  f9182ad2999d989c["UserRequirements.md/Validate Internal Consistency"];
  class f9182ad2999d989c requirement;
  click f9182ad2999d989c "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#validate-internal-consistency";
  eb3ba1b7474d0e60 -.->|deriveReqT| f9182ad2999d989c;
  36fd2b1e82621caf["model.rs"];
  class 36fd2b1e82621caf default;
  click 36fd2b1e82621caf "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/model.rs";
  36fd2b1e82621caf -->|satisfies| eb3ba1b7474d0e60;
  dfd639447d711d62["parser.rs"];
  class dfd639447d711d62 default;
  click dfd639447d711d62 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/parser.rs";
  dfd639447d711d62 -->|satisfies| eb3ba1b7474d0e60;
  3871ef72a30780e5["Excluded File Relation Validation"];
  click 3871ef72a30780e5 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#excluded-file-relation-validation";
  class 3871ef72a30780e5 requirement;
  48e8a0b4b18111c4["File Pattern Exclusion for Linting"];
  class 48e8a0b4b18111c4 requirement;
  click 48e8a0b4b18111c4 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  3871ef72a30780e5 -->|refines| 48e8a0b4b18111c4;
  dfd639447d711d62 -->|satisfies| 3871ef72a30780e5;
  df781ba215f4d187["Cross-Component Dependency Validator"];
  click df781ba215f4d187 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#cross-component-dependency-validator";
  class df781ba215f4d187 requirement;
  ee05a46627b568b7["UserRequirements.md/Validate Cross-Component Dependencies"];
  class ee05a46627b568b7 requirement;
  click ee05a46627b568b7 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#validate-cross-component-dependencies";
  df781ba215f4d187 -.->|deriveReqT| ee05a46627b568b7;
  36fd2b1e82621caf -->|satisfies| df781ba215f4d187;
  dfd639447d711d62 -->|satisfies| df781ba215f4d187;
  bff4e3e834a9ffcc["Relation Type Validation"];
  click bff4e3e834a9ffcc "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#relation-type-validation";
  class bff4e3e834a9ffcc requirement;
  3d2fe0b05ff9c8e3["UserRequirements.md/Enhanced Validation Error Reporting"];
  class 3d2fe0b05ff9c8e3 requirement;
  click 3d2fe0b05ff9c8e3 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  bff4e3e834a9ffcc -.->|deriveReqT| 3d2fe0b05ff9c8e3;
  a01a924ab0c27629["src/relation.rs"];
  class a01a924ab0c27629 default;
  click a01a924ab0c27629 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/relation.rs";
  a01a924ab0c27629 -->|satisfies| bff4e3e834a9ffcc;
  7b53d332f5da95a["Filesystem Structure Validator"];
  click 7b53d332f5da95a "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#filesystem-structure-validator";
  class 7b53d332f5da95a requirement;
  c390b990a63def2a["UserRequirements.md/Validate Filesystem Structure"];
  class c390b990a63def2a requirement;
  click c390b990a63def2a "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#validate-filesystem-structure";
  7b53d332f5da95a -.->|deriveReqT| c390b990a63def2a;
  36fd2b1e82621caf -->|satisfies| 7b53d332f5da95a;
  774d12db509b4a55["Relation Element Type Validator"];
  click 774d12db509b4a55 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#relation-element-type-validator";
  class 774d12db509b4a55 requirement;
  8a3ca9461643d887["../UserRequirements.md#Validate Relation Types"];
  class 8a3ca9461643d887 requirement;
  click 8a3ca9461643d887 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#validate-relation-types";
  774d12db509b4a55 -.->|deriveReqT| 8a3ca9461643d887;
  29eb679eef252d12["../SpecificationsRequirements.md#Relation Types And Behaviors"];
  class 29eb679eef252d12 requirement;
  click 29eb679eef252d12 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SpecificationsRequirements.md#relation-types-and-behaviors";
  774d12db509b4a55 -.->|deriveReqT| 29eb679eef252d12;
  36fd2b1e82621caf -->|satisfies| 774d12db509b4a55;
  dfd639447d711d62 -->|satisfies| 774d12db509b4a55;
  d4722c13c32cf9e0["Markdown Structure Validator"];
  click d4722c13c32cf9e0 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#markdown-structure-validator";
  class d4722c13c32cf9e0 requirement;
  7ec3cb7f400a2e8d["UserRequirements.md/Validate Markdown Structure"];
  class 7ec3cb7f400a2e8d requirement;
  click 7ec3cb7f400a2e8d "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#validate-markdown-structure";
  d4722c13c32cf9e0 -.->|deriveReqT| 7ec3cb7f400a2e8d;
  36fd2b1e82621caf -->|satisfies| d4722c13c32cf9e0;
  dfd639447d711d62 -->|satisfies| d4722c13c32cf9e0;
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

  ae7920e4b52a5854["Display Name-Regex Option in Help"];
  click ae7920e4b52a5854 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#display-name-regex-option-in-help";
  class ae7920e4b52a5854 requirement;
  2f21d2133dbfd205["../../cli/src/cli.rs"];
  class 2f21d2133dbfd205 default;
  click 2f21d2133dbfd205 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/cli/src/cli.rs";
  2f21d2133dbfd205 -->|satisfies| ae7920e4b52a5854;
  197dc113759da19b["CLI Summary Report Command"];
  class 197dc113759da19b requirement;
  click 197dc113759da19b "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#cli-summary-report-command";
  ae7920e4b52a5854 -->|refines| 197dc113759da19b;
  ecd5cbbaddffb824["../Verifications/ReportsTests.md#model-summary-tests"];
  class ecd5cbbaddffb824 verification;
  click ecd5cbbaddffb824 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/Verifications/ReportsTests.md#model-summary-tests";
  ecd5cbbaddffb824 -.->|verifies| ae7920e4b52a5854;
  d376638ec1fbd6e9["Validation Report Generator"];
  click d376638ec1fbd6e9 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#validation-report-generator";
  class d376638ec1fbd6e9 requirement;
  2d3cfde19fc6bb79["UserRequirements.md/Provide Validation Reports"];
  class 2d3cfde19fc6bb79 requirement;
  click 2d3cfde19fc6bb79 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#provide-validation-reports";
  d376638ec1fbd6e9 -.->|deriveReqT| 2d3cfde19fc6bb79;
  36fd2b1e82621caf["model.rs"];
  class 36fd2b1e82621caf default;
  click 36fd2b1e82621caf "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/model.rs";
  36fd2b1e82621caf -->|satisfies| d376638ec1fbd6e9;
  90e16b61e174ace5["Model Summary Fine Grained Filtering"];
  click 90e16b61e174ace5 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#model-summary-fine-grained-filtering";
  class 90e16b61e174ace5 requirement;
  11a8fc49a87327fe["../../core/src/reports.rs"];
  class 11a8fc49a87327fe default;
  click 11a8fc49a87327fe "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/core/src/reports.rs";
  11a8fc49a87327fe -->|satisfies| 90e16b61e174ace5;
  2f21d2133dbfd205 -->|satisfies| 90e16b61e174ace5;
  ce120a0d16cf2475["Model Summary Report Generator"];
  class ce120a0d16cf2475 requirement;
  click ce120a0d16cf2475 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#model-summary-report-generator";
  90e16b61e174ace5 -.->|deriveReqT| ce120a0d16cf2475;
  ecd5cbbaddffb824 -.->|verifies| 90e16b61e174ace5;
  40de7485b25294["UserRequirements.md/Model Structure and Summaries"];
  class 40de7485b25294 requirement;
  click 40de7485b25294 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/UserRequirements.md#model-structure-and-summaries";
  ce120a0d16cf2475 -.->|deriveReqT| 40de7485b25294;
  11a8fc49a87327fe -->|satisfies| ce120a0d16cf2475;
  197dc113759da19b -->|refines| ce120a0d16cf2475;
  197dc113759da19b -.->|deriveReqT| 90e16b61e174ace5;
  2f21d2133dbfd205 -->|satisfies| 197dc113759da19b;
  a862f56b69bd5819["Handle Invalid Regex Filter Patterns"];
  click a862f56b69bd5819 "https://github.com/Reqvire/reqvire/blob/30536324899084e88969175e3613c971ffa6e108/specifications/SystemRequirements/Requirements.md#handle-invalid-regex-filter-patterns";
  class a862f56b69bd5819 requirement;
  2f21d2133dbfd205 -->|satisfies| a862f56b69bd5819;
  a862f56b69bd5819 --o|contains| 197dc113759da19b;
  ecd5cbbaddffb824 -.->|verifies| a862f56b69bd5819;
```

---

### Model Summary Report Generator

The system shall implement a summary report generator that  produces comprehensive summaries of model relationships, including key metrics, element counts by type and counts.

#### Details

The summary report must include 

#### Relations
  * derivedFrom: [UserRequirements.md/Model Structure and Summaries](../UserRequirements.md#model-structure-and-summaries)
  * satisfiedBy: [model.rs](../../core/src/reports.rs)

---

### Model Summary Fine Grained Filtering

The system shall implement a fine grained filtering for the  summary report generator following the specifications.

#### Details

<details><summary>View Full Specification</summary>

## Summary

This specification defines the functional requirements for a filtering subsystem used within the `model-summary` reporting feature. The system must allow clients to selectively include or exclude elements from the summary output based on metadata, content, and traceability properties.

The filters shall be composable and applied conjunctively (i.e., all active filters must match for an element to be included). The filtering system must support both human-readable text output and structured machine-readable output (e.g., JSON).

---

## Filtering Scope

Filtering shall operate on the level of individual `Element` objects in the model registry. Each `Element` has the following relevant properties:

- `file_path: String`
- `name: String`
- `section: String`
- `element_type: ElementType`
- `content: String`
- `relations: Vec<Relation>`

---

## Supported Filters

The filtering system **must support the following filters**, which may be active simultaneously.

### 1. File Path Filter (Glob)

**Purpose:** Restrict summary to elements defined in files whose paths match a given glob pattern.

**Input:** A single string pattern using glob syntax (e.g., `"src/**/*Spec.md"`)

**Match Target:** `Element.file_path`

**Behavior:** Case-sensitive glob match. If the glob does not match any file, no elements are included.

---

### 2. Name Filter (Regex)

**Purpose:** Include only elements whose `name` matches a regular expression.

**Input:** A valid Rust-compatible regular expression (e.g., `"autonomous.*"`)

**Match Target:** `Element.name`

**Behavior:** Case-sensitive match by default. The filter is considered invalid if the regex fails to compile.

---

### 3. Section Filter (Glob)

**Purpose:** Include only elements belonging to sections with matching names.

**Input:** A glob pattern string (e.g., `"System Requirements*"`)

**Match Target:** `Element.section`

**Behavior:** Case-sensitive match. Globbing follows standard `globset` semantics.

---

### 4. Type Filter (Exact Match)

**Purpose:** Include only elements of a specific type.

**Input:** One of the following valid string identifiers:

- `"user-requirement"`
- `"system-requirement"`
- `"verification"`
- `"file"`
- Any user-defined type (e.g., `"interface"`, `"design"`)

**Match Target:** `Element.element_type`

**Behavior:** Matching must be exact. Internally, the filter string shall be mapped to an `ElementType` via a deterministic lookup function.

---

### 5. Content Filter (Regex)

**Purpose:** Include only elements whose body content matches a regular expression.

**Input:** A valid regex pattern applied to the element’s `content`.

**Match Target:** `Element.content`

**Behavior:** Case-sensitive regex match. Invalid patterns must cause an immediate user-facing error.

---

### 6. Not Verified Filter (Boolean)

**Purpose:** Include only requirement elements that are not connected via a `verifiedBy` or `verify` relation.

**Input:** Boolean flag

**Match Target:** `Element.relations`

**Behavior:** If enabled, any element with one or more verification-related relations must be excluded.

---

### 7. Not Satisfied Filter (Boolean)

**Purpose:** Include only requirement elements that are not connected via a `satisfiedBy` or `satisfy` relation.

**Input:** Boolean flag

**Match Target:** `Element.relations`

**Behavior:** If enabled, any element with one or more satisfaction-related relations must be excluded.

---

## Filter Composition

All filters are applied **conjunctively**. That is, an element is included in the summary **only if all active filters return `true`** for that element.

---

## Error Handling

- Invalid regular expressions must produce a fatal error with a descriptive message.
- Invalid glob patterns should fail at startup with appropriate feedback.
- Unknown or malformed `type` filters should be rejected with a list of accepted values.

---

## Extension Considerations

The filtering system must be designed to allow future additions, including:

- Filtering by relation type presence (e.g., "has any relation")
- Filtering by linked element types (e.g., "verifiedBy test-verification")
- Inversion (e.g., "not in section X")

---

## Output Behavior

Filtered results must be consistent across all output modes (text, JSON, HTML). The final summary must include only elements passing the full filter set, and global counters should reflect the filtered subset.

---

## Performance Considerations

The filtering system must evaluate filters with minimal passes over element data. Repeated relation scans (e.g., for verification/satisfaction) should be avoided in favor of single-pass accumulation.

---

## Test Cases (Examples)

| Filter Combination | Expected Result |
|--------------------|------------------|
| `type = verification` | Only verification elements |
| `section = "System*"` + `name = ".*GPS.*"` | System section elements with GPS in name |
| `type = system-requirement` + `not_verified = true` | Unverified system requirements only |

---


</details>

#### Relations
  * satisfiedBy: [../../core/src/reports.rs](../../core/src/reports.rs)
  * satisfiedBy: [../../cli/src/cli.rs](../../cli/src/cli.rs)    
  * derivedFrom: [Model Summary Report Generator](#model-summary-report-generator)  
  * verifiedBy: [../Verifications/ReportsTests.md#model-summary-tests](../Verifications/ReportsTests.md#model-summary-tests)  

---

### CLI Summary Report Command

The system shall provide a model summary report function, activated by the (--model-summary flag), which shall generate model summary report with ability to be passed several filters.

#### Details

Model summary CLI command:
- `--model-summary`:  Output model registry and summary, also supports json output.

All filters require --model-summary to be present. They can be combined:
- `--model-summary`:  Output model registry and summary, also supports json output.
  - By file path: ` --model-summary  --filter-file="src/**/*Reqs.md"`
  - By name: ` --model-summary  --filter-name=".*safety.*"`
  - By section: ` --model-summary  --filter-section="System*"`
  - By type: ` --model-summary  --filter-type="system-requirement"` (exact match)
  - By content: ` --model-summary  --filter-content="MUST"`
  - Not verified: ` --model-summary  --filter-is-not-verified`
  - Not satisfied: ` --model-summary  --filter-is-not-satisfied`

#### Relations
  * refine: [Model Summary Report Generator](#model-summary-report-generator)
  * derivedFrom: [Model Summary Fine Grained Filtering](#model-summary-fine-grained-filtering)  
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)    

---

### Handle Invalid Regex Filter Patterns

When the user invokes Reqvire with the `--model-summary` and where invalid regular expression to regex based filters are provided the system shall return an error showing the faulty pattern and exit without producing a summary.

#### Relations
  * satisfiedBy: [../../cli/src/cli.rs](../../cli/src/cli.rs)    
  * containedBy: [CLI Summary Report Command](#cli-summary-report-command)  
  * verifiedBy: [../Verifications/ReportsTests.md#model-summary-tests](../Verifications/ReportsTests.md#model-summary-tests)  

---

### Display Name-Regex Option in Help

When the user requests help (`--help` or `-h`), the system shall list module summary filter flags under the MODEL SUMMARY FILTERS heading, including its description and its dependency on `--model-summary`.

#### Relations
  * satisfiedBy: [../../cli/src/cli.rs](../../cli/src/cli.rs) 
  * refine: [CLI Summary Report Command](#cli-summary-report-command)  
  * verifiedBy: [../Verifications/ReportsTests.md#model-summary-tests](../Verifications/ReportsTests.md#model-summary-tests)  

---

### Validation Report Generator

The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Relations
  * derivedFrom: [UserRequirements.md/Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---