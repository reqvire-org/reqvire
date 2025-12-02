#!/bin/bash
set -uo pipefail

# Test: Merge Elements Feature
#
# Satisfies: specifications/System/Operations/Verifications/ElementManipulationVerifications.md#merge-elements-test
#
# This test uses expected output files and diff comparisons.

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

# ==================================
# Test 1: Basic merge - two requirements into one
# ==================================
echo "Test 1: Basic merge - two requirements into one..."

cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "Source Requirement One" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-basic-merge.md" "$TEST_DIR/specifications/Requirements.md" "Basic merge result does not match expected"

echo "Test 1 passed"
echo ""

# ==================================
# Test 2: Verify source is deleted
# ==================================
echo "Test 2: Verify source element was deleted..."

# Search for Source Requirement One - should not find it
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="Source Requirement One" --short 2>&1)

if echo "$SEARCH_OUTPUT" | grep -q "Source Requirement One"; then
  echo "FAILED: Source element should have been deleted"
  exit 1
fi

echo "Test 2 passed"
echo ""

# ==================================
# Test 3: Verify relation redirection
# ==================================
echo "Test 3: Verify relations pointing to source are redirected..."

# Both Unrelated Requirement and Child Requirement had derivedFrom pointing to Source Requirement One
# After merge, they should point to Target Requirement
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="Unrelated Requirement" --json 2>&1)

if ! echo "$SEARCH_OUTPUT" | grep -q "target-requirement"; then
  echo "FAILED: Unrelated Requirement relation should have been redirected to target"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="Child Requirement" --json 2>&1)

if ! echo "$SEARCH_OUTPUT" | grep -q "target-requirement"; then
  echo "FAILED: Child Requirement relation should have been redirected to target"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

echo "Test 3 passed"
echo ""

# ==================================
# Test 4: Multi-element merge
# ==================================
echo "Test 4: Multi-element merge..."

# Reset for multi-merge test - copy original
cp "${TEST_SCRIPT_DIR}/specifications/Requirements.md" "$TEST_DIR/specifications/Requirements.md"
cp "${TEST_SCRIPT_DIR}/specifications/Verifications.md" "$TEST_DIR/specifications/Verifications.md"

# Merge two sources at once
cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "Source Requirement One" "Source Requirement Two" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/04-multi-merge.md" "$TEST_DIR/specifications/Requirements.md" "Multi-merge result does not match expected"

echo "Test 4 passed"
echo ""

# ==================================
# Test 5: Type compatibility error
# ==================================
echo "Test 5: Type compatibility error..."

# Reset
cp "${TEST_SCRIPT_DIR}/specifications/Requirements.md" "$TEST_DIR/specifications/Requirements.md"
cp "${TEST_SCRIPT_DIR}/specifications/Verifications.md" "$TEST_DIR/specifications/Verifications.md"

set +e
MERGE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "Test Verification" 2>&1)
MERGE_EXIT=$?
set -e

if [ $MERGE_EXIT -eq 0 ]; then
  echo "FAILED: Merging verification into requirement should fail"
  exit 1
fi

if ! echo "$MERGE_OUTPUT" | grep -qi "type mismatch\|type.*mismatch"; then
  echo "FAILED: Error message should mention type mismatch"
  echo "$MERGE_OUTPUT"
  exit 1
fi

echo "Test 5 passed"
echo ""

# ==================================
# Test 6: Merge into self error
# ==================================
echo "Test 6: Merge into self error..."

set +e
MERGE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "Target Requirement" 2>&1)
MERGE_EXIT=$?
set -e

if [ $MERGE_EXIT -eq 0 ]; then
  echo "FAILED: Merging element into itself should fail"
  exit 1
fi

if ! echo "$MERGE_OUTPUT" | grep -qi "itself"; then
  echo "FAILED: Error message should mention merging into itself"
  echo "$MERGE_OUTPUT"
  exit 1
fi

echo "Test 6 passed"
echo ""

# ==================================
# Test 7: Non-existent source error
# ==================================
echo "Test 7: Non-existent source error..."

set +e
MERGE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "Non Existent Element" 2>&1)
MERGE_EXIT=$?
set -e

if [ $MERGE_EXIT -eq 0 ]; then
  echo "FAILED: Merging non-existent element should fail"
  exit 1
fi

if ! echo "$MERGE_OUTPUT" | grep -qi "not found"; then
  echo "FAILED: Error message should mention element not found"
  echo "$MERGE_OUTPUT"
  exit 1
fi

echo "Test 7 passed"
echo ""

# ==================================
# Test 8: Dry run mode
# ==================================
echo "Test 8: Dry run mode..."

# Reset
cp "${TEST_SCRIPT_DIR}/specifications/Requirements.md" "$TEST_DIR/specifications/Requirements.md"
cp "${TEST_SCRIPT_DIR}/specifications/Verifications.md" "$TEST_DIR/specifications/Verifications.md"

# Create backup outside TEST_DIR to avoid being parsed as specifications
BACKUP_DIR="/tmp/reqvire-merge-backup-$$"
mkdir -p "$BACKUP_DIR"
cp "$TEST_DIR/specifications/Requirements.md" "$BACKUP_DIR/backup_requirements.md"

cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "Source Requirement One" --dry-run > /dev/null 2>&1

# File should be unchanged
if ! cmp -s "$TEST_DIR/specifications/Requirements.md" "$BACKUP_DIR/backup_requirements.md"; then
  echo "FAILED: Dry-run mode should not modify files"
  diff "$BACKUP_DIR/backup_requirements.md" "$TEST_DIR/specifications/Requirements.md"
  rm -rf "$BACKUP_DIR"
  exit 1
fi

rm -rf "$BACKUP_DIR"
echo "Test 8 passed"
echo ""

# ==================================
# Test 9: Relation deduplication
# ==================================
echo "Test 9: Relation deduplication..."

# Reset
cp "${TEST_SCRIPT_DIR}/specifications/Requirements.md" "$TEST_DIR/specifications/Requirements.md"
cp "${TEST_SCRIPT_DIR}/specifications/Verifications.md" "$TEST_DIR/specifications/Verifications.md"

# Both source and target have derivedFrom: System Requirements
# After merge, should only appear once
cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "Source Requirement One" > /dev/null 2>&1

# Count derivedFrom in target element (should be 1)
DEDUP_COUNT=$(grep -A 20 "### Target Requirement" "$TEST_DIR/specifications/Requirements.md" | grep -c "derivedFrom.*system-requirements" || true)

if [ "$DEDUP_COUNT" -gt 1 ]; then
  echo "FAILED: Duplicate relations should be deduplicated"
  echo "Found $DEDUP_COUNT instances of derivedFrom: System Requirements"
  exit 1
fi

echo "Test 9 passed"
echo ""

echo "===================================="
echo "All Merge Elements tests passed"
echo "===================================="
exit 0
