# Elements

### File Pattern Exclusion for Format

The system shall respect configured excluded filename patterns when performing formatting operations, ensuring that files intentionally excluded from processing do not receive inappropriate formatting suggestions.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Excluded File Relation Validation](../Core/Validation.md#excluded-file-relation-validation)
  * derivedFrom: [Ignoring Unstructured Documents](../Core/Configuration.md#ignoring-unstructured-documents)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
---

### Model Formatting

The system shall provide formatting capabilities to normalize and standardize System models for consistency and readability.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Format Consistency Enforcement](#format-consistency-enforcement)
  * derive: [Formatting Output](#formatting-output)
  * derive: [Full Relations Insertion](#full-relations-insertion)
  * derive: [Replace Absolute Links with Relative Links](#replace-absolute-links-with-relative-links)
  * derivedFrom: [Formatting Model Documents](../../UserStories.md#formatting-model-documents)
---

### Format Consistency Enforcement

The system shall provide formatting capability to ensure consistent formatting in requirements documents.

#### Details
- The system shall detect and fix formatting inconsistencies following clearly defined specifications
- The system shall maintain consistent formatting across all requirements documents

#### Metadata
  * type: requirement

#### Relations
  * derive: [Document Structure Normalization](#document-structure-normalization)
  * derive: [Element Ordering Normalization](#element-ordering-normalization)
  * derive: [Relation Ordering Normalization](#relation-ordering-normalization)
  * derivedFrom: [Model Formatting](#model-formatting)
  * refinedBy: [Format Consistency Specification](Specifications.md#format-consistency-specification)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * verifiedBy: [Format Command Requirements Verification](Verifications/FormattingVerifications.md#format-command-requirements-verification)
---

### Document Structure Normalization

When generating formatted output, the system shall ensure all documents follow a consistent hierarchical structure.

#### Details
When generating formatted output, the system shall:
- Always output `# Elements` as the page header (all specification files must have this header)
- Add a default section header `## Elements` when elements exist without an explicit section header
- Preserve existing section headers when present (starting with `## `)
- Correctly distinguish between level 1 headers (`# `) and level 2 or deeper headers (`##`, `###`)

**Default Header Names:**
- Page header: Always `# Elements` (required for all specification files)
- Section header: "Elements" (the default section name used by parser)

**Normalization Rules:**
1. If document has `# Elements` then `###` (no `##`): Add section header only
2. If document has `# Elements` and `##`: No header additions needed

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * refinedBy: [Document Structure Specification](Specifications.md#document-structure-specification)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Format Command Requirements Verification](Verifications/FormattingVerifications.md#format-command-requirements-verification)
---

### Element Ordering Normalization

When formatting or persisting specification files, the system shall reorder elements following the Element Ordering Behavior.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * refinedBy: [Element Ordering Behavior](Behaviors.md#element-ordering-behavior)
  * verifiedBy: [Element Ordering Verification](Verifications/FormattingVerifications.md#element-ordering-verification)
---

### Format Duplicate Removal

The system shall remove duplicate entries within the same subsection during format fix operations.

#### Details
A behavior defines the detailed deduplication rules.

This operation only removes within-section duplicates (entries repeated in the same Relations or Attachments subsection). Cross-section duplicates are semantic errors that require user resolution.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * refinedBy: [Format Duplicate Removal Behavior](Behaviors.md#format-duplicate-removal-behavior)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * verifiedBy: [Format Duplicate Removal Test](Verifications/FormattingVerifications.md#format-duplicate-removal-test)
---

### Relation Ordering Normalization

When formatting or persisting specification files, the system shall sort relations within each element for deterministic and consistent output.

#### Details
Relations are sorted using the following criteria:
1. Primary sort: Alphabetically by relation type name (e.g., `derivedFrom` before `satisfiedBy`, `trace` before `verifiedBy`)
2. Secondary sort: Alphabetically by target identifier within the same relation type

This ensures:
- Deterministic output regardless of parsing order or HashMap iteration order
- Consistent diffs when comparing formatted files
- Predictable relation ordering for review and verification

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * refinedBy: [Relation Ordering Specification](Specifications.md#relation-ordering-specification)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Relation Ordering Verification](Verifications/FormattingVerifications.md#relation-ordering-verification)
---

### Formatting Output

The system shall display formatting changes suggestion in similar manner as git diffs.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Git-Style Diff Output for Format](#git-style-diff-output-for-format)
  * derivedFrom: [Model Formatting](#model-formatting)
---

### Git-Style Diff Output for Format

The system shall display formatting change suggestions in a git-style diff format, color-coded when possible, to clearly show what modifications will be or have been made to the documents.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Formatting Output](#formatting-output)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * verifiedBy: [Format Command Requirements Verification](Verifications/FormattingVerifications.md#format-command-requirements-verification)
---

### Full Relations Insertion

When the --with-full-relations flag is provided, the system shall insert all registered relations into elements, including both user-created and auto-generated relations.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [RelationTypes.md](../Core/DesignDocuments/RelationTypes.md)

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * refinedBy: [Full Relations Insertion Refinement Specification](Specifications.md#full-relations-insertion-refinement-specification)
  * verifiedBy: [Full Relations Insertion Verification](Verifications/FormattingVerifications.md#full-relations-insertion-verification)
---

### Replace Absolute Links with Relative Links

The system shall replace absolute links with relative links, where applicable and contextually appropriate, to conform to repository standards and enhance portability.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * verifiedBy: [Format Command Requirements Verification](Verifications/FormattingVerifications.md#format-command-requirements-verification)
---
