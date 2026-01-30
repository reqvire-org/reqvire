# Elements

### User Req A

Top-level user requirement that defines Spec-1 via refinedBy.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Req B](#req-b)
  * derive: [Req D](#req-d)
  * refinedBy: [Spec-1](Refinements.md#spec-1)
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

Separate branch requirement - can legitimately attach Spec-1.

#### Metadata
  * type: user-requirement

#### Attachments
  * [Spec-1](Refinements.md#spec-1)
---

### User Req Y

Another separate branch with its own hierarchy.

#### Metadata
  * type: user-requirement

#### Relations
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

A requirement that has children with refinedBy to a refinement.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Child With Refinement](#child-with-refinement)
---

### Child With Refinement

A child requirement that defines Spec-2 via refinedBy.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Grandchild Req](#grandchild-req)
  * derivedFrom: [Ancestor Req](#ancestor-req)
  * refinedBy: [Spec-2](Refinements.md#spec-2)
---

### Grandchild Req

A grandchild requirement under Child With Refinement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Child With Refinement](#child-with-refinement)
---

