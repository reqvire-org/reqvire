# Elements

### No Relations Requirement

A requirement without initial relations that link/unlink commands will modify.

#### Metadata
  * type: requirement
---

### System Requirements

Top-level container for test requirements.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Another Requirement](#another-requirement)
  * derive: [Feature Requirement](#feature-requirement)
  * specify: [Test Feature Test Link Unlink Specifications Requirements Md](#test-feature-test-link-unlink-specifications-requirements-md)
---

### Another Requirement

Another requirement for testing.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Feature Requirement](#feature-requirement)
  * derivedFrom: [System Requirements](#system-requirements)
  * verifiedBy: [Orphan Test](Verifications.md#orphan-test)
---

### Feature Requirement

A requirement to be linked/unlinked.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Another Requirement](#another-requirement)
  * derivedFrom: [System Requirements](#system-requirements)
  * verifiedBy: [Feature Test](Verifications.md#feature-test)
---

### Test Feature Test Link Unlink Specifications Requirements Md

Test feature root for migrated requirement fixtures.

#### Metadata
  * type: feature

#### Relations
  * specifiedBy: [System Requirements](#system-requirements)
---

