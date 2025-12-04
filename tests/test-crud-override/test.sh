#!/bin/bash
set -uo pipefail

# Test: Create Element Override Operation
#
# Satisfies: requirements/System/Operations/Verifications/ElementManipulationVerifications.md#create-element-test
#
# Acceptance Criteria:
# - Add command with --override replaces existing element with same name
# - Operation reports as "Updated" rather than "Added"
# - New content replaces old content completely
# - Model validates after override
# - Override is rejected when element has children that would be orphaned
# - Error message lists orphaned children with resolution guidance
# - Override succeeds when element has no children

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Helper function to compare files and show diff on failure
assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "❌ FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# ==================================
# Test 1: Override existing element
# ==================================

OVERRIDE_ELEMENT='### Feature A

This is the UPDATED feature A with new content.

#### Details
New details section added.

#### Metadata
  * type: user-requirement
'

set +e
OVERRIDE_OUTPUT=$(cd "$TEST_DIR" && echo "$OVERRIDE_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md --override 2>&1)
OVERRIDE_EXIT=$?
set -e

if [ $OVERRIDE_EXIT -ne 0 ]; then
  echo "❌ FAILED: Override command failed with exit code $OVERRIDE_EXIT"
  echo "$OVERRIDE_OUTPUT"
  exit 1
fi

# Check operation reports "Updated" not "Added"
if ! echo "$OVERRIDE_OUTPUT" | grep -q "Updated"; then
  echo "❌ FAILED: Output should show 'Updated' not 'Added'"
  echo "Got: $OVERRIDE_OUTPUT"
  exit 1
fi

assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-override.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "File content after override does not match expected"

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after override"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

# ==================================
# Test 2: Override without flag should fail
# ==================================

DUPLICATE_ELEMENT='### Feature A

This is a duplicate.

#### Metadata
  * type: user-requirement
'

set +e
DUPLICATE_OUTPUT=$(cd "$TEST_DIR" && echo "$DUPLICATE_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md 2>&1)
DUPLICATE_EXIT=$?
set -e

if [ $DUPLICATE_EXIT -eq 0 ]; then
  echo "❌ FAILED: Adding duplicate without --override should fail"
  exit 1
fi

if ! echo "$DUPLICATE_OUTPUT" | grep -qi "duplicate\|already exists\|unique"; then
  echo "❌ FAILED: Error message should mention duplicate/uniqueness"
  echo "Got: $DUPLICATE_OUTPUT"
  exit 1
fi

# ==================================
# Test 3: Override non-existent element (should add)
# ==================================

NEW_ELEMENT='### Feature B

This is a brand new feature.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
'

set +e
ADD_OUTPUT=$(cd "$TEST_DIR" && echo "$NEW_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md --override 2>&1)
ADD_EXIT=$?
set -e

if [ $ADD_EXIT -ne 0 ]; then
  echo "❌ FAILED: Override on non-existent element should succeed"
  echo "$ADD_OUTPUT"
  exit 1
fi

assert_file_matches "${TEST_SCRIPT_DIR}/expected/02-after-add-feature-b.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "File content after adding Feature B does not match expected"

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after adding new element with override"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

# ==================================
# Test 4: Override with orphaned children prevention (should fail)
# ==================================

# Feature A now has Feature B as a child (from Test 3)
# Attempting to override Feature A should be rejected

DRYRUN_ELEMENT='### Feature A

Attempted override that would orphan Feature B.

#### Metadata
  * type: user-requirement
'

set +e
DRYRUN_OUTPUT=$(cd "$TEST_DIR" && echo "$DRYRUN_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md --override --dry-run 2>&1)
DRYRUN_EXIT=$?
set -e

# Override should FAIL due to orphaned children prevention
if [ $DRYRUN_EXIT -eq 0 ]; then
  echo "❌ FAILED: Override should be rejected (would orphan Feature B)"
  echo "$DRYRUN_OUTPUT"
  exit 1
fi

# Verify error message mentions orphaned children
if ! echo "$DRYRUN_OUTPUT" | grep -qi "orphan"; then
  echo "❌ FAILED: Error message should mention orphaned children"
  echo "Got: $DRYRUN_OUTPUT"
  exit 1
fi

# Verify error message lists Feature B
if ! echo "$DRYRUN_OUTPUT" | grep -q "Feature B"; then
  echo "❌ FAILED: Error message should list Feature B as orphaned child"
  echo "Got: $DRYRUN_OUTPUT"
  exit 1
fi

# Verify error provides resolution guidance
if ! echo "$DRYRUN_OUTPUT" | grep -qi "delete.*child\|link.*parent"; then
  echo "❌ FAILED: Error message should provide resolution guidance"
  echo "Got: $DRYRUN_OUTPUT"
  exit 1
fi

# ==================================
# Test 5: Override element without children (should succeed)
# ==================================

# Feature B has no children, so override should succeed

OVERRIDE_B_ELEMENT='### Feature B

This is UPDATED Feature B content.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Feature A](#feature-a)
'

set +e
OVERRIDE_B_OUTPUT=$(cd "$TEST_DIR" && echo "$OVERRIDE_B_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md --override 2>&1)
OVERRIDE_B_EXIT=$?
set -e

if [ $OVERRIDE_B_EXIT -ne 0 ]; then
  echo "❌ FAILED: Override of element without children should succeed"
  echo "$OVERRIDE_B_OUTPUT"
  exit 1
fi

# Check operation reports "Updated"
if ! echo "$OVERRIDE_B_OUTPUT" | grep -q "Updated"; then
  echo "❌ FAILED: Output should show 'Updated'"
  echo "Got: $OVERRIDE_B_OUTPUT"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Model validation failed after override of childless element"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

exit 0
