#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

copy_fixture() {
  local fixture="$1"
  rm -rf "${TEST_DIR}/specifications"
  mkdir -p "${TEST_DIR}/specifications"
  cp "${TEST_SCRIPT_DIR}/fixtures/${fixture}" "${TEST_DIR}/specifications/RecursiveShacl.md"
  cp -a "${TEST_SCRIPT_DIR}/fixtures/references" "${TEST_DIR}/specifications/references"
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

copy_fixture "valid.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/shacl-recursive-ast-valid.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: valid recursive SHACL AST fixture should validate"
  cat /tmp/shacl-recursive-ast-valid.out
  exit 1
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/valid.txt" \
  /tmp/shacl-recursive-ast-valid.out \
  "valid recursive SHACL AST output mismatch"

copy_fixture "multi-base-hierarchy-valid.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/shacl-recursive-ast-multi-base-valid.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: valid multi-base ontology hierarchy fixture should validate"
  cat /tmp/shacl-recursive-ast-multi-base-valid.out
  exit 1
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/valid.txt" \
  /tmp/shacl-recursive-ast-multi-base-valid.out \
  "valid multi-base ontology hierarchy output mismatch"

copy_fixture "external-source-valid.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/shacl-recursive-ast-external-source-valid.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: valid external-source SHACL fixture should validate"
  cat /tmp/shacl-recursive-ast-external-source-valid.out
  exit 1
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/valid.txt" \
  /tmp/shacl-recursive-ast-external-source-valid.out \
  "valid external-source SHACL output mismatch"

copy_fixture "invalid-undeclared-nested-path.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/shacl-recursive-ast-invalid-nested-path.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: invalid recursive SHACL AST fixture should fail validation"
  cat /tmp/shacl-recursive-ast-invalid-nested-path.out
  exit 1
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/invalid-undeclared-nested-path.txt" \
  /tmp/shacl-recursive-ast-invalid-nested-path.out \
  "invalid nested SHACL path output mismatch"

copy_fixture "outside-context-multi-base.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/shacl-recursive-ast-outside-context-multi-base.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: outside-context multi-base fixture should fail validation"
  cat /tmp/shacl-recursive-ast-outside-context-multi-base.out
  exit 1
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/outside-context-multi-base.txt" \
  /tmp/shacl-recursive-ast-outside-context-multi-base.out \
  "outside-context multi-base SHACL output mismatch"

copy_fixture "external-source-outside-context.md.txt"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/shacl-recursive-ast-external-source-outside-context.out 2>&1)
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: outside-context external-source fixture should fail validation"
  cat /tmp/shacl-recursive-ast-external-source-outside-context.out
  exit 1
fi
assert_diff \
  "${TEST_SCRIPT_DIR}/expected/external-source-outside-context.txt" \
  /tmp/shacl-recursive-ast-external-source-outside-context.out \
  "outside-context external-source SHACL output mismatch"

exit 0
