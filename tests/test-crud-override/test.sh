#!/bin/bash
set -uo pipefail

# Test: Create Element Override Operation
#
# Satisfies: requirements/System/Operations/Verifications/ElementManipulationVerifications.md#create-element-override-test
#
# Acceptance Criteria:
# - Add command with --override replaces existing element with same name
# - Operation reports as "Updated" rather than "Added"
# - New content replaces old content completely
# - Model validates after override

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
# Test 4: Dry-run override should not modify files
# ==================================

# Store current file content
cp "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/before-dryrun.md.bak"

DRYRUN_ELEMENT='### Feature A

Dry run should not persist this.

#### Metadata
  * type: user-requirement
'

set +e
DRYRUN_OUTPUT=$(cd "$TEST_DIR" && echo "$DRYRUN_ELEMENT" | "$REQVIRE_BIN" add specifications/Requirements.md --override --dry-run 2>&1)
DRYRUN_EXIT=$?
set -e

if [ $DRYRUN_EXIT -ne 0 ]; then
  echo "❌ FAILED: Dry-run override should succeed"
  echo "$DRYRUN_OUTPUT"
  exit 1
fi

# Verify file was not modified (should still match post-Feature B state)
assert_file_matches "$TEST_DIR/before-dryrun.md.bak" \
  "$TEST_DIR/specifications/Requirements.md" \
  "Dry-run should not modify files"

# Check that output shows "Updated" for dry-run override
if ! echo "$DRYRUN_OUTPUT" | grep -q "Updated"; then
  echo "❌ FAILED: Dry-run override should show 'Updated'"
  echo "Got: $DRYRUN_OUTPUT"
  exit 1
fi

exit 0
