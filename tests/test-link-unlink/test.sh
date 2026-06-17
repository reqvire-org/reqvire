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

echo "===================================="
echo "Link/Unlink Commands Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Basic link by element name
# ==================================
echo "Test 1: Link relation by element name..."

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Capability Requirement" "derivedFrom" "Another Requirement" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-link.md" "$TEST_DIR/specifications/Requirements.md" "File content after link does not match expected"

echo "Test 1 passed"
echo ""

# ==================================
# Test 2: Link duplicate returns error
# ==================================
echo "Test 2: Link duplicate returns error..."

set +e
LINK_DUP_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Capability Requirement" "derivedFrom" "Another Requirement" 2>&1)
LINK_DUP_EXIT=$?
set -e

if [ $LINK_DUP_EXIT -eq 0 ]; then
  echo "FAILED: Duplicate link should fail with error"
  exit 1
fi

if ! echo "$LINK_DUP_OUTPUT" | grep -qi "already exists"; then
  echo "FAILED: Error message should mention 'already exists'"
  echo "$LINK_DUP_OUTPUT"
  exit 1
fi

# File should be unchanged (operation failed)
assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-link.md" "$TEST_DIR/specifications/Requirements.md" "Failed link should not modify file"

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
# Test 5: Unlink command (auto-detects relation type) + Setup for Test 6
# ==================================
echo "Test 5: Unlink command..."

cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Capability Requirement" "Another Requirement" > /dev/null 2>&1

# Add another parent to "No Relations Requirement" so it won't be orphaned in Test 6
# Link creates derive on Capability Requirement (in file) and derivedFrom on No Relations (in-memory only)
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Capability Requirement" "derive" "No Relations Requirement" > /dev/null 2>&1

# Format with --with-full-relations to write the opposite derivedFrom to file
# This ensures No Relations has a parent in file after Test 6 unlinks System Requirements
cd "$TEST_DIR" && "$REQVIRE_BIN" format --with-full-relations --fix > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/05-after-unlink.md" "$TEST_DIR/specifications/Requirements.md" "File content after unlink does not match expected"

echo "Test 5 passed"
echo ""

# ==================================
# Test 6: Unlink removes relation AND its opposite
# ==================================
echo "Test 6: Unlink removes derive relation and its derivedFrom opposite..."

cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "System Requirements" "No Relations Requirement" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/06-unlink-removes-subsection.md" "$TEST_DIR/specifications/Requirements.md" "Unlink should remove both the relation and its opposite"

echo "Test 6 passed"
echo ""

# ==================================
# Test 7: Link to non-existent target fails
# ==================================
echo "Test 7: Link to non-existent target fails..."

# Reset to a valid state for error tests
cp "${TEST_SCRIPT_DIR}/expected/05-after-unlink.md" "$TEST_DIR/specifications/Requirements.md"

set +e
LINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Capability Requirement" "derivedFrom" "Nonexistent Element" 2>&1)
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
LINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Capability Requirement" "invalidRelation" "Another Requirement" 2>&1)
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
UNLINK_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Capability Requirement" "Nonexistent Element XYZ" 2>&1)
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

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Capability Requirement" "trace" "Another Requirement" --dry-run > /dev/null 2>&1

# File should be unchanged
assert_file_matches "${TEST_SCRIPT_DIR}/expected/05-after-unlink.md" "$TEST_DIR/specifications/Requirements.md" "Dry-run mode should not modify the file"

echo "Test 10 passed"
echo ""

# ==================================
# Test 11: Link trace to external URL succeeds
# ==================================
echo "Test 11: Link trace to external URL succeeds..."

set +e
LINK_URL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Capability Requirement" "trace" "https://example.com/spec.html" 2>&1)
LINK_URL_EXIT=$?
set -e

if [ $LINK_URL_EXIT -ne 0 ]; then
  echo "FAILED: Link to external URL should succeed"
  echo "$LINK_URL_OUTPUT"
  exit 1
fi

assert_file_matches "${TEST_SCRIPT_DIR}/expected/11-link-external-url.md" "$TEST_DIR/specifications/Requirements.md" "Link to external URL does not match expected"

echo "Test 11 passed"
echo ""

# ==================================
# Test 12: Attaching external URL fails with helpful message
# ==================================
echo "Test 12: Attaching external URL fails with helpful message..."

set +e
ATTACH_URL_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Capability Requirement" attaching "https://example.com/doc.pdf" 2>&1)
ATTACH_URL_EXIT=$?
set -e

if [ $ATTACH_URL_EXIT -eq 0 ]; then
  echo "FAILED: Attaching external URL should fail"
  exit 1
fi

if ! echo "$ATTACH_URL_OUTPUT" | grep -qi "external url"; then
  echo "FAILED: Error message should mention 'external URL'"
  echo "$ATTACH_URL_OUTPUT"
  exit 1
fi

if ! echo "$ATTACH_URL_OUTPUT" | grep -qi "trace"; then
  echo "FAILED: Error message should suggest using 'trace' relation"
  echo "$ATTACH_URL_OUTPUT"
  exit 1
fi

echo "Test 12 passed"
echo ""

# ==================================
# Test 13: Unlink Scenario 2 - Both relations in file (after format --with-full-relations)
# ==================================
echo "Test 13: Unlink when both relations exist in file..."

