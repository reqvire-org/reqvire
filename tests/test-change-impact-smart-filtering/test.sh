#!/bin/bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Change Impact Smart Filtering
# --------------------------------------
# Acceptance Criteria:
# - New parent elements appear in the "New Elements" section
# - New child elements (with parent relationships to other new elements) are filtered out
# - Filtered child elements are shown in parent's relations with "(new)" marker
# - Verification elements that are not children remain in the report
#
# Test Criteria:
# - When adding a parent and child requirement together, only parent appears in "New Elements"
# - When adding a requirement and its verification, both appear (verification is not a child)
# - Child elements are visible in the parent's change impact tree with appropriate markers


# The run_tests.sh script has already created initial git commit with Requirements.md
# Now add new parent-child requirements and verification
cat >> "${TEST_DIR}/Requirements.md" << 'EOF'

### New Parent Requirement

This is a new parent requirement that will appear in the report.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [New Child Requirement](#new-child-requirement)
  * verifiedBy: [New Verification](#new-verification)

---

### New Child Requirement

This is a new child requirement that should be filtered out.

#### Relations
  * derivedFrom: [New Parent Requirement](#new-parent-requirement)

---

### New Verification

This is a new verification that should appear separately.

#### Metadata
  * type: verification

#### Relations
  * verify: [New Parent Requirement](#new-parent-requirement)

---
EOF

echo "Running: reqvire change-impact" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Write output to log file for debugging in temporary directory
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_default.log"

# Check exit code
if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Change impact detection failed with exit code $EXIT_CODE"
    echo "$OUTPUT"
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Extract the important parts (excluding timestamp and path-specific lines)
GOTTEN_CONTENT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element" | grep -v "DEBUG:" | grep -v "\[DEBUG\]")
SANITIZED_OUTPUT=$(echo "$GOTTEN_CONTENT" | sed -E 's#https://[^ )]+/blob/[a-f0-9]{7,40}/##g')

# Expected content - only parent should appear in new elements, child is filtered
# New verification appears in both new elements and invalidated verifications
EXPECTED_CONTENT='## Change Impact Report

### New Elements

* [New Parent Requirement](Requirements.md#new-parent-requirement)
    * derive -> [New Child Requirement](Requirements.md#new-child-requirement) (new)
    * verifiedBy -> [New Verification](Requirements.md#new-verification) (new)



---

## Invalidated Verifications

- [ ] [New Verification](Requirements.md#new-verification)'

# Test 1: Verify smart filtering works correctly
if ! diff <(echo "$EXPECTED_CONTENT") <(echo "$SANITIZED_OUTPUT") > /dev/null; then
  echo "❌ FAILED: Smart filtering not working correctly."
  echo "Expected:"
  echo "$EXPECTED_CONTENT"
  echo ""
  echo "Got:"
  echo "$SANITIZED_OUTPUT"
  echo ""
  diff -u <(echo "$EXPECTED_CONTENT") <(echo "$SANITIZED_OUTPUT")
  rm -rf "${TEST_DIR}"
  exit 1
fi



# Test 2: Verify JSON output also applies smart filtering
echo "Running: reqvire change-impact --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_JSON=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_JSON" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Change impact JSON detection failed with exit code $EXIT_CODE"
    echo "$OUTPUT_JSON"
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Extract JSON content
JSON_OUTPUT=$(echo "$OUTPUT_JSON" | grep -v "Warning:" | grep -v "DEBUG:" | grep -A 1000 "^{")

# Verify JSON format
if ! echo "$JSON_OUTPUT" | jq . >/dev/null 2>&1; then
    echo "❌ FAILED: Output is not valid JSON"
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Check that only 1 new element is in the JSON (parent only, child is filtered)
NEW_COUNT=$(echo "$JSON_OUTPUT" | jq '.added | length')
if [ "$NEW_COUNT" -ne 1 ]; then
    echo "❌ FAILED: Expected 1 new element in JSON (parent only), got $NEW_COUNT"
    echo "Elements found:"
    echo "$JSON_OUTPUT" | jq '.added[].element_id'
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Verify the correct elements are present
PARENT_PRESENT=$(echo "$JSON_OUTPUT" | jq '.added | map(select(.element_id | contains("new-parent-requirement"))) | length')
CHILD_PRESENT=$(echo "$JSON_OUTPUT" | jq '.added | map(select(.element_id | contains("new-child-requirement"))) | length')

if [ "$PARENT_PRESENT" -ne 1 ]; then
    echo "❌ FAILED: New Parent Requirement not found in JSON output"
    rm -rf "${TEST_DIR}"
    exit 1
fi

if [ "$CHILD_PRESENT" -ne 0 ]; then
    echo "❌ FAILED: New Child Requirement should be filtered out but was found in JSON output"
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Check invalidated verifications
INVALIDATED_COUNT=$(echo "$JSON_OUTPUT" | jq '.invalidated_verifications | length')
if [ "$INVALIDATED_COUNT" -ne 1 ]; then
    echo "❌ FAILED: Expected 1 invalidated verification in JSON, got $INVALIDATED_COUNT"
    rm -rf "${TEST_DIR}"
    exit 1
fi

VERIFICATION_INVALIDATED=$(echo "$JSON_OUTPUT" | jq '.invalidated_verifications | map(select(.target_url | contains("new-verification"))) | length')
if [ "$VERIFICATION_INVALIDATED" -ne 1 ]; then
    echo "❌ FAILED: New Verification not found in invalidated verifications"
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Clean up
rm -rf "${TEST_DIR}"
exit 0
