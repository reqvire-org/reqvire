# Elements

### Design Details Refinement

A refinement element attached to Leaf From B.

#### Metadata
  * type: specification

#### Relations
  * satisfy: [Refinement Owner](#refinement-owner)
---

### Refinement Owner

Owner for the refinement (separate from main hierarchy).

#### Metadata
  * type: user-requirement
---

### Root Requirement

The top-level requirement with both attachments.

#### Metadata
  * type: user-requirement

#### Attachments
  * [spec-a.md](docs/spec-a.md)
  * [spec-b.md](docs/spec-b.md)
---

### Branch A

First branch derived from root, attaches only Spec A (redundant).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Leaf From A

Child of Branch A, also attaches Spec A (redundant from Branch A, which is redundant from Root).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Branch A](#branch-a)
---

### Deep Leaf

Grand-grandchild that attaches Spec A (redundant from ancestor Root via Branch A -> Leaf From A).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Leaf From A](#leaf-from-a)
---

### Branch B

Second branch derived from root, attaches only Spec B (redundant).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Leaf From B

Child of Branch B with a refinement attachment.

#### Metadata
  * type: requirement

#### Attachments
  * [Design Details Refinement](Requirements.md#design-details-refinement)

#### Relations
  * derivedFrom: [Branch B](#branch-b)
---
