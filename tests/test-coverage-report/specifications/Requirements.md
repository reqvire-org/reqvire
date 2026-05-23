# Elements

This document contains test requirements and verifications to validate the coverage report functionality.

### Test Verification Satisfied

This is a test verification that should appear as satisfied in the coverage report.

#### Metadata
* type: test-verification

#### Relations
* verify: [Leaf Requirement Verified](#leaf-requirement-verified)
* satisfiedBy: [test-satisfied.sh](test-satisfied.sh)

---

### Test Verification Unsatisfied

This is a test verification that should appear as unsatisfied in the coverage report.

#### Metadata
* type: test-verification

#### Relations
* verify: [Another Leaf Requirement Verified](#another-leaf-requirement-verified)

---

### Formal Proof Verification Satisfied

This is a formal-proof verification that should appear as satisfied in the evidence-backed verification coverage report.

#### Metadata
* type: formal-proof-verification

#### Relations
* verify: [Leaf Requirement Verified By Formal Proof](#leaf-requirement-verified-by-formal-proof)
* satisfiedBy: [proof-satisfied.txt](proof-satisfied.txt)

---

### Formal Proof Verification Unsatisfied

This is a formal-proof verification that should appear as unsatisfied when no proof evidence is linked.

#### Metadata
* type: formal-proof-verification

#### Relations
* verify: [Leaf Requirement Formal Proof Unsatisfied](#leaf-requirement-formal-proof-unsatisfied)

---

### Analysis Verification Test

This is an analysis-type verification for testing verification type breakdown. Analysis verifications are considered satisfied by default. This one is orphaned (no verify relation).

#### Metadata
* type: analysis-verification

---

### Inspection Verification Test

This is an inspection-type verification for testing verification type breakdown. Inspection verifications are considered satisfied by default. This one is orphaned (no verify relation).

#### Metadata
* type: inspection-verification

---

### Demonstration Verification Test

This is a demonstration-type verification for testing verification type breakdown. Demonstration verifications are considered satisfied by default. This one is orphaned (no verify relation).

#### Metadata
* type: demonstration-verification

---

### Coverage Feature

Feature for coverage roll-up testing.

#### Metadata
* type: feature

#### Relations
* specifiedBy: [Parent Requirement](#parent-requirement)
* specifiedBy: [Another Leaf Requirement Verified](#another-leaf-requirement-verified)
* specifiedBy: [Leaf Requirement Verified By Formal Proof](#leaf-requirement-verified-by-formal-proof)
* specifiedBy: [Leaf Requirement Formal Proof Unsatisfied](#leaf-requirement-formal-proof-unsatisfied)

---

### Parent Requirement

This is a parent requirement that derives child requirements. It MAY be verified but it's not required.

#### Metadata
* type: requirement

#### Relations
* specify: [Coverage Feature](#coverage-feature)
* derive: [Leaf Requirement Verified](#leaf-requirement-verified)
* derive: [Leaf Requirement Unverified](#leaf-requirement-unverified)

---

### Leaf Requirement Verified

This is a leaf requirement (no forward relations) that should be verified. MUST be verified.

#### Relations
* derivedFrom: [Parent Requirement](#parent-requirement)
* verifiedBy: [Test Verification Satisfied](#test-verification-satisfied)

---

### Leaf Requirement Unverified

This is a leaf requirement that is NOT verified. Should be flagged as missing verification.

#### Relations
* derivedFrom: [Parent Requirement](#parent-requirement)

---

### Another Leaf Requirement Verified

This is another leaf requirement that is verified.

#### Metadata
* type: requirement

#### Relations
* specify: [Coverage Feature](#coverage-feature)
* verifiedBy: [Test Verification Unsatisfied](#test-verification-unsatisfied)

---

### Leaf Requirement Verified By Formal Proof

This leaf requirement is verified by a satisfied formal-proof verification.

#### Metadata
* type: requirement

#### Relations
* specify: [Coverage Feature](#coverage-feature)
* verifiedBy: [Formal Proof Verification Satisfied](#formal-proof-verification-satisfied)

---

### Leaf Requirement Formal Proof Unsatisfied

This leaf requirement is verified by a formal-proof verification that lacks proof evidence.

#### Metadata
* type: requirement

#### Relations
* specify: [Coverage Feature](#coverage-feature)
* verifiedBy: [Formal Proof Verification Unsatisfied](#formal-proof-verification-unsatisfied)

---
