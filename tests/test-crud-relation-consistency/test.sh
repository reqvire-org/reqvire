#!/bin/bash
set -uo pipefail

# Test: Relation Consistency Maintenance
# ----------------------------------------------------
# Satisfies: specifications/Verifications/ElementManipulationTests.md#relation-consistency-test
#
# Acceptance Criteria:
# - After element creation: bidirectional relations are properly established
# - After element deletion: both forward and backward relations are removed
# - After element move: both forward and backward relations are updated
# - Model validation passes after each operation
# - No dangling or inconsistent relations exist
#
# Test Criteria:
# - Commands exit with success (0) return code
# - Bidirectional relations are maintained correctly
# - Model validates successfully after each operation

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "====================================" >> "${TEST_DIR}/test_results.log"
echo "Relation Consistency Tests" >> "${TEST_DIR}/test_results.log"
echo "====================================" >> "${TEST_DIR}/test_results.log"

# Track overall test result
OVERALL_RESULT=0

# ==================================
# Setup: Create test model with bidirectional relations
# ==================================
mkdir -p "${TEST_DIR}/specifications/Verifications"

cat > "${TEST_DIR}/specifications/Requirements.md" << 'EOF'
# Elements


### Root Requirement

This is the root requirement.

#### Metadata
  * type: user-requirement

### Derived Requirement 1

This requirement is derived from root.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-requirement
  * verifiedBy: Verifications/Tests.md#test-1

### Derived Requirement 2

This requirement is also derived from root.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-requirement
  * verifiedBy: Verifications/Tests.md#test-2
EOF

cat > "${TEST_DIR}/specifications/Verifications/Tests.md" << 'EOF'
# Elements


### Test 1

This test verifies derived requirement 1.

#### Metadata
  * type: test-verification

#### Relations
  * verify: ../Requirements.md#derived-requirement-1

### Test 2

This test verifies derived requirement 2.

#### Metadata
  * type: test-verification

#### Relations
  * verify: ../Requirements.md#derived-requirement-2
EOF

# ==================================
# Test 1: Create Element with Relations - Verify Bidirectional Consistency
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 1: Create element with relations..." >> "${TEST_DIR}/test_results.log"

NEW_ELEMENT='### Derived Requirement 3

This is a new derived requirement.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-requirement
'

set +e
OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md "System Requirements" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Add command failed"
  echo "Output: $OUTPUT"
  OVERALL_RESULT=1
else
  # Verify element was created
  if ! grep -q "### Derived Requirement 3" "${TEST_DIR}/specifications/Requirements.md"; then
    echo "❌ FAILED: Element was not created"
    OVERALL_RESULT=1
  fi

  # Verify forward relation exists (derivedFrom)
  if ! grep -A 5 "### Derived Requirement 3" "${TEST_DIR}/specifications/Requirements.md" | grep -q "derivedFrom:.*#root-requirement"; then
    echo "❌ FAILED: Forward relation (derivedFrom) not found"
    OVERALL_RESULT=1
  fi

  # Verify model validates (bidirectional consistency check)
  set +e
  VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  VALIDATE_EXIT=$?
  set -e

  if [ $VALIDATE_EXIT -ne 0 ]; then
    echo "❌ FAILED: Model validation failed after create"
    echo "Output: $VALIDATE_OUTPUT"
    OVERALL_RESULT=1
  else
    echo "✓ Element created with bidirectional relations" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 2: Delete Element - Verify Bidirectional Relation Removal
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 2: Delete element with incoming relations..." >> "${TEST_DIR}/test_results.log"

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Derived Requirement 1" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Delete command failed"
  echo "Output: $OUTPUT"
  OVERALL_RESULT=1
else
  # Verify element was deleted
  if grep -q "### Derived Requirement 1" "${TEST_DIR}/specifications/Requirements.md"; then
    echo "❌ FAILED: Element was not deleted"
    OVERALL_RESULT=1
  fi

  # Verify incoming relation removed (verify relation from Test 1)
  if grep -q "verify:.*derived-requirement-1" "${TEST_DIR}/specifications/Verifications/Tests.md"; then
    echo "❌ FAILED: Incoming verify relation was not removed"
    OVERALL_RESULT=1
  fi

  # Verify model validates
  set +e
  VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  VALIDATE_EXIT=$?
  set -e

  if [ $VALIDATE_EXIT -ne 0 ]; then
    echo "❌ FAILED: Model validation failed after delete"
    echo "Output: $VALIDATE_OUTPUT"
    OVERALL_RESULT=1
  else
    echo "✓ Element deleted with bidirectional relation cleanup" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 3: Move Element - Verify Bidirectional Relation Updates
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 3: Move element with incoming relations..." >> "${TEST_DIR}/test_results.log"

