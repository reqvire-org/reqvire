#!/bin/bash
set -uo pipefail

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
ADD_OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md 2>&1)
ADD_EXIT=$?
set -e

if [ $ADD_EXIT -ne 0 ]; then
  echo "❌ FAILED: Add command failed with exit code $ADD_EXIT"
  echo "$ADD_OUTPUT"
  exit 1
fi

# Compare output with expected diff
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-add-diff.txt" <(echo "$ADD_OUTPUT"); then
  echo "❌ FAILED: Add command output does not match expected diff"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-add-diff.txt"
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
# Test 1b: Add Element with Attachments
# ==================================
echo "Test 1b: Add element with attachments..."

# First add a refinement element (constraint) that will be attached
# The constraint must be owned by a requirement (via refinedBy), and Feature E must be outside that hierarchy
CONSTRAINT_ELEMENT='### Feature D Constraint

Rate limiting constraint for Feature D.

#### Metadata
  * type: constraint
'

set +e
ADD_CONSTRAINT_OUTPUT=$(cd "$TEST_DIR" && echo "$CONSTRAINT_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md 2>&1)
ADD_CONSTRAINT_EXIT=$?
set -e

if [ $ADD_CONSTRAINT_EXIT -ne 0 ]; then
  echo "❌ FAILED: Add constraint element failed with exit code $ADD_CONSTRAINT_EXIT"
  echo "$ADD_CONSTRAINT_OUTPUT"
  exit 1
fi

# Link the constraint to its owning requirement via refinedBy
set +e
LINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Separate Requirement Branch" refinedBy "Feature D Constraint" 2>&1)
LINK_EXIT=$?
set -e

if [ $LINK_EXIT -ne 0 ]; then
  echo "❌ FAILED: Link refinedBy failed with exit code $LINK_EXIT"
  echo "$LINK_OUTPUT"
  exit 1
fi

# Now add a requirement with attachment to the constraint
ELEMENT_WITH_ATTACHMENT='### Feature E

This feature has an attachment to a refinement element.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)

