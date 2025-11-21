#!/bin/bash
set -euo pipefail

# Test: Element Manipulation File Persistence
# ----------------------------------------------------
# Satisfies: specifications/Verifications/ElementManipulationTests.md#file-persistence-test
#
# Acceptance Criteria:
# - All changes are persisted to disk after manipulation completes
# - File content on disk matches in-memory model state exactly
# - Only modified files are written (optimization verified)
# - Unmodified files have unchanged timestamps
# - File format and structure are maintained
#
# Test Criteria:
# - Commands exit with success (0) return code
# - Modified files are written to disk
# - Unmodified files remain untouched (timestamp check)
# - Content persists correctly

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "====================================" >> "${TEST_DIR}/test_results.log"
echo "File Persistence Tests" >> "${TEST_DIR}/test_results.log"
echo "====================================" >> "${TEST_DIR}/test_results.log"

# Track overall test result
OVERALL_RESULT=0

# ==================================
# Setup: Create test model with multiple files
# ==================================
mkdir -p "${TEST_DIR}/specifications"

cat > "${TEST_DIR}/specifications/FileA.md" << 'EOF'
# Requirements


### Root Requirement

This is the root requirement.

#### Metadata
  * type: user-requirement

### Requirement A1

This is requirement A1.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-requirement

### Requirement A2

This is requirement A2.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-requirement
EOF

cat > "${TEST_DIR}/specifications/FileB.md" << 'EOF'
# Requirements


### Requirement B1

This is requirement B1.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: FileA.md#root-requirement

### Requirement B2

This is requirement B2.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: FileA.md#root-requirement
  * derivedFrom: FileA.md#requirement-a1
EOF

cat > "${TEST_DIR}/specifications/FileC.md" << 'EOF'
# Requirements


### Requirement C1

This is requirement C1.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: FileA.md#root-requirement
EOF

# Record initial timestamps
sleep 1
TIMESTAMP_A_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileA.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileA.md")
TIMESTAMP_B_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileB.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileB.md")
TIMESTAMP_C_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileC.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileC.md")

# ==================================
# Test 1: Add Element - File Modified
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 1: Add element and verify file is modified..." >> "${TEST_DIR}/test_results.log"

NEW_ELEMENT='### Requirement A3

This is requirement A3.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-requirement
'

sleep 1
set +e
OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/FileA.md "Section A" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Add command failed"
  echo "Output: $OUTPUT"
  OVERALL_RESULT=1
else
  # Verify element exists in file
  if ! grep -q "### Requirement A3" "${TEST_DIR}/specifications/FileA.md"; then
    echo "❌ FAILED: Element was not persisted to FileA.md"
    OVERALL_RESULT=1
  else
    echo "✓ Element added and persisted to FileA.md" >> "${TEST_DIR}/test_results.log"
  fi

  # Verify FileA timestamp changed
  TIMESTAMP_A_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileA.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileA.md")
  if [ "$TIMESTAMP_A_AFTER" -le "$TIMESTAMP_A_BEFORE" ]; then
    echo "❌ FAILED: FileA.md timestamp did not change (file not written)"
    OVERALL_RESULT=1
  else
    echo "✓ FileA.md was written (timestamp changed)" >> "${TEST_DIR}/test_results.log"
  fi

  # Verify FileB and FileC timestamps unchanged
  TIMESTAMP_B_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileB.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileB.md")
  TIMESTAMP_C_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileC.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileC.md")

  if [ "$TIMESTAMP_B_AFTER" -ne "$TIMESTAMP_B_BEFORE" ]; then
    echo "❌ FAILED: FileB.md was modified but should not have been"
    OVERALL_RESULT=1
  else
    echo "✓ FileB.md timestamp unchanged (not modified)" >> "${TEST_DIR}/test_results.log"
  fi

  if [ "$TIMESTAMP_C_AFTER" -ne "$TIMESTAMP_C_BEFORE" ]; then
    echo "❌ FAILED: FileC.md was modified but should not have been"
    OVERALL_RESULT=1
  else
    echo "✓ FileC.md timestamp unchanged (not modified)" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 2: Delete Element - File Modified, Relations Updated
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 2: Delete element and verify changes persist..." >> "${TEST_DIR}/test_results.log"

# Update timestamps
sleep 1
TIMESTAMP_A_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileA.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileA.md")
TIMESTAMP_B_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileB.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileB.md")
TIMESTAMP_C_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileC.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileC.md")

sleep 1
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Requirement A1" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Delete command failed"
  echo "Output: $OUTPUT"
  OVERALL_RESULT=1
