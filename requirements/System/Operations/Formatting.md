# Elements

### File Pattern Exclusion for Format

The system shall respect configured excluded filename patterns when performing formatting operations, ensuring that files intentionally excluded from processing do not receive inappropriate formatting suggestions.

#### Relations
  * derivedFrom: [Ignoring Unstructured Documents](../Core/Configuration.md#ignoring-unstructured-documents)
  * satisfiedBy: [utils.rs](../../../core/src/utils.rs)
---

### Model Formatting

The system shall provide formatting capabilities to normalize and standardize System models for consistency and readability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
---

### Format Consistency Enforcement

The system shall provide formatting capability to ensure consistent formatting in requirements documents.

#### Details
The system shall implement the following formatting fixes:

**Excess Whitespace:**
- Detect and fix excess whitespace after element headers, subsection headers, and relation identifiers
- Maintain consistent formatting across all requirements documents

**Inconsistent Newlines:**
- Detect and fix excess or missing newlines before element headers and subsection headers
- Normalize to exactly two newlines before subsections (e.g., "#### Details")
- Maintain consistent formatting across all requirements documents

**Missing Separators:**
- Detect consecutive element sections that lack a separator line (---) between them
- Insert the separator to maintain consistent visual separation in the documentation
- Automatically insert separator lines between elements if not already present
- Normalize consecutive separators to single separators

**Reserved Subsections:**
- Identify and fix inconsistent indentation and bullet types in relation lists and other reserved subsections
- Ensure consistent indentation in relation lists (2-space format)
- Normalize relation entries to proper 2-space indentation format
- Standardize to a consistent format across all requirements documents

**Output Formatting:**
- Display changes with sequential line numbering that reflects final file positions
- Provide context lines with proper line number continuity

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * derivedFrom: [Align with Industry Standards](../../UserStories.md#align-with-industry-standards)
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

#### Relations
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
---

### Element Ordering Normalization

When formatting or persisting specification files, the system shall reorder elements following the Element Ordering Behavior.

#### Attachments
  * [Element Ordering Behavior](Refinements.md#element-ordering-behavior)

#### Relations
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
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

#### Relations
  * derivedFrom: [Format Consistency Enforcement](#format-consistency-enforcement)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
---

### Formatting Output

The system shall display formatting changes suggestion in similar manner as git diffs.

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
---

### Git-Style Diff Output for Format

The system shall display formatting change suggestions in a git-style diff format, color-coded when possible, to clearly show what modifications will be or have been made to the documents.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Formatting Output](#formatting-output)
  * satisfiedBy: [format.rs](../../../core/src/format.rs)
  * satisfiedBy: [diff.rs](../../../core/src/diff.rs)
  * verifiedBy: [Format Command Requirements Verification](Verifications/FormattingVerifications.md#format-command-requirements-verification)
---

### Replace Absolute Links with Relative Links

The system shall replace absolute links with relative links, where applicable and contextually appropriate, to conform to repository standards and enhance portability.

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * verifiedBy: [Format Command Requirements Verification](Verifications/FormattingVerifications.md#format-command-requirements-verification)
---

### Full Relations Insertion

When the --with-full-relations flag is provided, the system shall insert all registered relations into elements, including both user-created and auto-generated relations.

#### Details
Auto-generated relations are inverse relations created by the parser during model loading but not persisted to files by default. See Relation Types Specification for opposite relation pairs.

When --with-full-relations is active:
- All relations from the model registry are written to the Relations subsection
- Relations are sorted according to the Relation Ordering Normalization requirement

#### Attachments
  * [Relation Types Specification](../Core/DesignDocuments/RelationTypes.md)

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * verifiedBy: [Full Relations Insertion Verification](Verifications/FormattingVerifications.md#full-relations-insertion-verification)
---
