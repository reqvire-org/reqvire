# Elements


### User Req A Capability

Capability root for User Req A hierarchy.

#### Metadata
  * type: capability
---

### User Req X Capability

Capability root for User Req X hierarchy.

#### Metadata
  * type: capability
---

### User Req Y Capability

Capability root for User Req Y hierarchy.

#### Metadata
  * type: capability
---

### Ancestor Req Capability

Capability root for Ancestor Req hierarchy.

#### Metadata
  * type: capability
---

### User Req A

Top-level capability that defines Spec-1 via definedBy.

#### Metadata
  * type: requirement
#### Relations
  * specify: [User Req A Capability](#user-req-a-capability)
  * derive: [Req B](#req-b)
  * derive: [Req D](#req-d)
  * definedBy: [Spec-1](Refinements.md#spec-1)
---

### Req B

Child requirement of User Req A (first level descendant).

#### Metadata
  * type: requirement

#### Relations
  * derive: [Req C](#req-c)
  * derivedFrom: [User Req A](#user-req-a)
---

### Req C

Grandchild requirement (second level descendant of User Req A).

#### Metadata
  * type: requirement

#### Relations
  * derive: [Req C1](#req-c1)
  * derivedFrom: [Req B](#req-b)
---

### Req C1

Great-grandchild requirement (third level descendant of User Req A).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Req C](#req-c)
---

### Req D

Sibling branch of Req B (still child of User Req A).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [User Req A](#user-req-a)
---

### User Req X

Separate branch requirement - can legitimately attach Spec-1 and defines Spec-X.

#### Metadata
  * type: requirement
#### Attachments
  * [Spec-1](Refinements.md#spec-1)

#### Relations
  * specify: [User Req X Capability](#user-req-x-capability)
  * definedBy: [Spec-X](Refinements.md#spec-x)
---

### User Req Y

Another separate branch with its own hierarchy.

#### Metadata
  * type: requirement
#### Relations
  * specify: [User Req Y Capability](#user-req-y-capability)
  * derive: [Req Y1](#req-y1)
---

### Req Y1

Child of User Req Y in a completely separate hierarchy - can attach Spec-1.

#### Metadata
  * type: requirement

#### Attachments
  * [Spec-1](Refinements.md#spec-1)

#### Relations
  * derivedFrom: [User Req Y](#user-req-y)
---

### Ancestor Req

A requirement that has children with definedBy to a refinement.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Ancestor Req Capability](#ancestor-req-capability)
  * derive: [Child With Refinement](#child-with-refinement)
---

### Child With Refinement

A child requirement that defines Spec-2 via definedBy.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Grandchild Req](#grandchild-req)
  * derivedFrom: [Ancestor Req](#ancestor-req)
  * definedBy: [Spec-2](Refinements.md#spec-2)
---

### Grandchild Req

A grandchild requirement under Child With Refinement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Child With Refinement](#child-with-refinement)
---
