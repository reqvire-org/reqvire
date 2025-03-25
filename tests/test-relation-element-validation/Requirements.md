# Requirements

This is a test file for validating relation element type validation.

## System Requirements

### Valid-Requirement
A requirement with a valid verifiedBy relation to a verification element.

#### Metadata
* type: requirement

#### Relations
* verifiedBy: [#Valid-Verification](#Valid-Verification)
* satisfiedBy: [#Valid-Implementation](#Valid-Implementation)

---

### Invalid-Requirement
A requirement with an invalid verifiedBy relation to a non-verification element.

#### Metadata
* type: requirement

#### Relations
* verifiedBy: [#Another-Requirement](#Another-Requirement)
* satisfiedBy: [#Another-Requirement](#Another-Requirement)

---

### Another-Requirement
Another requirement used for testing.

#### Metadata
* type: requirement

#### Relations
* derive: [#Valid-Requirement](#Valid-Requirement)

---

### Valid-Verification
A verification element with a valid verify relation to a requirement.

#### Metadata
* type: verification

---

### Invalid-Verification
A verification element with an invalid verify relation to a non-requirement.

#### Metadata
* type: verification

#### Relations
* verify: [#Valid-Verification](#Valid-Verification)

---

### Valid-Implementation
An implementation element with a valid satisfy relation to a requirement.

#### Metadata
* type: implementation

#### Relations
* satisfy: [#Valid-Requirement](#Valid-Requirement)

---

### Other-Relation-Element
An element with other relation types, which should not be validated for element types.

#### Metadata
* type: other

#### Relations
* trace: [#Valid-Requirement](#Valid-Requirement)
* derive: [#Another-Requirement](#Another-Requirement)
* contain: [#Valid-Verification](#Valid-Verification)
