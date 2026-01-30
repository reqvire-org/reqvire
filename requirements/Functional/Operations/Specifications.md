# Elements

### Create Element Workflow Specification

Detailed workflow for creating new model elements.

#### Details
When creating a new element, the system shall:
- Accept a string containing the full element definition in Markdown format
- Accept target location: file path
- Validate the target location using path validation rules
- Create target file if it does not exist (subject to validation constraints)
- Parse and validate the element definition string
- Verify the element name is globally unique in the model
- Validate and normalize all relations following clearly defined specifications
- Insert the element into the target file following Element Ordering Behavior
- Reject the operation and report validation errors if validation fails
- Provide updates report following Diff Output Format Specification
- The system shall support override mode to replace existing element with same name following rules defined in Create Element Override Behavior

#### Metadata
  * type: specification
---

### Delete Element Workflow Specification

Detailed workflow for deleting existing model elements.

#### Details
When deleting an element, the system shall:
- Check if any child elements would become orphaned (have no remaining parent hierarchical relations after deletion)
- Reject the operation if any child would become orphaned
- Provide clear error message listing orphaned children with resolution guidance
- Allow deletion if children have other parent hierarchical relations
- Remove the element and all its content from the source file
- Identify all relations pointing to the deleted element (incoming relations)
- Remove all relations that reference the deleted element from other elements
- Identify all relations from the deleted element (outgoing relations)
- Remove the complete element section including separators
- Maintain file structure and formatting after deletion
- Provide updates report following Diff Output Format Specification

**Empty File Cleanup:**
- After deleting the element, check if the source file contains any remaining elements
- If no elements remain and all sections are empty (only page content, headers, or whitespace), remove the file from the filesystem
- If the file is removed, report the file deletion in the operation output

**Relation Handling:**
- All `derivedFrom` relations pointing to the deleted element shall be removed
- All `verifiedBy` relations pointing to the deleted element shall be removed
- All `verify` relations pointing to the deleted element shall be removed
- All `satisfiedBy` relations pointing to the deleted element shall be removed
- Relations from the deleted element are automatically removed with the element

#### Metadata
  * type: specification
---

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

### Lint Output Specification

Specification for lint command output format and content structure.

#### Details
**Text Output Structure:**
- Section headers: "Auto-fixable Issues" and "Needs Manual Review" (when applicable)
- For each issue category:
  * Issue type heading (e.g., "Safe Redundant Hierarchical Relations", "Redundant Verify Relations")
  * List of affected elements with file paths and identifiers
  * Specific relations flagged as redundant
  * Brief explanation of why the relation is redundant, including which intermediate paths provide alternate routes
- For auto-fixable issues: indicate these can be fixed with `--fix` flag
- For manual review issues: explain why human judgment is required

**JSON Output Structure:**
- Issue categorization (auto_fixable vs. needs_manual_review)
- Issue type classification
- Affected element identifiers
- Specific relation details (type, target)
- Rationale text explaining the redundancy
- Intermediate paths that make the direct relation redundant

#### Metadata
  * type: specification
---

### Merge Element Workflow Specification

Detailed workflow for merging multiple source elements into a target element.

#### Details
When merging elements, the system shall:
- Accept target element name (must exist in the model)
- Accept one or more source element names (must exist in the model)
- Validate type compatibility following clearly defined rules in Merge Type Compatibility Constraint
- Transform and merge content following clearly defined rules in Merge Content Transformation Behavior
- Preserve target element's metadata (discard source metadata)
- Delete source elements after successful merge
- Update all relations pointing to source elements to point to target
- Remove empty source files when no elements remain
- Provide updates report following Diff Output Format Specification

The system shall reject the operation with a clear error message if:
- The target element does not exist
- Any source element does not exist
- Source and target element types are incompatible per Merge Type Compatibility Constraint
- Merged result would have cross-section duplicates per Merge Content Transformation Behavior

#### Metadata
  * type: specification
---

### Move Element Workflow Specification

Detailed workflow for moving existing model elements to different file locations.

#### Details
When moving an element, the system shall:
- Validate the target location using path validation rules
- Create target file if it does not exist (subject to validation constraints)
- Remove the element from the source file
- Insert the element into the target file following Element Ordering Behavior
- Preserve all element content, metadata, and relations
- Update the element's identifier to reflect the new location
- Identify all relations pointing to the moved element (incoming relations)
- Update all relations that reference the moved element with the new identifier
- Maintain file structure and formatting in both source and target files
- Ensure the element name is globally unique in the model
- Provide updates report following Diff Output Format Specification

**Empty Source File Cleanup:**
- After moving the element, check if the source file contains any remaining elements
- If no elements remain (only page content, headers, or whitespace), remove the source file from the filesystem
- If the file is removed, report the file deletion in the operation output

**Relation Update Requirements:**
- All relations (both forward and backward) pointing to the moved element shall be updated to the new identifier
- Relations within the moved element (outgoing relations) shall be preserved unchanged

**Identifier Update:**
- The element's identifier changes from `<old-file>#<element-name>` to `<new-file>#<element-name>`
- All references to the old identifier shall be updated to the new identifier

