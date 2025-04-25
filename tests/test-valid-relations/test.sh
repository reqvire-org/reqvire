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


OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" --validate --json 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"


# Verify exit code indicates success (0)
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Validation should have succeeded but returned error ($EXIT_CODE)"
  echo "Output: $OUTPUT"
  exit 1
fi


# Verify that successful validation message is displayed
if ! echo "$OUTPUT" | grep -q "Validation completed successfully with no errors."; then
  echo "FAILED: Found unexpected errors"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "SUCCESS: All valid relation targets were correctly validated"
exit 0
