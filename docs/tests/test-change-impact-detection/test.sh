#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Change Impact Detection
# --------------------------------------
# Acceptance Criteria:
# - System should properly construct change impact report after changes in requirements
# - System should show new requirements correctly in change impact report
# - Smart filtering should only show top-level new elements (not children)
#
# Test Criteria:
# - Command exits with success (0) return code 
# - Change impact report shows correct relationships between elements
# - Default commit is HEAD when --git-commit is not provided
# - New parent requirements appear in "New Elements" section
# - New child requirements are filtered out but shown in parent's relations
# - New verifications appear as separate elements

# Modify requirements after commit
sed -i 's/The systsem shall activate power-saving mode when the battery level drops below 20%./The systsem shall activate power-saving mode when the battery level drops below 30%./g' "${TEST_DIR}/Requirements.md"

sed -i 's/Power saving./Power saving.../g' "${TEST_DIR}/Requirements.md"



# Test 1: Run change impact detection with default commit (HEAD)
echo "Running: reqvire change-impact" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Write output to log file for debugging in temporary directory
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_default.log"


# Check exit code
if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Change impact detection with default commit failed with exit code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi


# Test 0b: Check that at least one blob URL is present in raw output
if ! echo "$OUTPUT" | grep -qE 'https://[^ )]+/blob/[a-f0-9]{7,40}/'; then
    echo "❌ FAILED: Expected at least one blob URL (GitHub-style) in the report, but none was found."
    exit 1
fi
# Extract only the important parts (excluding timestamp and path-specific lines)
GOTTEN_CONTENT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element")
SANITIZED_OUTPUT=$(echo "$GOTTEN_CONTENT" | sed -E 's#https://[^ )]+/blob/[a-f0-9]{7,40}/##g')

# Test 1: Verify that change impact report shows correct relationships between elements
# Note: "Power Saving" is filtered out from standalone changed elements by enhanced smart filtering
# since it appears in the change impact tree of "Power Saving Mode" with ⚠️ symbol
if ! diff -u "${TEST_SCRIPT_DIR}/expected/change-impact-report.txt" <(echo "$SANITIZED_OUTPUT"); then
  echo "❌ FAILED: Extracted content not matching expected content."
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/change-impact-report.txt"
  exit 1
fi

# Test 2: Verify that change impact detection works with specified commit
# Use HEAD as the explicit commit
echo "Running: reqvire change-impact --git-commit HEAD" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact --git-commit HEAD 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Write output to log file for debugging in temporary directory
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_explicit.log"

# Check exit code
if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Change impact detection with explicit commit failed with exit code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Test 3: Verify JSON output format for change impact detection
echo "Running: reqvire change-impact --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Write output to log file for debugging in temporary directory
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_json.log"

# Check exit code
if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Change impact detection with JSON output failed with exit code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Skip warnings and find the actual JSON content
JSON_OUTPUT=$(echo "$OUTPUT" | grep -v "Warning:" | grep -A 1000 "^{")
printf "%s\n" "$JSON_OUTPUT" > "${TEST_DIR}/test_results_json_clean.log"

# Verify JSON format by testing with jq
if ! echo "$JSON_OUTPUT" | jq . >/dev/null 2>&1; then
    echo "❌ FAILED: Output is not valid JSON"
    exit 1
fi

exit 0
