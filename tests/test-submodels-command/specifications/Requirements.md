# Elements

### Root One

#### Metadata
  * type: user-requirement
---

### Payments Requirement

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root One](#root-one)
---

### Invoice Requirement

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Payments Requirement](#payments-requirement)
  * trace: [Identity Requirement](#identity-requirement)
---

### Billing Requirement

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root One](#root-one)
---

### Receipt Requirement

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Billing Requirement](#billing-requirement)
---

### Root Two

#### Metadata
  * type: user-requirement
---

### Identity Requirement

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Two](#root-two)
---

### Session Requirement

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Identity Requirement](#identity-requirement)
  * trace: [Payments Requirement](#payments-requirement)
---
