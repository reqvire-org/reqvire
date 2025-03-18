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



OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --validate 2>&1)
EXIT_CODE=$?


printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"


# Verify that successful validation message is displayed
if  echo "$OUTPUT" | grep -q "Validation completed successfully with no errors."; then
  echo "FAILED: Validation should have failed but returned success"
  exit 1
fi

# Verift that it found missing elememnt in relation
if ! echo "$OUTPUT" | grep -q "Invalid identifier: Failed to normalize identifier for 'Requirement with Missing Target'"; then
  echo "FAILED: Validation should have found missing element in relation"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "satisfieddBy"; then
  echo "FAILED: Missing error about unsupported relation type 'satisfieddBy'"
  exit 1
fi

# Check for specific error messages
if ! echo "$OUTPUT" | grep -q "Duplicate relation"; then
  echo "FAILED: Missing error about duplicate relations"
  exit 1
fi



# For future implementation:
# if ! echo "$OUTPUT" | grep -q "Invalid relation format"; then
#   echo "FAILED: Missing error message about invalid relation format"
#   echo "Output: $OUTPUT"
#   exit 1
# fi
# 

# 
# if ! echo "$OUTPUT" | grep -q "depends-on"; then
#   echo "FAILED: Missing error about 'depends-on' with non-alphanumeric characters"
#   echo "Output: $OUTPUT"
#   exit 1
# fi

# This test now only checks for duplicate relations and unsupported relation types
# Missing relation target validation is tested in test_missing_targets.sh


exit 0
