#!/bin/bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Validation of Same-File Fragment Relations
# -----------------------------------------------
# Acceptance Criteria:
# - System should correctly validate relations to fragments within the same file
# - System should not report errors for valid fragment references
#
# Test Criteria:
# - Command exits with success (zero) return code
# - No error output about missing relation targets when using #fragment references
#
# Test Implementation for Verification:
# - Tests requirements with fragment-only references like "#fragment-id"
# - Tests fragments referenced by proper element ID

echo "Running: reqvire model-summary --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model-summary --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Verify exit code indicates success (0)
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Validation should have succeeded but returned error ($EXIT_CODE)"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify that JSON output is valid (indicates successful validation)
if ! echo "$OUTPUT" | jq . > /dev/null 2>&1; then
  echo "FAILED: Invalid JSON output, validation likely failed"
  echo "Output: $OUTPUT"
  exit 1
fi

exit 0
