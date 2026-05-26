# Elements


### Capability One

First capability root for submodels command fixtures.

#### Metadata
  * type: capability
---

### Capability Two

Second capability root for submodels command fixtures.

#### Metadata
  * type: capability
---

### Root One

#### Metadata
  * type: requirement

#### Relations
  * specify: [Capability One](#capability-one)
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
  * type: requirement

#### Relations
  * specify: [Capability Two](#capability-two)
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
