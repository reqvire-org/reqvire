# Elements

### Atomic Relink Validity Constraint

Validation rules for atomic relation relink operations.

#### Details
- Relink operations shall be atomic: no partial persistence is allowed on failure.
- Candidate relink state shall satisfy all model validation rules before commit.
- For hierarchical relations (`derivedFrom`/`derive`), relink shall be rejected when it would:
  - create circular dependencies,
  - orphan requirements without required capability or parent requirement ownership,
  - violate single-root hierarchy ownership.
- Error output shall identify the failing validation constraints and impacted elements.

#### Metadata
  * type: constraint

#### Relations
  * define: [Atomic Relation Relink Operation](ElementManipulationRequirements.md#atomic-relation-relink-operation)
---

### Merge Type Compatibility Constraint

Type compatibility rules for merging elements.

#### Details
Canonical merge compatibility categories are defined by the Reqvire operation ontology and core element type vocabulary.

**Compatibility Rules:**
- Source and target must be in the same semantic merge compatibility category.
- Merge categories must preserve ontology, capability, requirement, verification, contract, and semantic-contract ownership.
- Merging across semantic categories is forbidden.

**Error Messages:**
- When incompatible: "Cannot merge {source_type} into {target_type}: type mismatch"

#### Ontology Merge Category
Ontology elements are merge-compatible only with ontology elements, and merge shall fold authored Turtle into the target ontology block after rewriting to the target boundary.

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
