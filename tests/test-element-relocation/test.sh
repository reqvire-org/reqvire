#!/bin/bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Element Relocation Detection
# --------------------------------------
# Acceptance Criteria:
# - System correctly identifies element relocations (same Element ID, different file_path)
# - Relocated elements without content changes do not trigger impact propagation
# - Relocated elements appear in a separate "Relocated" section in the report
# - Element IDs remain stable when elements are relocated between files
#
# Test Criteria:
# - Command exits with success (0) return code
# - Relocated elements are reported with old location → new location format
# - Pure relocations do NOT appear in "Removed" + "Added" sections
# - Pure relocations do NOT appear in impact propagation tree
# - Summary statistics include count of relocated elements

# Test Scenario: Move "Battery Saver" requirement from FirstFile.md to SecondFile/Requirements.md
# without changing its content

# Initial state is already committed (FirstFile.md contains "Battery Saver")

# Move the element to a different file (relocation)
BATTERY_SAVER_CONTENT=$(sed -n '/^### Battery Saver$/,/^---$/p' "${TEST_DIR}/specifications/FirstFile.md")

# Remove from original file
sed -i '/^### Battery Saver$/,/^---$/d' "${TEST_DIR}/specifications/FirstFile.md"

# Add to new file
echo "" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
echo "$BATTERY_SAVER_CONTENT" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"

# Run change impact detection
echo "Running: reqvire change-impact" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Save output for debugging
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_relocation.log"

# Check exit code
if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Change impact detection failed with exit code $EXIT_CODE"
    echo "$OUTPUT"
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Extract only the important parts (excluding timestamp and path-specific lines)
GOTTEN_CONTENT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element")
SANITIZED_OUTPUT=$(echo "$GOTTEN_CONTENT" | sed -E 's#https://[^ )]+/blob/[a-f0-9]{7,40}/##g')

# Compare against expected output file
if ! diff "${TEST_DIR}/expected-output.txt" <(echo "$SANITIZED_OUTPUT") > /dev/null; then
  echo "❌ FAILED: Output does not match expected content."
  echo ""
  echo "DIFF (expected vs actual):"
  diff -u "${TEST_DIR}/expected-output.txt" <(echo "$SANITIZED_OUTPUT") || true
  echo ""
  echo "FULL OUTPUT:"
  echo "$OUTPUT"
  rm -rf "${TEST_DIR}"
  exit 1
fi

# Test 7: Test with JSON output
echo "Running: reqvire change-impact --json" >> "${TEST_DIR}/test_results.log"
set +e
JSON_OUTPUT=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact --json 2>&1)
EXIT_CODE_JSON=$?
set -e

if [ $EXIT_CODE_JSON -ne 0 ]; then
    echo "❌ FAILED: Change impact JSON output failed with exit code $EXIT_CODE_JSON"
    echo "$JSON_OUTPUT"
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Verify JSON contains relocated elements
CLEAN_JSON=$(echo "$JSON_OUTPUT" | grep -v "Warning:" | grep -A 1000 "^{")
if ! echo "$CLEAN_JSON" | jq -e '.relocated | length > 0' >/dev/null 2>&1; then
    echo "❌ FAILED: JSON output should contain 'relocated' array with elements"
    echo "$CLEAN_JSON"
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Verify relocated element has old and new locations
if ! echo "$CLEAN_JSON" | jq -e '.relocated[0] | has("old_location") and has("new_location")' >/dev/null 2>&1; then
    echo "❌ FAILED: Relocated element should have 'old_location' and 'new_location' fields"
    echo "$CLEAN_JSON" | jq '.relocated[0]'
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Clean up
rm -rf "${TEST_DIR}"
exit 0
