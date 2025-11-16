#!/bin/bash
set -euo pipefail

# Test: Target Location Validation and Auto-Creation
# ----------------------------------------------------
# Satisfies: specifications/Verifications/ElementManipulationTests.md#target-location-validation-test
#
# Acceptance Criteria:
# - Paths excluded by .gitignore are rejected
# - Paths excluded by .reqvireignore are rejected
# - Paths exceeding 10 subdirectory depth are rejected
# - Valid paths are accepted
# - Non-existent files are created with proper structure
# - Non-existent sections are added to existing files
#
# Test Criteria:
# - Commands exit with error code for invalid paths
# - Error messages indicate which constraint was violated
# - Commands succeed for valid paths
# - Files and sections are auto-created as needed

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "====================================" >> "${TEST_DIR}/test_results.log"
echo "Target Location Validation Tests" >> "${TEST_DIR}/test_results.log"
echo "====================================" >> "${TEST_DIR}/test_results.log"

# Track overall test result
OVERALL_RESULT=0

# ==================================
# Setup: Create gitignore and reqvireignore
# ==================================
cat > "${TEST_DIR}/.gitignore" << 'EOF'
**/build/**
temp-*.md
EOF

cat > "${TEST_DIR}/.reqvireignore" << 'EOF'
**/draft-*.md
EOF

# Create a simple base element for testing
cat > "${TEST_DIR}/specifications/Base.md" << 'EOF'
# Base Requirements

## Base Section

### Base Requirement

This is the base requirement.

#### Metadata
  * type: user-requirement
EOF

# ==================================
# Test 1: Gitignore Exclusion
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 1: Gitignore exclusion..." >> "${TEST_DIR}/test_results.log"

NEW_ELEMENT='### Test Requirement

This should be rejected.

#### Metadata
  * type: requirement
'

set +e
OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/build/Ignored.md "Section" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Should reject path excluded by .gitignore"
  OVERALL_RESULT=1
elif ! echo "$OUTPUT" | grep -qi "gitignore\|ignored\|excluded"; then
  echo "❌ FAILED: Error message should mention gitignore exclusion"
  echo "Got: $OUTPUT"
  OVERALL_RESULT=1
else
  echo "✓ Gitignore exclusion working" >> "${TEST_DIR}/test_results.log"
fi

# ==================================
# Test 2: Reqvireignore Exclusion
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 2: Reqvireignore exclusion..." >> "${TEST_DIR}/test_results.log"

set +e
OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/draft-Ideas.md "Section" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Should reject path excluded by .reqvireignore"
  OVERALL_RESULT=1
elif ! echo "$OUTPUT" | grep -qi "reqvireignore\|ignored\|excluded"; then
  echo "❌ FAILED: Error message should mention reqvireignore exclusion"
  echo "Got: $OUTPUT"
  OVERALL_RESULT=1
else
  echo "✓ Reqvireignore exclusion working" >> "${TEST_DIR}/test_results.log"
fi

# ==================================
# Test 3: Path Depth Limit (11 subdirectories - should fail)
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 3: Path depth limit (11 levels)..." >> "${TEST_DIR}/test_results.log"

DEEP_PATH="specifications/a/b/c/d/e/f/g/h/i/j/k/DeepFile.md"

set +e
OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add "$DEEP_PATH" "Section" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Should reject path exceeding 10 subdirectory depth"
  OVERALL_RESULT=1
elif ! echo "$OUTPUT" | grep -qi "depth\|nested\|deep\|limit"; then
  echo "❌ FAILED: Error message should mention depth limit"
  echo "Got: $OUTPUT"
  OVERALL_RESULT=1
else
  echo "✓ Path depth limit working" >> "${TEST_DIR}/test_results.log"
fi

# ==================================
# Test 4: Valid Path Depth (10 subdirectories - should succeed)
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 4: Valid path depth (10 levels)..." >> "${TEST_DIR}/test_results.log"

VALID_DEEP_PATH="specifications/a/b/c/d/e/f/g/h/i/j/ValidFile.md"

set +e
OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add "$VALID_DEEP_PATH" "Section" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Should accept path with 10 subdirectories"
  echo "Got: $OUTPUT"
  OVERALL_RESULT=1
else
  echo "✓ Valid depth path accepted" >> "${TEST_DIR}/test_results.log"
fi

# ==================================
# Test 5: Auto-Create Non-Existent File
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 5: Auto-create non-existent file..." >> "${TEST_DIR}/test_results.log"

set +e
OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/NewFile.md "New Section" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Should auto-create non-existent file"
  echo "Got: $OUTPUT"
  OVERALL_RESULT=1
elif [ ! -f "${TEST_DIR}/specifications/NewFile.md" ]; then
  echo "❌ FAILED: File was not created"
  OVERALL_RESULT=1
else
  # Verify file structure
  if ! grep -q "^# " "${TEST_DIR}/specifications/NewFile.md"; then
    echo "❌ FAILED: Created file missing level 1 header"
    OVERALL_RESULT=1
  elif ! grep -q "^## New Section" "${TEST_DIR}/specifications/NewFile.md"; then
    echo "❌ FAILED: Created file missing section header"
    OVERALL_RESULT=1
  elif ! grep -q "^### Test Requirement" "${TEST_DIR}/specifications/NewFile.md"; then
    echo "❌ FAILED: Created file missing element"
    OVERALL_RESULT=1
  else
    echo "✓ File auto-creation working" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 6: Auto-Create Section in Existing File
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 6: Auto-create section in existing file..." >> "${TEST_DIR}/test_results.log"

set +e
OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/Base.md "New Section" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Should auto-create section in existing file"
  echo "Got: $OUTPUT"
  OVERALL_RESULT=1
elif ! grep -q "^## New Section" "${TEST_DIR}/specifications/Base.md"; then
  echo "❌ FAILED: Section was not added to existing file"
  OVERALL_RESULT=1
elif ! grep -q "^## Base Section" "${TEST_DIR}/specifications/Base.md"; then
  echo "❌ FAILED: Original section was removed"
  OVERALL_RESULT=1
else
  echo "✓ Section auto-creation working" >> "${TEST_DIR}/test_results.log"
fi

# ==================================
# Final Result
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
if [ $OVERALL_RESULT -eq 0 ]; then
  echo "✓ All target location validation tests passed" >> "${TEST_DIR}/test_results.log"
  exit 0
else
  echo "❌ One or more target location validation tests failed" >> "${TEST_DIR}/test_results.log"
  cat "${TEST_DIR}/test_results.log"
  exit 1
fi
