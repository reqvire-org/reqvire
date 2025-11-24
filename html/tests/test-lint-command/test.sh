#!/usr/bin/env bash
set -euo pipefail

# Test: Lint Command Verification
# ---------------------------------
# Satisfies: specifications/Verifications/Misc.md#lint-command-verification

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test 1: Default lint (all issues)
set +e
LINT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint 2>&1)
LINT_EXIT=$?
set -e

if [ $LINT_EXIT -ne 0 ]; then
  echo "❌ FAILED: lint should exit with code 0, got $LINT_EXIT"
  echo "$LINT_OUTPUT"
  exit 1
fi

if ! diff -u "${TEST_SCRIPT_DIR}/expected-default.txt" <(echo "$LINT_OUTPUT") > /dev/null; then
  echo "❌ FAILED: Default lint output does not match expected"
  diff -u "${TEST_SCRIPT_DIR}/expected-default.txt" <(echo "$LINT_OUTPUT")
  exit 1
fi

# Test 2: --fixable flag
set +e
FIXABLE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --fixable 2>&1)
FIXABLE_EXIT=$?
set -e

if [ $FIXABLE_EXIT -ne 0 ]; then
  echo "❌ FAILED: lint --fixable exit code $FIXABLE_EXIT"
  echo "$FIXABLE_OUTPUT"
  exit 1
fi

if ! diff -u "${TEST_SCRIPT_DIR}/expected-fixable.txt" <(echo "$FIXABLE_OUTPUT") > /dev/null; then
  echo "❌ FAILED: --fixable output mismatch"
  diff -u "${TEST_SCRIPT_DIR}/expected-fixable.txt" <(echo "$FIXABLE_OUTPUT")
  exit 1
fi

# Test 3: --auditable flag
set +e
AUDITABLE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --auditable 2>&1)
AUDITABLE_EXIT=$?
set -e

if [ $AUDITABLE_EXIT -ne 0 ]; then
  echo "❌ FAILED: lint --auditable exit code $AUDITABLE_EXIT"
  echo "$AUDITABLE_OUTPUT"
  exit 1
fi

if ! diff -u "${TEST_SCRIPT_DIR}/expected-auditable.txt" <(echo "$AUDITABLE_OUTPUT") > /dev/null; then
  echo "❌ FAILED: --auditable output mismatch"
  diff -u "${TEST_SCRIPT_DIR}/expected-auditable.txt" <(echo "$AUDITABLE_OUTPUT")
  exit 1
fi

# Test 4: JSON output
set +e
JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --json 2>&1)
JSON_EXIT=$?
set -e

if [ $JSON_EXIT -ne 0 ]; then
  echo "❌ FAILED: lint --json exit code $JSON_EXIT"
  echo "$JSON_OUTPUT"
  exit 1
fi

if ! diff -u <(jq -S . "${TEST_SCRIPT_DIR}/expected-json.json") <(echo "$JSON_OUTPUT" | jq -S .) > /dev/null; then
  echo "❌ FAILED: JSON output mismatch"
  diff -u <(jq -S . "${TEST_SCRIPT_DIR}/expected-json.json") <(echo "$JSON_OUTPUT" | jq -S .)
  exit 1
fi

# Test 4a: --fixable --json (should only show auto_fixable items)
set +e
FIXABLE_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --fixable --json 2>&1)
FIXABLE_JSON_EXIT=$?
set -e

if [ $FIXABLE_JSON_EXIT -ne 0 ]; then
  echo "❌ FAILED: lint --fixable --json exit code $FIXABLE_JSON_EXIT"
  echo "$FIXABLE_JSON_OUTPUT"
  exit 1
fi

# Verify needs_manual_review is empty
MANUAL_REVIEW_COUNT=$(echo "$FIXABLE_JSON_OUTPUT" | jq '.needs_manual_review | length')
if [ "$MANUAL_REVIEW_COUNT" -ne 0 ]; then
  echo "❌ FAILED: --fixable --json should not show needs_manual_review items (found $MANUAL_REVIEW_COUNT)"
  echo "$FIXABLE_JSON_OUTPUT" | jq '.needs_manual_review'
  exit 1
fi

# Verify auto_fixable is not empty
AUTO_FIXABLE_COUNT=$(echo "$FIXABLE_JSON_OUTPUT" | jq '.auto_fixable | length')
if [ "$AUTO_FIXABLE_COUNT" -eq 0 ]; then
  echo "❌ FAILED: --fixable --json should show auto_fixable items (found 0)"
  exit 1
fi

# Test 4b: --auditable --json (should only show needs_manual_review items)
set +e
AUDITABLE_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --auditable --json 2>&1)
AUDITABLE_JSON_EXIT=$?
set -e

