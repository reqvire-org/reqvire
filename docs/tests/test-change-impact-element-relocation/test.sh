#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Element Relocation Detection
# --------------------------------------
# Acceptance Criteria:
# - System correctly identifies element relocations (same Element ID, different file_path)
# - Relocated elements without content changes do not trigger impact propagation
# - Relocated elements WITH content/relation changes appear in BOTH Relocated and Changed sections
# - Relocated elements appear in a separate "Relocated" section in the report
# - Element IDs remain stable when elements are relocated between files
# - Relations are compared semantically by element name, not identifier
# - Relocated parent with new relation to relocated+changed child is detected correctly
#
# Test Criteria:
# - Command exits with success (0) return code
# - Relocated elements are reported with old location → new location format
# - Pure relocations do NOT appear in "Removed" + "Added" sections
# - Pure relocations do NOT appear in impact propagation tree
# - Relocated+changed elements appear in BOTH Relocated AND Changed sections
# - Parent element with added relation to relocated+changed child shows in Changed with impact tree
#
# Test Scenarios:
# 1. Battery Monitoring: Relocated + content changed (child)
# 2. Battery Saver: Relocated + NEW derive relation added to Battery Monitoring (parent)
# 3. Power Efficiency: Relocated + content changed
# 4. Display Settings: Relocated + relation added

# Test Scenario 1: Relocated parent + added relation to relocated+changed child
# First move Battery Monitoring (will be relocated + content changed)
BATTERY_MONITORING_CONTENT=$(sed -n '/^### Battery Monitoring$/,/^---$/p' "${TEST_DIR}/specifications/FirstFile.md")
sed -i '/^### Battery Monitoring$/,/^---$/d' "${TEST_DIR}/specifications/FirstFile.md"
echo "" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
echo "$BATTERY_MONITORING_CONTENT" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
# Change Battery Monitoring content
sed -i 's/monitor battery levels continuously/monitor battery levels in real-time and log events/' "${TEST_DIR}/specifications/SecondFile/Requirements.md"

# Now move Battery Saver and add derive relation to Battery Monitoring (relocated+changed)
BATTERY_SAVER_CONTENT=$(sed -n '/^### Battery Saver$/,/^---$/p' "${TEST_DIR}/specifications/FirstFile.md")
sed -i '/^### Battery Saver$/,/^---$/d' "${TEST_DIR}/specifications/FirstFile.md"
echo "" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
echo "$BATTERY_SAVER_CONTENT" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
# Add derive relation from Battery Saver to Battery Monitoring (after Metadata section, before the ---)
sed -i '/^#### Metadata/,/^---$/ { /^  \* type: user-requirement/a\
\
#### Relations\
  * derive: [Battery Monitoring](#battery-monitoring)
}' "${TEST_DIR}/specifications/SecondFile/Requirements.md"

# Test Scenario 2: Relocated + content changed
POWER_EFFICIENCY_CONTENT=$(sed -n '/^### Power Efficiency$/,/^---$/p' "${TEST_DIR}/specifications/FirstFile.md")
sed -i '/^### Power Efficiency$/,/^---$/d' "${TEST_DIR}/specifications/FirstFile.md"
echo "" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
echo "$POWER_EFFICIENCY_CONTENT" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
sed -i 's/optimize power consumption during idle periods/reduce power consumption during idle and active periods/' "${TEST_DIR}/specifications/SecondFile/Requirements.md"

# Test Scenario 3: Relocated + relation added
DISPLAY_SETTINGS_CONTENT=$(sed -n '/^### Display Settings$/,/^---$/p' "${TEST_DIR}/specifications/FirstFile.md")
sed -i '/^### Display Settings$/,/^---$/d' "${TEST_DIR}/specifications/FirstFile.md"
echo "" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
echo "$DISPLAY_SETTINGS_CONTENT" >> "${TEST_DIR}/specifications/SecondFile/Requirements.md"
sed -i '/satisfiedBy: display_impl.py/a\  * verifiedBy: display_test.py' "${TEST_DIR}/specifications/SecondFile/Requirements.md"

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
    exit 1
fi

# Extract only the important parts (excluding timestamp and path-specific lines)
GOTTEN_CONTENT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element")
SANITIZED_OUTPUT=$(echo "$GOTTEN_CONTENT" | sed -E 's#https://[^ )]+/blob/[a-f0-9]{7,40}/##g')

# Compare against expected output file
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-output.txt" <(echo "$SANITIZED_OUTPUT"); then
  echo "❌ FAILED: Output does not match expected content."
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-output.txt"
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
    exit 1
fi

# Verify JSON contains relocated elements
CLEAN_JSON=$(echo "$JSON_OUTPUT" | grep -v "Warning:" | grep -A 1000 "^{")
if ! echo "$CLEAN_JSON" | jq -e '.relocated | length > 0' >/dev/null 2>&1; then
    echo "❌ FAILED: JSON output should contain 'relocated' array with elements"
    echo "$CLEAN_JSON"
    exit 1
fi

# Verify relocated element has old and new locations
if ! echo "$CLEAN_JSON" | jq -e '.relocated[0] | has("old_location") and has("new_location")' >/dev/null 2>&1; then
    echo "❌ FAILED: Relocated element should have 'old_location' and 'new_location' fields"
    echo "$CLEAN_JSON" | jq '.relocated[0]'
    exit 1
fi

exit 0
