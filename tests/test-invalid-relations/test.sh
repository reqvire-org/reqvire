#!/bin/bash

# Test: Validation of Invalid Relation Types and Formats
# ----------------------------------------------------
# Acceptance Criteria:
# - System should detect and report Duplicate element
# - System should detect and report Invalid metadata format
# - System should detect and report Invalid relation format
# - System should detect and report Unsupported relation type
# - System should detect and report Invalid identifier
# - System should detect and report Duplicate subsection
# - System should detect and report Incompatible element types for relation
# - System should detect and report Circular dependency error
# - System should detect and report Missing parent relation"
#
# Test Criteria:
# - Command exits with 0 error code but outputs expected validation errors
# - Error output contains specific error messages for each type of invalid relation
#



OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --validate --json 2>&1)
EXIT_CODE=$?


printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# Verify exit code indicates doesn't indicate error 
if ! [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Validation should have failed but returned success (0)"
  exit 1
fi

# Check for specific error messages
if [[ "$(echo "$OUTPUT" | jq -r '.errors[]' | sed -r 's/:.*//' | paste -sd,)" != "Duplicate element,Invalid metadata format,Invalid relation format,Unsupported relation type,Invalid identifier,Duplicate subsection,Incompatible element types for relation,Circular dependency error,Missing parent relation" ]]; then
  echo "❌ FAILED: Expected errors missing."
  exit 1
fi


exit 0
