#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

contract_bindings_count() {
  local element_name="$1"
  (cd "$TEST_DIR" && "$REQVIRE_BIN" search --json) | jq -r --arg name "$element_name" '
    [.. | objects | select(.name? == $name and has("contract_bindings")) | .contract_bindings[]?] | length
  '
}

has_contract_bindings() {
  local element_name="$1"
  local expected_target="$2"
  (cd "$TEST_DIR" && "$REQVIRE_BIN" search --json) | jq -r --arg name "$element_name" '
    .. | objects | select(.name? == $name and has("contract_bindings")) | .contract_bindings[]?
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
echo "Identifier Contract Bindings Capability Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Reuse contract identifier
# ==================================
echo "Test 1: Reuse contract identifier..."
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" bindContract "#test-constraint-element" > /dev/null 2>&1

if [ "$(contract_bindings_count "Performance Requirement")" -ne 1 ]; then
  echo "❌ FAILED: Performance Requirement should have 1 contract_bindings"
  exit 1
fi

if ! has_contract_bindings "Performance Requirement" "specifications/Requirements.md#test-constraint-element"; then
  echo "❌ FAILED: Expected contract_bindings target not found"
  exit 1
fi

echo "✅ Test 1 passed"
echo ""

# ==================================
# Test 2: Duplicate reuse fails
# ==================================
echo "Test 2: Duplicate reuse returns error..."
set +e
ATTACH_DUP_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" bindContract "#test-constraint-element" 2>&1)
ATTACH_DUP_EXIT=$?
set -e

if [ $ATTACH_DUP_EXIT -eq 0 ]; then
  echo "❌ FAILED: Duplicate reuse should fail"
  exit 1
fi

if ! echo "$ATTACH_DUP_OUTPUT" | grep -qi "already exists"; then
  echo "❌ FAILED: Duplicate reuse error should mention 'already exists'"
  echo "$ATTACH_DUP_OUTPUT"
  exit 1
fi

echo "✅ Test 2 passed"
echo ""

# ==================================
# Test 3: Multiple contract contract_bindings
# ==================================
echo "Test 3: Multiple contract contract_bindings..."
cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" bindContract "#test-behavior-element" > /dev/null 2>&1

if [ "$(contract_bindings_count "Performance Requirement")" -ne 2 ]; then
  echo "❌ FAILED: Performance Requirement should have 2 contract_bindings"
  exit 1
fi

echo "✅ Test 3 passed"
echo ""

# ==================================
# Test 4: Same contract to multiple elements
# ==================================
echo "Test 4: Same contract on multiple elements..."
cd "$TEST_DIR" && "$REQVIRE_BIN" link "No Contract Bindings Requirement" bindContract "#test-constraint-element" > /dev/null 2>&1

if [ "$(contract_bindings_count "No Contract Bindings Requirement")" -ne 1 ]; then
  echo "❌ FAILED: No Contract Bindings Requirement should have 1 contract_bindings"
  exit 1
fi

echo "✅ Test 4 passed"
echo ""

# ==================================
# Test 5: Remove Contract Binding isolation
# ==================================
echo "Test 5: Remove Contract Binding one contract_bindings without affecting others..."
cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Performance Requirement" "Test Behavior Element" > /dev/null 2>&1

if [ "$(contract_bindings_count "Performance Requirement")" -ne 1 ]; then
  echo "❌ FAILED: Performance Requirement should have 1 contract_bindings after remove contract binding"
  exit 1
fi

if [ "$(contract_bindings_count "No Contract Bindings Requirement")" -ne 1 ]; then
  echo "❌ FAILED: No Contract Bindings Requirement contract_bindings should remain unchanged"
  exit 1
fi

echo "✅ Test 5 passed"
echo ""

# ==================================
# Test 6: Remove Contract Binding all from source element
# ==================================
echo "Test 6: Remove Contract Binding remaining contract_bindings from source..."
cd "$TEST_DIR" && "$REQVIRE_BIN" unlink "Performance Requirement" "Test Constraint Element" > /dev/null 2>&1

if [ "$(contract_bindings_count "Performance Requirement")" -ne 0 ]; then
  echo "❌ FAILED: Performance Requirement should have no contract_bindings"
  exit 1
fi

echo "✅ Test 6 passed"
echo ""

# ==================================
# Test 7: Search filter has-contract_bindings
# ==================================
echo "Test 7: Search --has-contract-bindings..."
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --has-contract-bindings --short 2>&1)

if ! echo "$SEARCH_OUTPUT" | grep -q "No Contract Bindings Requirement"; then
  echo "❌ FAILED: Search should include 'No Contract Bindings Requirement'"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

