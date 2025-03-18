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


OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --lint 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# Verify results
# There should be no linting errors for the excluded file
if echo "$OUTPUT" | grep -q "ExcludedFile.md"; then
  echo "FAILED: Excluded file was incorrectly linted"
  exit 1
fi

# There should be no linting errors about extra spaces in the DSD-001 element
if echo "$OUTPUT" | grep -q "DSD-001"; then
  echo "FAILED: Elements from excluded files are being linted"
  exit 1
fi


# Now exit with the actual test result (success or failure)
exit 0