# Create target file
cat > "${TEST_DIR}/specifications/OtherRequirements.md" << 'EOF'
# Elements

EOF

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Derived Requirement 2" "specifications/OtherRequirements.md" "Other Section" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Move command failed"
  echo "Output: $OUTPUT"
  OVERALL_RESULT=1
else
  # Verify element was moved
  if grep -q "### Derived Requirement 2" "${TEST_DIR}/specifications/Requirements.md"; then
    echo "❌ FAILED: Element was not removed from source"
    OVERALL_RESULT=1
  fi

  if ! grep -q "### Derived Requirement 2" "${TEST_DIR}/specifications/OtherRequirements.md"; then
    echo "❌ FAILED: Element was not added to target"
    OVERALL_RESULT=1
  fi

  # Verify incoming relation updated (verify relation from Test 2)
  # Use more precise pattern to avoid matching OtherRequirements.md
  if grep -q "verify:.*\.\.\/Requirements\.md#derived-requirement-2" "${TEST_DIR}/specifications/Verifications/Tests.md"; then
    echo "❌ FAILED: Incoming verify relation was not updated (still points to old location)"
    OVERALL_RESULT=1
  fi

  if ! grep -q "verify:.*OtherRequirements\.md#derived-requirement-2" "${TEST_DIR}/specifications/Verifications/Tests.md"; then
    echo "❌ FAILED: Incoming verify relation was not updated to new location"
    OVERALL_RESULT=1
  fi

  # Verify outgoing relation preserved (derivedFrom to root-requirement)
  if ! grep -A 5 "### Derived Requirement 2" "${TEST_DIR}/specifications/OtherRequirements.md" | grep -q "derivedFrom:.*Requirements.md#root-requirement"; then
    echo "❌ FAILED: Outgoing derivedFrom relation was not preserved"
    OVERALL_RESULT=1
  fi

  # Verify outgoing verifiedBy relation preserved and moved with element
  if ! grep -A 10 "### Derived Requirement 2" "${TEST_DIR}/specifications/OtherRequirements.md" | grep -q "verifiedBy:.*Verifications/Tests.md#test-2"; then
    echo "❌ FAILED: Outgoing verifiedBy relation was not preserved during move"
    OVERALL_RESULT=1
  fi

  # Verify model validates
  set +e
  VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  VALIDATE_EXIT=$?
  set -e

  if [ $VALIDATE_EXIT -ne 0 ]; then
    echo "❌ FAILED: Model validation failed after move"
    echo "Output: $VALIDATE_OUTPUT"
    OVERALL_RESULT=1
  else
    echo "✓ Element moved with bidirectional relation updates" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 4: Move Verification Element - Verify VerifiedBy Relations Updated
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 4: Move verification element and check verifiedBy relation updates..." >> "${TEST_DIR}/test_results.log"

# Create target file for verification
cat > "${TEST_DIR}/specifications/SystemTests.md" << 'EOF'
# Elements

EOF

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Test 2" "specifications/SystemTests.md" "Integration Tests" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Move verification command failed"
  echo "Output: $OUTPUT"
  OVERALL_RESULT=1