# Setup: Start from state after Test 3 (link created)
cp "${TEST_SCRIPT_DIR}/expected/03-link-creates-subsection.md" "$TEST_DIR/specifications/Requirements.md"

# Format with full relations to write opposites to disk
cd "$TEST_DIR" && "$REQVIRE_BIN" format --with-full-relations --fix > /dev/null 2>&1

# Now unlink - both relations should be removed from files
cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "System Requirements" "No Relations Requirement" > /dev/null 2>&1

# Expected: Both System Requirements' derive AND No Relations Requirement's derivedFrom removed
assert_file_matches "${TEST_SCRIPT_DIR}/expected/13-unlink-both-in-file.md" "$TEST_DIR/specifications/Requirements.md" "Unlink should remove both relations when both are in file"

echo "Test 13 passed"
echo ""

# ==================================
# Test 14: Unlink Scenario 3 - Unlink from opposite side
# ==================================
echo "Test 14: Unlink from opposite side (with only in-memory opposite)..."

# Setup: Create state where Child has derivedFrom (user_created) but Parent has NO derive in file
# Remove Verifications.md to avoid validation errors (it references elements that won't exist)
rm -f "$TEST_DIR/specifications/Verifications.md"
cp "${TEST_SCRIPT_DIR}/fixtures/opposite-side-unlink.md.txt" "$TEST_DIR/specifications/Requirements.md"

# Unlink from Parent side (which has only in-memory opposite derive)
# This should remove Child's derivedFrom from file
cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Parent Req" "Child Req" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/14-unlink-opposite-side.md" "$TEST_DIR/specifications/Requirements.md" "Unlink from opposite side should remove user-created relation"

echo "Test 14 passed"
echo ""

# ==================================
# Test 15: Link rejects multi-root hierarchy ownership violation
# ==================================
echo "Test 15: Link rejects multi-root hierarchy ownership violation..."

# Keep fixture valid so this test asserts the command-level single-root rejection only.
rm -f "$TEST_DIR/specifications/Verifications.md"
cp "${TEST_SCRIPT_DIR}/fixtures/multiroot-violation.md.txt" "$TEST_DIR/specifications/Requirements.md"

BEFORE_TEST15="$(mktemp /tmp/reqvire-link-before15-XXXXXX.md)"
cp "$TEST_DIR/specifications/Requirements.md" "$BEFORE_TEST15"

set +e
LINK_MULTIROOT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Child" "derivedFrom" "Parent B" 2>&1)
LINK_MULTIROOT_EXIT=$?
set -e

if [ $LINK_MULTIROOT_EXIT -eq 0 ]; then
  echo "FAILED: Link should fail deterministically with single-root ownership violation"
  exit 1
fi

printf "%s\n" "$LINK_MULTIROOT_OUTPUT" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | sed -E 's/^\[[^]]+\][[:space:]]*//' \
  | grep -E "Single-root hierarchy ownership violation" \
  > "$TEST_DIR/output/15-link-multiroot-error.actual.txt"

assert_output_matches \
  "${TEST_SCRIPT_DIR}/expected/15-link-multiroot-error.txt" \
  "$TEST_DIR/output/15-link-multiroot-error.actual.txt" \
  "Deterministic single-root error output mismatch for link"

assert_file_matches "$BEFORE_TEST15" "$TEST_DIR/specifications/Requirements.md" "Failed link must not modify file"

rm -f "$BEFORE_TEST15"

echo "Test 15 passed"
echo ""

# ==================================
# Test 16: Link semantic-contract relation types
# ==================================
echo "Test 16: Link semantic-contract relation types..."

rm -f "$TEST_DIR/specifications/Verifications.md"
cp "${TEST_SCRIPT_DIR}/fixtures/semantic-contract-relations.md.txt" "$TEST_DIR/specifications/Requirements.md"

cd "$TEST_DIR" && "$REQVIRE_BIN" link "API Shape Contract" use "Extra Semantic Ontology" > /dev/null 2>&1
cd "$TEST_DIR" && "$REQVIRE_BIN" link "API Requirement" constrainedBy "API Shape Contract" > /dev/null 2>&1
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Audit Shape Contract" constrain "Audit Requirement" > /dev/null 2>&1
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Extra Semantic Ontology" usedBy "Audit Shape Contract" > /dev/null 2>&1

if ! grep -Fq "  * use: [Extra Semantic Ontology](#extra-semantic-ontology)" "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: CLI link did not write semantic-contract use relation"
  exit 1
fi

if ! grep -Fq "  * constrainedBy: [API Shape Contract](#api-shape-contract)" "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: CLI link did not write requirement constrainedBy relation"
  exit 1
fi

if ! grep -Fq "  * constrain: [Audit Requirement](#audit-requirement)" "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: CLI link did not write semantic-contract constrain relation"
  exit 1
fi

if ! grep -Fq "  * usedBy: [Audit Shape Contract](#audit-shape-contract)" "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: CLI link did not write ontology usedBy relation"
  exit 1
fi

cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /dev/null 2>&1

echo "Test 16 passed"
echo ""

echo "===================================="
echo "All Link/Unlink tests passed"
echo "===================================="
exit 0
