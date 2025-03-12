#!/bin/bash

# Test: Excluded Patterns in Linting
# ------------------------------------------------------------
# Acceptance Criteria:
# - Files matching excluded_filename_patterns should not be linted
# - ReqFlow should skip linting checks on excluded files
#
# Test Criteria:
# - Command should not lint files matching excluded patterns
# - Linting output should not include issues from excluded files

# Setup
TEST_NAME="Excluded Patterns in Linting"
TEST_DIR="../fixtures/test-excluded-linting"
REQFLOW_BIN="${REQFLOW_BIN:-$(cd ../../ && pwd)/target/debug/reqflow}"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Note: Test fixtures are now permanently created in the fixtures directory

# Run linting in verbose mode
echo "Running linting command..."
OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" . --lint --verbose 2>&1)
EXIT_CODE=$?

# Verify results
echo "Checking results..."

# There should be no linting errors for the excluded file
if echo "$OUTPUT" | grep -q "ExcludedFile.md"; then
  echo "FAILED: Excluded file was incorrectly linted"
  echo ""
  echo "Output: $OUTPUT"
  exit 1
fi

# There should be no linting errors about extra spaces in the DSD-001 element
if echo "$OUTPUT" | grep -q "DSD-001"; then
  echo "FAILED: Elements from excluded files are being linted"
  echo ""
  echo "Output: $OUTPUT"
  exit 1
fi

echo "SUCCESS: Excluded patterns functionality works correctly in linting"
echo "- Files matching excluded patterns are not linted"
echo "- Issues in excluded files are not reported"
echo ""
echo "Output: $OUTPUT"

# Now exit with the actual test result (success or failure)
exit 0