#### Attachments
  * [Feature D Constraint](#feature-d-constraint)
'

set +e
ADD_ATTACH_OUTPUT=$(cd "$TEST_DIR" && echo "$ELEMENT_WITH_ATTACHMENT" | "$REQVIRE_BIN" add specifications/Requirements.md 2>&1)
ADD_ATTACH_EXIT=$?
set -e

if [ $ADD_ATTACH_EXIT -ne 0 ]; then
  echo "❌ FAILED: Add element with attachment failed with exit code $ADD_ATTACH_EXIT"
  echo "$ADD_ATTACH_OUTPUT"
  exit 1
fi

# Verify attachment section exists in file
if ! grep -q "#### Attachments" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Attachments section not found in file"
  exit 1
fi

# Verify attachment link exists
if ! grep -q "Feature D Constraint" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Attachment link not found in file"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after add with attachment"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Element with attachment added successfully"
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
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-rm-diff.txt" <(echo "$DELETE_OUTPUT"); then
  echo "❌ FAILED: Delete command output does not match expected diff"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-rm-diff.txt"
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
cp "${TEST_SCRIPT_DIR}/fixtures/specifications/OtherRequirements.md" "$TEST_DIR/specifications/OtherRequirements.md"

set +e
MOVE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Feature C" "specifications/OtherRequirements.md" 2>&1)
MOVE_EXIT=$?
set -e

if [ $MOVE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Move command failed with exit code $MOVE_EXIT"
  echo "$MOVE_OUTPUT"
  exit 1
fi

# Compare output with expected diff
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-mv-diff.txt" <(echo "$MOVE_OUTPUT"); then
  echo "❌ FAILED: Move command output does not match expected diff"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-mv-diff.txt"
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
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-rename-diff.txt" <(echo "$RENAME_OUTPUT"); then
  echo "❌ FAILED: Rename command output does not match expected diff"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-rename-diff.txt"
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
# Test 4b: Move Element with Special Characters
# ==================================
echo "Test 4b: Move element with special characters..."

set +e
MOVE_SPECIAL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Complex chars, element/name example" "specifications/OtherRequirements.md" 2>&1)
MOVE_SPECIAL_EXIT=$?
set -e

if [ $MOVE_SPECIAL_EXIT -ne 0 ]; then
  echo "❌ FAILED: Move command for special chars element failed with exit code $MOVE_SPECIAL_EXIT"
  echo "$MOVE_SPECIAL_OUTPUT"
  exit 1
fi

# Compare output with expected diff
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-mv-special-chars-diff.txt" <(echo "$MOVE_SPECIAL_OUTPUT"); then
  echo "❌ FAILED: Move special chars element output does not match expected diff"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-mv-special-chars-diff.txt"
  exit 1
fi

# Verify element was removed from source
if grep -q "### Complex chars, element/name example" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Special chars element was not removed from source file"
  exit 1
fi

# Verify element was added to target
if ! grep -q "### Complex chars, element/name example" "$TEST_DIR/specifications/OtherRequirements.md"; then
  echo "❌ FAILED: Special chars element was not added to target file"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after moving special chars element"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✓ Element with special characters moved successfully"
echo ""

# ==================================
# Test 5: Move File
# ==================================
echo "Test 5: Move file operation..."

# Create a file with multiple elements to move
cp "${TEST_SCRIPT_DIR}/fixtures/specifications/ToMove.md" "$TEST_DIR/specifications/ToMove.md"

# Create element in another file that references element in file to be moved
cat "${TEST_SCRIPT_DIR}/fixtures/specifications/OtherRequirements-referencer.md" >> "$TEST_DIR/specifications/OtherRequirements.md"

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
ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Non Existent Element" "specifications/OtherRequirements.md" 2>&1)
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
ERROR_OUTPUT=$(cd "$TEST_DIR" && echo "$DUPLICATE_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md 2>&1)
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
ERROR_OUTPUT=$(cd "$TEST_DIR" && echo "$INVALID_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md 2>&1)
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
ERROR_OUTPUT=$(cd "$TEST_DIR" && echo "$INVALID_RELATION_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md 2>&1)
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

# ==================================
# Test 7: InternalPath Files Protection
# ==================================
echo "Test 7: InternalPath files not cleared during mv-file..."

# Create InternalPath files (source code)
mkdir -p "$TEST_DIR/src"
cp "${TEST_SCRIPT_DIR}/fixtures/src/code1.rs" "$TEST_DIR/src/code1.rs"
cp "${TEST_SCRIPT_DIR}/fixtures/src/code2.rs" "$TEST_DIR/src/code2.rs"

# Store original file sizes and content
CODE1_SIZE=$(wc -c < "$TEST_DIR/src/code1.rs")
CODE2_SIZE=$(wc -c < "$TEST_DIR/src/code2.rs")
CODE1_CONTENT=$(cat "$TEST_DIR/src/code1.rs")
CODE2_CONTENT=$(cat "$TEST_DIR/src/code2.rs")

# Create stable specification file with satisfiedBy to code1.rs
cp "${TEST_SCRIPT_DIR}/fixtures/specifications/StableFile.md" "$TEST_DIR/specifications/StableFile.md"

# Create file to be moved with complex relations
cp "${TEST_SCRIPT_DIR}/fixtures/specifications/ToMove2.md" "$TEST_DIR/specifications/ToMove2.md"

# Commit files
(cd "$TEST_DIR" && git add -A && git commit -m "Add files for InternalPath test" >/dev/null 2>&1)

# Run mv-file
set +e
MVFILE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-file "specifications/ToMove2.md" "specifications/Moved2.md" 2>&1)
MVFILE_EXIT=$?
set -e


if [ $MVFILE_EXIT -ne 0 ]; then
  echo "❌ FAILED: mv-file command failed with exit code $MVFILE_EXIT"
  echo "$MVFILE_OUTPUT"
  exit 1
fi

# CRITICAL CHECK: Verify InternalPath files were NOT cleared/modified
if [ ! -f "$TEST_DIR/src/code1.rs" ]; then
  echo "❌ FAILED: src/code1.rs was deleted!"
  exit 1
fi

if [ ! -f "$TEST_DIR/src/code2.rs" ]; then
  echo "❌ FAILED: src/code2.rs was deleted!"
  exit 1
fi

# Check file sizes
CODE1_SIZE_AFTER=$(wc -c < "$TEST_DIR/src/code1.rs")
CODE2_SIZE_AFTER=$(wc -c < "$TEST_DIR/src/code2.rs")

if [ "$CODE1_SIZE" -ne "$CODE1_SIZE_AFTER" ]; then
  echo "❌ FAILED: src/code1.rs size changed from $CODE1_SIZE to $CODE1_SIZE_AFTER (file was overwritten!)"
  echo "Original content:"
  echo "$CODE1_CONTENT"
  echo "New content:"
  cat "$TEST_DIR/src/code1.rs"
  exit 1
fi

if [ "$CODE2_SIZE" -ne "$CODE2_SIZE_AFTER" ]; then
  echo "❌ FAILED: src/code2.rs size changed from $CODE2_SIZE to $CODE2_SIZE_AFTER (file was overwritten!)"
  echo "Original content:"
  echo "$CODE2_CONTENT"
  echo "New content:"
  cat "$TEST_DIR/src/code2.rs"
  exit 1
fi

# Check content integrity
CODE1_CONTENT_AFTER=$(cat "$TEST_DIR/src/code1.rs")
CODE2_CONTENT_AFTER=$(cat "$TEST_DIR/src/code2.rs")

if [ "$CODE1_CONTENT" != "$CODE1_CONTENT_AFTER" ]; then
  echo "❌ FAILED: src/code1.rs content was modified!"
  echo "Expected:"
  echo "$CODE1_CONTENT"
  echo "Got:"
  echo "$CODE1_CONTENT_AFTER"
  exit 1
fi

if [ "$CODE2_CONTENT" != "$CODE2_CONTENT_AFTER" ]; then
  echo "❌ FAILED: src/code2.rs content was modified!"
  echo "Expected:"
  echo "$CODE2_CONTENT"
  echo "Got:"
  echo "$CODE2_CONTENT_AFTER"
  exit 1
fi

# Verify specification files were handled correctly
if [ -f "$TEST_DIR/specifications/ToMove2.md" ]; then
  echo "❌ FAILED: Source file ToMove2.md was not deleted"
  exit 1
fi

if [ ! -f "$TEST_DIR/specifications/Moved2.md" ]; then
  echo "❌ FAILED: Target file Moved2.md was not created"
  exit 1
fi

if [ ! -f "$TEST_DIR/specifications/StableFile.md" ]; then
  echo "❌ FAILED: StableFile.md was deleted (should remain unchanged)"
  exit 1
fi

# Verify elements were moved
if ! grep -q "### Requirement B" "$TEST_DIR/specifications/Moved2.md"; then
  echo "❌ FAILED: Requirement B was not moved to target file"
  exit 1
fi

if ! grep -q "### Requirement C" "$TEST_DIR/specifications/Moved2.md"; then
  echo "❌ FAILED: Requirement C was not moved to target file"
  exit 1
fi

# Verify relations are correct in moved file
if ! grep -q "StableFile.md#requirement-a" "$TEST_DIR/specifications/Moved2.md"; then
  echo "❌ FAILED: derivedFrom relation was not preserved correctly"
  exit 1
fi

if ! grep -q "../src/code2.rs" "$TEST_DIR/specifications/Moved2.md"; then
  echo "❌ FAILED: satisfiedBy relation to code2.rs was not preserved"
  exit 1
fi

# Verify stable file wasn't modified
if ! grep -q "../src/code1.rs" "$TEST_DIR/specifications/StableFile.md"; then
  echo "❌ FAILED: StableFile.md was modified (should remain unchanged)"
  exit 1
fi

echo "✓ InternalPath files protected: src/code1.rs and src/code2.rs unchanged"
echo "✓ Specification files moved correctly with relations intact"
echo ""

echo "===================================="
echo "✓ All tests passed"
echo "===================================="
exit 0
