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

# Compare output with expected diff
if ! diff -u "${TEST_SCRIPT_DIR}/expected-add-diff.txt" <(echo "$ADD_OUTPUT"); then
  echo "❌ FAILED: Add command output does not match expected diff"
  echo ""
  echo "Differences shown above (expected vs actual)"
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

# Make backup for comparison (use .bak extension to avoid parsing)
cp "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.bak"

set +e
DELETE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Feature B" 2>&1)
DELETE_EXIT=$?
set -e

if [ $DELETE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Delete command failed with exit code $DELETE_EXIT"
  echo "$DELETE_OUTPUT"
  exit 1
fi

# Compare output with expected diff
if ! diff -u "${TEST_SCRIPT_DIR}/expected-rm-diff.txt" <(echo "$DELETE_OUTPUT"); then
  echo "❌ FAILED: Delete command output does not match expected diff"
  echo ""
  echo "Differences shown above (expected vs actual)"
  exit 1
fi

# Verify element was removed
if grep -q "### Feature B" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Element was not removed from file"
  exit 1
fi

# Verify file was modified
if cmp -s "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.bak"; then
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
MOVE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Feature C" "specifications/OtherRequirements.md" "Other Features" 2>&1)
MOVE_EXIT=$?
set -e

if [ $MOVE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Move command failed with exit code $MOVE_EXIT"
  echo "$MOVE_OUTPUT"
  exit 1
fi

# Compare output with expected diff
if ! diff -u "${TEST_SCRIPT_DIR}/expected-mv-diff.txt" <(echo "$MOVE_OUTPUT"); then
  echo "❌ FAILED: Move command output does not match expected diff"
  echo ""
  echo "Differences shown above (expected vs actual)"
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

# Verify relation was updated in verification file (check for relative path)
if ! grep -q "OtherRequirements.md#feature-c" "$TEST_DIR/specifications/Verifications/Tests.md"; then
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
# Test 4: Rename Element
# ==================================
echo "Test 4: Rename element operation..."

set +e
RENAME_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rename "Feature A" "Feature Alpha" 2>&1)
RENAME_EXIT=$?
set -e

if [ $RENAME_EXIT -ne 0 ]; then
  echo "❌ FAILED: Rename command failed with exit code $RENAME_EXIT"
  echo "$RENAME_OUTPUT"
  exit 1
fi

# Compare output with expected diff
if ! diff -u "${TEST_SCRIPT_DIR}/expected-rename-diff.txt" <(echo "$RENAME_OUTPUT"); then
  echo "❌ FAILED: Rename command output does not match expected diff"
  echo ""
  echo "Differences shown above (expected vs actual)"
  exit 1
fi

# Verify element heading was updated
if ! grep -q "### Feature Alpha" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Element heading was not renamed"
  exit 1
fi

# Verify old heading is gone
if grep -q "### Feature A$" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Old element heading still exists"
  exit 1
fi

# Verify relations were updated (Feature D should now reference feature-alpha)
if ! grep -q "#feature-alpha" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Relations were not updated with new identifier"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after rename"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Element renamed successfully and relations updated"
echo ""

# ==================================
# Test 5: Move File
# ==================================
echo "Test 5: Move file operation..."

# Create a file with multiple elements to move
cat > "$TEST_DIR/specifications/ToMove.md" <<'EOF'
# To Move

## Section One

### Element One

Content for element one.

#### Metadata
  * type: requirement

### Element Two

Content for element two.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature Alpha](Requirements.md#feature-alpha)
EOF

# Create element in another file that references element in file to be moved
cat >> "$TEST_DIR/specifications/OtherRequirements.md" <<'EOF'

### Referencer

This element has a relation to element in file to be moved.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Element One](ToMove.md#element-one)
EOF

# Commit files so reqvire can find them
(cd "$TEST_DIR" && git add specifications/ToMove.md specifications/OtherRequirements.md && git commit -m "Add file to move and referencer" >/dev/null 2>&1)

set +e
MVFILE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-file "specifications/ToMove.md" "specifications/Moved.md" 2>&1)
MVFILE_EXIT=$?
set -e