else
  # Verify element removed from FileA
  if grep -q "### Requirement A1" "${TEST_DIR}/specifications/FileA.md"; then
    echo "❌ FAILED: Element was not removed from FileA.md"
    OVERALL_RESULT=1
  else
    echo "✓ Element deleted from FileA.md" >> "${TEST_DIR}/test_results.log"
  fi

  # Verify relation removed from FileB
  if grep -q "derivedFrom:.*FileA.md#requirement-a1" "${TEST_DIR}/specifications/FileB.md"; then
    echo "❌ FAILED: Relation was not removed from FileB.md"
    OVERALL_RESULT=1
  else
    echo "✓ Relation removed from FileB.md" >> "${TEST_DIR}/test_results.log"
  fi

  # Verify FileA and FileB timestamps changed
  TIMESTAMP_A_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileA.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileA.md")
  TIMESTAMP_B_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileB.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileB.md")
  TIMESTAMP_C_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileC.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileC.md")

  if [ "$TIMESTAMP_A_AFTER" -le "$TIMESTAMP_A_BEFORE" ]; then
    echo "❌ FAILED: FileA.md timestamp did not change"
    OVERALL_RESULT=1
  fi

  if [ "$TIMESTAMP_B_AFTER" -le "$TIMESTAMP_B_BEFORE" ]; then
    echo "❌ FAILED: FileB.md timestamp did not change (relation not updated)"
    OVERALL_RESULT=1
  fi

  if [ "$TIMESTAMP_C_AFTER" -ne "$TIMESTAMP_C_BEFORE" ]; then
    echo "❌ FAILED: FileC.md was modified but should not have been"
    OVERALL_RESULT=1
  else
    echo "✓ FileC.md timestamp unchanged" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 3: Move Element - Multiple Files Modified
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 3: Move element and verify all affected files are modified..." >> "${TEST_DIR}/test_results.log"

# Update timestamps
sleep 1
TIMESTAMP_A_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileA.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileA.md")
TIMESTAMP_B_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileB.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileB.md")
TIMESTAMP_C_BEFORE=$(stat -c %Y "${TEST_DIR}/specifications/FileC.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileC.md")

sleep 1
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Requirement A2" "specifications/FileC.md" "Section C" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
echo "Output: $OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Move command failed"
  echo "Output: $OUTPUT"
  OVERALL_RESULT=1
else
  # Verify element removed from FileA
  if grep -q "### Requirement A2" "${TEST_DIR}/specifications/FileA.md"; then
    echo "❌ FAILED: Element was not removed from FileA.md"
    OVERALL_RESULT=1
  fi

  # Verify element added to FileC
  if ! grep -q "### Requirement A2" "${TEST_DIR}/specifications/FileC.md"; then
    echo "❌ FAILED: Element was not added to FileC.md"
    OVERALL_RESULT=1
  else
    echo "✓ Element moved from FileA.md to FileC.md" >> "${TEST_DIR}/test_results.log"
  fi

  # Verify FileA and FileC timestamps changed, FileB unchanged
  TIMESTAMP_A_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileA.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileA.md")
  TIMESTAMP_B_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileB.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileB.md")
  TIMESTAMP_C_AFTER=$(stat -c %Y "${TEST_DIR}/specifications/FileC.md" 2>/dev/null || stat -f %m "${TEST_DIR}/specifications/FileC.md")

  if [ "$TIMESTAMP_A_AFTER" -le "$TIMESTAMP_A_BEFORE" ]; then
    echo "❌ FAILED: FileA.md timestamp did not change"
    OVERALL_RESULT=1
  fi

  if [ "$TIMESTAMP_C_AFTER" -le "$TIMESTAMP_C_BEFORE" ]; then
    echo "❌ FAILED: FileC.md timestamp did not change"
    OVERALL_RESULT=1
  fi

  if [ "$TIMESTAMP_B_AFTER" -ne "$TIMESTAMP_B_BEFORE" ]; then
    echo "❌ FAILED: FileB.md was modified but should not have been"
    OVERALL_RESULT=1
  else
    echo "✓ Only affected files were modified" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 4: Verify Content Integrity
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 4: Verify file content integrity..." >> "${TEST_DIR}/test_results.log"

# Parse files to verify structure is still valid
set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
set -e

echo "Validation exit code: $VALIDATE_EXIT" >> "${TEST_DIR}/test_results.log"
echo "Validation output: $VALIDATE_OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $VALIDATE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after manipulations"
  echo "Output: $VALIDATE_OUTPUT"
  OVERALL_RESULT=1
else
  echo "✓ Model validation passed - file structure maintained" >> "${TEST_DIR}/test_results.log"
fi

# ==================================
# Final Result
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
if [ $OVERALL_RESULT -eq 0 ]; then
  echo "✓ All file persistence tests passed" >> "${TEST_DIR}/test_results.log"
  exit 0
else
  echo "❌ One or more file persistence tests failed" >> "${TEST_DIR}/test_results.log"
  cat "${TEST_DIR}/test_results.log"
  exit 1
fi
