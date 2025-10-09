#!/bin/bash

# Test: Trace Relations Do Not Cause Circular Dependencies
# --------------------------------------------------------
# Acceptance Criteria:
# - System should not trigger circular dependency errors for trace relations
# - Trace relations being non-directional should form cycles without validation errors
#
# Test Criteria:
# - Command exits with success (zero) return code
# - No circular dependency errors in output
# - Model processes successfully with trace cycles
#
# Test Implementation for Verification:
# - Tests requirements with trace relations forming cycles (Alpha->Beta->Gamma->Alpha)
# - Verifies the fix for bug where trace relations incorrectly triggered circular dependency detection

echo "Starting test..." > "${TEST_DIR}/test_results.log"

echo "Running: reqvire model-summary --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model-summary --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Test should succeed - trace cycles should not cause validation failure
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Validation failed when it should have succeeded"
  echo "Trace relations should not cause circular dependency errors"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify that JSON output is valid (indicates successful validation)
if ! echo "$OUTPUT" | jq . > /dev/null 2>&1; then
  echo "FAILED: Invalid JSON output, validation likely failed"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify no circular dependency errors in output
if echo "$OUTPUT" | grep -q "Circular dependency"; then
  echo "FAILED: Circular dependency error reported for trace relations"
  echo "Trace relations should not trigger circular dependency detection"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify expected number of requirements were processed using jq
# We expect 8 requirements total (all user requirements due to test location)
TOTAL_USER_REQS=$(echo "$OUTPUT" | jq '.global_counters.total_requirements_user')
TOTAL_ELEMENTS=$(echo "$OUTPUT" | jq '.global_counters.total_elements')

if [ "$TOTAL_ELEMENTS" != "8" ]; then
  echo "FAILED: Expected 8 total elements, found $TOTAL_ELEMENTS"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify we have the expected trace cycle elements by checking specific requirements exist
ALPHA_EXISTS=$(echo "$OUTPUT" | jq '.files."specifications/Requirements.md".sections.Requirements.elements[] | select(.name == "Requirement Alpha")')
BETA_EXISTS=$(echo "$OUTPUT" | jq '.files."specifications/Requirements.md".sections.Requirements.elements[] | select(.name == "Requirement Beta")')
GAMMA_EXISTS=$(echo "$OUTPUT" | jq '.files."specifications/Requirements.md".sections.Requirements.elements[] | select(.name == "Requirement Gamma")')

if [ -z "$ALPHA_EXISTS" ]; then
  echo "FAILED: Requirement Alpha not found in output"
  exit 1
fi

if [ -z "$BETA_EXISTS" ]; then
  echo "FAILED: Requirement Beta not found in output"
  exit 1
fi

if [ -z "$GAMMA_EXISTS" ]; then
  echo "FAILED: Requirement Gamma not found in output"
  exit 1
fi

exit 0