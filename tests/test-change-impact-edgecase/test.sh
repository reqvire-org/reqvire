#!/bin/bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Change Impact Detection for Details Changes
# --------------------------------------
# Acceptance Criteria:
# - When a requirement's Details section is modified, the change should be detected
# - The change impact tree should show verifiedBy relations
# - Invalidated Verifications section should list affected verifications
#
# Test Criteria:
# - Command exits with success (0) return code
# - Change impact report shows the changed requirement
# - The verifiedBy relation appears in the change impact tree
# - The verification appears in "Invalidated Verifications" section


#  Dummy Commit
cd "${TEST_DIR}"
echo "test" > test.txt
git add -A :/
git commit -m "dummy commit" > /dev/null 2>&1


# Now modify the Details section of Add Plugin requirement
sed -i 's/The plugin should be added to all nodes part of the deployment./The plugin should be added to all nodes part of the deployment including replicas./' "${TEST_DIR}/Requirements.md"

# Also modify the main content of Remove Plugin requirement
sed -i 's/The plugin should be removied from all nodes part of the deployment./The plugin should be removied from all nodes part of the deployment including replica./' "${TEST_DIR}/Requirements.md"

# Dummy Commit
cd "${TEST_DIR}"
git add -A :/
git commit -m "dummy commit" > /dev/null 2>&1

#  Dummy Commit
cd "${TEST_DIR}"
echo "test2" > test.txt
git add -A :/
git commit -m "dummy commit" > /dev/null 2>&1


# Test 1: Run change impact detection comparing HEAD to HEAD~1
echo "Running: reqvire change-impact --git-commit=HEAD~1" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact --git-commit=HEAD~2 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Write output to log file for debugging
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_default.log"

# Extract important parts (excluding log messages)
GOTTEN_CONTENT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element" | grep -v "DEBUG:")
SANITIZED_OUTPUT=$(echo "$GOTTEN_CONTENT" | sed -E 's#https://[^ )]+/blob/[a-f0-9]{7,40}/##g')

# Load expected content from file
EXPECTED_CONTENT=$(cat "${TEST_DIR}/expected_impact.md")

# Check exit code AFTER extracting output for better error messages
if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Change impact detection failed with exit code $EXIT_CODE"
    echo ""
    echo "Expected:"
    echo "$EXPECTED_CONTENT"
    echo ""
    echo "Got:"
    echo "$SANITIZED_OUTPUT"
    echo ""
    diff -u <(echo "$EXPECTED_CONTENT") <(echo "$SANITIZED_OUTPUT") || true
    rm -rf "${TEST_DIR}"
    exit 1
fi

# Test 1: Verify that change impact report shows verifiedBy relation
if ! diff <(echo "$EXPECTED_CONTENT") <(echo "$SANITIZED_OUTPUT") > /dev/null; then
  echo "❌ FAILED: Change impact report missing verifiedBy relation or invalidated verifications."
  echo ""
  echo "Expected:"
  echo "$EXPECTED_CONTENT"
  echo ""
  echo "Got:"
  echo "$SANITIZED_OUTPUT"
  echo ""
  diff -u <(echo "$EXPECTED_CONTENT") <(echo "$SANITIZED_OUTPUT") || true
  rm -rf "${TEST_DIR}"
  exit 1
fi


# Clean up
rm -rf "${TEST_DIR}"
exit 0
