# Elements

### System Requirements

Top-level container for test requirements.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Another Requirement](#another-requirement)
  * derive: [Feature Requirement](#feature-requirement)
  * derive: [No Relations Requirement](#no-relations-requirement)
---

### Another Requirement

Another requirement for testing.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
  * verifiedBy: [Orphan Test](Verifications.md#orphan-test)
---

### Feature Requirement

A requirement to be linked/unlinked.

#### Metadata
  * type: requirement

#### Relations
  * derive: [No Relations Requirement](#no-relations-requirement)
  * derivedFrom: [System Requirements](#system-requirements)
  * verifiedBy: [Feature Test](Verifications.md#feature-test)
---

### No Relations Requirement

A requirement without initial relations that link/unlink commands will modify.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Requirement](#feature-requirement)
  * derivedFrom: [System Requirements](#system-requirements)
---
