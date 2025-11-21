#!/bin/bash
set -euo pipefail

# Test: Empty File Cleanup After Move
#
# Satisfies: specifications/Verifications/ElementManipulationTests.md#move-element-test
#
# Acceptance Criteria:
# - Moving the only element from a file should delete the empty source file
# - The element should be correctly moved to the target file
# - Model should validate after the operation
#
# Test Criteria:
# - Move command exits with code 0
# - Source file is deleted after move (because it becomes empty)
# - Target file contains the moved element
# - Model validates successfully

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "===================================="
echo "Empty File Cleanup After Move Test"
echo "===================================="
echo ""

# ==================================
# Test: Move only element and verify file deletion
# ==================================
echo "Test: Move only element from file..."

# Verify source file exists before move
if [ ! -f "$TEST_DIR/specifications/Source.md" ]; then
  echo "❌ FAILED: Source file does not exist before move"
  exit 1
fi

# Verify element exists in source
if ! grep -q "### Only Element" "$TEST_DIR/specifications/Source.md"; then
  echo "❌ FAILED: Element not found in source file before move"
  exit 1
fi

# Perform move operation (no section argument - sections were removed)
set +e
MOVE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Only Element" "specifications/Target.md" 2>&1)
MOVE_EXIT=$?
set -e

if [ $MOVE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Move command failed with exit code $MOVE_EXIT"
  echo "$MOVE_OUTPUT"
  exit 1
fi

echo "Move output:"
echo "$MOVE_OUTPUT"
echo ""

# Verify source file was deleted (since it's now empty)
if [ -f "$TEST_DIR/specifications/Source.md" ]; then
  echo "❌ FAILED: Empty source file was not deleted after move"
  echo "File contents:"
  cat "$TEST_DIR/specifications/Source.md"
  exit 1
fi

echo "✓ Empty source file was deleted"

# Verify element was added to target file
if ! grep -q "### Only Element" "$TEST_DIR/specifications/Target.md"; then
  echo "❌ FAILED: Element was not added to target file"
  echo "Target file contents:"
  cat "$TEST_DIR/specifications/Target.md"
  exit 1
fi

echo "✓ Element was moved to target file"

# Verify the element exists in target file
ELEMENT_LINE=$(grep -n "### Only Element" "$TEST_DIR/specifications/Target.md" | cut -d: -f1)

if [ -z "$ELEMENT_LINE" ]; then
  echo "❌ FAILED: Could not find element in target file"
  exit 1
fi

echo "✓ Element is correctly placed in target file"

# Verify model validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after move"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Model validates successfully"
echo ""
echo "✅ All tests passed - empty file cleanup works correctly"

exit 0
