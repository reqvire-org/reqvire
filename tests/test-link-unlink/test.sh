#!/bin/bash
set -uo pipefail

# Test: Link/Unlink Commands
#
# Satisfies: Link Command Verification, Unlink Command Verification
#
# This test uses expected output files and diff comparisons.
# Each test step modifies files and compares against expected/*.md files.

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

echo "===================================="
echo "Link/Unlink Commands Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Basic link by element name
# ==================================
echo "Test 1: Link relation by element name..."

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Feature Requirement" "derivedFrom" "Another Requirement" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-link.md" "$TEST_DIR/specifications/Requirements.md" "File content after link does not match expected"

echo "Test 1 passed"
echo ""

# ==================================
# Test 2: Link idempotency
# ==================================
echo "Test 2: Link idempotency (duplicate link)..."

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Feature Requirement" "derivedFrom" "Another Requirement" > /dev/null 2>&1

# File should be unchanged (no duplicate)
assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-link.md" "$TEST_DIR/specifications/Requirements.md" "Duplicate link should not modify file"

echo "Test 2 passed"
echo ""

# ==================================
# Test 3: Link adds derive to parent
# ==================================
echo "Test 3: Link derive relation to parent..."

cd "$TEST_DIR" && "$REQVIRE_BIN" link "System Requirements" "derive" "No Relations Requirement" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/03-link-creates-subsection.md" "$TEST_DIR/specifications/Requirements.md" "Link derive result does not match expected"

echo "Test 3 passed"
echo ""

# ==================================
# Test 4: Link verification relation
# ==================================
echo "Test 4: Link verifiedBy relation..."

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Another Requirement" "verifiedBy" "Orphan Test" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/04-link-verifiedby.md" "$TEST_DIR/specifications/Requirements.md" "verifiedBy link result does not match expected"

echo "Test 4 passed"
echo ""

# ==================================
# Test 5: Unlink command (auto-detects relation type)
# ==================================
echo "Test 5: Unlink command..."

cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Feature Requirement" "Another Requirement" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/05-after-unlink.md" "$TEST_DIR/specifications/Requirements.md" "File content after unlink does not match expected"

echo "Test 5 passed"
echo ""

# ==================================
# Test 6: Unlink removes relation from parent
# ==================================
echo "Test 6: Unlink removes derive relation..."

cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "System Requirements" "No Relations Requirement" > /dev/null 2>&1
cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "No Relations Requirement" "System Requirements" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/06-unlink-removes-subsection.md" "$TEST_DIR/specifications/Requirements.md" "Unlink should remove relation"

echo "Test 6 passed"
echo ""

# ==================================
# Test 7: Link to non-existent target fails
# ==================================
echo "Test 7: Link to non-existent target fails..."

# Reset to a valid state for error tests
cp "${TEST_SCRIPT_DIR}/expected/05-after-unlink.md" "$TEST_DIR/specifications/Requirements.md"

set +e
LINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Feature Requirement" "derivedFrom" "Nonexistent Element" 2>&1)
LINK_EXIT=$?
set -e

if [ $LINK_EXIT -eq 0 ]; then
  echo "FAILED: Link to non-existent target should fail"
  exit 1
fi

if ! echo "$LINK_OUTPUT" | grep -qi "not found\|does not exist\|error"; then
  echo "FAILED: Error message should indicate target not found"
  echo "$LINK_OUTPUT"
  exit 1
fi

echo "Test 7 passed"
echo ""

# ==================================
# Test 8: Link with invalid relation type fails
# ==================================
echo "Test 8: Link with invalid relation type fails..."

set +e
LINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Feature Requirement" "invalidRelation" "Another Requirement" 2>&1)
LINK_EXIT=$?
set -e

if [ $LINK_EXIT -eq 0 ]; then
  echo "FAILED: Link with invalid relation type should fail"
  exit 1
fi

if ! echo "$LINK_OUTPUT" | grep -qi "invalid\|unknown\|unsupported\|relation type"; then
  echo "FAILED: Error message should indicate invalid relation type"
  echo "$LINK_OUTPUT"
  exit 1
fi

echo "Test 8 passed"
echo ""

# ==================================
# Test 9: Unlink non-existent relation/attachment fails
# ==================================
echo "Test 9: Unlink non-existent relation fails..."

set +e
UNLINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Feature Requirement" "Nonexistent Element XYZ" 2>&1)
UNLINK_EXIT=$?
set -e

if [ $UNLINK_EXIT -eq 0 ]; then
  echo "FAILED: Unlink non-existent relation should fail"
  exit 1
fi

if ! echo "$UNLINK_OUTPUT" | grep -qi "not found\|does not exist\|error\|no relation"; then
  echo "FAILED: Error message should indicate relation not found"
  echo "$UNLINK_OUTPUT"
  exit 1
fi

echo "Test 9 passed"
echo ""

# ==================================
# Test 10: Dry-run mode
# ==================================
echo "Test 10: Dry-run mode..."

# Reset to valid state for dry-run test
cp "${TEST_SCRIPT_DIR}/expected/05-after-unlink.md" "$TEST_DIR/specifications/Requirements.md"

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Feature Requirement" "trace" "Another Requirement" --dry-run > /dev/null 2>&1

# File should be unchanged
assert_file_matches "${TEST_SCRIPT_DIR}/expected/05-after-unlink.md" "$TEST_DIR/specifications/Requirements.md" "Dry-run mode should not modify the file"

echo "Test 10 passed"
echo ""

# ==================================
# Test 11: Link trace to external URL
# ==================================
echo "Test 11: Link trace to external URL..."

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Feature Requirement" "trace" "https://example.com/spec.html" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/11-link-external-url.md" "$TEST_DIR/specifications/Requirements.md" "Link to external URL does not match expected"

echo "Test 11 passed"
echo ""

echo "===================================="
echo "All Link/Unlink tests passed"
echo "===================================="
exit 0
