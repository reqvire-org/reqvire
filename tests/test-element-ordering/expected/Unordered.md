# Elements

### Parent A

Parent A is a root element with children.

#### Metadata
  * type: user-requirement
---

### Child A

Child A derives from Parent A.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent A](#parent-a)
---

### Grandchild M

Grandchild M derives from Child A.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Child A](#child-a)
---

### Grandchild Z

Grandchild Z derives from Child A.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Child A](#child-a)
---

### Child B

Child B element that derives from Parent A.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent A](#parent-a)
---

### Standalone Element

Standalone element with no file-local parents.

#### Metadata
  * type: user-requirement
---
