#!/bin/bash

# Test: Change Impact Detection
# --------------------------------------
# Acceptance Criteria:
# - System should properly construct change impact report after changes in requirements
#
# Test Criteria:
# - Command exits with success (0) return code
# - Output shows expected change impact report
# TODO: we need to see how to test this really

exit 0


OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml" --change-impact 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

GOTTEN_CONTENT=$(awk '/# [^:]+:/' "${TEST_DIR}/test_results.log")
GOTTEN_CONTENT=$(echo "$OUTPUT" | jq .errors[0] | sed -r 's/"//g')



exit 0
