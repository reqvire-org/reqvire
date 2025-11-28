# Elements

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
---

### Two-Pass Validation Behavior

Two-phase validation process for model parsing.

#### Details
**Pass 1: Element Collection**
- Parse all markdown files
- Extract elements with metadata
- Local validation (uniqueness, format, syntax)
- Report errors if found

**Pass 2: Graph Validation**
- Build in-memory model representation from elements
- Validate relations (existence, type compatibility)
- Cross-component validation
- Report errors if found

#### Metadata
  * type: behavior
---

### Validation Error Reporting Behavior

Error message structure for validation issues.

#### Details
- File path and line number included
- Element name and relation details shown
- Color coding per Color Scheme Specification
- Actionable suggestions when possible

#### Metadata
  * type: behavior
---
