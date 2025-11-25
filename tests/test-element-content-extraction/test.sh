#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Element content extraction
# --------------------------------------
# Acceptance Criteria:
# - System should properly extract Requirement body for change impact detection
# - Requirement body consists of normalized main text and content from '#### Details' subsection
# - Details subsections should include the '#### Details' header in the extracted content
#
# Test Criteria:
# - Command exits with success (0) return code
# - Output shows expected content for each element including Details headers
#

echo "Running: reqvire search --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" search --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

GOTTEN_CONTENT=$(echo "$OUTPUT" | jq -r '
  [
    .files
    | ..
    | objects
    | select(.content != null)
    | (
        (.name + ":" + (.content | gsub("\n+"; " ")))
        | gsub("(^\\s+)|(\\s+$)"; "")
        + "\n"
      )
  ] | sort | .[]
')

GOTTEN_CONTENT=$(printf "\n%s" "$GOTTEN_CONTENT")

# Write actual content for comparison
printf "%s" "$GOTTEN_CONTENT" > "${TEST_DIR}/actual-content.txt"

if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-content.txt" "${TEST_DIR}/actual-content.txt"; then
  echo "❌ FAILED: Extracted content does not match expected"
  echo ""
  echo "If changes are intentional, update the expected file:"
  echo "  cp ${TEST_DIR}/actual-content.txt ${TEST_SCRIPT_DIR}/expected/expected-content.txt"
  exit 1
fi

exit 0
