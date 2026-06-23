#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Contract Bindings Change Impact
# --------------------------------------
# Satisfies: specifications/System/Core/Verifications/ContractBindingVerifications.md#contract-bindings-change-impact-verification
#
# Tests:
# 1. Contract content change is detected (with propagation to bound requirements)
# 2. File contract_bindings content change is detected (propagates to reused requirement)
# 3. mv Contract to different file - relocation reported, identifiers updated
# 4. rm Contract element - validation fails

echo "===================================="
echo "Contract Bindings Change Impact Tests"
echo "===================================="
echo ""

# Add remote for change-impact URL generation
git remote add origin https://example.com/test/repo.git 2>/dev/null || true

# ==================================
# Test 1: Contract Content Change Detection
# ==================================
echo "Test 1: Contract content change is detected..."

# Modify the Data Format Spec content
sed -i 's/maximum length of 255 characters/maximum length of 512 characters/' "$TEST_DIR/specifications/Requirements.md"

# Run change-impact
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" change-impact 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Change impact command failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Sanitize output
SANITIZED_OUTPUT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element" | grep -v "ERROR reqvire" | sed -E 's#https://[^ )]+/blob/[a-f0-9]{7,40}/##g')

# Compare against expected output
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-content-change.txt" <(echo "$SANITIZED_OUTPUT"); then
  echo "❌ FAILED: Contract content change output does not match expected."
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-content-change.txt"
  exit 1
fi

# Commit the change for next test
cd "$TEST_DIR" && git add -A && git commit -q -m "Change Contract content"

echo "Test 1 passed"
echo ""

# ==================================
# Test 2: Second Reused Contract Content Change Detection
# ==================================
echo "Test 2: Second bound contract content change is detected..."

# Modify second bound contract content
sed -i 's/export_YYYYMMDD.json/export_YYYY-MM-DD.json/' "$TEST_DIR/specifications/Requirements.md"

# Run change-impact
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" change-impact 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Change impact command failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Sanitize output
SANITIZED_OUTPUT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element" | grep -v "ERROR reqvire" | sed -E 's#https://[^ )]+/blob/[a-f0-9]{7,40}/##g')

# Compare against expected output
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-file-contract-bindings-change.txt" <(echo "$SANITIZED_OUTPUT"); then
  echo "❌ FAILED: Second bound contract content change output does not match expected."
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-file-contract-bindings-change.txt"
  exit 1
fi

# Commit the change for next test
cd "$TEST_DIR" && git add -A && git commit -q -m "Change bound contract content"

echo "Test 2 passed"
echo ""

# ==================================
# Test 3: mv Contract to Different File
# ==================================
echo "Test 3: mv Contract to different file..."

# Create a new file for contracts
cat > "$TEST_DIR/specifications/Contracts.md" << 'EOF'
# Elements
EOF

# Move Data Format Spec to Contracts.md
set +e
MV_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Data Format Spec" specifications/Contracts.md 2>&1)
MV_EXIT=$?
set -e

if [ $MV_EXIT -ne 0 ]; then
  echo "FAILED: mv command failed"
  echo "$MV_OUTPUT"
  exit 1
fi

# Verify contract_bindings identifiers were updated in Requirements.md
if ! grep -q "Contracts.md#data-format-spec" "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: ContractBindingEntry identifiers should be updated after mv"
  cat "$TEST_DIR/specifications/Requirements.md"
  exit 1
fi

# Run change-impact
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" change-impact 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Change impact command failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Sanitize output
SANITIZED_OUTPUT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element" | grep -v "ERROR reqvire" | sed -E 's#https://[^ )]+/blob/[a-f0-9]{7,40}/##g')

# Compare against expected output
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-mv-file.txt" <(echo "$SANITIZED_OUTPUT"); then
  echo "❌ FAILED: mv to different file output does not match expected."
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-mv-file.txt"
  exit 1
fi

# Commit the change for next test
cd "$TEST_DIR" && git add -A && git commit -q -m "Move Contract to different file"

echo "Test 3 passed"
echo ""

# ==================================
# Test 4: rm Contract Element - Validation Fails
# ==================================
echo "Test 4: rm Contract element with contract_bindings - validation fails..."

# Remove Data Format Spec (bound to 3 requirements)
set +e
RM_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Data Format Spec" 2>&1)
RM_EXIT=$?
set -e

# Validation should now fail because contract_bindings are broken
set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
set -e

if [ $VALIDATE_EXIT -eq 0 ]; then
  echo "FAILED: Validation should fail when Contract with contract_bindings is removed"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

# Verify error mentions the broken contract_bindings
if ! echo "$VALIDATE_OUTPUT" | grep -qi "data-format-spec\|not found\|invalid\|missing"; then
  echo "FAILED: Validation error should mention the broken contract_bindings identifier"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

echo "Test 4 passed"
echo ""

# ==================================
# Final Result
# ==================================
echo "===================================="
echo "All Contract Bindings Change Impact tests passed"
echo "===================================="
exit 0
