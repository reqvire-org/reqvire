# Invalid Refinement Chain Requirements

## Invalid: Refine followed by Derive

### Parent Requirement E

This is a parent requirement.

#### Metadata
  * type: user-requirement

---

### Child Requirement E1

This requirement refines Parent Requirement E.

#### Metadata
  * type: user-requirement

#### Relations
  * refine: #parent-requirement-e

---

### Grandchild Requirement E11

This requirement INCORRECTLY uses derive after refine was used.
This violates refinement chain purity.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: #child-requirement-e1

---

## Invalid: Refine followed by Contain

### Parent Requirement F

This is a parent requirement.

#### Metadata
  * type: user-requirement

---

### Child Requirement F1

This requirement refines Parent Requirement F.

#### Metadata
  * type: user-requirement

#### Relations
  * refinedBy: #parent-requirement-f

---

### Grandchild Requirement F11

This requirement INCORRECTLY uses contain after refine was used.
This violates refinement chain purity.

#### Metadata
  * type: user-requirement

#### Relations
  * containedBy: #child-requirement-f1

---

## Invalid: Deep chain with purity violation

### Parent Requirement G

This is a parent requirement.

#### Metadata
  * type: user-requirement

---

### Child Requirement G1

This requirement is derived from Parent Requirement G.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: #parent-requirement-g

---

### Grandchild Requirement G11

This requirement refines Child Requirement G1 (refine starts here).

#### Metadata
  * type: user-requirement

#### Relations
  * refine: #child-requirement-g1

---

### Great-Grandchild Requirement G111

This requirement refines Grandchild Requirement G11.

#### Metadata
  * type: user-requirement

#### Relations
  * refinedBy: #grandchild-requirement-g11

---

### Great-Great-Grandchild Requirement G1111

This requirement INCORRECTLY uses derive after refine chain started.
This violates refinement chain purity.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: #great-grandchild-requirement-g111

---
