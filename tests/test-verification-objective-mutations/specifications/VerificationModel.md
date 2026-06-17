# Elements

### Product Capability

Capability used by verification objective mutation tests.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [System Requirement](#system-requirement)
---

### System Requirement

Requirement verified by concrete verification elements.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Product Capability](#product-capability)
  * verifiedBy: [Concrete Verification](#concrete-verification)
  * verifiedBy: [Concrete Merge Candidate](#concrete-merge-candidate)
---

### Verification Objective

Planning objective for concrete verification work.

#### Metadata
  * type: verification-objective
---

### Second Verification Objective

Second objective used as a relink and move target.

#### Metadata
  * type: verification-objective
---

### Objective Merge Target

Target objective for merge compatibility testing.

#### Metadata
  * type: verification-objective
---

### Objective Merge Source

Source objective for merge compatibility testing.

#### Metadata
  * type: verification-objective
---

### Concrete Verification

Concrete verification derived from a verification objective.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [System Requirement](#system-requirement)
  * satisfiedBy: [test.sh](test.sh)
---

### Concrete Merge Candidate

Concrete verification used to prove objectives cannot merge with concrete verification.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [System Requirement](#system-requirement)
  * satisfiedBy: [test.sh](test.sh)
---
