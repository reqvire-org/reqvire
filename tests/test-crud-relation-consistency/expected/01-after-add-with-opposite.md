# Elements

### Root Feature

This is the feature root.

#### Metadata
  * type: feature
---

### Root Requirement

This is the root requirement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Root Feature](#root-feature)
---

### Derived Requirement 1

This requirement is derived from root.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
  * verifiedBy: [Test 1](Verifications/Tests.md#test-1)
---

### Derived Requirement 2

This requirement is also derived from root.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
  * verifiedBy: [Test 2](Verifications/Tests.md#test-2)
---

### Derived Requirement 3

This is a new derived requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

