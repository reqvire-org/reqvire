# Elements

### File Pattern Exclusion for Format

The system shall respect configured excluded filename patterns when performing formatting operations, ensuring that files intentionally excluded from processing do not receive inappropriate formatting suggestions.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Excluded File Relation Validation](../Validation/ValidationRequirements.md#excluded-file-relation-validation)
  * derivedFrom: [Ignoring Unstructured Documents](../../ModelStructure/Configuration.md#ignoring-unstructured-documents)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
---

### Format Consistency Enforcement

The system shall provide formatting capability to ensure consistent formatting in requirements documents.

#### Details
- The system shall detect and fix formatting inconsistencies following clearly defined specifications
- The system shall maintain consistent formatting across all requirements documents

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Format Consistency Specification](Specifications.md#format-consistency-specification)
  * derive: [Document Structure Normalization](#document-structure-normalization)
  * derive: [Element Ordering Normalization](#element-ordering-normalization)
  * derive: [Relation Ordering Normalization](#relation-ordering-normalization)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * specify: [Formatting Model Documents](../BehaviorValidationOperationsFeature.md#formatting-model-documents)
  * verifiedBy: [Format Command Requirements Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#format-command-requirements-verification)
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
  * definedBy: [Document Structure Specification](Specifications.md#document-structure-specification)
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Format Command Requirements Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#format-command-requirements-verification)
---

### Element Ordering Normalization

When formatting or persisting specification files, the system shall reorder elements following the Element Ordering Behavior.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Element Ordering Behavior](Behaviors.md#element-ordering-behavior)
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * verifiedBy: [Element Ordering Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#element-ordering-verification)
---

### Format Duplicate Removal

The system shall remove duplicate entries within the same subsection during format fix operations.

#### Details
A behavior defines the detailed deduplication rules.

This operation only removes within-section duplicates (entries repeated in the same Relations or Reused Contract Context subsection). Cross-section duplicates are semantic errors that require user resolution.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Format Duplicate Removal Behavior](Behaviors.md#format-duplicate-removal-behavior)
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * verifiedBy: [Format Duplicate Removal Test](../../Verifications/Operations/Formatting/FormattingVerifications.md#format-duplicate-removal-test)
---

### Relation Ordering Normalization

When formatting or persisting specification files, the system shall sort relations within each element for deterministic and consistent output.

#### Details
Relations are sorted using the following criteria:
1. Primary sort: Alphabetically by relation type name (e.g., `derivedFrom` before `satisfiedBy`, `satisfiedBy` before `verifiedBy`)
2. Secondary sort: Alphabetically by target identifier within the same relation type

This ensures:
- Deterministic output regardless of parsing order or HashMap iteration order
- Consistent diffs when comparing formatted files
- Predictable relation ordering for review and verification

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Relation Ordering Specification](Specifications.md#relation-ordering-specification)
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Relation Ordering Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#relation-ordering-verification)
---

### Formatting Output

The system shall display formatting changes suggestion in similar manner as git diffs.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Git-Style Diff Output for Format](#git-style-diff-output-for-format)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * specify: [Formatting Model Documents](../BehaviorValidationOperationsFeature.md#formatting-model-documents)
---

### Git-Style Diff Output for Format

The system shall display formatting change suggestions in a git-style diff format, color-coded when possible, to clearly show what modifications will be or have been made to the documents.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Formatting Output](#formatting-output)
  * verifiedBy: [Format Command Requirements Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#format-command-requirements-verification)
---

### Full Relations Insertion

When the --with-full-relations flag is provided, the system shall insert all registered relations into elements, including both user-created and auto-generated relations.

#### Details
Implementation details shall follow the associated contract specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Full Relations Insertion Contract Specification](Specifications.md#full-relations-insertion-contract-specification)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * specify: [Formatting Model Documents](../BehaviorValidationOperationsFeature.md#formatting-model-documents)
  * verifiedBy: [Full Relations Insertion Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#full-relations-insertion-verification)
---

### Replace Absolute Links with Relative Links

The system shall replace absolute links with relative links, where applicable and contextually appropriate, to conform to repository standards and enhance portability.

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
  * specify: [Formatting Model Documents](../BehaviorValidationOperationsFeature.md#formatting-model-documents)
  * verifiedBy: [Format Command Requirements Verification](../../Verifications/Operations/Formatting/FormattingVerifications.md#format-command-requirements-verification)
---
