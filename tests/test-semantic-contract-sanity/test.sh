#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

copy_fixture() {
  local fixture="$1"
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cp "${TEST_SCRIPT_DIR}/fixtures/${fixture}" "${TEST_DIR}/specifications/SemanticContract.md"
}

assert_diff() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: ${description}"
    echo ""
    echo "If changes are intentional, update ${expected}"
    exit 1
  fi
}

assert_invalid_contains() {
  local fixture="$1"
  local expected="$2"
  local marker="$3"
  local actual="$4"

  copy_fixture "$fixture"
  set +e
  OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  EXIT_CODE=$?
  set -e
  if [ $EXIT_CODE -eq 0 ]; then
    echo "FAILED: ${fixture} should fail validation"
    exit 1
  fi
  if echo "$OUTPUT" | grep -Fqi "$marker"; then
    printf "%s\n" "$expected" > "$actual"
  else
    printf "%s\n" "$OUTPUT" > "$actual"
  fi
}

copy_fixture "valid.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/semantic-contract-sanity-valid.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: valid semantic contract sanity fixture should validate"
  cat /tmp/semantic-contract-sanity-valid.out
  exit 1
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/valid.txt" \
  /tmp/semantic-contract-sanity-valid.out \
  "valid semantic contract sanity output mismatch"

for valid_fixture in attached-context-valid.md.txt capability-hierarchy-context-valid.md.txt requirement-owned-shape-valid.md.txt
do
  copy_fixture "$valid_fixture"
  set +e
  VALID_CONTEXT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  VALID_CONTEXT_EXIT=$?
  set -e
  if [ $VALID_CONTEXT_EXIT -ne 0 ]; then
    echo "FAILED: ${valid_fixture} should validate"
    echo "$VALID_CONTEXT_OUTPUT"
    exit 1
  fi
done

assert_invalid_contains \
  "capability-owned-shape-invalid.md.txt" \
  "Capability-owned semantic contract is rejected" \
  "semantic-contract must refine a requirement" \
  /tmp/semantic-contract-sanity-capability-owned.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/capability-owned-shape-invalid.txt" \
  /tmp/semantic-contract-sanity-capability-owned.out \
  "capability-owned semantic contract output mismatch"

assert_invalid_contains \
  "unknown-target-class.md.txt" \
  "Unknown target class is rejected" \
  "Semantic reference not found" \
  /tmp/semantic-contract-sanity-unknown-target.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/unknown-target-class.txt" \
  /tmp/semantic-contract-sanity-unknown-target.out \
  "unknown target class output mismatch"

assert_invalid_contains \
  "missing-property-path.md.txt" \
  "Property shape without sh:path is rejected" \
  "must define exactly one sh:path" \
  /tmp/semantic-contract-sanity-missing-path.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/missing-property-path.txt" \
  /tmp/semantic-contract-sanity-missing-path.out \
  "missing property path output mismatch"

assert_invalid_contains \
  "unknown-property-path.md.txt" \
  "Unknown property path is rejected" \
  "Semantic reference not found" \
  /tmp/semantic-contract-sanity-unknown-path.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/unknown-property-path.txt" \
  /tmp/semantic-contract-sanity-unknown-path.out \
  "unknown property path output mismatch"

assert_invalid_contains \
  "unknown-shacl-class.md.txt" \
  "Unknown SHACL class is rejected" \
  "Semantic reference not found" \
  /tmp/semantic-contract-sanity-unknown-shacl-class.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/unknown-shacl-class.txt" \
  /tmp/semantic-contract-sanity-unknown-shacl-class.out \
  "unknown SHACL class output mismatch"

assert_invalid_contains \
  "outside-context-reference.md.txt" \
  "Outside-context semantic reference is rejected" \
  "Semantic reference outside context" \
  /tmp/semantic-contract-sanity-outside-context.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/outside-context-reference.txt" \
  /tmp/semantic-contract-sanity-outside-context.out \
  "outside-context semantic reference output mismatch"

assert_invalid_contains \
  "invalid-cardinality.md.txt" \
  "Invalid SHACL cardinality is rejected" \
  "sh:maxCount must be greater than or equal to sh:minCount" \
  /tmp/semantic-contract-sanity-cardinality.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/invalid-cardinality.txt" \
  /tmp/semantic-contract-sanity-cardinality.out \
  "invalid cardinality output mismatch"

assert_invalid_contains \
  "invalid-shacl-list.md.txt" \
  "Invalid sh:in RDF list is rejected" \
  "sh:in RDF list nodes must have exactly one rdf:first and rdf:rest" \
  /tmp/semantic-contract-sanity-list.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/invalid-shacl-list.txt" \
  /tmp/semantic-contract-sanity-list.out \
  "invalid sh:in list output mismatch"

assert_invalid_contains \
  "duplicate-declaration.md.txt" \
  "Duplicate ontology term declaration is rejected" \
  "Duplicate ontology term declaration" \
  /tmp/semantic-contract-sanity-duplicate-declaration.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/duplicate-declaration.txt" \
  /tmp/semantic-contract-sanity-duplicate-declaration.out \
  "duplicate ontology term declaration output mismatch"

assert_invalid_contains \
  "conflicting-declaration.md.txt" \
  "Conflicting ontology term declaration is rejected" \
  "Conflicting ontology term declaration" \
  /tmp/semantic-contract-sanity-conflicting-declaration.out
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/conflicting-declaration.txt" \
  /tmp/semantic-contract-sanity-conflicting-declaration.out \
  "conflicting ontology term declaration output mismatch"

exit 0
