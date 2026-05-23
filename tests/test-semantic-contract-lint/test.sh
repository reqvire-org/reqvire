#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

assert_json_diff() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u <(jq -S . "$expected") <(echo "$actual" | jq -S .); then
    echo "FAILED: ${description}"
    exit 1
  fi
}

assert_text_diff() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: ${description}"
    exit 1
  fi
}

set +e
AUDITABLE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --auditable 2>&1)
AUDITABLE_EXIT=$?
set -e
if [ $AUDITABLE_EXIT -ne 0 ]; then
  echo "FAILED: lint --auditable exit code ${AUDITABLE_EXIT}"
  echo "$AUDITABLE_OUTPUT"
  exit 1
fi
printf "%s\n" "$AUDITABLE_OUTPUT" > /tmp/semantic-contract-lint-auditable.out
assert_text_diff \
  "${TEST_SCRIPT_DIR}/expected/auditable.txt" \
  /tmp/semantic-contract-lint-auditable.out \
  "auditable semantic lint output mismatch"

set +e
AUDITABLE_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --auditable --json 2>&1)
AUDITABLE_JSON_EXIT=$?
set -e
if [ $AUDITABLE_JSON_EXIT -ne 0 ]; then
  echo "FAILED: lint --auditable --json exit code ${AUDITABLE_JSON_EXIT}"
  echo "$AUDITABLE_JSON_OUTPUT"
  exit 1
fi
assert_json_diff \
  "${TEST_SCRIPT_DIR}/expected/auditable.json" \
  "$AUDITABLE_JSON_OUTPUT" \
  "auditable semantic lint JSON mismatch"

set +e
FIXABLE_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --fixable --json 2>&1)
FIXABLE_JSON_EXIT=$?
set -e
if [ $FIXABLE_JSON_EXIT -ne 0 ]; then
  echo "FAILED: lint --fixable --json exit code ${FIXABLE_JSON_EXIT}"
  echo "$FIXABLE_JSON_OUTPUT"
  exit 1
fi
assert_json_diff \
  "${TEST_SCRIPT_DIR}/expected/fixable.json" \
  "$FIXABLE_JSON_OUTPUT" \
  "fixable semantic lint JSON mismatch"

cp "$TEST_DIR/specifications/SemanticContracts.md" /tmp/semantic-contract-lint-before-fix.md
set +e
FIX_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" lint --fix 2>&1)
FIX_EXIT=$?
set -e
if [ $FIX_EXIT -ne 0 ]; then
  echo "FAILED: lint --fix exit code ${FIX_EXIT}"
  echo "$FIX_OUTPUT"
  exit 1
fi
assert_text_diff \
  /tmp/semantic-contract-lint-before-fix.md \
  "$TEST_DIR/specifications/SemanticContracts.md" \
  "semantic lint --fix should not modify auditable issues"

exit 0
