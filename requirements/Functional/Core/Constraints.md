# Elements

### Cross-Section Duplicate Constraint

Rules for detecting duplicate link targets across subsections.

**Cross-section duplicates** (same target in BOTH Relations AND Attachments):
- Applies to all target types: element identifiers AND file paths
- Treated as semantic errors requiring user resolution
- The validate command fails with error
- The format command does not auto-fix (user must decide which section to keep)

**Within-section duplicates** (same entry repeated in Relations OR in Attachments):
- Treated as formatting issues, not validation errors
- The format fix operation removes duplicate entries
- The validate command does not fail for within-section duplicates

#### Metadata
  * type: constraint

#### Relations
  * satisfy: [Cross-Section Duplicate Validation](Validation.md#cross-section-duplicate-validation)
---

### Element Type Relation Compatibility Constraint

Validation rules for element type and relation type combinations.

#### Details
**Relation Type Restrictions:**
| Relation Type | Allowed Source Types | Allowed Target Types |
|---------------|---------------------|---------------------|
| derivedFrom/derive | requirement, user-requirement | requirement, user-requirement |
| satisfiedBy/satisfy | requirement, user-requirement, test-verification, refinement types | InternalPath, refinement types |
| verifiedBy/verify | requirement, user-requirement | All verification types |
| trace | Any (except refinement types) | Any |

**Key Constraints:**
- derivedFrom/derive restricted to requirement types only
- Refinement types (constraint, behavior, specification) can only have `satisfy` relations
- Only test-verification can use satisfiedBy among verification types
- Elements with type "other" can only use trace relations

#### Metadata
  * type: constraint

#### Relations
  * satisfy: [Element Type Relation Compatibility](ModelManagement.md#element-type-relation-compatibility)
---

### Attachment Hierarchical Independence Constraint

Rules for preventing attachments within the same requirement hierarchy.

A refinement element can only be attached to a requirement if that requirement has NO hierarchical relationship with the requirement that defines the refinement via `satisfiedBy`:
- Cannot attach to the requirement that has `satisfiedBy` to this refinement
- Cannot attach to any parent (ancestor) of that requirement via derivedFrom chain
- Cannot attach to any child (descendant) of that requirement via derive chain

Only requirements in a separate branch of the hierarchy (no derivedFrom chain connecting them) may attach the refinement.

**Rationale**: Attachments enable cross-submodel traceability while maintaining stakeholder separation. Attachments within the same hierarchy are redundant since traceability already flows through the satisfiedBy relationship.

#### Metadata
  * type: constraint

#### Relations
  * satisfy: [Attachment Scope Constraints](ModelManagement.md#attachment-scope-constraints)
---

### Attachment Satisfied Refinement Constraint

Rules requiring refinements to have satisfy relations before being attachable.

A refinement element can only be attached to requirements if:
- The refinement has at least one `satisfy` relation to a requirement
- Refinements without satisfy relations cannot be attached anywhere

**Rationale**: Enforces model hygiene by ensuring refinements are properly integrated into the model through explicit satisfaction relationships before being referenced elsewhere.

#### Metadata
  * type: constraint

#### Relations
  * satisfy: [Attachment Scope Constraints](ModelManagement.md#attachment-scope-constraints)
---
