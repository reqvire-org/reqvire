#!/bin/bash

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


OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --validate 2>&1)
EXIT_CODE=$?


printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# There should be no errors about excluded file relations being invalid
if echo "$OUTPUT" | grep -q "Missing relation target.*DesignSpecifications/ExcludedFile.md"; then
  echo "FAILED: Relation to excluded file was incorrectly reported as invalid"
  exit 1
fi

# There should also be no elements from excluded files in the registry
# Check this by looking for validation of relations FROM excluded files
# This is trickier to test without having access to the internal registry
# We'll use a heuristic: if the relation from DSD-001 is validated, it means 
# the element was processed, which is incorrect
if echo "$OUTPUT" | grep -q "DSD-001"; then
  echo "FAILED: Elements from excluded files are being processed"
  exit 1
fi

# Now exit with the actual test result (success or failure)
exit 0