if echo "$SEARCH_OUTPUT" | grep -q "Performance Requirement"; then
  echo "❌ FAILED: Search should not include 'Performance Requirement' after remove contract binding"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

echo "✅ Test 7 passed"
echo ""

# ==================================
# Test 8: File-path target rejected for bindContract
# ==================================
echo "Test 8: File-path target is rejected..."
set +e
ATTACH_PATH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" bindContract "docs/SLA.txt" 2>&1)
ATTACH_PATH_EXIT=$?
set -e

if [ $ATTACH_PATH_EXIT -eq 0 ]; then
  echo "❌ FAILED: File-path bindContract target should fail"
  exit 1
fi

if ! echo "$ATTACH_PATH_OUTPUT" | grep -qi "must use reusable element identifiers"; then
  echo "❌ FAILED: Error should explain identifier-only contract_bindings targets"
  echo "$ATTACH_PATH_OUTPUT"
  exit 1
fi

echo "✅ Test 8 passed"
echo ""

# ==================================
# Test 9: Validation rejects file-path contract_bindings syntax
# ==================================
echo "Test 9: Validation rejects file-path contract_bindings syntax..."
cat >> "$TEST_DIR/specifications/Requirements.md" << 'EOF'

### Invalid File ContractBindingEntry Requirement

This requirement intentionally uses invalid file-path contract_bindings syntax.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [SLA](../docs/SLA.txt)
---
EOF

set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
set -e

if [ $VALIDATE_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for file-path contract_bindings syntax"
  exit 1
fi

if ! echo "$VALIDATE_OUTPUT" | grep -qi "Invalid contract_bindings"; then
  echo "❌ FAILED: Validation error should mention invalid contract_bindings format"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

sed -i '/### Invalid File ContractBindingEntry Requirement/,/^---$/d' "$TEST_DIR/specifications/Requirements.md"

echo "✅ Test 9 passed"
echo ""

# ==================================
# Test 10: Dry-run mode
# ==================================
echo "Test 10: Dry-run mode..."
cp "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.bak"

cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" bindContract "#test-behavior-element" --dry-run > /dev/null 2>&1

if ! cmp -s "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.bak"; then
  echo "❌ FAILED: Dry-run mode should not modify the file"
  diff -u "$TEST_DIR/requirements_backup.bak" "$TEST_DIR/specifications/Requirements.md" || true
  exit 1
fi

echo "✅ Test 10 passed"
echo ""

# ==================================
# Test 11: Non-contract target rejected
# ==================================
echo "Test 11: Non-contract contract_bindings target is rejected..."
set +e
ATTACH_NON_CONTRACT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" bindContract "#no-contract-bindings-requirement" 2>&1)
ATTACH_NON_CONTRACT_EXIT=$?
set -e

if [ $ATTACH_NON_CONTRACT_EXIT -eq 0 ]; then
  echo "❌ FAILED: Non-contract contract_bindings target should fail"
  exit 1
fi

if ! echo "$ATTACH_NON_CONTRACT_OUTPUT" | grep -qi "not an reusable type"; then
  echo "❌ FAILED: Error should mention reusable type constraint"
  echo "$ATTACH_NON_CONTRACT_OUTPUT"
  exit 1
fi

echo "✅ Test 11 passed"
echo ""

# ==================================
# Test 12: Unresolved identifier rejected
# ==================================
echo "Test 12: Unresolved identifier target is rejected..."
set +e
ATTACH_MISSING_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Performance Requirement" bindContract "#missing-contract" 2>&1)
ATTACH_MISSING_EXIT=$?
set -e

if [ $ATTACH_MISSING_EXIT -eq 0 ]; then
  echo "❌ FAILED: Unresolved identifier target should fail"
  exit 1
fi

if ! echo "$ATTACH_MISSING_OUTPUT" | grep -qi "could not be resolved"; then
  echo "❌ FAILED: Error should mention unresolved contract_bindings target"
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

if [ "$(contract_bindings_count "No Contract Bindings Requirement")" -ne 1 ]; then
  echo "❌ FAILED: mv-asset should not change contract identifier contract_bindings"
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

if [ "$(contract_bindings_count "No Contract Bindings Requirement")" -ne 1 ]; then
  echo "❌ FAILED: rm-asset should not change contract identifier contract_bindings"
  exit 1
fi

echo "✅ Test 14 passed"
echo ""

echo "===================================="
echo "All Identifier Contract Bindings tests passed"
echo "===================================="
exit 0
