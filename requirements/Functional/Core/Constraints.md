# Elements

### Attachment Hierarchical Independence Constraint

Rules for preventing attachments within the same requirement hierarchy.

**For refinement elements:**
A refinement element can only be attached to a requirement if that requirement has NO hierarchical relationship with the requirement that owns the refinement via `refinedBy`:
- Cannot attach to the requirement that has `refinedBy` to this refinement
- Cannot attach to any parent (ancestor) of that requirement via derivedFrom chain
- Cannot attach to any child (descendant) of that requirement via derive chain

**For file attachments (assets):**
When a file is referenced via `refinedBy` from a requirement (establishing ownership), the same hierarchical independence rules apply:
- Cannot attach to the requirement that owns the file via `refinedBy`
- Cannot attach to any parent (ancestor) of the owner via derivedFrom chain
- Cannot attach to any child (descendant) of the owner via derive chain

**Upstream attachment propagation:**
If an attachment (refinement or file) is already attached to an ancestor requirement in the derivation hierarchy, descendants cannot attach the same target:
- Attachments propagate downstream through the derivedFrom chain
- Re-attaching at a descendant level is redundant
- Only the highest-level requirement in a hierarchy branch should attach

Only requirements in a separate branch of the hierarchy (no derivedFrom chain connecting them to the owner or existing attacher) may attach the refinement or file.

**Rationale**: Attachments enable cross-submodel traceability while maintaining stakeholder separation. Attachments within the same hierarchy are redundant since traceability already flows through the refinedBy relationship and propagates to derived requirements.

#### Metadata
  * type: constraint
---

### Attachment Satisfied Refinement Constraint

Rules requiring refinements to have a refine relation before being attachable.

A refinement element can only be attached to requirements if:
- The refinement has a `refine` relation to a requirement (establishing ownership)
- Refinements without a refine relation cannot be attached anywhere

**Rationale**: Enforces model hygiene by ensuring refinements are properly integrated into the model through an explicit refine relationship to an owning requirement before being referenced elsewhere.

#### Metadata
  * type: constraint
---

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
---

### Element Type Relation Compatibility Constraint

Validation rules for element type and relation type combinations.

#### Details
**Relation Type Restrictions:**
| Relation Type | Allowed Source Types | Allowed Target Types |
|---------------|---------------------|---------------------|
| derivedFrom/derive | requirement, user-requirement | requirement, user-requirement |
| satisfiedBy/satisfy | requirement, user-requirement, test-verification | InternalPath |
| refinedBy/refine | requirement, user-requirement | refinement types, InternalPath |
| verifiedBy/verify | requirement, user-requirement | All verification types |
| trace | Any (except refinement types) | Any |

**Key Constraints:**
- derivedFrom/derive restricted to requirement types only
- Refinement types (constraint, behavior, specification) can only have `refine` relations
- Only test-verification can use satisfiedBy among verification types
- Elements with type "other" can only use trace relations

#### Metadata
  * type: constraint
---
