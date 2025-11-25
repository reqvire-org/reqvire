#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Attachments Change Impact
# --------------------------------------
# Satisfies: specifications/System/Core/Verifications/AttachmentsVerifications.md#attachments-change-impact-verification
#
# Tests:
# 1. Refinement content change is detected (with propagation to attached requirements)
# 2. File attachment content change is detected (propagates to attached requirement)
# 3. mv Refinement to different file - relocation reported, identifiers updated
# 4. rm Refinement element - validation fails

echo "===================================="
echo "Attachments Change Impact Tests"
echo "===================================="
echo ""

# Add remote for change-impact URL generation
git remote add origin https://example.com/test/repo.git 2>/dev/null || true

# ==================================
# Test 1: Refinement Content Change Detection
# ==================================
echo "Test 1: Refinement content change is detected..."

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
  echo "❌ FAILED: Refinement content change output does not match expected."
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-content-change.txt"
  exit 1
fi

# Commit the change for next test
cd "$TEST_DIR" && git add -A && git commit -q -m "Change Refinement content"

echo "Test 1 passed"
echo ""

# ==================================
# Test 2: File Attachment Content Change Detection
# ==================================
echo "Test 2: File attachment content change is detected..."

# Modify the file attachment content
sed -i 's/export_YYYYMMDD.json/export_YYYY-MM-DD.json/' "$TEST_DIR/specifications/docs/export-spec.md"

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
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-file-attachment-change.txt" <(echo "$SANITIZED_OUTPUT"); then
  echo "❌ FAILED: File attachment content change output does not match expected."
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-file-attachment-change.txt"
  exit 1
fi

# Commit the change for next test
cd "$TEST_DIR" && git add -A && git commit -q -m "Change file attachment content"

echo "Test 2 passed"
echo ""

# ==================================
# Test 3: mv Refinement to Different File
# ==================================
echo "Test 3: mv Refinement to different file..."

# Create a new file for refinements
cat > "$TEST_DIR/specifications/Refinements.md" << 'EOF'
# Elements
EOF

# Move Data Format Spec to Refinements.md
set +e
MV_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Data Format Spec" --to-file specifications/Refinements.md 2>&1)
MV_EXIT=$?
set -e

if [ $MV_EXIT -ne 0 ]; then
  echo "FAILED: mv command failed"
  echo "$MV_OUTPUT"
  exit 1
fi

# Verify attachment identifiers were updated in Requirements.md
if ! grep -q "Refinements.md#data-format-spec" "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: Attachment identifiers should be updated after mv"
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
cd "$TEST_DIR" && git add -A && git commit -q -m "Move Refinement to different file"

echo "Test 3 passed"
echo ""

# ==================================
# Test 4: rm Refinement Element - Validation Fails
# ==================================
echo "Test 4: rm Refinement element with attachments - validation fails..."

# Remove Data Format Spec (attached to 3 requirements)
set +e
RM_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm "Data Format Spec" 2>&1)
RM_EXIT=$?
set -e

# Validation should now fail because attachments are broken
set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
set -e

if [ $VALIDATE_EXIT -eq 0 ]; then
  echo "FAILED: Validation should fail when Refinement with attachments is removed"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

# Verify error mentions the broken attachment
if ! echo "$VALIDATE_OUTPUT" | grep -qi "data-format-spec\|not found\|invalid\|missing"; then
  echo "FAILED: Validation error should mention the broken attachment identifier"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

echo "Test 4 passed"
echo ""

# ==================================
# Final Result
# ==================================
echo "===================================="
echo "All Attachments Change Impact tests passed"
echo "===================================="
exit 0
