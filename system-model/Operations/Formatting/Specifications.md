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
---

### Full Relations Insertion Contract Specification

#### Details
Auto-generated relations are inverse relations created by the parser during model loading but not persisted to files by default. See Relation Types Specification for opposite relation pairs.

When --with-full-relations is active:
- All relations from the model registry are written to the Relations subsection
- Relations are sorted according to the Relation Ordering Normalization requirement

#### Metadata
  * type: specification
---

### Relation Ordering Specification

Rules for sorting relations within elements for deterministic output.

#### Details
**Sort Criteria:**
1. Primary sort: Alphabetically by relation type name (e.g., `derivedFrom` before `satisfiedBy`, `satisfiedBy` before `verifiedBy`)
2. Secondary sort: Alphabetically by target identifier within the same relation type

**Benefits:**
- Deterministic output regardless of parsing order or HashMap iteration order
- Consistent formatting across all specification files
- Easier diff comparison between file versions

#### Metadata
  * type: specification
---
