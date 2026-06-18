# Elements

### Cross-Submodel Hierarchical Relation Detection Specification

- Determine ownership roots from hierarchical relations using only hierarchy edges (for example `derivedFrom` and `derive`).
- For each user-created hierarchical relation where source root differs from target root, create a manual-review lint item.
- Include in that item:
 - `source`, `target`, and `relation_type`
 - owning source root
 - owning target root
 - rationale that states why the relation breaks submodel ownership and should be converted to attachment-based coupling.
- Report such findings in:
 - default text output under **Needs Manual Review**
 - `--auditable` output
 - `needs_manual_review` JSON array
- Do not include cross-submodel hierarchical items in auto-fix output (`--fixable`).
- Never auto-remove cross-submodel hierarchical relations; they require explicit model refactoring.

#### Metadata
  * type: specification

#### Relations
  * define: [Cross-Submodel Hierarchical Relation Detection](LintingRequirements.md#cross-submodel-hierarchical-relation-detection)
---

### Lint Auto-fix Capability Refinement Specification

#### Details
Auto-fix behavior:
- Applies fixes only for issues categorized as auto-fixable.
- Modifies affected markdown files directly.
- Removes redundant verify relations from verification elements where safe.
- Preserves unrelated content and formatting.
- Reports all applied changes (files modified, relations removed).
- Skips issues categorized as needing manual review.

#### Metadata
  * type: specification

#### Relations
  * define: [Lint Auto-fix Capability](LintingRequirements.md#lint-auto-fix-capability)
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
- Semantic-contract references are validated before lint output; outside-context semantic references are validation errors and are not emitted as lint findings

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

Detection is expected to:
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

The system is expected to use verification trace tree logic for detection:
- Create a virtual/dummy verification element
- Connect the virtual verification to ALL leaf requirements (requirements with no derived children) via virtual verify relations
- Apply the same trace tree building logic used for verification upward traceability
- The trace tree will naturally identify when leaf requirements have derivedFrom relations to both a parent and its ancestor
- Identify which intermediate paths provide the alternate routes to the ancestor

This approach reuses the proven trace tree logic for redundancy detection, ensuring consistency with verify relation redundancy detection.

**Safe Auto-Removal Criteria:**

A redundant hierarchical derivation relation is expected to be considered safe to auto-remove when ALL of the following conditions are met:
1. **Direct relation exists**: Element A has a direct derivedFrom relation to element C
2. **Alternate path exists**: There exists at least one path from A to C through intermediate elements (single or multiple convergent paths)
3. **Transitive redundancy**: The direct A → C relation is redundant because C is reachable through other derivedFrom relations

**Examples:**

*Single-chain redundancy (auto-removable):*
```
Requirement A
 → Requirement B
 → Requirement C

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

When auto-fix mode is activated, the system is expected to:
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

### Redundant Verify Relations Detection Refinement Specification

#### Details
Redundant verify relation detection behavior:
- Detects cases where a verification directly verifies both a leaf requirement and its ancestor.
- Uses verification trace tree analysis to determine ancestor reachability.
- Treats leaf verification as sufficient when hierarchy roll-up already covers ancestors.
- Reuses trace-tree logic from [Verification Trace Builder](../Processing/VerificationTraces.md#verification-trace-builder).
- Reports redundant direct verify relations as model noise.
- Categorizes these findings as auto-fixable.

#### Metadata
  * type: specification

#### Relations
  * define: [Redundant Verify Relations Detection](LintingRequirements.md#redundant-verify-relations-detection)
---
