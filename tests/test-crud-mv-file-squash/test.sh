#!/bin/bash
set -euo pipefail

# Test: Move File with Squash Flag
#
# Satisfies: specifications/Verifications/ElementManipulationTests.md#move-file-squash-test
#
# Acceptance Criteria:
# - Without --squash: moving to existing file should error
# - With --squash: all elements from source moved to end of the target file
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
# - Elements from source are added to the end of the target
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

if ! grep -q "### Source Ontology" "$TEST_DIR/specifications/Target.md"; then
  echo "❌ FAILED: Source Ontology not found in target file"
  exit 1
fi

if ! grep -q "<https://example.test/source-ontology> a owl:Ontology" "$TEST_DIR/specifications/Target.md"; then
  echo "❌ FAILED: Source Ontology Turtle block was not preserved in target file"
  exit 1
fi

echo "✓ All elements found in target file"
echo ""

# ==================================
# Test 5: Verify element count after squash
# ==================================
echo "Test 5: Verify correct element count after squash..."

# Count total elements (both existing and moved)
ELEMENT_COUNT=$(grep -c "^### " "$TEST_DIR/specifications/Target.md")
EXPECTED_COUNT=6  # capability + 2 existing + 3 moved

if [ "$ELEMENT_COUNT" -ne "$EXPECTED_COUNT" ]; then
  echo "❌ FAILED: Expected $EXPECTED_COUNT elements, found $ELEMENT_COUNT"
  exit 1
fi

echo "✓ Correct number of elements in target file"
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

if ! grep -q "derivedFrom:.*Target.md#source-ontology" "$TEST_DIR/specifications/Related.md"; then
  echo "❌ FAILED: Ontology relation in Related.md was not updated to point to Target.md"
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
