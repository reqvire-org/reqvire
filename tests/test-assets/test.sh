#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

attachment_count() {
  local element_name="$1"
  (cd "$TEST_DIR" && "$REQVIRE_BIN" search --json) | jq -r --arg name "$element_name" '
    [.files | to_entries[]?.value.elements[]? | select(.name==$name) | .attachments[]?] | length
  '
}

has_attachment() {
  local element_name="$1"
  local expected_target="$2"
  (cd "$TEST_DIR" && "$REQVIRE_BIN" search --json) | jq -r --arg name "$element_name" '
    .files | to_entries[]?.value.elements[]? | select(.name==$name) | .attachments[]?
  ' | grep -Fxq "$expected_target"
}

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
echo "Identifier Attachments Capability Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Attach refinement identifier
# ==================================
echo "Test 1: Attach refinement identifier..."
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "#test-constraint-element" > /dev/null 2>&1

if [ "$(attachment_count "Performance Requirement")" -ne 1 ]; then
  echo "❌ FAILED: Performance Requirement should have 1 attachment"
  exit 1
fi

if ! has_attachment "Performance Requirement" "specifications/Requirements.md#test-constraint-element"; then
  echo "❌ FAILED: Expected attachment target not found"
  exit 1
fi

echo "✅ Test 1 passed"
echo ""

# ==================================
# Test 2: Duplicate attach fails
# ==================================
echo "Test 2: Duplicate attach returns error..."
set +e
ATTACH_DUP_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "#test-constraint-element" 2>&1)
ATTACH_DUP_EXIT=$?
set -e

if [ $ATTACH_DUP_EXIT -eq 0 ]; then
  echo "❌ FAILED: Duplicate attach should fail"
  exit 1
fi

if ! echo "$ATTACH_DUP_OUTPUT" | grep -qi "already exists"; then
  echo "❌ FAILED: Duplicate attach error should mention 'already exists'"
  echo "$ATTACH_DUP_OUTPUT"
  exit 1
fi

echo "✅ Test 2 passed"
echo ""

# ==================================
# Test 3: Multiple refinement attachments
# ==================================
echo "Test 3: Multiple refinement attachments..."
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "#test-behavior-element" > /dev/null 2>&1

if [ "$(attachment_count "Performance Requirement")" -ne 2 ]; then
  echo "❌ FAILED: Performance Requirement should have 2 attachments"
  exit 1
fi

echo "✅ Test 3 passed"
echo ""

# ==================================
# Test 4: Same refinement to multiple elements
# ==================================
echo "Test 4: Same refinement on multiple elements..."
cd "$TEST_DIR" && "$REQVIRE_BIN" link "No Attachments Requirement" attaching "#test-constraint-element" > /dev/null 2>&1

if [ "$(attachment_count "No Attachments Requirement")" -ne 1 ]; then
  echo "❌ FAILED: No Attachments Requirement should have 1 attachment"
  exit 1
fi

echo "✅ Test 4 passed"
echo ""

# ==================================
# Test 5: Detach isolation
# ==================================
echo "Test 5: Detach one attachment without affecting others..."
cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Performance Requirement" "Test Behavior Element" > /dev/null 2>&1

if [ "$(attachment_count "Performance Requirement")" -ne 1 ]; then
  echo "❌ FAILED: Performance Requirement should have 1 attachment after detach"
  exit 1
fi

if [ "$(attachment_count "No Attachments Requirement")" -ne 1 ]; then
  echo "❌ FAILED: No Attachments Requirement attachment should remain unchanged"
  exit 1
fi

echo "✅ Test 5 passed"
echo ""

# ==================================
# Test 6: Detach all from source element
# ==================================
echo "Test 6: Detach remaining attachment from source..."
cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Performance Requirement" "Test Constraint Element" > /dev/null 2>&1

if [ "$(attachment_count "Performance Requirement")" -ne 0 ]; then
  echo "❌ FAILED: Performance Requirement should have no attachments"
  exit 1
fi

echo "✅ Test 6 passed"
echo ""

# ==================================
# Test 7: Search filter has-attachments
# ==================================
echo "Test 7: Search --has-attachments..."
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --has-attachments --short 2>&1)

if ! echo "$SEARCH_OUTPUT" | grep -q "No Attachments Requirement"; then
  echo "❌ FAILED: Search should include 'No Attachments Requirement'"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

if echo "$SEARCH_OUTPUT" | grep -q "Performance Requirement"; then
  echo "❌ FAILED: Search should not include 'Performance Requirement' after detach"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

echo "✅ Test 7 passed"
echo ""

# ==================================
# Test 8: File-path target rejected for attaching
# ==================================
echo "Test 8: File-path target is rejected..."
set +e
ATTACH_PATH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "docs/SLA.txt" 2>&1)
ATTACH_PATH_EXIT=$?
set -e

if [ $ATTACH_PATH_EXIT -eq 0 ]; then
  echo "❌ FAILED: File-path attaching target should fail"
  exit 1
fi

if ! echo "$ATTACH_PATH_OUTPUT" | grep -qi "must use attachable element identifiers"; then
  echo "❌ FAILED: Error should explain identifier-only attachment targets"
  echo "$ATTACH_PATH_OUTPUT"
  exit 1
fi

echo "✅ Test 8 passed"
echo ""

