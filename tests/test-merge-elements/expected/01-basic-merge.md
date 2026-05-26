# Elements

### Child Requirement

A child requirement derived from Source Requirement One.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Requirement](#target-requirement)
---

### System Requirements

Top level container for requirements.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability Test Merge Elements Specifications Requirements Md](#test-capability-test-merge-elements-specifications-requirements-md)
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
  * verifiedBy: [Source Two Test](Verifications.md#source-two-test)
---

### Target Requirement

This is the target requirement that will receive merged content.

Main content of target requirement.

#### Details
Target details section content.
This is source requirement one.

Main content of source one.

#### Merged Details (Source Requirement One)
Details from source one.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Child Requirement](#child-requirement)
  * derivedFrom: [System Requirements](#system-requirements)
  * trace: [Another Link](#another-link)
  * trace: [Placeholder Requirement](#placeholder-requirement)
  * verifiedBy: [Target Test](Verifications.md#target-test)
---

### Test Capability Test Merge Elements Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Test Verification

A verification element for testing type compatibility error.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Target Requirement](#target-requirement)
---

### Unrelated Requirement

This requirement points to Source Requirement One.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Requirement](#target-requirement)
---

