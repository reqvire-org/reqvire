#!/bin/bash
set -uo pipefail

# Test: Merge Elements Capability
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

assert_output_matches() {
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

# ==================================
# Test 10: Bug reproduction - Multiple source merge with many elements
# ==================================
echo "Test 10: Bug reproduction - Multiple source merge..."

# Reset
cp "${TEST_SCRIPT_DIR}/specifications/Requirements.md" "$TEST_DIR/specifications/Requirements.md"
cp "${TEST_SCRIPT_DIR}/specifications/Verifications.md" "$TEST_DIR/specifications/Verifications.md"

# This reproduces the bug where merging multiple sources causes unexpected deletion
# Attempting to merge 3 sources into Target Requirement
set +e
MERGE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "Source Requirement One" "Source Requirement Two" "Unrelated Requirement" 2>&1)
MERGE_EXIT=$?
set -e

# Save output for inspection
echo "$MERGE_OUTPUT" > "${TEST_DIR}/test10_merge_output.txt"

if [ $MERGE_EXIT -ne 0 ]; then
  echo "FAILED: Multi-source merge failed with exit code $MERGE_EXIT"
  echo "Output:"
  cat "${TEST_DIR}/test10_merge_output.txt"
  exit 1
fi

# Verify target still exists
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="Target Requirement" --short 2>&1)

if ! echo "$SEARCH_OUTPUT" | grep -q "Target Requirement"; then
  echo "FAILED: Target requirement should still exist after merge"
  echo "Search output: $SEARCH_OUTPUT"
  echo "Merge output:"
  cat "${TEST_DIR}/test10_merge_output.txt"
  echo ""
  echo "File contents:"
  cat "$TEST_DIR/specifications/Requirements.md"
  exit 1
fi

# Verify sources are deleted
for SOURCE in "Source Requirement One" "Source Requirement Two" "Unrelated Requirement"; do
  SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="$SOURCE" --short 2>&1)
  if echo "$SEARCH_OUTPUT" | grep -q "$SOURCE"; then
    echo "FAILED: Source '$SOURCE' should have been deleted"
    exit 1
  fi
done

# Verify other elements are not affected
for ELEMENT in "System Requirements" "Child Requirement" "Another Link" "Placeholder Requirement"; do
  SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-name="$ELEMENT" --short 2>&1)
  if ! echo "$SEARCH_OUTPUT" | grep -q "$ELEMENT"; then
    echo "FAILED: Unrelated element '$ELEMENT' should not be deleted"
    echo "File contents:"
    cat "$TEST_DIR/specifications/Requirements.md"
    exit 1
  fi
done

# Verify model still validates
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "FAILED: Model should validate after merge"
  echo "Validation output:"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "Test 10 passed"
echo ""

# ==================================
# Test 11: Merge rejects multi-root hierarchy ownership violation
# ==================================
echo "Test 11: Merge rejects multi-root hierarchy ownership violation..."

# Reset
cp "${TEST_SCRIPT_DIR}/specifications/Requirements.md" "$TEST_DIR/specifications/Requirements.md"
cp "${TEST_SCRIPT_DIR}/specifications/Verifications.md" "$TEST_DIR/specifications/Verifications.md"

cat >> "$TEST_DIR/specifications/Requirements.md" << 'EOF'

### External Capability

External root branch for ownership violation scenario.

#### Metadata
  * type: capability
---

### External Source

Source requirement under a different root.

#### Metadata
  * type: requirement

#### Relations
  * specify: [External Capability](#external-capability)
---
EOF

BEFORE_TEST11="$(mktemp /tmp/reqvire-merge-before11-XXXXXX.md)"
cp "$TEST_DIR/specifications/Requirements.md" "$BEFORE_TEST11"

set +e
MERGE_MULTIROOT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Target Requirement" "External Source" 2>&1)
MERGE_MULTIROOT_EXIT=$?
set -e

if [ $MERGE_MULTIROOT_EXIT -eq 0 ]; then
  echo "FAILED: Merge should fail deterministically with single-root ownership violation"
  exit 1
fi

printf "%s\n" "$MERGE_MULTIROOT_OUTPUT" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | sed -E 's/^\[[^]]+\][[:space:]]*//' \
  | grep -E "Single-root hierarchy ownership violation" \
  > "$TEST_DIR/output/11-merge-multiroot-error.actual.txt"

assert_output_matches \
  "${TEST_SCRIPT_DIR}/expected/11-merge-multiroot-error.txt" \
  "$TEST_DIR/output/11-merge-multiroot-error.actual.txt" \
  "Deterministic single-root error output mismatch for merge"

if ! cmp -s "$TEST_DIR/specifications/Requirements.md" "$BEFORE_TEST11"; then
  echo "FAILED: Failed merge must not modify files"
  diff -u "$BEFORE_TEST11" "$TEST_DIR/specifications/Requirements.md"
  exit 1
fi

rm -f "$BEFORE_TEST11"

echo "Test 11 passed"
echo ""

echo "===================================="
echo "All Merge Elements tests passed"
echo "===================================="
exit 0