if [ $MVFILE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Move file command failed with exit code $MVFILE_EXIT"
  echo "$MVFILE_OUTPUT"
  exit 1
fi

# Verify source file was deleted
if [ -f "$TEST_DIR/specifications/ToMove.md" ]; then
  echo "❌ FAILED: Source file was not deleted"
  exit 1
fi

# Verify target file was created
if [ ! -f "$TEST_DIR/specifications/Moved.md" ]; then
  echo "❌ FAILED: Target file was not created"
  exit 1
fi

# Verify both elements were moved to target file
if ! grep -q "### Element One" "$TEST_DIR/specifications/Moved.md"; then
  echo "❌ FAILED: Element One was not moved to target file"
  exit 1
fi

if ! grep -q "### Element Two" "$TEST_DIR/specifications/Moved.md"; then
  echo "❌ FAILED: Element Two was not moved to target file"
  exit 1
fi

# Verify element content was preserved
if ! grep -q "Content for element one" "$TEST_DIR/specifications/Moved.md"; then
  echo "❌ FAILED: Element content was not preserved"
  exit 1
fi

# Verify relations were updated in other files
if ! grep -q "Moved.md#element-one" "$TEST_DIR/specifications/OtherRequirements.md"; then
  echo "❌ FAILED: Relation in other file was not updated"
  exit 1
fi

# Verify old file reference is gone from other files
if grep -q "ToMove.md#element-one" "$TEST_DIR/specifications/OtherRequirements.md"; then
  echo "❌ FAILED: Old file reference still exists in other file"
  exit 1
fi

# Verify outgoing relation was preserved (Element Two -> Feature Alpha)
if ! grep -q "Requirements.md#feature-alpha" "$TEST_DIR/specifications/Moved.md"; then
  echo "❌ FAILED: Outgoing relation was not preserved"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after file move"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ File moved successfully with all elements and relations updated"
echo ""

# ==================================
# Test 6: Error Cases
# ==================================
echo "Test 6: Error case handling..."

# Test 4a: Move non-existent element
echo "  4a: Move non-existent element..."
set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Non Existent Element" "specifications/OtherRequirements.md" "Other Features" 2>&1)
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
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Non Existent Element" 2>&1)
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
DUPLICATE_ELEMENT='### Feature Alpha

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

# Test 4e: Add element with non-existent relation target
echo "  4e: Add element with non-existent relation target..."
INVALID_RELATION_ELEMENT='### Feature With Bad Relation

This element has a relation to a non-existent target.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: specifications/NonExistent.md#missing-element
'

set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && echo "$INVALID_RELATION_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md "Features" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Adding element with non-existent relation target should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "not found\|missing\|does not exist\|unknown"; then
  echo "❌ FAILED: Error message should indicate relation target not found"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Invalid relation target error handled"

# Test 5f: Rename non-existent element
echo "  5f: Rename non-existent element..."
set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rename "Non Existent Element" "New Name" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Renaming non-existent element should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "not found\|does not exist\|missing"; then
  echo "❌ FAILED: Error message should mention element not found"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Non-existent element rename error handled"

# Test 5g: Rename to duplicate name
echo "  5g: Rename to duplicate name..."
set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rename "Feature D" "Parent Feature" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Renaming to duplicate name should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "duplicate\|already exists\|conflict\|unique"; then
  echo "❌ FAILED: Error message should mention duplicate/conflict"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Duplicate name rename error handled"

# Test 6h: Move non-existent file
echo "  6h: Move non-existent file..."
set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-file "specifications/NonExistent.md" "specifications/Target.md" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Moving non-existent file should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "not found\|does not exist\|missing"; then
  echo "❌ FAILED: Error message should mention file not found"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Non-existent file move error handled"

# Test 6i: Move file to existing target
echo "  6i: Move file to existing target..."
set +e
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-file "specifications/Moved.md" "specifications/Requirements.md" 2>&1)
ERROR_EXIT=$?
set -e

if [ $ERROR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Moving to existing file should fail"
  exit 1
fi

if ! echo "$ERROR_OUTPUT" | grep -qi "exists\|already\|conflict"; then
  echo "❌ FAILED: Error message should mention target already exists"
  echo "Got: $ERROR_OUTPUT"
  exit 1
fi

echo "  ✓ Existing target file error handled"
echo ""

echo "===================================="
echo "✓ All tests passed"
echo "===================================="
exit 0
