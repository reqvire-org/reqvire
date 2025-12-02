#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test: Redundant Hierarchical Attachment Detection
# Satisfies: requirements/System/Operations/Verifications/LintingVerifications.md#redundant-hierarchical-attachment-test
#
# Acceptance Criteria:
# - System shall detect when a child element has the same attachment as an ancestor
# - System shall report which attachment is redundant and on which ancestor it exists
# - System shall categorize as auto-fixable
# - System shall remove the redundant attachment from the child when --fix is used
# - System shall preserve the attachment on the ancestor
#
# Test Criteria:
# - lint --fixable output matches expected format
# - lint --fix removes redundant attachment from child
# - After fix, no more issues (empty output)
# - File content matches expected output

# Helper function to compare outputs
assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# Test 1: Detect redundant attachment - compare output
cd "$TEST_DIR" && "$REQVIRE_BIN" lint --fixable > "${TEST_DIR}/actual-lint-output.txt" 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/lint-output.txt" \
  "${TEST_DIR}/actual-lint-output.txt" \
  "Lint output does not match expected"

# Test 2: Fix removes the attachment
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --fix 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: lint --fix returned error code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Test 3: After fix, no more issues (empty output)
cd "$TEST_DIR" && "$REQVIRE_BIN" lint --fixable > "${TEST_DIR}/actual-after-fix-output.txt" 2>&1

# After fix, output should be empty (no issues)
if [ -s "${TEST_DIR}/actual-after-fix-output.txt" ]; then
    echo "FAILED: Expected no issues after fix, but got:"
    cat "${TEST_DIR}/actual-after-fix-output.txt"
    exit 1
fi

# Test 4: Verify file content after fix
assert_file_matches "${TEST_SCRIPT_DIR}/expected/after-fix.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "Fixed file content does not match expected"

exit 0
