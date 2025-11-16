#!/bin/bash
set -euo pipefail

# Test: Element Manipulation Operations
#
# Satisfies: specifications/Verifications/ElementManipulationTests.md
#
# Acceptance Criteria:
# - Add command creates elements with proper structure
# - Delete command removes elements and cleans up relations
# - Move command relocates elements and updates relations
# - All operations persist changes to files
#
# Test Criteria:
# - Commands exit with code 0 on success
# - Output matches expected format
# - Files are modified as expected
# - Relations are properly maintained

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "===================================="
echo "Element Manipulation Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Add Element
# ==================================
echo "Test 1: Add element operation..."

NEW_ELEMENT='### Feature D

This is a newly added feature.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
'

set +e
ADD_OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md "Features" 2>&1)
ADD_EXIT=$?
set -e

if [ $ADD_EXIT -ne 0 ]; then
  echo "❌ FAILED: Add command failed with exit code $ADD_EXIT"
  echo "$ADD_OUTPUT"
  exit 1
fi

# Verify element was added
if ! grep -q "### Feature D" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Element was not added to file"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after add"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Element added successfully"
echo ""

# ==================================
# Test 2: Delete Element
# ==================================
echo "Test 2: Delete element operation..."

# Make backup for comparison
cp "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.md"

set +e
DELETE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "specifications/Requirements.md#feature-b" 2>&1)
DELETE_EXIT=$?
set -e

if [ $DELETE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Delete command failed with exit code $DELETE_EXIT"
  echo "$DELETE_OUTPUT"
  exit 1
fi

# Verify element was removed
if grep -q "### Feature B" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Element was not removed from file"
  exit 1
fi

# Verify file was modified
if cmp -s "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.md"; then
  echo "❌ FAILED: File was not modified"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after delete"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Element deleted successfully"
echo ""

# ==================================
# Test 3: Move Element
# ==================================
echo "Test 3: Move element operation..."

# Create target file for move
cat > "$TEST_DIR/specifications/OtherRequirements.md" <<'EOF'
# Other Requirements

## Other Features
EOF

set +e
MOVE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "specifications/Requirements.md#feature-c" "specifications/OtherRequirements.md" "Other Features" 2>&1)
MOVE_EXIT=$?
set -e

if [ $MOVE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Move command failed with exit code $MOVE_EXIT"
  echo "$MOVE_OUTPUT"
  exit 1
fi

# Verify element was removed from source
if grep -q "### Feature C" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Element was not removed from source file"
  exit 1
fi

# Verify element was added to target
if ! grep -q "### Feature C" "$TEST_DIR/specifications/OtherRequirements.md"; then
  echo "❌ FAILED: Element was not added to target file"
  exit 1
fi

# Verify relation was updated in verification file
if ! grep -q "specifications/OtherRequirements.md#feature-c" "$TEST_DIR/specifications/Verifications/Tests.md"; then
  echo "❌ FAILED: Relation was not updated to new location"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after move"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Element moved successfully and relations updated"
echo ""

# ==================================
# Test 4: Error Cases
# ==================================
echo "Test 4: Error case handling..."

# Test 4a: Move non-existent element
echo "  4a: Move non-existent element..."
set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "specifications/Requirements.md#non-existent" "specifications/OtherRequirements.md" "Other Features" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Moving non-existent element should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "not found\|does not exist\|missing"; then
  echo "❌ FAILED: Error message should mention element not found"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Non-existent element error handled"

# Test 4b: Delete non-existent element
echo "  4b: Delete non-existent element..."
set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "specifications/Requirements.md#non-existent" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Deleting non-existent element should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "not found\|does not exist\|missing"; then
  echo "❌ FAILED: Error message should mention element not found"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Non-existent element delete error handled"

# Test 4c: Add element with duplicate name
echo "  4c: Add element with duplicate name..."
DUPLICATE_ELEMENT='### Feature A

This is a duplicate.

#### Metadata
  * type: requirement
'

set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && echo "$DUPLICATE_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md "Features" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Adding duplicate element should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "duplicate\|already exists\|unique"; then
  echo "❌ FAILED: Error message should mention duplicate/uniqueness"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Duplicate element error handled"

# Test 4d: Add element with invalid markdown
echo "  4d: Add element with invalid markdown..."
INVALID_ELEMENT='This is invalid

No header here
'

set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && echo "$INVALID_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md "Features" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Adding invalid element should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "invalid\|malformed\|header\|format"; then
  echo "❌ FAILED: Error message should mention invalid format"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Invalid element error handled"
echo ""

echo "===================================="
echo "✓ All tests passed"
echo "===================================="
exit 0
