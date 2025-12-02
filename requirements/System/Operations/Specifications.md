# Elements

### Document Structure Specification

Rules for normalizing document hierarchical structure during formatting.

#### Details
**Page Header:**
- Always output `# Elements` as the page header (all specification files must have this header)

**Section Header:**
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
  * type: specification

#### Relations
  * satisfy: [Document Structure Normalization](Formatting.md#document-structure-normalization)
---

### Format Consistency Specification

Rules for detecting and fixing formatting inconsistencies in requirements documents.

#### Details
**Excess Whitespace:**
- Detect and fix excess whitespace after element headers, subsection headers, and relation identifiers
- Maintain consistent formatting across all requirements documents

**Inconsistent Newlines:**
- Detect and fix excess or missing newlines before element headers and subsection headers
- Normalize to exactly two newlines before subsections (e.g., "#### Details")

**Missing Separators:**
- Detect consecutive element sections that lack a separator line (---) between them
- Insert separator to maintain consistent visual separation
- Normalize consecutive separators to single separators

**Reserved Subsections:**
- Identify and fix inconsistent indentation and bullet types in relation lists
- Ensure consistent indentation in relation lists (2-space format)
- Normalize relation entries to proper 2-space indentation format

**Output Formatting:**
- Display changes with sequential line numbering that reflects final file positions
- Provide context lines with proper line number continuity

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Format Consistency Enforcement](Formatting.md#format-consistency-enforcement)
---

### Relation Operations Specification

Technical specification for relation link and unlink operations.

#### Details
**Source Resolution:**
- Source parameter accepts either an existing internal file path OR an element name
- Resolution order: first check if source exists as internal file path, if not search for element by name in registry
- Source must resolve to an existing element or file; report error if not found

**Target Resolution:**
- Target parameter must always be an existing element name
- Target must exist in the element registry; report error if not found

**Link Operation:**
- Create Relations subsection in source element if doesn't exist
- Add relation entry with format `* <relation-type>: [target-name](target-path)`
- Validate relation type against supported types
- Validate element type compatibility for the relation
- Skip if relation already exists (idempotent)

**Unlink Operation:**
- Remove relation entry from source element's Relations subsection
- Remove Relations subsection if no relations remain
- Report error if relation doesn't exist

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Relation Management Operations](../Core/ModelManagement.md#relation-management-operations)
---

### Relation Ordering Specification

Rules for sorting relations within elements for deterministic output.

#### Details
**Sort Criteria:**
1. Primary sort: Alphabetically by relation type name (e.g., `derivedFrom` before `satisfiedBy`, `trace` before `verifiedBy`)
2. Secondary sort: Alphabetically by target identifier within the same relation type

**Benefits:**
- Deterministic output regardless of parsing order or HashMap iteration order
- Consistent formatting across all specification files
- Easier diff comparison between file versions

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Relation Ordering Normalization](Formatting.md#relation-ordering-normalization)
---

### Relation Validation Specification

Rules for validating and normalizing relation targets during element creation and manipulation.

#### Details
**Target Format Support:**
- Relative paths from the target file location (e.g., `../UserReqs.md#requirement`)
- Paths relative to git repository root (e.g., `specifications/UserReqs.md#requirement`)
- Same-file references (e.g., `#other-requirement`)

**Normalization Rules:**
- All relation targets must be normalized to git repository root relative format before insertion
- All relation targets must reference existing elements in the model
- External links (http://, https://, etc.) are allowed and not validated

**Validation Behavior:**
- Parse relation targets from the markdown
- Normalize relation targets to be relative to the git repository root
- Validate that each relation target element exists in the model
- Reject the operation if any relation target does not exist
- Provide clear error messages indicating which relation target was not found

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Create Element Operation](ElementManipulation.md#create-element-operation)
---
