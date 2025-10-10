# Valid Refinement Chain Requirements

## Pure Refine Chain Test

### Parent Requirement A

This is a parent requirement that will be refined.

#### Metadata
  * type: user-requirement

---

### Child Requirement A1

This requirement refines Parent Requirement A.

#### Metadata
  * type: user-requirement

#### Relations
  * refine: #parent-requirement-a

---

### Grandchild Requirement A11

This requirement refines Child Requirement A1, maintaining chain purity.

#### Metadata
  * type: user-requirement

#### Relations
  * refinedBy: #child-requirement-a1

---

## Mixed Hierarchy with Refine Starting Mid-Chain

### Parent Requirement B

This is a parent requirement.

#### Metadata
  * type: user-requirement

---

### Child Requirement B1

This requirement is derived from Parent Requirement B.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: #parent-requirement-b

---

### Grandchild Requirement B11

This requirement refines Child Requirement B1 (refine starts here).

#### Metadata
  * type: user-requirement

#### Relations
  * refine: #child-requirement-b1

---

### Great-Grandchild Requirement B111

This requirement refines Grandchild Requirement B11 (purity maintained after refine started).

#### Metadata
  * type: user-requirement

#### Relations
  * refinedBy: #grandchild-requirement-b11

---

## Pure Derive Chain Test (for comparison)

### Parent Requirement C

This is a parent requirement.

#### Metadata
  * type: user-requirement

---

### Child Requirement C1

This requirement is derived from Parent Requirement C.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: #parent-requirement-c

---

### Grandchild Requirement C11

This requirement is derived from Child Requirement C1.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: #child-requirement-c1

---

## Pure Contain Chain Test (for comparison)

### Container Requirement D

This is a container requirement.

#### Metadata
  * type: user-requirement

#### Relations
  * contain: #child-requirement-d1

---

### Child Requirement D1

This requirement is contained by Container Requirement D.

#### Metadata
  * type: user-requirement

#### Relations
  * containedBy: #container-requirement-d
  * contain: #grandchild-requirement-d11

---

### Grandchild Requirement D11

This requirement is contained by Child Requirement D1.

#### Metadata
  * type: user-requirement

#### Relations
  * containedBy: #child-requirement-d1

---
