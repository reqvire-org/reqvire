# Test Requirements for Coverage Report

This document contains test requirements and verifications to validate the coverage report functionality.

## Verifications

### Test Verification Satisfied

This is a test verification that should appear as satisfied in the coverage report.

#### Metadata
* type: test-verification

#### Relations
* satisfiedBy: [test-satisfied.sh](test-satisfied.sh)

---

### Test Verification Unsatisfied

This is a test verification that should appear as unsatisfied in the coverage report.

#### Metadata
* type: test-verification

---

### Analysis Verification Test

This is an analysis-type verification for testing verification type breakdown. Analysis verifications are considered satisfied by default.

#### Metadata
* type: analysis-verification

---

### Inspection Verification Test

This is an inspection-type verification for testing verification type breakdown. Inspection verifications are considered satisfied by default.

#### Metadata
* type: inspection-verification

---

### Demonstration Verification Test

This is a demonstration-type verification for testing verification type breakdown. Demonstration verifications are considered satisfied by default.

#### Metadata
* type: demonstration-verification

---

## Requirements

### Parent Requirement

This is a parent requirement that derives child requirements. It MAY be verified but it's not required.

#### Relations
* derive: [Leaf Requirement Verified](#leaf-requirement-verified)
* derive: [Leaf Requirement Unverified](#leaf-requirement-unverified)

---

### Leaf Requirement Verified

This is a leaf requirement (no forward relations) that should be verified. MUST be verified.

#### Relations
* derivedFrom: [Parent Requirement](#parent-requirement)
* verifiedBy: [Test Verification Satisfied](#test-verification-satisfied)
* verifiedBy: [Analysis Verification Test](#analysis-verification-test)

---

### Leaf Requirement Unverified

This is a leaf requirement that is NOT verified. Should be flagged as missing verification.

#### Relations
* derivedFrom: [Parent Requirement](#parent-requirement)

---

### Another Leaf Requirement Verified

This is another leaf requirement that is verified by multiple verification types.

#### Relations
* verifiedBy: [Test Verification Unsatisfied](#test-verification-unsatisfied)
* verifiedBy: [Inspection Verification Test](#inspection-verification-test)
* verifiedBy: [Demonstration Verification Test](#demonstration-verification-test)

---