#!/bin/bash

# Test: Validation of Missing Relation Targets
# -------------------------------------------
# Acceptance Criteria:
# - System should detect and report relations to non-existent elements or files
# - System should provide clear error messages identifying the missing targets
#
# Test Criteria:
# - Command exits with error (non-zero) return code
# - Error output contains specific error messages about missing relation targets
#

# Setup
TEST_NAME="Missing Relation Targets Validation"
TEST_DIR="../fixtures/test-missing-targets"
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

# Check for specific error message about missing relation target
if ! echo "$OUTPUT" | grep -q "Missing relation target"; then
  echo "FAILED: Missing error about non-existent relation target"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check for specific mention of the missing targets
if ! echo "$OUTPUT" | grep -q "MissingRequirement"; then
  echo "FAILED: Missing error about non-existent requirement 'MissingRequirement'"
  echo "Output: $OUTPUT"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "NonExistentFile.md"; then
  echo "FAILED: Missing error about non-existent file 'NonExistentFile.md'"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check for validation of markdown link format with incorrect target
if ! echo "$OUTPUT" | grep -q "DesignSpecifications/API2.md"; then
  echo "FAILED: Missing error about non-existent file 'DesignSpecifications/API2.md' in markdown link format"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "SUCCESS: All expected missing target validation errors were detected"
exit 0
