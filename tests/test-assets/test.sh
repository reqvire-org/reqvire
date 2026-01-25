#!/bin/bash
set -uo pipefail

# Test: Attachments Feature
#
# Satisfies: specifications/System/AttachmentsVerifications.md
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
    echo "❌ FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

echo "===================================="
echo "Attachments Feature Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Attach file to element (using link ... attaching)
# ==================================
echo "Test 1: Attach file to element..."

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "docs/SLA.txt" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-attach.md" "$TEST_DIR/specifications/Requirements.md" "File content after attach does not match expected"

echo "✅ Test 1 passed"
echo ""

# ==================================
# Test 2: Attach duplicate returns error
# ==================================
echo "Test 2: Attach duplicate returns error..."

set +e
ATTACH_DUP_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "docs/SLA.txt" 2>&1)
ATTACH_DUP_EXIT=$?
set -e

if [ $ATTACH_DUP_EXIT -eq 0 ]; then
  echo "❌ FAILED: Duplicate attach should fail with error"
  exit 1
fi

if ! echo "$ATTACH_DUP_OUTPUT" | grep -qi "already exists"; then
  echo "❌ FAILED: Error message should mention 'already exists'"
  echo "$ATTACH_DUP_OUTPUT"
  exit 1
fi

# File should be unchanged (operation failed)
assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-attach.md" "$TEST_DIR/specifications/Requirements.md" "Failed attach should not modify file"

echo "✅ Test 2 passed"
echo ""

# ==================================
# Test 3: Multiple attachments
# ==================================
echo "Test 3: Multiple attachments on same element..."

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "docs/benchmarks.txt" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/03-multiple-attachments.md" "$TEST_DIR/specifications/Requirements.md" "Multiple attachments result does not match expected"

echo "✅ Test 3 passed"
echo ""

# ==================================
# Test 4: Same file to multiple elements (in different hierarchies)
# ==================================
echo "Test 4: Same file attached to multiple elements..."

# Note: Elements must be in different hierarchies to comply with attachment scope constraints
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Refinement Target Requirement" attaching "docs/SLA.txt" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/04-many-to-many.md" "$TEST_DIR/specifications/Requirements.md" "Many-to-many attachment result does not match expected"

echo "✅ Test 4 passed"
echo ""

# ==================================
# Test 5: Detach command (using unlink)
# ==================================
echo "Test 5: Detach command (using unlink)..."

cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Performance Requirement" "docs/benchmarks.txt" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/05-after-detach.md" "$TEST_DIR/specifications/Requirements.md" "File content after detach does not match expected"

echo "✅ Test 5 passed"
echo ""

# ==================================
# Test 6: Detach isolation
# ==================================
echo "Test 6: Detach from one element doesn't affect others..."

cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Performance Requirement" "docs/SLA.txt" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/06-detach-isolation.md" "$TEST_DIR/specifications/Requirements.md" "Detach isolation result does not match expected"

echo "✅ Test 6 passed"
echo ""

# ==================================
# Test 7: Search filters
# ==================================
echo "Test 7: Search filters for attachments..."

# Re-attach for search tests
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "docs/SLA.txt" > /dev/null 2>&1

SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --has-attachments --short 2>&1)

if ! echo "$SEARCH_OUTPUT" | diff -u "${TEST_SCRIPT_DIR}/expected/07-search-has-attachments.txt" -; then
  echo "❌ FAILED: Search --has-attachments output does not match expected"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/07-search-has-attachments.txt"
  exit 1
fi

echo "✅ Test 7 passed"
echo ""

# ==================================
# Test 8: mv-asset command (Attachments)
# ==================================
echo "Test 8: mv-asset command updates Attachments..."

mkdir -p "$TEST_DIR/documents"
cd "$TEST_DIR" && "$REQVIRE_BIN" mv-asset "docs/SLA.txt" "documents/SLA.txt" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/08-after-mv-asset.md" "$TEST_DIR/specifications/Requirements.md" "File content after mv-asset does not match expected"

