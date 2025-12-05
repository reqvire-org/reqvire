# Elements

### No Relations Requirement

A requirement without initial relations that link/unlink commands will modify.

#### Metadata
  * type: requirement
---

### System Requirements

Top-level container for test requirements.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Feature Requirement](#feature-requirement)
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
  * derivedFrom: [System Requirements](#system-requirements)
---

