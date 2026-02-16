#!/bin/bash
set -euo pipefail

echo "===================================="
echo "Document Refinement Format Tests"
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
    | grep -E "[Ii]ncompatible element types for relation: Relation 'refinedBy'|refinedBy target '.*' is invalid\\." \
    | sed -E 's/^[[:space:]]+//' \
    | sed -E 's/^[0-9]+\\. //' \
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
    auto_fail "Could not extract expected refinedBy validation error line"
  }
  printf "%s\n" "$normalized" > "$actual_file"
  if ! diff -u "$expected_file" "$actual_file"; then
    echo "$out"
    auto_fail "Validation output mismatch for expected failure"
  fi
}

REQ_FILE="$TEST_DIR/specifications/Requirements.md"
DOC_FILE="$TEST_DIR/specifications/DesignDocuments/ChangePropagation.md"

# 1) Valid refinedBy identifier target into #Documents element passes
OUT=$(run_validate) || {
  echo "$OUT"
  auto_fail "Valid refinedBy element target in #Documents should pass validation"
}

# 2) Document parsed as one element
SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json)
DOC_COUNT=$(echo "$SEARCH_JSON" | jq '[.files["specifications/DesignDocuments/ChangePropagation.md"].elements[]] | length')
[ "$DOC_COUNT" -eq 1 ] || auto_fail "#Documents file must produce exactly one parsed element"
DOC_TYPE=$(echo "$SEARCH_JSON" | jq -r '.files["specifications/DesignDocuments/ChangePropagation.md"].elements[0].type')
[ "$DOC_TYPE" = "specification" ] || auto_fail "Document element type must come from metadata"

# 3) refinedBy fails when targeted document element type is non-refinement
perl -0777 -i -pe 's/type: specification/type: requirement/' "$DOC_FILE"
set +e
OUT=$(run_validate)
CODE=$?
set -e
assert_validate_failure_matches \
  "$TEST_SCRIPT_DIR/expected/refinedby-non-refinement-type-error.txt" \
  "$TEST_DIR/output/refinedby-non-refinement-type-error.actual.txt" \
  "$OUT" "$CODE"

# restore valid doc type
perl -0777 -i -pe 's/type: requirement/type: specification/' "$DOC_FILE"

# 4) refinedBy file target fails (must point to element identifier, not plain document path)
sed -i "s@  \\* refinedBy: \\[ChangePropagation\\](DesignDocuments/ChangePropagation.md#changepropagation)@  * refinedBy: [ChangePropagation.md](DesignDocuments/ChangePropagation.md)@" "$REQ_FILE"
set +e
OUT=$(run_validate)
CODE=$?
set -e
[ $CODE -ne 0 ] || auto_fail "refinedBy plain file target should fail"
assert_validate_failure_matches \
  "$TEST_SCRIPT_DIR/expected/refinedby-file-target-error.txt" \
  "$TEST_DIR/output/refinedby-file-target-error.actual.txt" \
  "$OUT" "$CODE"

# restore refinedBy identifier target
sed -i "s@  \\* refinedBy: \\[ChangePropagation.md\\](DesignDocuments/ChangePropagation.md)@  * refinedBy: [ChangePropagation](DesignDocuments/ChangePropagation.md#changepropagation)@" "$REQ_FILE"

# 5) #Documents body may contain nested markdown headings after element name
cat > "$DOC_FILE" <<'DOC'
# Documents

## Metadata
  * type: specification

## Relations
  * refine: [Requirement Using Document Refinement](../Requirements.md#requirement-using-document-refinement)

## ChangePropagation

### Element One
Body

### Element Two
Body
DOC

# This is still one document element because headers are part of the body; validation should pass.
OUT=$(run_validate) || {
  echo "$OUT"
  auto_fail "Document body headers should be allowed in #Documents format"
}

echo "All document refinement format tests passed"
