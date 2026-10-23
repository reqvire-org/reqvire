# Elements

### Cross-Submodel Hierarchical Relation Detection Specification

- Determine ownership roots from hierarchical relations using only hierarchy edges (for example `derivedFrom` and `derive`).
- For each user-created hierarchical relation where source root differs from target root, create a manual-review lint item.
- Include in that item:
 - `source`, `target`, and `relation_type`
 - owning source root
 - owning target root
 - rationale that identifies the ownership boundary and directs the author to choose compatible coupling: Contract References for review context, Contract Bindings for independent implementation obligations, or another permitted relation for the actual source and target types. Contract dependencies belong on requirements and target uniquely requirement-owned contracts; the diagnostic must not prescribe a binding without establishing that intent.
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

### Lint Auto-fix Capability Contract Specification

#### Details
Auto-fix behavior:
- Applies fixes only for issues categorized as auto-fixable.
- Stages affected model documents through the complete-candidate validation and canonical transactional persistence gates.
- Removes redundant verify relations from verification elements where safe.
- Preserves unrelated content and untouched file bytes; normalizes affected documents through default canonical formatting.
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
- For auto-fixable issues: identify the safe repair as MCP `reqvire.lint` with `fix: true` and mutation enablement; CLI lint remains inspection only
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
- Inspect hierarchical ancestor reachability from each direct parent to identify the nearest eligible common ancestor reached through distinct branches
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

Contract for redundant direct hierarchical edges.

#### Details
- A direct `derivedFrom` edge from A to ancestor C is redundant when an alternate path from A to C exists through other `derivedFrom` edges. One intermediate chain or multiple converging paths both qualify.
- Determine redundancy from hierarchical graph reachability for the source element's direct parents. Verification graph presentation and synthetic verification elements do not determine the repair policy.
- Categorize eligible redundant direct edges as auto-fixable. Permitted repair removes those edges while preserving intermediate paths, unrelated content and manual-review findings.
- Convergence without a direct edge to the common ancestor is a separate manual-review case. Do not infer a new direct edge or choose which branch to remove.
- Cross-submodel ownership violations remain manual review and are excluded from automatic removal.
- Report affected identifiers, the redundant direct relations and alternate reachability. The shared mutation transaction owns validation and canonical persistence; this contract owns redundancy classification.

For example, A derivedFrom B and B derivedFrom C make A derivedFrom C redundant. A derivedFrom B and D, with B and D each derivedFrom C, also make a direct A derivedFrom C edge redundant; retaining only the two branches requires semantic review rather than automatic branch removal.

#### Metadata
  * type: specification
---

### Redundant Verify Relations Detection Contract Specification

Contract for redundant direct verification edges.

#### Details
- A directly verified requirement is redundant when it is reachable through requirement `derivedFrom` ancestry from at least two other requirements directly verified by the same concrete verification.
- Count distinct directly verified requirements that reach the candidate ancestor. Those starts need not be leaf requirements. One other directly verified descendant is insufficient for this finding.
- Use the shared ancestor-reachability analysis; retain every direct verification target in trace graphs regardless of whether lint classifies a relation as redundant.
- Report the verification, redundant direct ancestor targets and rationale; classify these findings as auto-fixable.
- Permitted repair removes only eligible redundant direct `verify` edges. Coverage roll-up alone does not establish that a separate parent integration verification is redundant.

#### Metadata
  * type: specification

#### Relations
  * define: [Redundant Verify Relations Detection](LintingRequirements.md#redundant-verify-relations-detection)
---

### Rust Workspace Lint Policy Specification

The Rust workspace lint policy is expected to provide a common clippy baseline for all workspace crates.

#### Details
Policy requirements:
- The root `Cargo.toml` owns `[workspace.lints.clippy]` so lint levels are centralized.
- `clippy::all`, `clippy::nursery`, `clippy::unwrap_used`, and `clippy::clone_on_ref_ptr` are warning-level checks.
- `clippy::pedantic` may remain staged until the warning baseline is clean and documented.
- The `Makefile` exposes a `clippy` target that runs `cargo clippy --workspace --all-targets`.
- The `Makefile` exposes formatting and e2e targets so implementation quality checks can be run consistently with tests.

#### Metadata
  * type: specification

#### Relations
  * define: [Rust Workspace Lint Policy](LintingRequirements.md#rust-workspace-lint-policy)
---
