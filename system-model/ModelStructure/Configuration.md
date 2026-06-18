# Elements

### Coexistence of Structured and Unstructured Documents

The system shall allow structured markdown and unstructured. (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same System model.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Contract Specification](Specifications.md#contract-specification)

#### Relations
  * derive: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * specify: [Defining Model Structure](ModelStructureFeature.md#defining-model-structure)
---

### Ignoring Unstructured Documents

The system shall support configurable glob patterns to exclude specific files from requirement processing.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ignoring Unstructured Documents Contract Specification](Specifications.md#ignoring-unstructured-documents-contract-specification)
  * derive: [Ignore Files Integration](#ignore-files-integration)
  * derive: [Requirements Processing](#requirements-processing)
  * derive: [File Pattern Exclusion for Format](../Operations/Formatting/FormattingRequirements.md#file-pattern-exclusion-for-format)
  * derivedFrom: [Coexistence of Structured and Unstructured Documents](#coexistence-of-structured-and-unstructured-documents)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)
---

### Ignore Files Integration

The system shall integrate with Git workflows by reading exclusion patterns from .gitignore and .reqvireignore files.

#### Details
- The system shall read exclusion patterns from repository root .gitignore and .reqvireignore files
- The system shall exclude files matching patterns from being parsed as structured markdown
- The system shall differentiate between .gitignore exclusions (complete) and .reqvireignore exclusions (parsing only)
- The system shall implement ignore file processing following clearly defined specifications

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ignore Files Specification](Specifications.md#ignore-files-specification)
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * satisfiedBy: [config.rs](../../cli/src/config.rs)
  * verifiedBy: [File Exclusion Test](../Verifications/Operations/Validation/ValidationVerifications.md#file-exclusion-test)
---

### Requirements Processing

The system shall parse the files in all folders and subfolders from the root of git repository which are not explicitly excluded using .gitignore and .reqvireignore files.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Requirements Processing Contract Specification](Specifications.md#requirements-processing-contract-specification)
  * derive: [Structured Markdown Files Search and Detection](#structured-markdown-files-search-and-detection)
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Same-File Fragment Relations Test](../Verifications/Operations/Validation/ValidationVerifications.md#same-file-fragment-relations-test)
  * verifiedBy: [Element Content Extraction Test](../Verifications/Processing/ChangeImpact/ChangeImpactVerifications.md#element-content-extraction-test)
---

### Structured Markdown Files Search and Detection

The system shall identify all structured markdown documents available for processing in all directories and sub-directories of the git repository root based on predefined rules.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Structured Markdown Files Search and Detection Contract Specification](Specifications.md#structured-markdown-files-search-and-detection-contract-specification)
  * derivedFrom: [Requirements Processing](#requirements-processing)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * verifiedBy: [Requirements Files Search and Detection Test](../Verifications/Operations/Validation/ValidationVerifications.md#requirements-files-search-and-detection-test)
---