else
  # Verify verification element was moved
  if grep -q "### Test 2" "${TEST_DIR}/specifications/Verifications/Tests.md"; then
    echo "❌ FAILED: Verification was not removed from source"
    OVERALL_RESULT=1
  fi

  if ! grep -q "### Test 2" "${TEST_DIR}/specifications/SystemTests.md"; then
    echo "❌ FAILED: Verification was not added to target"
    OVERALL_RESULT=1
  fi

  # CRITICAL: Verify verifiedBy relation in OtherRequirements.md was updated
  # (Derived Requirement 2 was moved to OtherRequirements.md in Test 3)
  if grep -q "verifiedBy:.*Verifications/Tests\.md#test-2" "${TEST_DIR}/specifications/OtherRequirements.md"; then
    echo "❌ FAILED: VerifiedBy relation still points to old verification location"
    cat "${TEST_DIR}/specifications/OtherRequirements.md"
    OVERALL_RESULT=1
  fi

  if ! grep -q "verifiedBy:.*SystemTests\.md#test-2" "${TEST_DIR}/specifications/OtherRequirements.md"; then
    echo "❌ FAILED: VerifiedBy relation was not updated to new verification location"
    cat "${TEST_DIR}/specifications/OtherRequirements.md"
    OVERALL_RESULT=1
  fi

  # Verify verify relation moved with the verification element and path updated
  # The verification moved from Verifications/Tests.md to SystemTests.md
  # Both are at specifications/ level, so relative path should be OtherRequirements.md
  if ! grep -A 10 "### Test 2" "${TEST_DIR}/specifications/SystemTests.md" | grep -q "verify:.*OtherRequirements\.md#derived-requirement-2"; then
    echo "❌ FAILED: Verify relation was not updated correctly during verification move"
    echo "Expected: verify: OtherRequirements.md#derived-requirement-2"
    echo "Actual content:"
    grep -A 10 "### Test 2" "${TEST_DIR}/specifications/SystemTests.md" || true
    OVERALL_RESULT=1
  fi

  # Verify model validates
  set +e
  VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  VALIDATE_EXIT=$?
  set -e

  if [ $VALIDATE_EXIT -ne 0 ]; then
    echo "❌ FAILED: Model validation failed after moving verification"
    echo "Output: $VALIDATE_OUTPUT"
    OVERALL_RESULT=1
  else
    echo "✓ Verification moved and verifiedBy relations updated in requirements" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 5: Delete Root Element - Verify All Derived Relations Removed
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 5: Delete root element with multiple outgoing relations..." >> "${TEST_DIR}/test_results.log"

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Root Requirement" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Delete root requirement command failed"
  echo "Output: $OUTPUT"
  OVERALL_RESULT=1
else
  # Verify root element was deleted
  if grep -q "### Root Requirement" "${TEST_DIR}/specifications/Requirements.md"; then
    echo "❌ FAILED: Root element was not deleted"
    OVERALL_RESULT=1
  fi

  # Verify all derivedFrom relations pointing to root are removed
  if grep -q "derivedFrom:.*#root-requirement" "${TEST_DIR}/specifications/Requirements.md"; then
    echo "❌ FAILED: derivedFrom relations to deleted root were not removed from Requirements.md"
    OVERALL_RESULT=1
  fi

  if grep -q "derivedFrom:.*Requirements.md#root-requirement" "${TEST_DIR}/specifications/OtherRequirements.md"; then
    echo "❌ FAILED: derivedFrom relations to deleted root were not removed from OtherRequirements.md"
    OVERALL_RESULT=1
  fi

  # After deleting root, child elements (Derived Req 2 & 3) are orphaned
  # Verify that validation correctly identifies these orphaned elements
  set +e
  VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  VALIDATE_EXIT=$?
  set -e

  if [ $VALIDATE_EXIT -eq 0 ]; then
    echo "❌ FAILED: Model validation should fail after deleting root (orphaned children)"
    OVERALL_RESULT=1
  else
    # Verify specific expected errors for orphaned elements
    if ! echo "$VALIDATE_OUTPUT" | grep -q "Missing parent relation.*Derived Requirement 2"; then
      echo "❌ FAILED: Expected validation error for orphaned Derived Requirement 2"
      OVERALL_RESULT=1
    fi

    if ! echo "$VALIDATE_OUTPUT" | grep -q "Missing parent relation.*Derived Requirement 3"; then
      echo "❌ FAILED: Expected validation error for orphaned Derived Requirement 3"
      OVERALL_RESULT=1
    fi

    echo "✓ Root element deleted and orphaned children correctly identified" >> "${TEST_DIR}/test_results.log"
  fi
fi

# ==================================
# Test 6: Final Consistency Check - Verify Expected Validation Errors
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
echo "Test 6: Final consistency check - verify expected validation errors..." >> "${TEST_DIR}/test_results.log"

set +e
SUMMARY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json 2>&1)
SUMMARY_EXIT=$?
set -e

# Compare with expected output
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-summary.json" <(echo "$SUMMARY_OUTPUT"); then
  echo "❌ FAILED: Summary output does not match expected"
  echo ""
  echo "Differences shown above (expected vs actual)"
  OVERALL_RESULT=1
else
  echo "✓ Summary correctly reports orphaned elements as validation errors" >> "${TEST_DIR}/test_results.log"
fi

# ==================================
# Final Result
# ==================================
echo "" >> "${TEST_DIR}/test_results.log"
if [ $OVERALL_RESULT -eq 0 ]; then
  echo "✓ All relation consistency tests passed" >> "${TEST_DIR}/test_results.log"
  exit 0
else
  echo "❌ One or more relation consistency tests failed" >> "${TEST_DIR}/test_results.log"
  cat "${TEST_DIR}/test_results.log"
  exit 1
fi
