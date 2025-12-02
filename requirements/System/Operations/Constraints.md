# Elements

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

#### Relations
  * satisfy: [Merge Element Operation](ElementManipulation.md#merge-element-operation)
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

#### Relations
  * satisfy: [Target Location Validation and Auto-Creation](ElementManipulation.md#target-location-validation-and-auto-creation)
---
