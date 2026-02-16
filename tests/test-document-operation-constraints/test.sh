#!/bin/bash
set -euo pipefail

echo "===================================="
echo "Document Operation Constraint Tests"
echo "===================================="

auto_fail() {
  echo "FAILED: $1"
  exit 1
}

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

normalize_error_output() {
  local raw="$1"
  # Strip ANSI codes and extract deterministic error line
  echo "$raw" \
    | sed 's/\x1b\[[0-9;]*m//g' \
    | sed -E 's/^\[[^]]+\][[:space:]]*//' \
    | grep -E "Invalid operation:" \
    | tail -n 1
}

run_cmd_expect_fail_with_expected() {
  local expected_file="$1"
  local actual_file="$2"
  shift 2
  set +e
  local out
  out=$(cd "$TEST_DIR" && "$REQVIRE_BIN" "$@" 2>&1)
  local code=$?
  set -e
  [ $code -ne 0 ] || auto_fail "Command unexpectedly succeeded: reqvire $*"

  local normalized
  normalized="$(normalize_error_output "$out")"
  [ -n "$normalized" ] || {
    echo "$out"
    auto_fail "Could not extract deterministic error line for: reqvire $*"
  }

  printf "%s\n" "$normalized" > "$actual_file"
  if ! diff -u "$expected_file" "$actual_file"; then
    echo "Actual command output:"
    echo "$out"
    auto_fail "Output mismatch for: reqvire $*"
  fi
}

# Baseline model validity
set +e
INIT_VALIDATE_OUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
INIT_VALIDATE_CODE=$?
set -e
if [ $INIT_VALIDATE_CODE -ne 0 ]; then
  echo "$INIT_VALIDATE_OUT"
  auto_fail "Initial model should validate"
fi

# 1) merge from #Documents source into #Elements target must fail
run_cmd_expect_fail_with_expected \
  "$TEST_SCRIPT_DIR/expected/merge-documents-to-elements-error.txt" \
  "$TEST_DIR/output/merge-documents-to-elements-error.actual.txt" \
  merge "Target Requirement" "Document Source Requirement"

# 2) mv into existing #Documents file (single-element) must fail
run_cmd_expect_fail_with_expected \
  "$TEST_SCRIPT_DIR/expected/mv-into-documents-error.txt" \
  "$TEST_DIR/output/mv-into-documents-error.actual.txt" \
  mv "Move Source Requirement" "specifications/DesignDocuments/DocSource.md"

# 3) mv-file --squash into existing #Documents file must fail
run_cmd_expect_fail_with_expected \
  "$TEST_SCRIPT_DIR/expected/mv-file-squash-into-documents-error.txt" \
  "$TEST_DIR/output/mv-file-squash-into-documents-error.actual.txt" \
  mv-file --squash "specifications/SquashSource.md" "specifications/DesignDocuments/DocSource.md"

# Ensure failed operations did not corrupt model
set +e
FINAL_VALIDATE_OUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
FINAL_VALIDATE_CODE=$?
set -e
if [ $FINAL_VALIDATE_CODE -ne 0 ]; then
  echo "$FINAL_VALIDATE_OUT"
  auto_fail "Model should remain valid after rejected operations"
fi

echo "All document operation constraint tests passed"
