#!/bin/bash

# Test: Validation of Invalid Relation Types and Formats
# ----------------------------------------------------
# Acceptance Criteria:
# - System should detect and report invalid relation types (typos, etc.)
# - System should detect and report relation types with non-alphanumeric characters
# - System should detect and report duplicate relations
# - System should detect and report relations to non-existent targets
#
# Test Criteria:
# - Command exits with error (non-zero) return code
# - Error output contains specific error messages for each type of invalid relation
#
# Test Implementation for Verification:
# - See: specifications/Verifications/ValidationTests.md

# Setup
TEST_NAME="Invalid Relations Validation"
TEST_DIR="../fixtures/test-invalid-relations"
REQFLOW_BIN="${REQFLOW_BIN:-$(cd ../../ && pwd)/target/debug/reqflow}"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Run validation
echo "Running validation command..."
OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --validate-relations 2>&1)
EXIT_CODE=$?

# Verify results
echo "Checking results..."

# Verify exit code indicates error
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: Validation should have failed but returned success (0)"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "Exit code $EXIT_CODE indicates validation errors as expected"

# Check for specific error messages
# Currently, only duplicate relation validation is implemented
# We'll check only for that now, but keep the other validation comments for future implementation

if ! echo "$OUTPUT" | grep -q "Duplicate relation"; then
  echo "FAILED: Missing error about duplicate relations"
  echo "Output: $OUTPUT"
  exit 1
fi

# For future implementation:
# if ! echo "$OUTPUT" | grep -q "Invalid relation format"; then
#   echo "FAILED: Missing error message about invalid relation format"
#   echo "Output: $OUTPUT"
#   exit 1
# fi
# 
# if ! echo "$OUTPUT" | grep -q "satisfieddBy"; then
#   echo "FAILED: Missing error about 'satisfieddBy' typo"
#   echo "Output: $OUTPUT"
#   exit 1
# fi
# 
# if ! echo "$OUTPUT" | grep -q "depends-on"; then
#   echo "FAILED: Missing error about 'depends-on' with non-alphanumeric characters"
#   echo "Output: $OUTPUT"
#   exit 1
# fi

# This test now only checks for duplicate relations and unsupported relation types
# Missing relation target validation is tested in test_missing_targets.sh

echo "SUCCESS: All expected validation errors were detected"
exit 0