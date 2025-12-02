# Elements

### System Requirements

Top level container for requirements.

#### Metadata
  * type: user-requirement
---

### Target Requirement

This is the target requirement that will receive merged content.

Main content of target requirement.

#### Details
Target details section content.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
  * verifiedBy: Verifications.md#target-test
  * trace: [Another Link](#another-link)
---

### Source Requirement One

This is source requirement one.

Main content of source one.

#### Details
Details from source one.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
  * derive: [Child Requirement](#child-requirement)
  * trace: [Placeholder Requirement](#placeholder-requirement)
---

### Source Requirement Two

This is source requirement two.

Main content of source two.

#### Details
Details from source two with more info.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
  * trace: [Another Link](#another-link)
  * verifiedBy: Verifications.md#source-two-test
---

### Unrelated Requirement

This requirement points to Source Requirement One.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Source Requirement One](#source-requirement-one)
---

### Child Requirement

A child requirement derived from Source Requirement One.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Source Requirement One](#source-requirement-one)
---

### Test Verification

A verification element for testing type compatibility error.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Requirement](#target-requirement)
---

### Another Link

Just a placeholder for trace relation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
---

### Placeholder Requirement

Another placeholder for testing different relation types.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
---