if [ $AUDITABLE_JSON_EXIT -ne 0 ]; then
  echo "❌ FAILED: lint --auditable --json exit code $AUDITABLE_JSON_EXIT"
  echo "$AUDITABLE_JSON_OUTPUT"
  exit 1
fi

# Verify auto_fixable is empty
AUTO_FIXABLE_COUNT_AUDIT=$(echo "$AUDITABLE_JSON_OUTPUT" | jq '.auto_fixable | length')
if [ "$AUTO_FIXABLE_COUNT_AUDIT" -ne 0 ]; then
  echo "❌ FAILED: --auditable --json should not show auto_fixable items (found $AUTO_FIXABLE_COUNT_AUDIT)"
  echo "$AUDITABLE_JSON_OUTPUT" | jq '.auto_fixable'
  exit 1
fi

# Verify needs_manual_review is empty (before fix, there are no manual review items)
MANUAL_REVIEW_COUNT_AUDIT=$(echo "$AUDITABLE_JSON_OUTPUT" | jq '.needs_manual_review | length')
if [ "$MANUAL_REVIEW_COUNT_AUDIT" -ne 0 ]; then
  echo "❌ FAILED: --auditable --json should have empty needs_manual_review before fix (found $MANUAL_REVIEW_COUNT_AUDIT)"
  echo "$AUDITABLE_JSON_OUTPUT" | jq '.needs_manual_review'
  exit 1
fi

# Test 5: --fix flag
set +e
FIX_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --fix 2>&1)
FIX_EXIT=$?
set -e

if [ $FIX_EXIT -ne 0 ]; then
  echo "❌ FAILED: lint --fix exit code $FIX_EXIT"
  echo "$FIX_OUTPUT"
  exit 1
fi

# Check that authorization verify link was removed from API Integration Test
if grep -q 'verify:.*Authorization.*specifications/SystemRequirements.md#authorization' "$TEST_DIR/specifications/Verifications/Tests.md"; then
  echo "❌ FAILED: Redundant verify relation to Authorization not removed from API Integration Test"
  cat "$TEST_DIR/specifications/Verifications/Tests.md"
  exit 1
fi

# Check that the other verify links in API Integration Test are still present
if ! grep -q "verify:.*Public API" "$TEST_DIR/specifications/Verifications/Tests.md"; then
  echo "❌ FAILED: Public API verify relation was incorrectly removed"
  cat "$TEST_DIR/specifications/Verifications/Tests.md"
  exit 1
fi

# Verify Requirements.md was modified correctly
if ! diff -u "${TEST_SCRIPT_DIR}/expected-requirements.txt" "$TEST_DIR/specifications/Requirements.md" > /dev/null; then
  echo "❌ FAILED: Requirements.md does not match expected state after fix"
  diff -u "${TEST_SCRIPT_DIR}/expected-requirements.txt" "$TEST_DIR/specifications/Requirements.md"
  exit 1
fi

# Test 6: After fix
set +e
POST_FIX_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint 2>&1)
POST_FIX_EXIT=$?
set -e

if [ $POST_FIX_EXIT -ne 0 ]; then
  echo "❌ FAILED: lint after fix exit code $POST_FIX_EXIT"
  echo "$POST_FIX_OUTPUT"
  exit 1
fi

if ! diff -u "${TEST_SCRIPT_DIR}/expected-after-fix.txt" <(echo "$POST_FIX_OUTPUT") > /dev/null; then
  echo "❌ FAILED: Post-fix output mismatch"
  diff -u "${TEST_SCRIPT_DIR}/expected-after-fix.txt" <(echo "$POST_FIX_OUTPUT")
  exit 1
fi

# Test 7: Generate diagrams after fix
set +e
DIAGRAM_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" generate-diagrams 2>&1)
DIAGRAM_EXIT=$?
set -e

if [ $DIAGRAM_EXIT -ne 0 ]; then
  echo "❌ FAILED: generate-diagrams exit code $DIAGRAM_EXIT"
  echo "$DIAGRAM_OUTPUT"
  exit 1
fi

# Verify Verifications/Tests.md has correct diagrams after fix
if ! diff -u "${TEST_SCRIPT_DIR}/expected-verifications-after-fix.md" "$TEST_DIR/specifications/Verifications/Tests.md" > /dev/null; then
  echo "❌ FAILED: Verifications/Tests.md does not match expected state after fix and diagram generation"
  diff -u "${TEST_SCRIPT_DIR}/expected-verifications-after-fix.md" "$TEST_DIR/specifications/Verifications/Tests.md"
  exit 1
fi

echo "✅ PASSED: Lint command verification"
exit 0
