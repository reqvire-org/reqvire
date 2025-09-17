#!/bin/bash

# Test: Diagram Removal Functionality
# ------------------------------------
# Acceptance Criteria:
# - System should remove all generated mermaid diagrams from markdown files
# - System should preserve custom (non-generated) mermaid diagrams
# - System should handle files with multiple diagrams correctly
# - System should work on files without any diagrams
#
# Test Criteria:
# - Command exits with success (0) return code
# - Generated diagrams are removed from files
# - Custom diagrams are preserved (if any)
# - File structure remains intact except for diagram removal

# Use existing reqvire.yaml configuration

# Make backup copies of original files for comparison
mkdir -p "$TEST_DIR/backup"
cp -r "$TEST_DIR/specifications" "$TEST_DIR/backup/"

# First validate that all test data is valid before attempting diagram removal
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "$TEST_DIR/reqvire.yaml" validate 2>&1)
VALIDATION_EXIT_CODE=$?

if [ $VALIDATION_EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Test data validation failed. Fix test data before running diagram removal:"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

# First, ensure we have diagrams to remove by generating them
GENERATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "$TEST_DIR/reqvire.yaml" generate-diagrams 2>&1)
GENERATE_EXIT_CODE=$?

if [ $GENERATE_EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Initial diagram generation failed:"
  echo "$GENERATE_OUTPUT"
  exit 1
fi

# Verify diagrams were generated
if ! grep -q '```mermaid' "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: No mermaid diagrams found after generation"
  exit 1
fi

# Count the number of mermaid diagrams before removal
BEFORE_COUNT=$(grep -c '```mermaid' "$TEST_DIR/specifications/Requirements.md")

# Run reqvire to remove diagrams
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "$TEST_DIR/reqvire.yaml" remove-diagrams 2>&1)
EXIT_CODE=$?

# Save output to log
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# Check for basic success
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: remove-diagrams command failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify diagrams were removed
AFTER_COUNT=$(grep -c '```mermaid' "$TEST_DIR/specifications/Requirements.md" 2>/dev/null)
if [ $? -ne 0 ]; then
    AFTER_COUNT=0
fi

# Test 1: Check that auto-generated diagrams were removed but custom diagrams preserved
# We expect 1 custom diagram to remain after removing the 2 auto-generated ones
EXPECTED_REMAINING=1
if [ "$AFTER_COUNT" -ne "$EXPECTED_REMAINING" ]; then
  echo "❌ FAILED: Expected $EXPECTED_REMAINING diagram remaining (custom), but found $AFTER_COUNT"
  exit 1
fi

# Test 2: Check that the file still exists and has the requirements
if [ ! -f "$TEST_DIR/specifications/Requirements.md" ]; then
  echo "❌ FAILED: Requirements file was deleted"
  exit 1
fi

# Test 3: Verify that requirement text is still present
if ! grep -q "Test Requirement" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Requirement text was removed"
  exit 1
fi

if ! grep -q "Another Requirement" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Second requirement text was removed"
  exit 1
fi

# Test 4: Check that section headers are preserved
if ! grep -q "## Test Section" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Section headers were removed"
  exit 1
fi

if ! grep -q "## Another Section" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Second section header was removed"
  exit 1
fi

# Test 5: Verify that element headers are preserved
if ! grep -q "### Test Requirement" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Element headers were removed"
  exit 1
fi

# Test 6: Check command output for success indicators
if ! echo "$OUTPUT" | grep -q -i "diagram"; then
  echo "❌ FAILED: Output does not mention diagrams"
  exit 1
fi

# Test 7: Verify that diagrams in excluded files are preserved
IGNORED_FILE_DIAGRAMS=$(grep -c '```mermaid' "$TEST_DIR/specifications/IgnoredFile.md" 2>/dev/null)
if [ $? -ne 0 ]; then
    IGNORED_FILE_DIAGRAMS=0
fi
if [ "$IGNORED_FILE_DIAGRAMS" -ne 2 ]; then
  echo "❌ FAILED: Expected 2 diagrams in ignored file, but found $IGNORED_FILE_DIAGRAMS"
  exit 1
fi

# Verify the content of ignored diagrams is intact
if ! grep -q "Ignored Diagram" "$TEST_DIR/specifications/IgnoredFile.md"; then
  echo "❌ FAILED: Content in ignored file was modified"
  exit 1
fi

if ! grep -q "Another Ignored Diagram" "$TEST_DIR/specifications/IgnoredFile.md"; then
  echo "❌ FAILED: Second diagram in ignored file was modified"
  exit 1
fi

exit 0