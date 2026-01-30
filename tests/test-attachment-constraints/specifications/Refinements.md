# Elements

### Spec-1

A valid refinement that refines User Req A.

#### Details
This refinement is owned via refinedBy relation, so it can be attached to requirements outside its defining hierarchy.

#### Metadata
  * type: specification
---

### Spec-2

A valid refinement that refines Child With Refinement.

#### Details
This refinement tests the ancestor case - Ancestor Req is a parent of the defining requirement, so it cannot attach Spec-2.

#### Metadata
  * type: specification
---

### Orphan-Spec

An orphan refinement without any refine relations.

#### Details
This refinement has no refine relations, so it cannot be attached anywhere.

#### Metadata
  * type: specification
---

