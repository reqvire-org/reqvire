# Elements

### Atomic Relink Validity Constraint

Validation rules for atomic relation relink operations.

#### Details
- Relink operations shall be atomic: no partial persistence is allowed on failure.
- Candidate relink state shall satisfy all model validation rules before commit.
- For hierarchical relations (`derivedFrom`/`derive`), relink shall be rejected when it would:
  - create circular dependencies,
  - orphan system requirements without required parent hierarchy,
  - violate single-root hierarchy ownership.
- Error output shall identify the failing validation constraints and impacted elements.

#### Metadata
  * type: constraint

#### Relations
  * refine: [Atomic Relation Relink Operation](ElementManipulation.md#atomic-relation-relink-operation)
---

### Merge Type Compatibility Constraint

Type compatibility rules for merging elements.

#### Details
**Main Type Categories:**
- **Requirement types**: `requirement`, `user-requirement`
- **Verification types**: `test-verification`, `analysis-verification`, `inspection-verification`, `demonstration-verification`
- **Refinement types**: `constraint`, `behavior`, `specification`
- **Other**: Any element type not in above categories

**Compatibility Rules:**
- Source and target must be in the same main type category
- Subtype differences within a category are allowed (e.g., `user-requirement` can merge into `requirement`)
- Merging across categories is forbidden (e.g., requirement cannot merge into verification)

**Error Messages:**
- When incompatible: "Cannot merge {source_type} into {target_type}: type mismatch"

#### Metadata
  * type: constraint
---

### Target Location Constraint

Validation rules for target paths in element operations.

**Path Validation:**
- Path not excluded by .gitignore/.reqvireignore
- Maximum 10 subdirectory nesting depth
- Path must be accessible and writable

**Auto-creation:**
- Create missing target files with `# Elements` header

**Error Reporting:**
- Clear message indicating which constraint was violated

#### Metadata
  * type: constraint
---
