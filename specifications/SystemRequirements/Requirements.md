# System Requirements

## Linting    
```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef verification fill:#d6f9d6,stroke:#5fd75f,stroke-width:1px;
  classDef externalLink fill:#d0e0ff,stroke:#3080ff,stroke-width:1px;
  classDef default fill:#f5f5f5,stroke:#333333,stroke-width:1px;

  acd59cad64333401["Missing Separators Linting Implementation"];
  click acd59cad64333401 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#missing-separators-linting-implementation";
  class acd59cad64333401 requirement;
  b692557f47cee0f7["UserRequirements.md/Format Consistency Enforcement"];
  class b692557f47cee0f7 requirement;
  click b692557f47cee0f7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#format-consistency-enforcement";
  acd59cad64333401 -.->|deriveReqT| b692557f47cee0f7;
  7b8ca81a67f9ca1c["linting/separators.rs"];
  class 7b8ca81a67f9ca1c default;
  click 7b8ca81a67f9ca1c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/linting/separators.rs";
  7b8ca81a67f9ca1c -->|satisfies| acd59cad64333401;
  c2d5791e32e75081["File Pattern Exclusion for Linting"];
  click c2d5791e32e75081 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  class c2d5791e32e75081 requirement;
  348451e9313c44a3["Configurable Filename Exclusion Patterns"];
  class 348451e9313c44a3 requirement;
  click 348451e9313c44a3 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  c2d5791e32e75081 -->|refines| 348451e9313c44a3;
  cbb6fb87adcff9d["utils.rs"];
  class cbb6fb87adcff9d default;
  click cbb6fb87adcff9d "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/utils.rs";
  cbb6fb87adcff9d -->|satisfies| c2d5791e32e75081;
  51b41cf76fdb8ffc["Excess Whitespace Linting Implementation"];
  click 51b41cf76fdb8ffc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#excess-whitespace-linting-implementation";
  class 51b41cf76fdb8ffc requirement;
  51b41cf76fdb8ffc -.->|deriveReqT| b692557f47cee0f7;
  6e1ba12020c21f6c["linting/whitespace.rs"];
  class 6e1ba12020c21f6c default;
  click 6e1ba12020c21f6c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/linting/whitespace.rs";
  6e1ba12020c21f6c -->|satisfies| 51b41cf76fdb8ffc;
  47d58145636b9c4f["Reserved Subsections Linting Implementation"];
  click 47d58145636b9c4f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#reserved-subsections-linting-implementation";
  class 47d58145636b9c4f requirement;
  47d58145636b9c4f -.->|deriveReqT| b692557f47cee0f7;
  a7b10289a2855dbf["linting/reserved_subsections.rs"];
  class a7b10289a2855dbf default;
  click a7b10289a2855dbf "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/linting/indentation.rs";
  a7b10289a2855dbf -->|satisfies| 47d58145636b9c4f;
  a3c82f095e6506f9["Incosistent Newlines Linting Implementation"];
  click a3c82f095e6506f9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#incosistent-newlines-linting-implementation";
  class a3c82f095e6506f9 requirement;
  a3c82f095e6506f9 -.->|deriveReqT| b692557f47cee0f7;
  9b6c88b7652bc5f8["linting/newlines.rs"];
  class 9b6c88b7652bc5f8 default;
  click 9b6c88b7652bc5f8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/linting/newlines.rs";
  9b6c88b7652bc5f8 -->|satisfies| a3c82f095e6506f9;
  b8ef485644f8b634["Git-Style Diff Output for Linting"];
  click b8ef485644f8b634 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#git-style-diff-output-for-linting";
  class b8ef485644f8b634 requirement;
  fe8919c53f8115d7["UserRequirements.md/Linting Command Output"];
  class fe8919c53f8115d7 requirement;
  click fe8919c53f8115d7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#linting-command-output";
  b8ef485644f8b634 -.->|deriveReqT| fe8919c53f8115d7;
  332f748fff7d7070["linting/mod.rs"];
  class 332f748fff7d7070 default;
  click 332f748fff7d7070 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/linting/mod.rs";
  332f748fff7d7070 -->|satisfies| b8ef485644f8b634;
  32d2c19b923c0ac7["CLI Lint Flag"];
  click 32d2c19b923c0ac7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-lint-flag";
  class 32d2c19b923c0ac7 requirement;
  51a11693af2e41fb["UserRequirements.md/Linting Command Behavior"];
  class 51a11693af2e41fb requirement;
  click 51a11693af2e41fb "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#linting-command";
  32d2c19b923c0ac7 -.->|deriveReqT| 51a11693af2e41fb;
  b2b6ff676ff5d2cc["cli.rs"];
  class b2b6ff676ff5d2cc default;
  click b2b6ff676ff5d2cc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/cli/src/cli.rs";
  b2b6ff676ff5d2cc -->|satisfies| 32d2c19b923c0ac7;
  44453b835c3fc32e["Parallel Linting Processing"];
  click 44453b835c3fc32e "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#parallel-linting-processing";
  class 44453b835c3fc32e requirement;
  a479ae0b8d8c4fce["UserRequirements.md/Model Linting"];
  class a479ae0b8d8c4fce requirement;
  click a479ae0b8d8c4fce "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#model-linting";
  44453b835c3fc32e -.->|deriveReqT| a479ae0b8d8c4fce;
  332f748fff7d7070 -->|satisfies| 44453b835c3fc32e;
  bd9d6fae737d418f["Dry Run Mode"];
  click bd9d6fae737d418f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#dry-run-mode";
  class bd9d6fae737d418f requirement;
  bd9d6fae737d418f -.->|deriveReqT| 32d2c19b923c0ac7;
  b2b6ff676ff5d2cc -->|satisfies| bd9d6fae737d418f;
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

  854c934a3e442261["External Folders Support"];
  click 854c934a3e442261 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#external-folders-support";
  class 854c934a3e442261 requirement;
  fe433967471bc04f["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class fe433967471bc04f requirement;
  click fe433967471bc04f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  854c934a3e442261 -.->|deriveReqT| fe433967471bc04f;
  638f8a9a3383f12d["config.rs"];
  class 638f8a9a3383f12d default;
  click 638f8a9a3383f12d "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/cli/src/config.rs";
  638f8a9a3383f12d -->|satisfies| 854c934a3e442261;
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

  99e7b8dcc342d683["HTML Export"];
  click 99e7b8dcc342d683 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#html-export";
  class 99e7b8dcc342d683 requirement;
  6a4b7e96941effda["../UserRequirements.md/Export HTML specifications"];
  class 6a4b7e96941effda requirement;
  click 6a4b7e96941effda "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#export-html-specifications";
  99e7b8dcc342d683 -.->|deriveReqT| 6a4b7e96941effda;
  43008ed7c339f087["html_export.rs"];
  class 43008ed7c339f087 default;
  click 43008ed7c339f087 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/html_export.rs";
  43008ed7c339f087 -->|satisfies| 99e7b8dcc342d683;
  71debc495e095645["Interactive Mermaid Diagram Node Behavior"];
  click 71debc495e095645 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#interactive-mermaid-diagram-node-behavior";
  class 71debc495e095645 requirement;
  27cb448230f8b6b0["UserRequirements.md/Interactive Mermaid Diagrams"];
  class 27cb448230f8b6b0 requirement;
  click 27cb448230f8b6b0 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#interactive-mermaid-diagrams";
  71debc495e095645 -.->|deriveReqT| 27cb448230f8b6b0;
  3e6a4a60e7534058["html.rs"];
  class 3e6a4a60e7534058 default;
  click 3e6a4a60e7534058 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/diagrams.rs";
  3e6a4a60e7534058 -->|satisfies| 71debc495e095645;
  59cc05826edfa20c["HTML Navigation Enhancement"];
  click 59cc05826edfa20c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#html-navigation-enhancement";
  class 59cc05826edfa20c requirement;
  9f579ceba1c84b17["UserRequirements.md/Documentation Index HTML Integration"];
  class 9f579ceba1c84b17 requirement;
  click 9f579ceba1c84b17 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#documentation-index-html-integration";
  59cc05826edfa20c -.->|deriveReqT| 9f579ceba1c84b17;
  8c5748ba9b82ee1e["html.rs"];
  class 8c5748ba9b82ee1e default;
  click 8c5748ba9b82ee1e "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/html.rs";
  8c5748ba9b82ee1e -->|satisfies| 59cc05826edfa20c;
  43008ed7c339f087 -->|satisfies| 59cc05826edfa20c;
  1961923a36e51056["JSON Output Format"];
  click 1961923a36e51056 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#json-output-format";
  class 1961923a36e51056 requirement;
  ad1bea65795cf377["UserRequirements.md/Enhanced Validation Error Reporting"];
  class ad1bea65795cf377 requirement;
  click ad1bea65795cf377 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  1961923a36e51056 -.->|deriveReqT| ad1bea65795cf377;
  b2b6ff676ff5d2cc["cli.rs"];
  class b2b6ff676ff5d2cc default;
  click b2b6ff676ff5d2cc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/cli/src/cli.rs";
  b2b6ff676ff5d2cc -->|satisfies| 1961923a36e51056;
  15fa7a7cfb9bc2a3["Unstructured Documents"];
  click 15fa7a7cfb9bc2a3 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#unstructured-documents";
  class 15fa7a7cfb9bc2a3 requirement;
  ab0a6601a5fb42c8["ManagingMbseModelsRequirements.md#Coexistence of Structured and Unstructured Documents"];
  class ab0a6601a5fb42c8 requirement;
  click ab0a6601a5fb42c8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#coexistence-of-structured-and-unstructured-documents";
  15fa7a7cfb9bc2a3 -.->|deriveReqT| ab0a6601a5fb42c8;
  62ab248e5b824192["Index Generation"];
  click 62ab248e5b824192 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#index-generation";
  class 62ab248e5b824192 requirement;
  c7c34d508e89ee3c["UserRequirements.md/Generate Documentation Index"];
  class c7c34d508e89ee3c requirement;
  click c7c34d508e89ee3c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#generate-documentation-index";
  62ab248e5b824192 -.->|deriveReqT| c7c34d508e89ee3c;
  c2ec7779a185b1df["index_generator.rs"];
  class c2ec7779a185b1df default;
  click c2ec7779a185b1df "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/index_generator.rs";
  c2ec7779a185b1df -->|satisfies| 62ab248e5b824192;
  ebf05dbb02b07598["File Content Caching for Performance"];
  click ebf05dbb02b07598 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#file-content-caching-for-performance";
  class ebf05dbb02b07598 requirement;
  aac74edfeb367501["../ManagingMbseModelsRequirements.md#Efficient Processing"];
  class aac74edfeb367501 requirement;
  click aac74edfeb367501 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  ebf05dbb02b07598 -.->|deriveReqT| aac74edfeb367501;
  9efd257c4e6a45f9["model.rs"];
  class 9efd257c4e6a45f9 default;
  click 9efd257c4e6a45f9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/model.rs";
  9efd257c4e6a45f9 -->|satisfies| ebf05dbb02b07598;
  2c6c9d35cca20822["LLM Context Command"];
  click 2c6c9d35cca20822 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#llm-context-command";
  class 2c6c9d35cca20822 requirement;
  b1d0919d0418c0f1["UserRequirements.md/AI Agent Context"];
  class b1d0919d0418c0f1 requirement;
  click b1d0919d0418c0f1 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#ai-agent-context";
  2c6c9d35cca20822 -.->|deriveReqT| b1d0919d0418c0f1;
  fa97bbe2cc21e304["main.rs"];
  class fa97bbe2cc21e304 default;
  click fa97bbe2cc21e304 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/cli/src/main.rs";
  fa97bbe2cc21e304 -->|satisfies| 2c6c9d35cca20822;
  8cd1d72c54544cd9["Detailed Error Handling and Logging"];
  click 8cd1d72c54544cd9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#detailed-error-handling-and-logging";
  class 8cd1d72c54544cd9 requirement;
  8cd1d72c54544cd9 -.->|deriveReqT| ad1bea65795cf377;
  222ec862cb08c29a["src/error.rs"];
  class 222ec862cb08c29a default;
  click 222ec862cb08c29a "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/error.rs";
  222ec862cb08c29a -->|satisfies| 8cd1d72c54544cd9;
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

  fb2eb9df7ab72606["Requirements Files Search and Detection"];
  click fb2eb9df7ab72606 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#requirements-files-search-and-detection";
  class fb2eb9df7ab72606 requirement;
  aac74edfeb367501["ManagingMbseModelsRequirements.md/Efficient Processing"];
  class aac74edfeb367501 requirement;
  click aac74edfeb367501 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#efficient-processing";
  fb2eb9df7ab72606 -.->|deriveReqT| aac74edfeb367501;
  9efd257c4e6a45f9["model.rs"];
  class 9efd257c4e6a45f9 default;
  click 9efd257c4e6a45f9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/model.rs";
  9efd257c4e6a45f9 -->|satisfies| fb2eb9df7ab72606;
  6aeba4bf990bc9e4["Requirements Processing"];
  click 6aeba4bf990bc9e4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#requirements-processing";
  class 6aeba4bf990bc9e4 requirement;
  fe433967471bc04f["ManagingMbseModelsRequirements.md/Support for Distributed Requirements"];
  class fe433967471bc04f requirement;
  click fe433967471bc04f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#support-for-distributed-requirements";
  6aeba4bf990bc9e4 -.->|deriveReqT| fe433967471bc04f;
  9efd257c4e6a45f9 -->|satisfies| 6aeba4bf990bc9e4;
  f92be9685a777557["parser.rs"];
  class f92be9685a777557 default;
  click f92be9685a777557 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/parser.rs";
  f92be9685a777557 -->|satisfies| 6aeba4bf990bc9e4;
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

  4b9934c3f9197112["Automated Diagram Generation on PR Merge"];
  click 4b9934c3f9197112 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#automated-diagram-generation-on-pr-merge";
  class 4b9934c3f9197112 requirement;
  64be2a98bd80a653["UserRequirements.md/Automate Diagram Generation"];
  class 64be2a98bd80a653 requirement;
  click 64be2a98bd80a653 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#automate-diagram-generation";
  4b9934c3f9197112 -.->|deriveReqT| 64be2a98bd80a653;
  9f72b91320c287ce["UserRequirements.md/Automate Pull Request Validations"];
  class 9f72b91320c287ce requirement;
  click 9f72b91320c287ce "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#automate-pull-request-validations";
  4b9934c3f9197112 -.->|deriveReqT| 9f72b91320c287ce;
  aae2507df9855ad7["generate_diagrams.yml"];
  class aae2507df9855ad7 default;
  click aae2507df9855ad7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/.github/workflows/generate_diagrams.yml";
  aae2507df9855ad7 -->|satisfies| 4b9934c3f9197112;
  2e9043121ac50be5["Relationship Type Filter Implementation"];
  click 2e9043121ac50be5 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#relationship-type-filter-implementation";
  class 2e9043121ac50be5 requirement;
  aaa09eb94d160979["UserRequirements.md/Filter Relationships by Type"];
  class aaa09eb94d160979 requirement;
  click aaa09eb94d160979 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#filter-relationships-by-type";
  2e9043121ac50be5 -.->|deriveReqT| aaa09eb94d160979;
  a5a2bf1b85e49779["Visual Differential Rendering"];
  click a5a2bf1b85e49779 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#visual-differential-rendering";
  class a5a2bf1b85e49779 requirement;
  d3a1b6b68298a744["UserRequirements.md/Highlight Changes in Diagrams"];
  class d3a1b6b68298a744 requirement;
  click d3a1b6b68298a744 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#highlight-changes-in-diagrams";
  a5a2bf1b85e49779 -.->|deriveReqT| d3a1b6b68298a744;
  912d4893486223cd["Diagram Storage Path Configuration"];
  click 912d4893486223cd "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#diagram-storage-path-configuration";
  class 912d4893486223cd requirement;
  e5f8f9f127a22da["UserRequirements.md/Store Automated Diagrams in Designated Locations"];
  class e5f8f9f127a22da requirement;
  click e5f8f9f127a22da "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#store-automated-diagrams-in-designated-locations";
  912d4893486223cd -.->|deriveReqT| e5f8f9f127a22da;
  9efd257c4e6a45f9["model.rs"];
  class 9efd257c4e6a45f9 default;
  click 9efd257c4e6a45f9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/model.rs";
  9efd257c4e6a45f9 -->|satisfies| 912d4893486223cd;
  1c32c7c37cb489ec["SysML-Compatible Relationship Rendering"];
  click 1c32c7c37cb489ec "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#sysml-compatible-relationship-rendering";
  class 1c32c7c37cb489ec requirement;
  37611ee8059e0f03["UserRequirements.md/Visualize Model Relationships"];
  class 37611ee8059e0f03 requirement;
  click 37611ee8059e0f03 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#visualize-model-relationships";
  1c32c7c37cb489ec -.->|deriveReqT| 37611ee8059e0f03;
  3e6a4a60e7534058["diagrams.rs"];
  class 3e6a4a60e7534058 default;
  click 3e6a4a60e7534058 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/diagrams.rs";
  3e6a4a60e7534058 -->|satisfies| 1c32c7c37cb489ec;
  348451e9313c44a3["Configurable Filename Exclusion Patterns"];
  click 348451e9313c44a3 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#configurable-filename-exclusion-patterns";
  class 348451e9313c44a3 requirement;
  afba6897082c7c6a["ManagingMbseModelsRequirements.md/Project Configuration with YAML"];
  class afba6897082c7c6a requirement;
  click afba6897082c7c6a "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/ManagingMbseModelsRequirements.md#project-configuration-with-yaml";
  348451e9313c44a3 -.->|deriveReqT| afba6897082c7c6a;
  638f8a9a3383f12d["config.rs"];
  class 638f8a9a3383f12d default;
  click 638f8a9a3383f12d "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/cli/src/config.rs";
  638f8a9a3383f12d -->|satisfies| 348451e9313c44a3;
  6ece3acb30e93c6a["Mermaid Diagram Format Conversion"];
  click 6ece3acb30e93c6a "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#mermaid-diagram-format-conversion";
  class 6ece3acb30e93c6a requirement;
  3e72f83cabd0bad8["UserRequirements.md/Export Diagrams in Standard Formats"];
  class 3e72f83cabd0bad8 requirement;
  click 3e72f83cabd0bad8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#export-diagrams-in-standard-formats";
  6ece3acb30e93c6a -.->|deriveReqT| 3e72f83cabd0bad8;
  3e6a4a60e7534058 -->|satisfies| 6ece3acb30e93c6a;
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
- Build the ReqFlow tool from source
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

  6c01a7a878176f8e["Traceability Matrix Builder Implementation"];
  click 6c01a7a878176f8e "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#traceability-matrix-builder-implementation";
  class 6c01a7a878176f8e requirement;
  25ad41b0b912092b["UserRequirements.md/Traceability Matrix"];
  class 25ad41b0b912092b requirement;
  click 25ad41b0b912092b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#traceability-matrix";
  6c01a7a878176f8e -.->|deriveReqT| 25ad41b0b912092b;
  722018940c193779["matrix_generator.rs"];
  class 722018940c193779 default;
  click 722018940c193779 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/matrix_generator.rs";
  722018940c193779 -->|satisfies| 6c01a7a878176f8e;
  99fe584a54368e2c["CLI Traces Flag"];
  click 99fe584a54368e2c "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-traces-flag";
  class 99fe584a54368e2c requirement;
  99fe584a54368e2c -.->|deriveReqT| 25ad41b0b912092b;
  b2b6ff676ff5d2cc["cli.rs"];
  class b2b6ff676ff5d2cc default;
  click b2b6ff676ff5d2cc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/cli/src/cli.rs";
  b2b6ff676ff5d2cc -->|satisfies| 99fe584a54368e2c;
  c71f1cc579bb4db["CLI Git Commit Hash Flag"];
  click c71f1cc579bb4db "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-git-commit-hash-flag";
  class c71f1cc579bb4db requirement;
  98cbeaac067febd5["CLI Change Impact Report Flag"];
  class 98cbeaac067febd5 requirement;
  click 98cbeaac067febd5 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-change-impact-report-flag";
  c71f1cc579bb4db -.->|deriveReqT| 98cbeaac067febd5;
  b2b6ff676ff5d2cc -->|satisfies| c71f1cc579bb4db;
  6deadc5fd3fd1500["Markdown Matrix Formatter"];
  click 6deadc5fd3fd1500 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#markdown-matrix-formatter";
  class 6deadc5fd3fd1500 requirement;
  6deadc5fd3fd1500 -.->|deriveReqT| 25ad41b0b912092b;
  722018940c193779 -->|satisfies| 6deadc5fd3fd1500;
  8ada7921a5aa8d47["Structural Change Analyzer"];
  class 8ada7921a5aa8d47 requirement;
  click 8ada7921a5aa8d47 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#structural-change-analyzer";
  98cbeaac067febd5 -.->|deriveReqT| 8ada7921a5aa8d47;
  b2b6ff676ff5d2cc -->|satisfies| 98cbeaac067febd5;
  e42698fdbbf344aa["UserRequirements.md/Tracing Structural Changes"];
  class e42698fdbbf344aa requirement;
  click e42698fdbbf344aa "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#tracing-structural-changes";
  8ada7921a5aa8d47 -.->|deriveReqT| e42698fdbbf344aa;
  a2b02da80c17b86f["model.rs"];
  class a2b02da80c17b86f default;
  click a2b02da80c17b86f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/change_impact.rs";
  a2b02da80c17b86f -->|satisfies| 8ada7921a5aa8d47;
  3900a99fdbd871ae["CLI Traces SVG Flag"];
  click 3900a99fdbd871ae "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-traces-svg-flag";
  class 3900a99fdbd871ae requirement;
  3900a99fdbd871ae -.->|deriveReqT| 99fe584a54368e2c;
  b2b6ff676ff5d2cc -->|satisfies| 3900a99fdbd871ae;
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

  7390fcf6e2c328f4["Relation Type Validation"];
  click 7390fcf6e2c328f4 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#relation-type-validation";
  class 7390fcf6e2c328f4 requirement;
  ad1bea65795cf377["UserRequirements.md/Enhanced Validation Error Reporting"];
  class ad1bea65795cf377 requirement;
  click ad1bea65795cf377 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#enhanced-validation-error-reporting";
  7390fcf6e2c328f4 -.->|deriveReqT| ad1bea65795cf377;
  79753a9e8064fc6f["src/relation.rs"];
  class 79753a9e8064fc6f default;
  click 79753a9e8064fc6f "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/relation.rs";
  79753a9e8064fc6f -->|satisfies| 7390fcf6e2c328f4;
  9b1583400f6bf5ee["Relation Element Type Validator"];
  click 9b1583400f6bf5ee "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#relation-element-type-validator";
  class 9b1583400f6bf5ee requirement;
  d5b992a94f4d6669["../UserRequirements.md#Validate Relation Types"];
  class d5b992a94f4d6669 requirement;
  click d5b992a94f4d6669 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-relation-types";
  9b1583400f6bf5ee -.->|deriveReqT| d5b992a94f4d6669;
  fc5be549cefb3c18["../SpecificationsRequirements.md#Relation Types And Behaviors"];
  class fc5be549cefb3c18 requirement;
  click fc5be549cefb3c18 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SpecificationsRequirements.md#relation-types-and-behaviors";
  9b1583400f6bf5ee -.->|deriveReqT| fc5be549cefb3c18;
  9efd257c4e6a45f9["model.rs"];
  class 9efd257c4e6a45f9 default;
  click 9efd257c4e6a45f9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/model.rs";
  9efd257c4e6a45f9 -->|satisfies| 9b1583400f6bf5ee;
  f92be9685a777557["parser.rs"];
  class f92be9685a777557 default;
  click f92be9685a777557 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/parser.rs";
  f92be9685a777557 -->|satisfies| 9b1583400f6bf5ee;
  7935aaa03aa8e66b["Internal Consistency Validator"];
  click 7935aaa03aa8e66b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#internal-consistency-validator";
  class 7935aaa03aa8e66b requirement;
  38ec9e189e6980d7["UserRequirements.md/Validate Internal Consistency"];
  class 38ec9e189e6980d7 requirement;
  click 38ec9e189e6980d7 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-internal-consistency";
  7935aaa03aa8e66b -.->|deriveReqT| 38ec9e189e6980d7;
  9efd257c4e6a45f9 -->|satisfies| 7935aaa03aa8e66b;
  f92be9685a777557 -->|satisfies| 7935aaa03aa8e66b;
  7b4503ebc3c5f5a9["Cross-Component Dependency Validator"];
  click 7b4503ebc3c5f5a9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cross-component-dependency-validator";
  class 7b4503ebc3c5f5a9 requirement;
  a6c1d4d1f6866aa8["UserRequirements.md/Validate Cross-Component Dependencies"];
  class a6c1d4d1f6866aa8 requirement;
  click a6c1d4d1f6866aa8 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-cross-component-dependencies";
  7b4503ebc3c5f5a9 -.->|deriveReqT| a6c1d4d1f6866aa8;
  9efd257c4e6a45f9 -->|satisfies| 7b4503ebc3c5f5a9;
  f92be9685a777557 -->|satisfies| 7b4503ebc3c5f5a9;
  8e8279e74b2e2559["Excluded File Relation Validation"];
  click 8e8279e74b2e2559 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#excluded-file-relation-validation";
  class 8e8279e74b2e2559 requirement;
  c2d5791e32e75081["File Pattern Exclusion for Linting"];
  class c2d5791e32e75081 requirement;
  click c2d5791e32e75081 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#file-pattern-exclusion-for-linting";
  8e8279e74b2e2559 -->|refines| c2d5791e32e75081;
  f92be9685a777557 -->|satisfies| 8e8279e74b2e2559;
  262d5293f581f05["Markdown Structure Validator"];
  click 262d5293f581f05 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#markdown-structure-validator";
  class 262d5293f581f05 requirement;
  cc431fdb7d8cadde["UserRequirements.md/Validate Markdown Structure"];
  class cc431fdb7d8cadde requirement;
  click cc431fdb7d8cadde "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-markdown-structure";
  262d5293f581f05 -.->|deriveReqT| cc431fdb7d8cadde;
  9efd257c4e6a45f9 -->|satisfies| 262d5293f581f05;
  f92be9685a777557 -->|satisfies| 262d5293f581f05;
  9b1401003376cb94["Filesystem Structure Validator"];
  click 9b1401003376cb94 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#filesystem-structure-validator";
  class 9b1401003376cb94 requirement;
  c95011b2518dcd9d["UserRequirements.md/Validate Filesystem Structure"];
  class c95011b2518dcd9d requirement;
  click c95011b2518dcd9d "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#validate-filesystem-structure";
  9b1401003376cb94 -.->|deriveReqT| c95011b2518dcd9d;
  9efd257c4e6a45f9 -->|satisfies| 9b1401003376cb94;
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

  8d7c27b4431d39d0["Validation Report Generator"];
  click 8d7c27b4431d39d0 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#validation-report-generator";
  class 8d7c27b4431d39d0 requirement;
  a4b1fa740dda1d5["UserRequirements.md/Provide Validation Reports"];
  class a4b1fa740dda1d5 requirement;
  click a4b1fa740dda1d5 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#provide-validation-reports";
  8d7c27b4431d39d0 -.->|deriveReqT| a4b1fa740dda1d5;
  9efd257c4e6a45f9["model.rs"];
  class 9efd257c4e6a45f9 default;
  click 9efd257c4e6a45f9 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/model.rs";
  9efd257c4e6a45f9 -->|satisfies| 8d7c27b4431d39d0;
  9cd307abe52b0aee["CLI Summary Report Flag"];
  click 9cd307abe52b0aee "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#cli-summary-report-flag";
  class 9cd307abe52b0aee requirement;
  fc02b00d367074d6["Model Summary Report Generator"];
  class fc02b00d367074d6 requirement;
  click fc02b00d367074d6 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/SystemRequirements/Requirements.md#model-summary-report-generator";
  9cd307abe52b0aee -->|refines| fc02b00d367074d6;
  b2b6ff676ff5d2cc["cli.rs"];
  class b2b6ff676ff5d2cc default;
  click b2b6ff676ff5d2cc "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/cli/src/cli.rs";
  b2b6ff676ff5d2cc -->|satisfies| 9cd307abe52b0aee;
  738524091202242b["UserRequirements.md/Model Structure and Summaries"];
  class 738524091202242b requirement;
  click 738524091202242b "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/specifications/UserRequirements.md#model-structure-and-summaries";
  fc02b00d367074d6 -.->|deriveReqT| 738524091202242b;
  637cddc09584518["model.rs"];
  class 637cddc09584518 default;
  click 637cddc09584518 "https://github.com/ilijaljubicic/ReqFlow/blob/73d5ae1d93adea71fdf4cc508be4d1ae09be068f/core/src/reports.rs";
  637cddc09584518 -->|satisfies| fc02b00d367074d6;
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