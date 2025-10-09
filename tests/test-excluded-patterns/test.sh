#!/bin/bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Excluded Patterns Functionality
# ------------------------------------------------------------
# Acceptance Criteria:
# - Files matching excluded_filename_patterns should not be processed for validation
# - Files matching excluded_filename_patterns should only be tracked for relation targets
# - Only relations TO excluded files should be valid, not relations FROM excluded files
#
# Test Criteria:
# - Command should not validate elements within files matching excluded patterns
# - Elements in excluded files should not be in registry for direct access
# - Only the file itself should be in the registry for relation validation


echo "Running: reqvire model-summary" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model-summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [[ $EXIT_CODE -ne 0 ]]; then
    exit $EXIT_CODE
fi


# There should be no errors about excluded file relations being invalid
if echo "$OUTPUT" | grep -q "Missing relation target"; then
  echo "FAILED: Relation to excluded file was incorrectly reported as invalid"
  exit 1
fi

# There should also be no elements from excluded files in the registry
echo "Running: reqvire model-summary (2nd check)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model-summary 2>&1 | grep -n 'Element:')
set -e

if echo "$OUTPUT" | grep -q "DesignSpecifications"; then
  echo "FAILED: Elements from excluded files are being processed"
  exit 1
fi

# Now exit with the actual test result (success or failure)
exit 0
