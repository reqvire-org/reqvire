# Verification Tests

This document contains various test cases.

## Test Categories
Some introduction text here.

### Format Test
This test verifies formatting capabilities.

#### Metadata
  * type: verification

#### Relations
  * verify: ../SystemRequirements/Requirements.md#requirements-processing
  * satisfiedBy: ../tests/test-format/test.sh

### Validation Test

This test verifies validation capabilities.

#### Metadata
  * type: verification

#### Relations
  * verify: ../SystemRequirements/Requirements.md#validation-framework

---

### Absolute Path Verification

This verification uses absolute paths to rs files.

#### Metadata
  * type: verification

#### Relations
  * verify: [../SystemRequirements/Requirements.md#path-resolution-testing](/SystemRequirements/Requirements.md#path-resolution-testing)
  * satisfiedBy: /core/src/element.rs
