# Requirements

This document contains test verifications.

## Verifictions

### Search Command Verification

This test verifies that the search command supports all filter types and combines them with AND logic.

#### Details

##### Acceptance Criteria
- Search command accepts all documented filter flags
- Multiple filters combine with AND logic (results must match ALL filters)
- Invalid regex patterns produce clear error messages

##### Test Criteria
1. `reqvire search --json` returns valid JSON with all elements
2. `reqvire search --filter-type=user-requirement --filter-section="Requirements A"` returns only user-requirements in "Requirements A" section
3. Element count with two filters is <= element count with one filter (additive behavior)

#### Metadata
  * type: test-verification

#### Relations
  * verify: Requirements.md#requirement-with-valid-standard-relations
  * verify: Requirements.md#requirement-with-valid-markdown-relations
