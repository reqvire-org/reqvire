# Test Requirements for Coverage Report

This document contains test requirements and verifications to validate the coverage report functionality.

## Verifications

### Test Verification Satisfied

This is a test verification that should appear as satisfied in the coverage report.

#### Metadata
* type: verification

#### Relations
* satisfiedBy: [test-satisfied.sh](test-satisfied.sh)

---

### Test Verification Unsatisfied

This is a test verification that should appear as unsatisfied in the coverage report.

#### Metadata
* type: verification

---

### Analysis Verification Test

This is an analysis-type verification for testing verification type breakdown.

#### Metadata
* type: analysis-verification

#### Relations
* satisfiedBy: [analysis-test.md](analysis-test.md)

---

## Requirements

### Sample Requirement

This is a sample requirement for testing.

#### Relations
* verifiedBy: [Test Verification Satisfied](#test-verification-satisfied)
* verifiedBy: [Test Verification Unsatisfied](#test-verification-unsatisfied)

---