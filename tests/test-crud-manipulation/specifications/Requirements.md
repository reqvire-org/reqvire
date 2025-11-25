# Elements

### Parent Feature

This is a parent requirement.

#### Metadata
  * type: user-requirement

---

### Feature A

This is a test requirement for feature A.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Feature](#parent-feature)
---

### Feature B

This is a test requirement for feature B.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
---

### Feature C

This is a test requirement for feature C with relations.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
  * verifiedBy: [Test for Feature C](Verifications/Tests.md#test-for-feature-c)
---
