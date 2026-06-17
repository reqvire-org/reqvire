# Elements


### Format Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
This document contains various test cases.

### Format Test
This test verifies formatting capabilities.

#### Metadata
  * type: verification

#### Relations
  * derivedFrom: [Format Verification Objective](#format-verification-objective)
  * verify: ../SystemRequirements/Requirements.md#requirements-processing
  * satisfiedBy: ../tests/test-format/test.sh

### Validation Test

This test verifies validation capabilities.

#### Metadata
  * type: verification

#### Relations
  * derivedFrom: [Format Verification Objective](#format-verification-objective)
  * verify: ../SystemRequirements/Requirements.md#validation-framework

---

### Absolute Path Verification

This verification uses absolute paths to rs files.

#### Metadata
  * type: verification

#### Relations
  * derivedFrom: [Format Verification Objective](#format-verification-objective)
  * verify: [../SystemRequirements/Requirements.md#path-resolution-testing](/SystemRequirements/Requirements.md#path-resolution-testing)
  * satisfiedBy: /core/src/element.rs