#### Metadata
  * type: specification
---

### Multi-Branch Convergence Detection Specification

Technical specification for detecting when an element reaches a common ancestor through multiple distinct branch paths.

#### Details
A multi-branch convergence occurs when:
- An element reaches a common ancestor through two or more distinct derivedFrom branch paths
- There is NO direct derivedFrom relation from the element to the ancestor
- Each branch represents a potentially different semantic relationship
- The convergence may be intentional (element truly derives from ancestor through multiple contexts) OR may represent redundant modeling

**Key Distinction from Redundant Hierarchical Relations:**
- **Redundant Hierarchical Relations**: Element has a DIRECT relation to ancestor PLUS alternate paths → auto-fixable (remove direct relation)
- **Multi-Branch Convergence**: Element reaches ancestor through MULTIPLE branches with NO direct relation → needs manual review (determine if branches are semantically distinct)

**Example:**
```
Authorization (root)
  → Management API
    → API Specification
  → Public API
    → API Specification
```
API Specification reaches Authorization through two branches (Management API and Public API). Both branches might be semantically valid (spec derives from auth in context of both APIs), OR one might be a modeling error that should be removed.

Detection shall:
- Use the trace tree building logic to identify elements that reach common ancestors through multiple distinct branch paths
- Exclude cases where a direct relation exists (those are handled by Redundant Hierarchical Relations Detection)
- Report the element, the common ancestor, and all distinct branch paths
- Categorize as **needs manual review** since determining semantic necessity requires human judgment
- Explain that the user must decide whether all branches represent valid semantic relationships or if one is redundant

This enables the model author to review and decide:
- Are both branches semantically necessary? (keep both)
- Is one branch a modeling error? (remove that branch's intermediate relations)
- Should there be a direct relation instead? (restructure the model)

#### Metadata
  * type: specification
---

### Orphaned Children Error Message Specification

The error message for orphaned children prevention shall include:
- Statement that deletion cannot proceed due to orphaned children
- Element name being deleted
- Count of child elements that would be orphaned
- List of child element names that would be orphaned
- Resolution guidance: "Delete the child elements first, or update the child elements to link to a different parent element"

#### Metadata
  * type: specification
---

### Redundant Hierarchical Relations Specification

Technical specification for detecting and auto-removing redundant derivedFrom relations in the requirement hierarchy.

#### Details
**What is Redundant:**

A derivedFrom relation is redundant when:
- An element has a direct derivedFrom relation to an ancestor requirement
- The same element also reaches that ancestor through other derivedFrom relations via intermediate elements
- The hierarchy chain is already established through other paths (single or multiple convergent paths)

**Core Principle**: If an element has a direct relation to an ancestor AND that ancestor is reachable through any other path(s), the direct relation adds no traceability value and can be safely auto-removed.

This applies to:
- **Single-chain redundancy**: Element reaches ancestor through exactly one intermediate path
- **Multi-path/branching redundancy**: Element reaches ancestor through multiple convergent paths

**Detection Logic:**

The system shall use verification trace tree logic for detection:
- Create a virtual/dummy verification element
- Connect the virtual verification to ALL leaf requirements (requirements with no derived children) via virtual verify relations
- Apply the same trace tree building logic used for verification upward traceability
- The trace tree will naturally identify when leaf requirements have derivedFrom relations to both a parent and its ancestor
- Identify which intermediate paths provide the alternate routes to the ancestor

This approach reuses the proven trace tree logic for redundancy detection, ensuring consistency with verify relation redundancy detection.

**Safe Auto-Removal Criteria:**

A redundant hierarchical derivation relation shall be considered safe to auto-remove when ALL of the following conditions are met:
1. **Direct relation exists**: Element A has a direct derivedFrom relation to element C
2. **Alternate path exists**: There exists at least one path from A to C through intermediate elements (single or multiple convergent paths)
3. **Transitive redundancy**: The direct A → C relation is redundant because C is reachable through other derivedFrom relations

**Examples:**

*Single-chain redundancy (auto-removable):*
```
User Requirement A
  → System Requirement B
    → Implementation C

Redundant: A → C (can be safely auto-removed)
Reason: C is reachable via A → B → C
```

*Multi-path/branching redundancy (auto-removable):*
```
Authorization A
  → Public API B → API Specification D
  → Management API C → API Specification D

Redundant: A → D (can be safely auto-removed)
Reason: D is reachable via A → B → D and A → C → D
```

**Auto-Removal Behavior:**

When auto-fix mode is activated, the system shall:
- Remove ALL redundant derivedFrom relations where alternate paths exist
- Preserve traceability through intermediate elements
- Maintain model coherence by ensuring all elements remain reachable through non-redundant paths
- Report removed relations to the user for transparency
- Show which intermediate paths provide the alternate routes
- Categorize ALL redundant hierarchical relations as **auto-fixable** since the direct relation adds no value when alternate paths exist

**Implementation Note**: The current implementation only detects cases where a direct redundant relation EXISTS. It does not detect or suggest whether converging paths without a direct relation should have one added - that remains a semantic modeling decision.

#### Metadata
  * type: specification
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
---
