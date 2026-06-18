#!/bin/bash
set -euo pipefail

echo "===================================="
echo "Single Element Contract Format Tests"
echo "===================================="

auto_fail() {
  echo "FAILED: $1"
  exit 1
}

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

run_validate() {
  set +e
  local out
  out=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  local code=$?
  set -e
  echo "$out"
  return $code
}

normalize_error_output() {
  local raw="$1"
  echo "$raw" \
    | sed 's/\x1b\[[0-9;]*m//g' \
    | sed -E 's/^\[[^]]+\][[:space:]]*//' \
    | grep -E "[Ii]ncompatible element types for relation: Relation 'definedBy'|definedBy target '.*' is invalid\\." \
    | sed -E 's/^[[:space:]]*[0-9]+\.[[:space:]]+//' \
    | head -n 1
}

assert_validate_failure_matches() {
  local expected_file="$1"
  local actual_file="$2"
  local out="$3"
  local code="$4"

  [ "$code" -ne 0 ] || auto_fail "Validation unexpectedly succeeded"
  local normalized
  normalized="$(normalize_error_output "$out")"
  [ -n "$normalized" ] || {
    echo "$out"
    auto_fail "Could not extract expected definedBy validation error line"
  }
  printf "%s\n" "$normalized" > "$actual_file"
  if ! diff -u "$expected_file" "$actual_file"; then
    echo "$out"
    auto_fail "Validation output mismatch for expected failure"
  fi
}

REQ_FILE="$TEST_DIR/specifications/Requirements.md"
DOC_FILE="$TEST_DIR/specifications/DesignDocuments/ChangePropagation.md"

# 1) Valid definedBy identifier target into # Element element passes
OUT=$(run_validate) || {
  echo "$OUT"
  auto_fail "Valid definedBy element target in # Element should pass validation"
}

# 2) # Element file parsed as one element
SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json)
DOC_COUNT=$(echo "$SEARCH_JSON" | jq '[.files["specifications/DesignDocuments/ChangePropagation.md"].elements[]] | length')
[ "$DOC_COUNT" -eq 1 ] || auto_fail "# Element file must produce exactly one parsed element"
DOC_TYPE=$(echo "$SEARCH_JSON" | jq -r '.files["specifications/DesignDocuments/ChangePropagation.md"].elements[0].type')
[ "$DOC_TYPE" = "specification" ] || auto_fail "Single element type must come from metadata"

# 3) definedBy fails when targeted # Element file element type is non-contract
sed -i "s@  \\* type: specification@  * type: requirement@" "$DOC_FILE"
set +e
OUT=$(run_validate)
CODE=$?
set -e
assert_validate_failure_matches \
  "$TEST_SCRIPT_DIR/expected/definedby-non-contract-type-error.txt" \
  "$TEST_DIR/output/definedby-non-contract-type-error.actual.txt" \
  "$OUT" "$CODE"

# restore valid doc type
sed -i "s@  \\* type: requirement@  * type: specification@" "$DOC_FILE"

# 4) definedBy file target fails (must point to element identifier, not plain file path)
sed -i "s@  \\* definedBy: \\[ChangePropagation\\](DesignDocuments/ChangePropagation.md#changepropagation)@  * definedBy: [ChangePropagation.md](DesignDocuments/ChangePropagation.md)@" "$REQ_FILE"
set +e
OUT=$(run_validate)
CODE=$?
set -e
[ $CODE -ne 0 ] || auto_fail "definedBy plain file target should fail"
assert_validate_failure_matches \
  "$TEST_SCRIPT_DIR/expected/definedby-file-target-error.txt" \
  "$TEST_DIR/output/definedby-file-target-error.actual.txt" \
  "$OUT" "$CODE"

# restore definedBy identifier target
sed -i "s@  \\* definedBy: \\[ChangePropagation.md\\](DesignDocuments/ChangePropagation.md)@  * definedBy: [ChangePropagation](DesignDocuments/ChangePropagation.md#changepropagation)@" "$REQ_FILE"

# 5) # Element body may contain nested markdown headings after element name
cat > "$DOC_FILE" <<'DOC'
# Element

## Metadata
  * type: specification

## Relations
  * define: [Requirement Using Single Element Contract](../Requirements.md#requirement-using-single-element-contract)

## ChangePropagation

### Element One
Body

### Element Two
Body
DOC

# This is still one element because headers are part of the body; validation should pass.
OUT=$(run_validate) || {
  echo "$OUT"
  auto_fail "Body headers should be allowed in # Element format"
}

echo "All single element contract format tests passed"
