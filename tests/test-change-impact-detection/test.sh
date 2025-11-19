#!/bin/bash

# Test: Circular dependency validation
# --------------------------------------
# Acceptance Criteria:
# - System should properly detect and report circular dependency during validation
#
# Test Criteria:
# - Command exits with success (0) return code
# - Output shows expected validation errors
#

OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml" --validate --json 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

GOTTEN_CONTENT=$(awk '/# [^:]+:/' "${TEST_DIR}/test_results.log")
GOTTEN_CONTENT=$(echo "$OUTPUT" | jq .errors[0] | sed -r 's/"//g')



if [[ "$GOTTEN_CONTENT" != *"Circular dependency detected:"* ]]; then
  echo "FAILED: Validation didn't find expected circular dependency."
  diff -u <(echo "$EXPECTED_CONTENT") <(echo "$GOTTEN_CONTENT")
  exit 1
fi

exit 0
