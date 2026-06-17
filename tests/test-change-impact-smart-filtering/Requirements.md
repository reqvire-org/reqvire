# Elements


### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Existing Capability

Existing capability scope.

#### Metadata
* type: capability

---

### Existing Requirement One

This is an existing requirement.

#### Metadata
* type: requirement

#### Relations
  * specify: [Existing Capability](#existing-capability)
  * verifiedBy: [Existing Verification](#existing-verification)

---

### Existing Requirement Two

This is another existing requirement.

#### Metadata
* type: requirement

#### Relations
  * specify: [Existing Capability](#existing-capability)

---


### Existing Verification

This verifies the existing requirement.

#### Metadata
  * type: verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Existing Requirement One](#existing-requirement-one)
