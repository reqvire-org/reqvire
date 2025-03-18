#!/bin/bash

# Test: Validation of Valid Relation Targets
# -----------------------------------------
# Acceptance Criteria:
# - System should correctly validate relations to existing targets in both standard and markdown formats
# - System should not report errors for valid relations
#
# Test Criteria:
# - Command exits with success (zero) return code
# - No error output about missing relation targets
#
# Test Implementation for Verification:
# - See: specifications/Verifications/ValidationTests.md

# Setup
TEST_NAME="Valid Relation Targets Validation"
TEST_DIR="../fixtures/test-valid-relations"
REQFLOW_BIN="${REQFLOW_BIN:-$(cd ../../ && pwd)/target/debug/reqflow}"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Run validation
echo "Running validation command..."
OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --validate 2>&1)
EXIT_CODE=$?

# Verify results
echo "Checking results..."

# Verify exit code indicates success (0)
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Validation should have succeeded but returned error ($EXIT_CODE)"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "Exit code $EXIT_CODE indicates validation succeeded as expected"

# Check that output doesn't contain error messages about missing targets
if echo "$OUTPUT" | grep -q "Missing relation target"; then
  echo "FAILED: Found unexpected error about missing relation target"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify that successful validation message is displayed
if ! echo "$OUTPUT" | grep -q "Relations validation passed\|All validations passed"; then
  echo "FAILED: Missing success message for relation validation"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "SUCCESS: All valid relation targets were correctly validated"
exit 0
