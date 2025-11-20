#!/bin/bash
set -euo pipefail

# Test: Move File with Squash Flag
#
# Satisfies: specifications/Verifications/ElementManipulationTests.md#move-file-squash-test
#
# Acceptance Criteria:
# - Without --squash: moving to existing file should error
# - With --squash: all elements from source moved to target's first section
# - Source file is deleted after squash
# - Target file's existing elements remain unchanged
# - All relations are updated to reference new file location
# - Model validates after the operation
#
# Test Criteria:
# - mv-file command without --squash fails when target exists
# - mv-file --squash command exits with code 0
# - Source file is deleted after squash
# - Target file contains all elements from both files
# - Elements from source are added to first section of target
# - Existing target elements remain in their original positions
# - Relations in other files are updated
# - Model validates successfully

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "===================================="
echo "Move File with Squash Flag Test"
echo "===================================="
echo ""

# ==================================
# Test 1: Error when target exists without --squash
# ==================================
echo "Test 1: Move to existing file without --squash should fail..."

set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-file "specifications/Source.md" "specifications/Target.md" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Command should have failed when target exists without --squash"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -q -i "already exists"; then
  echo "❌ FAILED: Error message should mention target already exists"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "✓ Correctly fails when target exists without --squash"
echo ""

# ==================================
# Test 2: Squash move to existing file
# ==================================
echo "Test 2: Move file with --squash to existing target..."

# Verify files exist before move
if [ ! -f "$TEST_DIR/specifications/Source.md" ]; then
  echo "❌ FAILED: Source file does not exist before move"
  exit 1
fi

if [ ! -f "$TEST_DIR/specifications/Target.md" ]; then
  echo "❌ FAILED: Target file does not exist before move"
  exit 1
fi

# Count elements in target before
TARGET_BEFORE=$(grep -c "^### " "$TEST_DIR/specifications/Target.md" || true)

# Perform squash move operation
set +e
MOVE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-file --squash "specifications/Source.md" "specifications/Target.md" 2>&1)
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

# ==================================
# Test 3: Verify source file deleted
# ==================================
echo "Test 3: Verify source file was deleted..."

if [ -f "$TEST_DIR/specifications/Source.md" ]; then
  echo "❌ FAILED: Source file was not deleted after squash"
  exit 1
fi

echo "✓ Source file was deleted"
echo ""

# ==================================
# Test 4: Verify all elements in target file
# ==================================
echo "Test 4: Verify all elements are in target file..."

# Check for original target elements
if ! grep -q "### Existing Element" "$TEST_DIR/specifications/Target.md"; then
  echo "❌ FAILED: Original target element missing"
  exit 1
fi

if ! grep -q "### Another Existing Element" "$TEST_DIR/specifications/Target.md"; then
  echo "❌ FAILED: Original target element missing"
  exit 1
fi

# Check for moved source elements
if ! grep -q "### Source Element One" "$TEST_DIR/specifications/Target.md"; then
  echo "❌ FAILED: Source Element One not found in target file"
  exit 1
fi

if ! grep -q "### Source Element Two" "$TEST_DIR/specifications/Target.md"; then
  echo "❌ FAILED: Source Element Two not found in target file"
  exit 1
fi

echo "✓ All elements found in target file"
echo ""

# ==================================
# Test 5: Verify elements added to first section
# ==================================
echo "Test 5: Verify source elements added to first section..."

# Find line numbers
FIRST_SECTION_LINE=$(grep -n "^## Target First Section" "$TEST_DIR/specifications/Target.md" | cut -d: -f1)
SECOND_SECTION_LINE=$(grep -n "^## Target Second Section" "$TEST_DIR/specifications/Target.md" | cut -d: -f1)
SOURCE_ELEM_ONE_LINE=$(grep -n "### Source Element One" "$TEST_DIR/specifications/Target.md" | cut -d: -f1)
SOURCE_ELEM_TWO_LINE=$(grep -n "### Source Element Two" "$TEST_DIR/specifications/Target.md" | cut -d: -f1)

if [ -z "$FIRST_SECTION_LINE" ] || [ -z "$SECOND_SECTION_LINE" ] || [ -z "$SOURCE_ELEM_ONE_LINE" ] || [ -z "$SOURCE_ELEM_TWO_LINE" ]; then
  echo "❌ FAILED: Could not find required sections or elements"
  exit 1
fi

# Verify source elements are between first and second section
if [ "$SOURCE_ELEM_ONE_LINE" -le "$FIRST_SECTION_LINE" ] || [ "$SOURCE_ELEM_ONE_LINE" -ge "$SECOND_SECTION_LINE" ]; then
  echo "❌ FAILED: Source Element One is not in first section"
  echo "First section: $FIRST_SECTION_LINE, Second section: $SECOND_SECTION_LINE, Element: $SOURCE_ELEM_ONE_LINE"
  exit 1
fi

if [ "$SOURCE_ELEM_TWO_LINE" -le "$FIRST_SECTION_LINE" ] || [ "$SOURCE_ELEM_TWO_LINE" -ge "$SECOND_SECTION_LINE" ]; then
  echo "❌ FAILED: Source Element Two is not in first section"
  exit 1
fi

echo "✓ Source elements correctly placed in first section"
echo ""

# ==================================
# Test 6: Verify relations updated in other files
# ==================================
echo "Test 6: Verify relations updated in Related.md..."

# Check that the relation now points to Target.md
if ! grep -q "derivedFrom:.*Target.md#source-element-one" "$TEST_DIR/specifications/Related.md"; then
  echo "❌ FAILED: Relation in Related.md was not updated to point to Target.md"
  echo "Content:"
  cat "$TEST_DIR/specifications/Related.md"
  exit 1
fi

echo "✓ Relations updated in other files"
echo ""

# ==================================
# Test 7: Verify model validates
# ==================================
echo "Test 7: Verify model validates..."

set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after squash"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Model validates successfully"
echo ""
echo "✅ All tests passed - squash functionality works correctly"

exit 0
