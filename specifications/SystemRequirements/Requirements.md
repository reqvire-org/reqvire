# System Requirements

## Linting    
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  f540b21f7eced636["Missing Separators Linting Implementation"];
  click f540b21f7eced636 "Requirements.md#missing-separators-linting-implementation";
  class f540b21f7eced636 requirement;
  1ddbeea0cf8eaad5["UserRequirements.md/Format Consistency Enforcement"];
  class 1ddbeea0cf8eaad5 requirement;
  click 1ddbeea0cf8eaad5 "../UserRequirements.md#format-consistency-enforcement";
  f540b21f7eced636 -.->|deriveReqT| 1ddbeea0cf8eaad5;
  45e2af09e602182a["linting/separators.rs"];
  class 45e2af09e602182a default;
  click 45e2af09e602182a "../../core/src/linting/separators.rs";
  45e2af09e602182a -->|satisfies| f540b21f7eced636;
  c87ad8ce58149089["Parallel Linting Processing"];
  click c87ad8ce58149089 "Requirements.md#parallel-linting-processing";
  class c87ad8ce58149089 requirement;
  84c4dc11e82e8638["UserRequirements.md/Model Linting"];
  class 84c4dc11e82e8638 requirement;
  click 84c4dc11e82e8638 "../UserRequirements.md#model-linting";
  c87ad8ce58149089 -.->|deriveReqT| 84c4dc11e82e8638;
  3e39cdcf485c5250["linting/mod.rs"];
  class 3e39cdcf485c5250 default;
  click 3e39cdcf485c5250 "../../core/src/linting/mod.rs";
  3e39cdcf485c5250 -->|satisfies| c87ad8ce58149089;
  3f235c1000d5347f["CLI Lint Flag"];
  click 3f235c1000d5347f "Requirements.md#cli-lint-flag";
  class 3f235c1000d5347f requirement;
  28b0f9fa78937e61["UserRequirements.md/Linting Command Behavior"];
  class 28b0f9fa78937e61 requirement;
  click 28b0f9fa78937e61 "../UserRequirements.md#linting-command";
  3f235c1000d5347f -.->|deriveReqT| 28b0f9fa78937e61;
  2f21d2133dbfd205["cli.rs"];
  class 2f21d2133dbfd205 default;
  click 2f21d2133dbfd205 "../../cli/src/cli.rs";
  2f21d2133dbfd205 -->|satisfies| 3f235c1000d5347f;
  e42a0715e74ccd66["Incosistent Newlines Linting Implementation"];
  click e42a0715e74ccd66 "Requirements.md#incosistent-newlines-linting-implementation";
  class e42a0715e74ccd66 requirement;
  e42a0715e74ccd66 -.->|deriveReqT| 1ddbeea0cf8eaad5;
  411d20097517505a["linting/newlines.rs"];
  class 411d20097517505a default;
  click 411d20097517505a "../../core/src/linting/newlines.rs";
  411d20097517505a -->|satisfies| e42a0715e74ccd66;
  48e8a0b4b18111c4["File Pattern Exclusion for Linting"];
  click 48e8a0b4b18111c4 "Requirements.md#file-pattern-exclusion-for-linting";
  class 48e8a0b4b18111c4 requirement;
  be5c69c33c5a4cf6["Ignoring Unstructured Documents"];
  class be5c69c33c5a4cf6 requirement;
  click be5c69c33c5a4cf6 "Requirements.md#ignoring-unstructured-documents";
  48e8a0b4b18111c4 -.->|deriveReqT| be5c69c33c5a4cf6;
  85989fd7bb727157["utils.rs"];
  class 85989fd7bb727157 default;
  click 85989fd7bb727157 "../../core/src/utils.rs";
  85989fd7bb727157 -->|satisfies| 48e8a0b4b18111c4;
  d21b16b30de7350d["Dry Run Mode"];
  click d21b16b30de7350d "Requirements.md#dry-run-mode";
  class d21b16b30de7350d requirement;
  d21b16b30de7350d -.->|deriveReqT| 3f235c1000d5347f;
  2f21d2133dbfd205 -->|satisfies| d21b16b30de7350d;
  56174a2a92eb637["Reserved Subsections Linting Implementation"];
  click 56174a2a92eb637 "Requirements.md#reserved-subsections-linting-implementation";
  class 56174a2a92eb637 requirement;
  56174a2a92eb637 -.->|deriveReqT| 1ddbeea0cf8eaad5;
  8c7c2481d50c628c["linting/reserved_subsections.rs"];
  class 8c7c2481d50c628c default;
  click 8c7c2481d50c628c "../../core/src/linting/indentation.rs";
  8c7c2481d50c628c -->|satisfies| 56174a2a92eb637;
  7c2ad2603d27c318["Git-Style Diff Output for Linting"];
  click 7c2ad2603d27c318 "Requirements.md#git-style-diff-output-for-linting";
  class 7c2ad2603d27c318 requirement;
  62c066a5aad4dafe["UserRequirements.md/Linting Command Output"];
  class 62c066a5aad4dafe requirement;
  click 62c066a5aad4dafe "../UserRequirements.md#linting-command-output";
  7c2ad2603d27c318 -.->|deriveReqT| 62c066a5aad4dafe;
  3e39cdcf485c5250 -->|satisfies| 7c2ad2603d27c318;
  ab8dfb01e717d34["Excess Whitespace Linting Implementation"];
  click ab8dfb01e717d34 "Requirements.md#excess-whitespace-linting-implementation";
  class ab8dfb01e717d34 requirement;
  ab8dfb01e717d34 -.->|deriveReqT| 1ddbeea0cf8eaad5;
  fd5178bf78bae55c["linting/whitespace.rs"];
  class fd5178bf78bae55c default;
  click fd5178bf78bae55c "../../core/src/linting/whitespace.rs";
  fd5178bf78bae55c -->|satisfies| ab8dfb01e717d34;
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
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents) 
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

  4c79bf4ceda6e23f["User Requirement Root Folders Support"];
  click 4c79bf4ceda6e23f "Requirements.md#user-requirement-root-folders-support";
  class 4c79bf4ceda6e23f requirement;
  82d6413dc779c791["ManagingMbseModelsRequirements.md#Configurable User Requirements Root Folder"];
  class 82d6413dc779c791 requirement;
  click 82d6413dc779c791 "../ManagingMbseModelsRequirements.md#configurable-user-requirements-root-folder";
  4c79bf4ceda6e23f -.->|deriveReqT| 82d6413dc779c791;
  d89f4b0d2edaac20["config.rs"];
  class d89f4b0d2edaac20 default;
  click d89f4b0d2edaac20 "../../cli/src/config.rs";
  d89f4b0d2edaac20 -->|satisfies| 4c79bf4ceda6e23f;
  6ccd085f7c76afca["Subdirectory Processing Flag"];
  click 6ccd085f7c76afca "Requirements.md#subdirectory-processing-flag";
  class 6ccd085f7c76afca requirement;
  b3ef0fb91572bcf0["ManagingMbseModelsRequirements.md/Project Configuration with YAML"];
  class b3ef0fb91572bcf0 requirement;
  click b3ef0fb91572bcf0 "../ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  6ccd085f7c76afca -.->|deriveReqT| b3ef0fb91572bcf0;
  2f21d2133dbfd205["cli.rs"];
  class 2f21d2133dbfd205 default;
  click 2f21d2133dbfd205 "../../cli/src/cli.rs";
  2f21d2133dbfd205 -->|satisfies| 6ccd085f7c76afca;
  be5c69c33c5a4cf6["Ignoring Unstructured Documents"];
  click be5c69c33c5a4cf6 "Requirements.md#ignoring-unstructured-documents";
  class be5c69c33c5a4cf6 requirement;
  be5c69c33c5a4cf6 -.->|deriveReqT| b3ef0fb91572bcf0;
  da5831ca85881025["ManagingMbseModelsRequirements.md#Coexistence of Structured and Unstructured Documents"];
  class da5831ca85881025 requirement;
  click da5831ca85881025 "../ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  be5c69c33c5a4cf6 -.->|deriveReqT| da5831ca85881025;
  d89f4b0d2edaac20 -->|satisfies| be5c69c33c5a4cf6;
