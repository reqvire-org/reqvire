# Elements

### Root Requirement

The top-level requirement with both attachments.

#### Metadata
  * type: user-requirement

#### Attachments
  * [Spec A](docs/spec-a.md)
  * [Spec B](docs/spec-b.md)
---

### Branch A

First branch derived from root, attaches only Spec A (redundant).

#### Metadata
  * type: requirement

#### Attachments
  * [Spec A](docs/spec-a.md)

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Branch B

Second branch derived from root, attaches only Spec B (redundant).

#### Metadata
  * type: requirement

#### Attachments
  * [Spec B](docs/spec-b.md)

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Leaf From A

Child of Branch A, also attaches Spec A (redundant from Branch A, which is redundant from Root).

#### Metadata
  * type: requirement

#### Attachments
  * [Spec A](docs/spec-a.md)

#### Relations
  * derivedFrom: [Branch A](#branch-a)
---

### Leaf From B

Child of Branch B with a refinement attachment.

#### Metadata
  * type: requirement

#### Attachments
  * [Design Details](#design-details-refinement)

#### Relations
  * derivedFrom: [Branch B](#branch-b)
---

### Design Details Refinement

A refinement element attached to Leaf From B.

#### Metadata
  * type: specification
---

### Deep Leaf

Grand-grandchild that attaches Spec A (redundant from ancestor Root via Branch A -> Leaf From A).

#### Metadata
  * type: requirement

#### Attachments
  * [Spec A](docs/spec-a.md)

#### Relations
  * derivedFrom: [Leaf From A](#leaf-from-a)
---
