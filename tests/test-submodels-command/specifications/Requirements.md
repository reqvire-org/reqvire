# Elements


### Feature One

First feature root for submodels command fixtures.

#### Metadata
  * type: feature
---

### Feature Two

Second feature root for submodels command fixtures.

#### Metadata
  * type: feature
---

### Root One

#### Metadata
  * type: requirement

#### Relations
  * specify: [Feature One](#feature-one)
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
  * specify: [Feature Two](#feature-two)
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
