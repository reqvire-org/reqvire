# Elements

### Child Requirement

A child requirement derived from Source Requirement One.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Requirement](#target-requirement)
---

### Requirements Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---

### Test Verification

A verification element for testing type compatibility error.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Requirements Verification Objective](#requirements-verification-objective)
  * verify: [Target Requirement](#target-requirement)
---

### System Requirements

Top level container for requirements.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability Test Merge Elements Specifications Requirements Md](#test-capability-test-merge-elements-specifications-requirements-md)
---

### Another Link

Just a placeholder for merge relation content.

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

### Target Requirement

This is the target requirement that will receive merged content.

Main content of target requirement.

#### Details
Target details section content.
This is source requirement one.

Main content of source one.

#### Merged Details (Source Requirement One)
Details from source one.

This is source requirement two.

Main content of source two.

#### Merged Details (Source Requirement Two)
Details from source two with more info.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Child Requirement](#child-requirement)
  * derivedFrom: [System Requirements](#system-requirements)
  * verifiedBy: [Source Two Test](Verifications.md#source-two-test)
  * verifiedBy: [Target Test](Verifications.md#target-test)
---

### Test Capability Test Merge Elements Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Unrelated Requirement

This requirement points to Source Requirement One.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Target Requirement](#target-requirement)
---

