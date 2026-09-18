# Elements

### Skill Reference Capability

CLI skill reference fixture capability.

#### Metadata
  * type: capability
---

### Skill Reference Requirement

The system shall provide executable command references.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Skill Reference Capability](#skill-reference-capability)
  * verifiedBy: [Skill Reference Test](#skill-reference-test)
---

### Skill Reference Verification Objective

Group CLI reference fixture verification.

#### Metadata
  * type: verification-objective
---

### Skill Reference Test

Verify CLI references against the fixture requirement.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Skill Reference Verification Objective](#skill-reference-verification-objective)
  * verify: [Skill Reference Requirement](#skill-reference-requirement)
  * satisfiedBy: [test.sh](../test.sh)
---
