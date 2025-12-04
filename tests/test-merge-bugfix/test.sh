#!/bin/bash
set -uo pipefail

# Test: Merge Elements Verification
#
# This test verifies the merge command works correctly:
#
# Expected behavior:
# - Target element survives with merged content from source element
# - Source element is deleted
# - Relations pointing to source element redirect to target element
# - Diff output shows the changes (note: line-based diffs show moved content as context)

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Helper function to compare files and show diff on failure
assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "❌ FAILED: $description"
    echo ""
    echo "Expected file: $expected"
    echo "Actual file: $actual"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

echo "========================================="
echo "Test: Merge command bug reproduction"
echo "========================================="
echo ""

# Perform the merge and capture diff output
echo "Running merge command: CLI Interface Structure <- CLI Search Command"
MERGE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "CLI Interface Structure" "CLI Search Command" 2>&1)
MERGE_EXIT=$?

if [ $MERGE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Merge command failed with exit code $MERGE_EXIT"
  echo "Error output:"
  echo "$MERGE_OUTPUT"
  exit 1
fi

# Save merge output and strip ANSI color codes for comparison
echo "$MERGE_OUTPUT" | sed 's/\x1b\[[0-9;]*m//g' > "$TEST_DIR/actual-merge-diff.txt"

echo "✓ Merge command completed successfully"
echo ""

# Check that diff output matches expected diff
# Note: Line-based diff algorithms show moved content as "context" rather than "additions"
# when the same lines appear in both before/after states. This is expected behavior.
# The merge operation itself works correctly (verified by comparing final file state).
echo "Checking that diff output matches expected format..."
if ! diff -u "${TEST_SCRIPT_DIR}/expected/merge-diff.txt" "$TEST_DIR/actual-merge-diff.txt"; then
  echo ""
  echo "❌ FAILED: Diff output doesn't match expected"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/merge-diff.txt"
  exit 1
fi

echo "✓ Diff output matches expected"
echo ""

# Check that CLI Interface Structure still exists
echo "Checking that CLI Interface Structure survived the merge..."
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="CLI Interface Structure" --short 2>&1)

if ! echo "$SEARCH_OUTPUT" | grep -q "CLI Interface Structure"; then
  echo "❌ FAILED: CLI Interface Structure was deleted (BUG CONFIRMED)"
  echo "Search output: $SEARCH_OUTPUT"
  echo ""
  echo "File contents after merge:"
  cat "$TEST_DIR/specifications/CLI.md"
  exit 1
fi

echo "✓ CLI Interface Structure exists"
echo ""

# Check that CLI Search Command was deleted
echo "Checking that CLI Search Command was deleted..."
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="CLI Search Command" --short 2>&1)

if echo "$SEARCH_OUTPUT" | grep -q "CLI Search Command"; then
  echo "❌ FAILED: CLI Search Command should have been deleted"
  exit 1
fi

echo "✓ CLI Search Command was deleted"
echo ""

# Check merged content
echo "Checking that CLI Interface Structure received merged content..."
assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-merge.md" \
  "$TEST_DIR/specifications/CLI.md" \
  "Merged file content does not match expected"

echo "✓ Merged content matches expected"
echo ""

# Verify model still validates
echo "Verifying model validity..."
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model should validate after merge"
  echo "Validation output:"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Model validates successfully"
echo ""

echo "========================================="
echo "✅ All tests PASSED"
echo "========================================="
exit 0