```
---

### User Requirement Root Folders Support

The system shall implement configuration parameter that would specify a single folder path, relative to the Git repository root, that is designated as the primary location for user requirements.

#### Details

'paths.user_requirements_root_folder' parameter of type  String defines default folder for the user-requirements.

All elements in markdown files (except those matching exclusion patterns) in root of this folders are considered **user requirements** unless explicitly set as other element type in the metadata.

#### Relations
  * derivedFrom: [ManagingMbseModelsRequirements.md#Configurable User Requirements Root Folder](../ManagingMbseModelsRequirements.md#configurable-user-requirements-root-folder)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)

---

### Ignoring Unstructured Documents

The system shall support configurable glob patterns to exclude specific files from requirement processing.

#### Details
```reqvire.yaml
paths:

  # Glob patterns to exclude from structured documents processing
  excluded_filename_patterns:
    - "**/Logical*.md"
    - "**/Physical*.md"    
```

#### Relations
  * derivedFrom: [ManagingMbseModelsRequirements.md/Project Configuration with YAML](../ManagingMbseModelsRequirements.md#project-configuration-with-yaml)
  * derivedFrom: [ManagingMbseModelsRequirements.md#Coexistence of Structured and Unstructured Documents](../ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)

---

### Subdirectory Processing Flag

The system shall provide a flag (--subdirectory) that allows processing only files within a specific subdirectory relative to the git repository root, enabling focused analysis and improved performance when working with large repositories.

#### Details

The subdirectory flag is designed to limit the scope of processing to a specific subdirectory, which is especially useful in large repositories with many requirements files. This flag allows users to:

1. Process only files within the specified subdirectory and its nested folders
2. Generate reports, diagrams, and validations based on the limited scope
3. Improve performance by reducing the number of files that need to be processed

The flag takes a path that is relative to the git repository root and should be used as follows:
```
reqvire --subdirectory="specifications/Verifications" --validate
```

#### Relations
  * derivedFrom: [ManagingMbseModelsRequirements.md/Project Configuration with YAML](../ManagingMbseModelsRequirements.md#project-configuration-with-yaml)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

---

## CLI
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  1fa02ea7ed4b2adb["Export Related System Elements"];
  click 1fa02ea7ed4b2adb "Requirements.md#export-related-system-elements";
  class 1fa02ea7ed4b2adb requirement;
  a2b1d4ec64cbd441["#HTML Export"];
  class a2b1d4ec64cbd441 requirement;
  click a2b1d4ec64cbd441 "Requirements.md#html-export";
  1fa02ea7ed4b2adb -->|refines| a2b1d4ec64cbd441;
  5dbf1a3141d3defb["html_export.rs"];
  class 5dbf1a3141d3defb default;
  click 5dbf1a3141d3defb "../../core/src/html_export.rs";
  5dbf1a3141d3defb -->|satisfies| 1fa02ea7ed4b2adb;
  d9686a154fe87b2["../UserRequirements.md/Export HTML specifications"];
  class d9686a154fe87b2 requirement;
  click d9686a154fe87b2 "../UserRequirements.md#export-html-specifications";
  a2b1d4ec64cbd441 -.->|deriveReqT| d9686a154fe87b2;
  5dbf1a3141d3defb -->|satisfies| a2b1d4ec64cbd441;
  a17b8052345712d7["Interactive Mermaid Diagram Node Behavior"];
  click a17b8052345712d7 "Requirements.md#interactive-mermaid-diagram-node-behavior";
  class a17b8052345712d7 requirement;
  b8997351b6f34048["UserRequirements.md/Interactive Mermaid Diagrams"];
  class b8997351b6f34048 requirement;
  click b8997351b6f34048 "../UserRequirements.md#interactive-mermaid-diagrams";
  a17b8052345712d7 -.->|deriveReqT| b8997351b6f34048;
  95f7dce6aff47396["html.rs"];
  class 95f7dce6aff47396 default;
  click 95f7dce6aff47396 "../../core/src/diagrams.rs";
  95f7dce6aff47396 -->|satisfies| a17b8052345712d7;
  3b4bfa0725509a0e["Index Generation"];
  click 3b4bfa0725509a0e "Requirements.md#index-generation";
  class 3b4bfa0725509a0e requirement;
  9019be8bfdc22b35["UserRequirements.md/Generate Documentation Index"];
  class 9019be8bfdc22b35 requirement;
  click 9019be8bfdc22b35 "../UserRequirements.md#generate-documentation-index";
  3b4bfa0725509a0e -.->|deriveReqT| 9019be8bfdc22b35;
  3461f4a59371fd45["index_generator.rs"];
  class 3461f4a59371fd45 default;
  click 3461f4a59371fd45 "../../core/src/index_generator.rs";
  3461f4a59371fd45 -->|satisfies| 3b4bfa0725509a0e;
  d72f6096b9a5dd8e["Detailed Error Handling and Logging"];
  click d72f6096b9a5dd8e "Requirements.md#detailed-error-handling-and-logging";
  class d72f6096b9a5dd8e requirement;
  3d2fe0b05ff9c8e3["../UserRequirements.md#Enhanced Validation Error Reporting"];
  class 3d2fe0b05ff9c8e3 requirement;
  click 3d2fe0b05ff9c8e3 "../UserRequirements.md#enhanced-validation-error-reporting";
  d72f6096b9a5dd8e -.->|deriveReqT| 3d2fe0b05ff9c8e3;
  af1374ae222fbd72["src/error.rs"];
  class af1374ae222fbd72 default;
  click af1374ae222fbd72 "../../core/src/error.rs";
  af1374ae222fbd72 -->|satisfies| d72f6096b9a5dd8e;
  1550bb73b8029cb["HTML Navigation Enhancement"];
  click 1550bb73b8029cb "Requirements.md#html-navigation-enhancement";
  class 1550bb73b8029cb requirement;
  b5146db7aedfd66["UserRequirements.md/Documentation Index HTML Integration"];
  class b5146db7aedfd66 requirement;
  click b5146db7aedfd66 "../UserRequirements.md#documentation-index-html-integration";
  1550bb73b8029cb -.->|deriveReqT| b5146db7aedfd66;
  59e3b5b3087497da["html.rs"];
  class 59e3b5b3087497da default;
  click 59e3b5b3087497da "../../core/src/html.rs";
  59e3b5b3087497da -->|satisfies| 1550bb73b8029cb;
  5dbf1a3141d3defb -->|satisfies| 1550bb73b8029cb;
  d177913fffd1ed11["LLM Context Command"];
  click d177913fffd1ed11 "Requirements.md#llm-context-command";
  class d177913fffd1ed11 requirement;
  7fd9156eac77c270["UserRequirements.md/AI Agent Context"];
  class 7fd9156eac77c270 requirement;
  click 7fd9156eac77c270 "../UserRequirements.md#ai-agent-context";
  d177913fffd1ed11 -.->|deriveReqT| 7fd9156eac77c270;
  615a0f4ad2620118["main.rs"];
  class 615a0f4ad2620118 default;
  click 615a0f4ad2620118 "../../cli/src/main.rs";
  615a0f4ad2620118 -->|satisfies| d177913fffd1ed11;
```
---

