# Elements

### Ignoring Unstructured Documents

The system shall support configurable glob patterns to exclude specific files from requirement processing.

#### Details
Exclusion patterns are defined in `.reqvireignore` file at the repository root, using the same format as `.gitignore`:

```.reqvireignore
# Example patterns to exclude from structured documents processing
**/Logical*.md
**/Physical*.md
**/draft-*.md
examples/**
```

The `.reqvireignore` file provides Reqvire-specific exclusions for files that should remain in version control but not be processed as requirements.

#### Relations
  * derive: [Ignore Files Integration](#ignore-files-integration)
  * derive: [Requirements Processing](#requirements-processing)
  * derive: [Reserved Repository Files Exclusion](#reserved-repository-files-exclusion)
  * derive: [File Pattern Exclusion for Format](../Operations/Formatting.md#file-pattern-exclusion-for-format)
  * derivedFrom: [Coexistence of Structured and Unstructured Documents](ModelManagement.md#coexistence-of-structured-and-unstructured-documents)
  * satisfiedBy: [config.rs](../../../cli/src/config.rs)
---

### Ignore Files Integration

The system shall integrate with Git workflows by reading exclusion patterns from .gitignore and .reqvireignore files.

#### Details
- The system shall read exclusion patterns from repository root .gitignore and .reqvireignore files
- The system shall exclude files matching patterns from being parsed as structured markdown
- The system shall differentiate between .gitignore exclusions (complete) and .reqvireignore exclusions (parsing only)
- The system shall implement ignore file processing following clearly defined specifications

#### Attachments
  * [Ignore Files Specification](Specifications.md#ignore-files-specification)

#### Relations
  * derive: [Target Location Validation and Auto-Creation](../Operations/ElementManipulation.md#target-location-validation-and-auto-creation)
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * satisfiedBy: [config.rs](../../../cli/src/config.rs)
  * satisfiedBy: [Ignore Files Specification](Specifications.md#ignore-files-specification)
  * verifiedBy: [File Exclusion Test](Verifications/ValidationVerifications.md#file-exclusion-test)
---

### Requirements Processing

The system shall parse the files in all folders and subfolders from the root of git repository which are not explicitly excluded using .gitignore and .reqvireignore files.

#### Details
File exclusion is handled through:
- .gitignore patterns (files not in version control)
- .reqvireignore patterns (files in version control but excluded from requirements processing)
- Reserved repository files (README.md, LICENSE.md, etc.)

#### Relations
  * derive: [Structured Markdown Files Search and Detection](#structured-markdown-files-search-and-detection)
  * derive: [GraphRegistry as Primary Registry](Validation.md#graphregistry-as-primary-registry)
  * derive: [Two-Pass Validation Strategy](Validation.md#two-pass-validation-strategy)
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Same-File Fragment Relations Test](Verifications/ValidationVerifications.md#same-file-fragment-relations-test)
  * verifiedBy: [Element Content Extraction Test](../Processing/Verifications/ChangeImpactVerifications.md#element-content-extraction-test)
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
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Requirements Files Search and Detection Test](Verifications/ValidationVerifications.md#requirements-files-search-and-detection-test)
---

### Reserved Repository Files Exclusion

The system shall automatically exclude certain common repository documentation files from structured markdown processing.

#### Details
The following reserved filenames are always excluded from structured markdown processing, as they are typically used for general repository documentation purposes rather than system requirements:

- `README.md`
- `CHANGELOG.md`, `CHANGES.md`
- `CONTRIBUTING.md`
- `LICENSE.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- `AUTHORS.md`
- `ROADMAP.md`

**Rules:**
- Reserved filenames shall be automatically excluded from structured markdown parsing across the entire repository
- Reserved files can still be referenced in file relations to elements (they are excluded from parsing but can be linked to)
- The exclusion of reserved filenames shall be combined with .gitignore and .reqvireignore patterns
- Files matching reserved filenames shall be excluded from parsing regardless of .gitignore or .reqvireignore configuration

**Rationale:**
- These files serve general repository documentation purposes and should not be processed as structured requirements
- Standard repository files like README.md or LICENSE.md contain free-form documentation rather than structured requirements
- Automatically excluding these files prevents confusion and reduces the need for explicit .reqvireignore patterns

#### Relations
  * derivedFrom: [Ignoring Unstructured Documents](#ignoring-unstructured-documents)
  * satisfiedBy: [config.rs](../../../cli/src/config.rs)
  * verifiedBy: [File Exclusion Test](Verifications/ValidationVerifications.md#file-exclusion-test)
---
