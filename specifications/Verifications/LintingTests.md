# Linting Tests

This document verifies the requirements for ReqFlow's linting functionality.

## Whitespace Linting Tests

### Excess Whitespace Detection and Correction

The verification test checks that ReqFlow correctly identifies and fixes excess whitespace after headers.

#### Metadata
  * type: verification

#### Acceptance Criteria
- System should detect excess whitespace after headers
- System should fix excess whitespace in linting mode
- The output should show before/after changes

#### Test Criteria
- Command exits with success (0) return code
- Output shows whitespace being fixed
- Output should contain diff-style formatting

#### Test Procedure
1. Create a test fixture in `/tests/fixtures/test-lint-headers/` with excess whitespace after headers
2. Run ReqFlow linting in dry-run mode to check detection
3. Verify that excess whitespace is reported
4. Run ReqFlow linting in normal mode to apply fixes
5. Verify that the whitespace has been corrected in the output files

#### Relations
  * verifies: [SystemRequirements/Requirements.md/Excess Whitespace Linting Implementation](../SystemRequirements/Requirements.html#excess-whitespace-linting-implementation)
  * verifies: [SystemRequirements/Requirements.md/Dry Run Mode Implementation](../SystemRequirements/Requirements.html#dry-run-mode-implementation)
  * trace: [tests/e2e-linting/test_whitespace_linting.sh](../../../tests/e2e-linting/test_whitespace_linting.sh)

## External Folders Tests

---

### External Folders Support Verification

The verification test checks that ReqFlow correctly processes requirements in external folders.

#### Metadata
  * type: verification

#### Acceptance Criteria
- System should process requirements in external folders
- System should treat external requirements as system requirements
- User Requirements in external folders should cause validation errors

#### Test Criteria
- Validation succeeds with proper external folder setup
- Validation fails when external folder contains user requirements
- Diagram generation includes external folder requirements

#### Test Procedure
1. Set up test fixtures in `/tests/fixtures/test-external-folders/` with external folders configuration
2. Run ReqFlow with invalid setup (user requirements in external folder)
3. Verify that validation reports the appropriate error
4. Run ReqFlow with valid setup (only system requirements in external folder)
5. Verify that validation succeeds and requirements are properly processed

#### Relations
  * verifies: [SystemRequirements/Requirements.md/External Folders Support](../SystemRequirements/Requirements.html#external-folders-support)
  * verifies: [SystemRequirements/Requirements.md/Unified System Requirements Processing](../SystemRequirements/Requirements.html#unified-system-requirements-processing)
  * trace: [tests/e2e-external-folders/test_external_folders.sh](../../../tests/e2e-external-folders/test_external_folders.sh)