# ==================================
# Test 9: Validation rejects file-path attachment syntax
# ==================================
echo "Test 9: Validation rejects file-path attachment syntax..."
cat >> "$TEST_DIR/specifications/Requirements.md" << 'EOF'

### Invalid File Attachment Requirement

This requirement intentionally uses invalid file-path attachment syntax.

#### Metadata
  * type: requirement

#### Attachments
  * [SLA](../docs/SLA.txt)
---
EOF

set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
set -e

if [ $VALIDATE_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for file-path attachment syntax"
  exit 1
fi

if ! echo "$VALIDATE_OUTPUT" | grep -qi "Invalid attachment"; then
  echo "❌ FAILED: Validation error should mention invalid attachment format"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

sed -i '/### Invalid File Attachment Requirement/,/^---$/d' "$TEST_DIR/specifications/Requirements.md"

echo "✅ Test 9 passed"
echo ""

# ==================================
# Test 10: Dry-run mode
# ==================================
echo "Test 10: Dry-run mode..."
cp "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.bak"

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "#test-behavior-element" --dry-run > /dev/null 2>&1

if ! cmp -s "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.bak"; then
  echo "❌ FAILED: Dry-run mode should not modify the file"
  diff -u "$TEST_DIR/requirements_backup.bak" "$TEST_DIR/specifications/Requirements.md" || true
  exit 1
fi

echo "✅ Test 10 passed"
echo ""

# ==================================
# Test 11: Non-refinement target rejected
# ==================================
echo "Test 11: Non-refinement attachment target is rejected..."
set +e
ATTACH_NON_REFINEMENT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "#no-attachments-requirement" 2>&1)
ATTACH_NON_REFINEMENT_EXIT=$?
set -e

if [ $ATTACH_NON_REFINEMENT_EXIT -eq 0 ]; then
  echo "❌ FAILED: Non-refinement attachment target should fail"
  exit 1
fi

if ! echo "$ATTACH_NON_REFINEMENT_OUTPUT" | grep -qi "not an attachable type"; then
  echo "❌ FAILED: Error should mention attachable type constraint"
  echo "$ATTACH_NON_REFINEMENT_OUTPUT"
  exit 1
fi

echo "✅ Test 11 passed"
echo ""

# ==================================
# Test 12: Unresolved identifier rejected
# ==================================
echo "Test 12: Unresolved identifier target is rejected..."
set +e
ATTACH_MISSING_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" attaching "#missing-refinement" 2>&1)
ATTACH_MISSING_EXIT=$?
set -e

if [ $ATTACH_MISSING_EXIT -eq 0 ]; then
  echo "❌ FAILED: Unresolved identifier target should fail"
  exit 1
fi

if ! echo "$ATTACH_MISSING_OUTPUT" | grep -qi "could not be resolved"; then
  echo "❌ FAILED: Error should mention unresolved attachment target"
  echo "$ATTACH_MISSING_OUTPUT"
  exit 1
fi

echo "✅ Test 12 passed"
echo ""

# ==================================
# Test 13: mv-asset updates InternalPath relations
# ==================================
echo "Test 13: mv-asset updates satisfiedBy relations..."

cat > "$TEST_DIR/specifications/RelationsTest.md" << 'EOF'
# Elements

### Relations Test System

Top level container.

#### Metadata
  * type: capability
---

### Verified Requirement

This requirement is satisfied by a script.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Relations Test System](#relations-test-system)
  * satisfiedBy: [test_script.sh](../scripts/test_script.sh)
---
EOF

mkdir -p "$TEST_DIR/scripts"
echo "#!/bin/bash" > "$TEST_DIR/scripts/test_script.sh"
echo "echo 'test'" >> "$TEST_DIR/scripts/test_script.sh"

mkdir -p "$TEST_DIR/src"
cd "$TEST_DIR" && "$REQVIRE_BIN" mv-asset "scripts/test_script.sh" "src/test_script.sh" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/17-after-mv-asset-relation.md" "$TEST_DIR/specifications/RelationsTest.md" "satisfiedBy relation not updated after mv-asset"

if [ -f "$TEST_DIR/scripts/test_script.sh" ]; then
  echo "❌ FAILED: Old file still exists after mv-asset"
  exit 1
fi

if [ ! -f "$TEST_DIR/src/test_script.sh" ]; then
  echo "❌ FAILED: File was not moved to new location"
  exit 1
fi

if [ "$(attachment_count "No Attachments Requirement")" -ne 1 ]; then
  echo "❌ FAILED: mv-asset should not change refinement identifier attachments"
  exit 1
fi

echo "✅ Test 13 passed"
echo ""

# ==================================
# Test 14: rm-asset removes InternalPath relations
# ==================================
echo "Test 14: rm-asset removes satisfiedBy relations..."

cd "$TEST_DIR" && "$REQVIRE_BIN" rm-asset "src/test_script.sh" > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/18-after-rm-asset-relation.md" "$TEST_DIR/specifications/RelationsTest.md" "satisfiedBy relation not removed after rm-asset"

if [ -f "$TEST_DIR/src/test_script.sh" ]; then
  echo "❌ FAILED: File was not deleted by rm-asset"
  exit 1
fi

if [ "$(attachment_count "No Attachments Requirement")" -ne 1 ]; then
  echo "❌ FAILED: rm-asset should not change refinement identifier attachments"
  exit 1
fi

echo "✅ Test 14 passed"
echo ""

echo "===================================="
echo "All Identifier Attachments tests passed"
echo "===================================="
exit 0
