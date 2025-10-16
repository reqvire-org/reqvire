#!/bin/bash

# Test: Change Impact Detection with Multi-File Structure (HEAD~2 comparison)
# Acceptance Criteria:
# - When requirements in one file reference verifications in another file
# - And those requirements are modified in Details section
# - Then change impact should show verifiedBy relations and invalidated verifications
#
# Test Criteria:
# - Command exits with success (0)
# - Changed elements show verifiedBy relations to verification elements
# - Invalidated Verifications section lists affected verifications

# Initialize git repository
git init >/dev/null 2>&1
git config user.email "test@example.com"
git config user.name "Test User"

# Commit initial version
git add . >/dev/null 2>&1
git commit -m "Initial version" >/dev/null 2>&1

# Intermediate commit (to establish HEAD~2)
echo "# Documentation" > "${TEST_DIR}/README.md"
git add . >/dev/null 2>&1
git commit -m "Add docs" >/dev/null 2>&1

# Now modify the Details section of Add Plugin requirement and Remove Plugins
sed -i 's/The plugin should be added to all nodes part of the deployment\./The plugin should be added to all nodes part of the deployment including replicas./' "${TEST_DIR}/Requirements.md"
sed -i 's/The plugin should be removied from all nodes part of the deployment\./The plugin should be removied from all nodes part of the deployment including replica./' "${TEST_DIR}/Requirements.md"

# Commit the changes
git add "${TEST_DIR}/Requirements.md" >/dev/null 2>&1
git commit -m "Update plugin requirements" >/dev/null 2>&1

# Run change-impact comparing HEAD to HEAD~2
OUTPUT=$("$REQVIRE_BIN" change-impact --git-commit=HEAD~2 2>&1)
EXIT_CODE=$?

# Save output for debugging
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/debug_output.log"

# Extract just the report (after any debug/info messages)
REPORT=$(echo "$OUTPUT" | sed -n '/^## Change Impact Report/,$p')

# Save the report for comparison
printf "%s\n" "$REPORT" > "${TEST_DIR}/actual_output.md"

# Check exit code
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Command returned error: $EXIT_CODE"
  cat "${TEST_DIR}/debug_output.log"
  exit 1
fi

# Compare expected vs actual
if ! diff -w "${TEST_DIR}/expected_impact.md" "${TEST_DIR}/actual_output.md" > "${TEST_DIR}/diff.txt" 2>&1; then
  echo "❌ FAILED: Change impact report missing verifiedBy relation or invalidated verifications."
  echo ""
  echo "Expected:"
  cat "${TEST_DIR}/expected_impact.md"
  echo ""
  echo "Got:"
  cat "${TEST_DIR}/actual_output.md"
  echo ""
  diff -u <(cat "${TEST_DIR}/expected_impact.md") <(cat "${TEST_DIR}/actual_output.md")
  exit 1
fi

exit 0