### Index Generation

The system shall implement an IndexGenerator component that traverses the specifications directory structure and creates a hierarchical SpecificationIndex.md file with links to documents and elements in the repository root.

#### Details

The index generator shall:
1. Traverse all specifications and documents in the model
2. Group elements by file and section
3. Create a hierarchical index with links to documents and elements
4. Generate summary statistics including total files, sections, and elements
5. Write the index as SpecificationIndex.md to the repository root, not the output folder

#### Relations
  * derivedFrom: [UserRequirements.md/Generate Documentation Index](../UserRequirements.md#generate-documentation-index)
  * satisfiedBy: [index_generator.rs](../../core/src/index_generator.rs)

---

### HTML Navigation Enhancement 

The system shall enhance the HTML generator to process SpecificationIndex.md as a special file, adding navigation elements and ensuring it serves as the primary entry point.

#### Details

SpecificationIndex.md file must be saved as index.html file when exported to the HTML output directory.

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

### HTML Export

The system shall generate HTML output for all markdown files, not just requirements documents, to provide consistent representation of the entire model.

#### Relations
  * derivedFrom: [../UserRequirements.md/Export HTML specifications](../UserRequirements.md#export-html-specifications)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * satisfiedBy: [html.rs](../../core/src/export.rs)  

---

### Export Related System Elements

The system shall ensure that any related system elements are also copied into output folder to ensure consistency of exported model.

#### Relations
  * refine: [#HTML Export](#html-export)
  * satisfiedBy: [html_export.rs](../../core/src/html_export.rs)
  * satisfiedBy: [html.rs](../../core/src/export.rs)  

---

### Detailed Error Handling and Logging

The system shall implement detailed error handling and logging throughout the application to facilitate troubleshooting and provide meaningful feedback.

#### Relations
  * derivedFrom: [../UserRequirements.md#Enhanced Validation Error Reporting](../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [src/error.rs](../../core/src/error.rs)

---

## Logic
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  5071808e4276f33a["SysML-Compatible Relationship Rendering"];
  click 5071808e4276f33a "Requirements.md#sysml-compatible-relationship-rendering";
  class 5071808e4276f33a requirement;
  eed0b020b6ddeae1["UserRequirements.md/Visualize Model Relationships"];
  class eed0b020b6ddeae1 requirement;
  click eed0b020b6ddeae1 "../UserRequirements.md#visualize-model-relationships";
  5071808e4276f33a -.->|deriveReqT| eed0b020b6ddeae1;
  95f7dce6aff47396["diagrams.rs"];
  class 95f7dce6aff47396 default;
  click 95f7dce6aff47396 "../../core/src/diagrams.rs";
  95f7dce6aff47396 -->|satisfies| 5071808e4276f33a;
  bd72564894314bf6["Relationship Type Filter Implementation"];
  click bd72564894314bf6 "Requirements.md#relationship-type-filter-implementation";
  class bd72564894314bf6 requirement;
  66e9d8186acafd13["UserRequirements.md/Filter Relationships by Type"];
  class 66e9d8186acafd13 requirement;
  click 66e9d8186acafd13 "../UserRequirements.md#filter-relationships-by-type";
  bd72564894314bf6 -.->|deriveReqT| 66e9d8186acafd13;
  b8208c9b1d6a312a["Mermaid Diagram Format Conversion"];
  click b8208c9b1d6a312a "Requirements.md#mermaid-diagram-format-conversion";
  class b8208c9b1d6a312a requirement;
  a0274ca0625d8493["UserRequirements.md/Export Diagrams in Standard Formats"];
  class a0274ca0625d8493 requirement;
  click a0274ca0625d8493 "../UserRequirements.md#export-diagrams-in-standard-formats";
  b8208c9b1d6a312a -.->|deriveReqT| a0274ca0625d8493;
  95f7dce6aff47396 -->|satisfies| b8208c9b1d6a312a;
  d812c13722abb71d["Visual Differential Rendering"];
  click d812c13722abb71d "Requirements.md#visual-differential-rendering";
  class d812c13722abb71d requirement;
  ac914f743d73674e["UserRequirements.md/Highlight Changes in Diagrams"];
  class ac914f743d73674e requirement;
  click ac914f743d73674e "../UserRequirements.md#highlight-changes-in-diagrams";
  d812c13722abb71d -.->|deriveReqT| ac914f743d73674e;
  793154acc336992c["Automated Diagram Generation on PR Merge"];
  click 793154acc336992c "Requirements.md#automated-diagram-generation-on-pr-merge";
  class 793154acc336992c requirement;
  c522cf4c404bdc24["UserRequirements.md/Automate Diagram Generation"];
  class c522cf4c404bdc24 requirement;
  click c522cf4c404bdc24 "../UserRequirements.md#automate-diagram-generation";
  793154acc336992c -.->|deriveReqT| c522cf4c404bdc24;
  b41f362e18fb2449["UserRequirements.md/Automate Pull Request Validations"];
  class b41f362e18fb2449 requirement;
  click b41f362e18fb2449 "../UserRequirements.md#automate-pull-request-validations";
  793154acc336992c -.->|deriveReqT| b41f362e18fb2449;
  fdd29f919065644d["generate_diagrams.yml"];
  class fdd29f919065644d default;
  click fdd29f919065644d "../../.github/workflows/generate_diagrams.yml";
  fdd29f919065644d -->|satisfies| 793154acc336992c;
  cb1ad39c7552a3cd["Diagram Storage Path Configuration"];
  click cb1ad39c7552a3cd "Requirements.md#diagram-storage-path-configuration";
  class cb1ad39c7552a3cd requirement;
  89097c1311055b72["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class 89097c1311055b72 requirement;
  click 89097c1311055b72 "../UserRequirements.md#store-automated-diagrams-in-designated-locations";
  cb1ad39c7552a3cd -.->|deriveReqT| 89097c1311055b72;
  36fd2b1e82621caf["model.rs"];
  class 36fd2b1e82621caf default;
  click 36fd2b1e82621caf "../../core/src/model.rs";
  36fd2b1e82621caf -->|satisfies| cb1ad39c7552a3cd;
  480cb73e8bcb0786["Structured Markdown Files Search and Detection"];
  click 480cb73e8bcb0786 "Requirements.md#structured-markdown-files-search-and-detection";
  class 480cb73e8bcb0786 requirement;
  f24f11691f55af62["Requirements Processing"];
  class f24f11691f55af62 requirement;
  click f24f11691f55af62 "Requirements.md#requirements-processing";
  480cb73e8bcb0786 -.->|deriveReqT| f24f11691f55af62;
  36fd2b1e82621caf -->|satisfies| 480cb73e8bcb0786;
  4c79bf4ceda6e23f["User Requirement Root Folders Support"];
  class 4c79bf4ceda6e23f requirement;
  click 4c79bf4ceda6e23f "Requirements.md#user-requirement-root-folders-support";
  f24f11691f55af62 -.->|deriveReqT| 4c79bf4ceda6e23f;
  be5c69c33c5a4cf6["Ignoring Unstructured Documents"];
  class be5c69c33c5a4cf6 requirement;
  click be5c69c33c5a4cf6 "Requirements.md#ignoring-unstructured-documents";
  f24f11691f55af62 -.->|deriveReqT| be5c69c33c5a4cf6;
  36fd2b1e82621caf -->|satisfies| f24f11691f55af62;
  dfd639447d711d62["parser.rs"];
  class dfd639447d711d62 default;
  click dfd639447d711d62 "../../core/src/parser.rs";
  dfd639447d711d62 -->|satisfies| f24f11691f55af62;
```
---

### Requirements Processing

The system shall parse the files in all folders and subfolders from the root of git repository which are not explicitly excluded using the configuration from reqvire.yaml.

#### Relations
  * derivedFrom: [User Requirement Root Folders Support](#user-requirement-root-folders-support)
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)  
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)  

---

### Structured Markdown Files Search and Detection

The system shall identify all structured markdown documents available for processing in all directories and sub-directories of the git repository root based on predefined rules.

#### Details

Identification Process:
1. **File Selection**: The process scans all files in the the git repository root and all sub folders.
2. **Excluded Patterns Check**: If a file matches any excluded patterns, it is marked as **not a structured document file**.
3. **File Extension Check**: If the file does not have a `.md` extension, it is marked as **not a structured document file**.

#### Relations
  * derivedFrom: [Requirements Processing](#requirements-processing)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

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

## Change Impact Analisys
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  c020978403254f09["CLI Change Impact Report Flag"];
  click c020978403254f09 "Requirements.md#cli-change-impact-report-flag";
  class c020978403254f09 requirement;
  48e08ff33bc860c5["Structural Change Analyzer"];
  class 48e08ff33bc860c5 requirement;
  click 48e08ff33bc860c5 "Requirements.md#structural-change-analyzer";
  c020978403254f09 -.->|deriveReqT| 48e08ff33bc860c5;
  2f21d2133dbfd205["cli.rs"];
  class 2f21d2133dbfd205 default;
  click 2f21d2133dbfd205 "../../cli/src/cli.rs";
  2f21d2133dbfd205 -->|satisfies| c020978403254f09;
  3b2f98c43f1ed3bb["Markdown Matrix Formatter"];
  click 3b2f98c43f1ed3bb "Requirements.md#markdown-matrix-formatter";
  class 3b2f98c43f1ed3bb requirement;
  c5b8a7944b6943e2["UserRequirements.md/Traceability Matrix"];
  class c5b8a7944b6943e2 requirement;
  click c5b8a7944b6943e2 "../UserRequirements.md#traceability-matrix";
  3b2f98c43f1ed3bb -.->|deriveReqT| c5b8a7944b6943e2;
  4966ff561112ec56["matrix_generator.rs"];
  class 4966ff561112ec56 default;
  click 4966ff561112ec56 "../../core/src/matrix_generator.rs";
  4966ff561112ec56 -->|satisfies| 3b2f98c43f1ed3bb;
  a0943a440707d910["CLI Traces SVG Flag"];
  click a0943a440707d910 "Requirements.md#cli-traces-svg-flag";
  class a0943a440707d910 requirement;
  cdab2d3174ce86a9["CLI Traces Flag"];
  class cdab2d3174ce86a9 requirement;
  click cdab2d3174ce86a9 "Requirements.md#cli-traces-flag";
  a0943a440707d910 -.->|deriveReqT| cdab2d3174ce86a9;
  2f21d2133dbfd205 -->|satisfies| a0943a440707d910;
  a8066f495e5ed5dd["Traceability Matrix Builder Implementation"];
  click a8066f495e5ed5dd "Requirements.md#traceability-matrix-builder-implementation";
  class a8066f495e5ed5dd requirement;
  a8066f495e5ed5dd -.->|deriveReqT| c5b8a7944b6943e2;
  4966ff561112ec56 -->|satisfies| a8066f495e5ed5dd;
  79259d512a5c44a6["CLI Git Commit Hash Flag"];
  click 79259d512a5c44a6 "Requirements.md#cli-git-commit-hash-flag";
  class 79259d512a5c44a6 requirement;
  79259d512a5c44a6 -.->|deriveReqT| c020978403254f09;
  2f21d2133dbfd205 -->|satisfies| 79259d512a5c44a6;
  9b9c33c7182d6eeb["UserRequirements.md/Tracing Structural Changes"];
  class 9b9c33c7182d6eeb requirement;
  click 9b9c33c7182d6eeb "../UserRequirements.md#tracing-structural-changes";
  48e08ff33bc860c5 -.->|deriveReqT| 9b9c33c7182d6eeb;
  76acbdfdc0fba01e["model.rs"];
  class 76acbdfdc0fba01e default;
  click 76acbdfdc0fba01e "../../core/src/change_impact.rs";
  76acbdfdc0fba01e -->|satisfies| 48e08ff33bc860c5;
  cdab2d3174ce86a9 -.->|deriveReqT| c5b8a7944b6943e2;
  2f21d2133dbfd205 -->|satisfies| cdab2d3174ce86a9;
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

#### Details

Must support `--json` flag to output json formated string.

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

#### Details

Must support `--json` and `--svg` flags to output either json formated string or svg vector image.

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

  df781ba215f4d187["Cross-Component Dependency Validator"];
  click df781ba215f4d187 "Requirements.md#cross-component-dependency-validator";
  class df781ba215f4d187 requirement;
  ee05a46627b568b7["UserRequirements.md/Validate Cross-Component Dependencies"];
  class ee05a46627b568b7 requirement;
  click ee05a46627b568b7 "../UserRequirements.md#validate-cross-component-dependencies";
  df781ba215f4d187 -.->|deriveReqT| ee05a46627b568b7;
  36fd2b1e82621caf["model.rs"];
  class 36fd2b1e82621caf default;
  click 36fd2b1e82621caf "../../core/src/model.rs";
  36fd2b1e82621caf -->|satisfies| df781ba215f4d187;
  dfd639447d711d62["parser.rs"];
  class dfd639447d711d62 default;
  click dfd639447d711d62 "../../core/src/parser.rs";
  dfd639447d711d62 -->|satisfies| df781ba215f4d187;
  3871ef72a30780e5["Excluded File Relation Validation"];
  click 3871ef72a30780e5 "Requirements.md#excluded-file-relation-validation";
  class 3871ef72a30780e5 requirement;
  be5c69c33c5a4cf6["Ignoring Unstructured Documents"];
  class be5c69c33c5a4cf6 requirement;
  click be5c69c33c5a4cf6 "Requirements.md#ignoring-unstructured-documents";
  3871ef72a30780e5 -.->|deriveReqT| be5c69c33c5a4cf6;
  48e8a0b4b18111c4["File Pattern Exclusion for Linting"];
  class 48e8a0b4b18111c4 requirement;
  click 48e8a0b4b18111c4 "Requirements.md#file-pattern-exclusion-for-linting";
  3871ef72a30780e5 --o|contains| 48e8a0b4b18111c4;
  dfd639447d711d62 -->|satisfies| 3871ef72a30780e5;
  d4722c13c32cf9e0["Markdown Structure Validator"];
  click d4722c13c32cf9e0 "Requirements.md#markdown-structure-validator";
  class d4722c13c32cf9e0 requirement;
  7ec3cb7f400a2e8d["UserRequirements.md/Validate Markdown Structure"];
  class 7ec3cb7f400a2e8d requirement;
  click 7ec3cb7f400a2e8d "../UserRequirements.md#validate-markdown-structure";
  d4722c13c32cf9e0 -.->|deriveReqT| 7ec3cb7f400a2e8d;
  36fd2b1e82621caf -->|satisfies| d4722c13c32cf9e0;
  dfd639447d711d62 -->|satisfies| d4722c13c32cf9e0;
  774d12db509b4a55["Relation Element Type Validator"];
  click 774d12db509b4a55 "Requirements.md#relation-element-type-validator";
  class 774d12db509b4a55 requirement;
  8a3ca9461643d887["../UserRequirements.md#Validate Relation Types"];
  class 8a3ca9461643d887 requirement;
  click 8a3ca9461643d887 "../UserRequirements.md#validate-relation-types";
  774d12db509b4a55 -.->|deriveReqT| 8a3ca9461643d887;
  29eb679eef252d12["../SpecificationsRequirements.md#Relation Types And Behaviors"];
  class 29eb679eef252d12 requirement;
  click 29eb679eef252d12 "../SpecificationsRequirements.md#relation-types-and-behaviors";
  774d12db509b4a55 -.->|deriveReqT| 29eb679eef252d12;
  36fd2b1e82621caf -->|satisfies| 774d12db509b4a55;
  dfd639447d711d62 -->|satisfies| 774d12db509b4a55;
  bff4e3e834a9ffcc["Relation Type Validation"];
  click bff4e3e834a9ffcc "Requirements.md#relation-type-validation";
  class bff4e3e834a9ffcc requirement;
  3d2fe0b05ff9c8e3["UserRequirements.md/Enhanced Validation Error Reporting"];
  class 3d2fe0b05ff9c8e3 requirement;
  click 3d2fe0b05ff9c8e3 "../UserRequirements.md#enhanced-validation-error-reporting";
  bff4e3e834a9ffcc -.->|deriveReqT| 3d2fe0b05ff9c8e3;
  a01a924ab0c27629["src/relation.rs"];
  class a01a924ab0c27629 default;
  click a01a924ab0c27629 "../../core/src/relation.rs";
  a01a924ab0c27629 -->|satisfies| bff4e3e834a9ffcc;
  eb3ba1b7474d0e60["Internal Consistency Validator"];
  click eb3ba1b7474d0e60 "Requirements.md#internal-consistency-validator";
  class eb3ba1b7474d0e60 requirement;
  f9182ad2999d989c["UserRequirements.md/Validate Internal Consistency"];
  class f9182ad2999d989c requirement;
  click f9182ad2999d989c "../UserRequirements.md#validate-internal-consistency";
  eb3ba1b7474d0e60 -.->|deriveReqT| f9182ad2999d989c;
  36fd2b1e82621caf -->|satisfies| eb3ba1b7474d0e60;
  dfd639447d711d62 -->|satisfies| eb3ba1b7474d0e60;
```
---

### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces Reqvire's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Relations
  * derivedFrom: [UserRequirements.md/Validate Markdown Structure](../UserRequirements.md#validate-markdown-structure)
  * satisfiedBy: [model.rs](../../core/src/model.rs)    
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)

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

#### Relations
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents) 
  * containedBy: [File Pattern Exclusion for Linting](#file-pattern-exclusion-for-linting)
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

  a862f56b69bd5819["Handle Invalid Regex Filter Patterns"];
  click a862f56b69bd5819 "Requirements.md#handle-invalid-regex-filter-patterns";
  class a862f56b69bd5819 requirement;
  2f21d2133dbfd205["../../cli/src/cli.rs"];
  class 2f21d2133dbfd205 default;
  click 2f21d2133dbfd205 "../../cli/src/cli.rs";
  2f21d2133dbfd205 -->|satisfies| a862f56b69bd5819;
  197dc113759da19b["CLI Summary Report Command"];
  class 197dc113759da19b requirement;
  click 197dc113759da19b "Requirements.md#cli-summary-report-command";
  a862f56b69bd5819 --o|contains| 197dc113759da19b;
  ecd5cbbaddffb824["../Verifications/ReportsTests.md#model-summary-tests"];
  class ecd5cbbaddffb824 verification;
  click ecd5cbbaddffb824 "../Verifications/ReportsTests.md#model-summary-tests";
  ecd5cbbaddffb824 -.->|verifies| a862f56b69bd5819;
  ce120a0d16cf2475["Model Summary Report Generator"];
  class ce120a0d16cf2475 requirement;
  click ce120a0d16cf2475 "Requirements.md#model-summary-report-generator";
  197dc113759da19b -.->|deriveReqT| ce120a0d16cf2475;
  2f21d2133dbfd205 -->|satisfies| 197dc113759da19b;
  40de7485b25294["UserRequirements.md/Model Structure and Summaries"];
  class 40de7485b25294 requirement;
  click 40de7485b25294 "../UserRequirements.md#model-structure-and-summaries";
  ce120a0d16cf2475 -.->|deriveReqT| 40de7485b25294;
  11a8fc49a87327fe["model.rs"];
  class 11a8fc49a87327fe default;
  click 11a8fc49a87327fe "../../core/src/reports.rs";
  11a8fc49a87327fe -->|satisfies| ce120a0d16cf2475;
  90e16b61e174ace5["Model Summary Fine Grained Filtering"];
  click 90e16b61e174ace5 "Requirements.md#model-summary-fine-grained-filtering";
  class 90e16b61e174ace5 requirement;
  11a8fc49a87327fe -->|satisfies| 90e16b61e174ace5;
  2f21d2133dbfd205 -->|satisfies| 90e16b61e174ace5;
  90e16b61e174ace5 -.->|deriveReqT| ce120a0d16cf2475;
  ecd5cbbaddffb824 -.->|verifies| 90e16b61e174ace5;
  ae7920e4b52a5854["Display Name-Regex Option in Help"];
  click ae7920e4b52a5854 "Requirements.md#display-name-regex-option-in-help";
  class ae7920e4b52a5854 requirement;
  2f21d2133dbfd205 -->|satisfies| ae7920e4b52a5854;
  ae7920e4b52a5854 -.->|deriveReqT| 197dc113759da19b;
  ecd5cbbaddffb824 -.->|verifies| ae7920e4b52a5854;
  d376638ec1fbd6e9["Validation Report Generator"];
  click d376638ec1fbd6e9 "Requirements.md#validation-report-generator";
  class d376638ec1fbd6e9 requirement;
  2d3cfde19fc6bb79["UserRequirements.md/Provide Validation Reports"];
  class 2d3cfde19fc6bb79 requirement;
  click 2d3cfde19fc6bb79 "../UserRequirements.md#provide-validation-reports";
  d376638ec1fbd6e9 -.->|deriveReqT| 2d3cfde19fc6bb79;
  36fd2b1e82621caf["model.rs"];
  class 36fd2b1e82621caf default;
  click 36fd2b1e82621caf "../../core/src/model.rs";
  36fd2b1e82621caf -->|satisfies| d376638ec1fbd6e9;
```
---

### Model Summary Report Generator

The system shall implement a summary report generator that  produces comprehensive summaries of model relationships, including key metrics, element counts by type and counts.

#### Details

The summary report must include: TODO write the rest.

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

**Behavior:** When enabled, any element with one or more verification-related relations must be excluded.

---

### 7. Not Satisfied Filter (Boolean)

**Purpose:** Include only requirement elements that are not connected via a `satisfiedBy` or `satisfy` relation.

**Input:** Boolean flag

**Match Target:** `Element.relations`

**Behavior:** when enabled, any element with one or more satisfaction-related relations must be excluded.

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
- `--model-summary`:  Output model registry and summary, also supports json and cypher output.

All filters require --model-summary to be present. They can be combined:
- `--model-summary`:  Output model registry and summary, also supports json and cypher output.
  - By file path: ` --model-summary  --filter-file="src/**/*Reqs.md"`
  - By name: ` --model-summary  --filter-name=".*safety.*"`
  - By section: ` --model-summary  --filter-section="System*"`
  - By type: ` --model-summary  --filter-type="system-requirement"` (exact match)
  - By content: ` --model-summary  --filter-content="MUST"`
  - Not verified: ` --model-summary  --filter-is-not-verified`
  - Not satisfied: ` --model-summary  --filter-is-not-satisfied`

Must support `--json` and `--cypher` flags to output either json formated string or valid Cyper queries that when executed in graph database produce valid grapjh of a system model.

#### Relations
  * derivedFrom: [Model Summary Report Generator](#model-summary-report-generator)
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
  * derivedFrom: [CLI Summary Report Command](#cli-summary-report-command)  
  * verifiedBy: [../Verifications/ReportsTests.md#model-summary-tests](../Verifications/ReportsTests.md#model-summary-tests)  

---

### Validation Report Generator

The system shall implement a validation report generator that compiles and formats validation results from all validators, providing a unified view of model quality with categorized issues, remediation suggestions, and compliance metrics.

#### Relations
  * derivedFrom: [UserRequirements.md/Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

---