# Verify file was moved
if [ -f "$TEST_DIR/docs/SLA.txt" ]; then
  echo "❌ FAILED: Old file still exists after mv-asset"
  exit 1
fi

if [ ! -f "$TEST_DIR/documents/SLA.txt" ]; then
  echo "❌ FAILED: File was not moved to new location"
  exit 1
fi

echo "✅ Test 8 passed"
echo ""

# ==================================
# Test 9: rm-asset command (Attachments)
# ==================================
echo "Test 9: rm-asset command removes from Attachments..."

cd "$TEST_DIR" && "$REQVIRE_BIN" rm-asset "documents/SLA.txt" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/09-after-rm-asset.md" "$TEST_DIR/specifications/Requirements.md" "File content after rm-asset does not match expected"

# Verify file was deleted
if [ -f "$TEST_DIR/documents/SLA.txt" ]; then
  echo "❌ FAILED: File was not deleted by rm-asset"
  exit 1
fi

echo "✅ Test 9 passed"
echo ""

# ==================================
# Test 10: Validation - Missing Attachment
# ==================================
echo "Test 10: Validation detects missing attachment files..."

# Manually add element with missing attachment
cat >> "$TEST_DIR/specifications/Requirements.md" << 'EOF'

### Test Missing Attachment

This element has a missing attachment.

#### Attachments
* [missing/file.txt](missing/file.txt)

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
---

EOF

set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for missing attachment file"
  exit 1
fi

if ! echo "$VALIDATION_OUTPUT" | grep -qi "missing\|not found\|attachment"; then
  echo "❌ FAILED: Validation error should mention missing attachment"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "✅ Test 10 passed"
echo ""

# ==================================
# Test 11: Dry-run mode
# ==================================
echo "Test 11: Dry-run mode..."

# Clean up invalid element
sed -i '/### Test Missing Attachment/,/^---$/d' "$TEST_DIR/specifications/Requirements.md"

# Create fresh test file
cat > "$TEST_DIR/specifications/DryRunTest.md" << 'EOF'
# Elements

### Dry Run Element

Test element for dry-run.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: Requirements.md#system-requirements
---

EOF

cp "$TEST_DIR/specifications/DryRunTest.md" "$TEST_DIR/dryrun_backup.bak"

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Dry Run Element" attaching "docs/benchmarks.txt" --dry-run > /dev/null 2>&1

if ! cmp -s "$TEST_DIR/specifications/DryRunTest.md" "$TEST_DIR/dryrun_backup.bak"; then
  echo "❌ FAILED: Dry-run mode should not modify the file"
  diff "$TEST_DIR/dryrun_backup.bak" "$TEST_DIR/specifications/DryRunTest.md"
  exit 1
fi

echo "✅ Test 11 passed"
echo ""

# ==================================
# Test 12: Attach Refinement element
# ==================================
echo "Test 12: Attach Refinement element by display name..."

# Reset Requirements.md to clean state for element attachment tests
cp "${TEST_SCRIPT_DIR}/expected/09-after-rm-asset.md" "$TEST_DIR/specifications/Requirements.md"

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "Test Constraint Element" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/12-after-element-attach.md" "$TEST_DIR/specifications/Requirements.md" "Element attachment result does not match expected"

echo "✅ Test 12 passed"
echo ""

# ==================================
# Test 13: File path takes priority
# ==================================
echo "Test 13: Auto-detect - file path takes priority over element name..."

mkdir -p "$TEST_DIR/Test Constraint Element"
echo "test content" > "$TEST_DIR/Test Constraint Element/data.txt"

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Implementation Detail" attaching "Test Constraint Element/data.txt" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/13-file-priority.md" "$TEST_DIR/specifications/Requirements.md" "File path priority result does not match expected"

echo "✅ Test 13 passed"
echo ""

# ==================================
# Test 14: Non-Refinement element fails
# ==================================
echo "Test 14: Attach non-Refinement element fails with error..."

set +e
ATTACH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "No Attachments Requirement" 2>&1)
ATTACH_EXIT=$?
set -e

