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

if grep -q "parent-requirement" "$TEST_DIR/specifications/Verifications/Tests.md"; then
  echo "❌ FAILED: Redundant verify relation not removed"
  cat "$TEST_DIR/specifications/Verifications/Tests.md"
  exit 1
fi

if ! grep -q "leaf-requirement" "$TEST_DIR/specifications/Verifications/Tests.md"; then
  echo "❌ FAILED: Original verify relation removed"
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

echo "✅ PASSED: Lint command verification"
exit 0
