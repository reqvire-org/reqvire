# Elements

### Spec-1

A valid refinement that satisfies User Req A.

#### Details
This refinement has a satisfy relation, so it can be attached to requirements outside its defining hierarchy.

#### Metadata
  * type: specification

#### Relations
  * satisfy: [User Req A](Requirements.md#user-req-a)
---

### Spec-2

A valid refinement that satisfies Child With Refinement.

#### Details
This refinement tests the ancestor case - Ancestor Req is a parent of the defining requirement, so it cannot attach Spec-2.

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Child With Refinement](Requirements.md#child-with-refinement)
---

### Orphan-Spec

An orphan refinement without any satisfy relations.

#### Details
This refinement has no satisfy relations, so it cannot be attached anywhere.

#### Metadata
  * type: specification
---

