#!/bin/bash

# Test: Full Relations Insertion (--with-full-relations flag)
# -----------------------------------------------------------
# This test verifies that format --with-full-relations correctly inserts
# all registered relations (user-created and auto-generated) into elements.
#
# Test cases:
# 1. Basic inverse relation insertion (refinedBy -> refine)
# 2. Multiple inverse relation types (derivedFrom -> derive, verifiedBy -> verify)
# 3. Idempotency (running twice produces same result)
# 4. Default behavior unchanged (without flag, no auto-generated relations)
# 5. Preview mode shows diff without modifying files

set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Helper function to compare files and show diff on failure
assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# Test 1: Verify initial state validates
echo "Test 1: Verify initial state validates"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Initial state should validate"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test 2: Default format --fix should NOT insert auto-generated relations
echo "Test 2: Default format preserves only user-created relations"
cp "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/specifications/Requirements.md.backup"

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" format --fix 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: format --fix should succeed"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that Test Constraint does NOT have refine: (auto-generated)
if grep -q "  \* refine:" "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: Default format should NOT insert auto-generated refine relations"
  cat "$TEST_DIR/specifications/Requirements.md"
  exit 1
fi

# Restore backup for next test
cp "$TEST_DIR/specifications/Requirements.md.backup" "$TEST_DIR/specifications/Requirements.md"

# Test 3: format --with-full-relations --fix should insert auto-generated relations
echo "Test 3: --with-full-relations inserts auto-generated relations"

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" format --with-full-relations --fix 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: format --with-full-relations --fix should succeed"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify Requirements.md has auto-generated relations
assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-full-relations-Requirements.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "Requirements.md should have auto-generated relations"

# Verify Verifications.md (should be unchanged since verify is user-created)
assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-full-relations-Verifications.md" \
  "$TEST_DIR/specifications/Verifications.md" \
  "Verifications.md should be unchanged"

# Test 4: Idempotency - running again should produce same result
echo "Test 4: Idempotency check"

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" format --with-full-relations --fix 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Second format run should succeed"
  echo "Output: $OUTPUT"
  exit 1
fi

# Should still match expected (no duplicates)
assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-full-relations-Requirements.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "Running format twice should produce same result (no duplicates)"

# Test 5: Validate after full relations insertion
echo "Test 5: Validate after full relations insertion"

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Model should validate after full relations insertion"
  echo "Output: $OUTPUT"
  exit 1
fi

exit 0