if [ $ATTACH_EXIT -eq 0 ]; then
  echo "❌ FAILED: Attaching non-Refinement element should fail"
  exit 1
fi

if ! echo "$ATTACH_OUTPUT" | grep -qi "refinement\|constraint\|behavior\|specification"; then
  echo "❌ FAILED: Error message should indicate only Refinement types allowed"
  echo "$ATTACH_OUTPUT"
  exit 1
fi

echo "✅ Test 14 passed"
echo ""

# ==================================
# Test 15: Detach element by name (using unlink)
# ==================================
echo "Test 15: Detach Refinement element by display name..."

cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Performance Requirement" "Test Constraint Element" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/15-after-element-detach.md" "$TEST_DIR/specifications/Requirements.md" "Element detach result does not match expected"

echo "✅ Test 15 passed"
echo ""

# ==================================
# Test 16: Not found error
# ==================================
echo "Test 16: Error when neither file nor element found..."

set +e
ATTACH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "nonexistent_thing_xyz" 2>&1)
ATTACH_EXIT=$?
set -e

if [ $ATTACH_EXIT -eq 0 ]; then
  echo "❌ FAILED: Should fail when neither file nor element found"
  exit 1
fi

if ! echo "$ATTACH_OUTPUT" | grep -qi "not found\|does not exist\|could not find"; then
  echo "❌ FAILED: Error message should indicate what was not found"
  echo "$ATTACH_OUTPUT"
  exit 1
fi

echo "✅ Test 16 passed"
echo ""

# ==================================
# Test 17: mv-asset updates Relations
# ==================================
echo "Test 17: mv-asset command updates satisfiedBy relations..."

# Create a test file with satisfiedBy relation pointing to a script
cat > "$TEST_DIR/specifications/RelationsTest.md" << 'EOF'
# Elements

### Relations Test System

Top level container.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Verified Requirement](#verified-requirement)
---

### Verified Requirement

This requirement is satisfied by a script.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Relations Test System](#relations-test-system)
  * satisfiedBy: [test_script.sh](../scripts/test_script.sh)
---

EOF

# Create the script file
mkdir -p "$TEST_DIR/scripts"
echo "#!/bin/bash" > "$TEST_DIR/scripts/test_script.sh"
echo "echo 'test'" >> "$TEST_DIR/scripts/test_script.sh"

# Move the script
mkdir -p "$TEST_DIR/src"
cd "$TEST_DIR" && "$REQVIRE_BIN" mv-asset "scripts/test_script.sh" "src/test_script.sh" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/17-after-mv-asset-relation.md" "$TEST_DIR/specifications/RelationsTest.md" "satisfiedBy relation not updated after mv-asset"

# Verify file was moved
if [ -f "$TEST_DIR/scripts/test_script.sh" ]; then
  echo "❌ FAILED: Old file still exists after mv-asset"
  exit 1
fi

if [ ! -f "$TEST_DIR/src/test_script.sh" ]; then
  echo "❌ FAILED: File was not moved to new location"
  exit 1
fi

echo "✅ Test 17 passed"
echo ""

# ==================================
# Test 18: rm-asset removes Relations
# ==================================
echo "Test 18: rm-asset command removes satisfiedBy relations..."

cd "$TEST_DIR" && "$REQVIRE_BIN" rm-asset "src/test_script.sh" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/18-after-rm-asset-relation.md" "$TEST_DIR/specifications/RelationsTest.md" "satisfiedBy relation not removed after rm-asset"

# Verify file was deleted
if [ -f "$TEST_DIR/src/test_script.sh" ]; then
  echo "❌ FAILED: File was not deleted by rm-asset"
  exit 1
fi

echo "✅ Test 18 passed"
echo ""

# Note: Test 19 was removed - it tested having the same file in both Attachments AND Relations,
# which is now prohibited by cross-section duplicate validation.

echo "===================================="
echo "All Attachments tests passed"
echo "===================================="
exit 0
