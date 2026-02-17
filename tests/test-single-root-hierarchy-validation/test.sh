#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "❌ FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# Scenario 1: valid single-root hierarchy
cp "${TEST_SCRIPT_DIR}/fixtures/valid.txt" "${TEST_DIR}/Requirements.md"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/single-root-valid.out 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: valid single-root hierarchy should pass"
  cat /tmp/single-root-valid.out
  exit 1
fi

assert_file_matches \
  "${TEST_SCRIPT_DIR}/expected/valid-output.txt" \
  /tmp/single-root-valid.out \
  "Valid single-root output mismatch"

# Scenario 2: invalid multi-root hierarchy ownership
cp "${TEST_SCRIPT_DIR}/fixtures/multi-root.txt" "${TEST_DIR}/Requirements.md"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/single-root-multi-root.out 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: multi-root hierarchy should fail validation"
  cat /tmp/single-root-multi-root.out
  exit 1
fi

assert_file_matches \
  "${TEST_SCRIPT_DIR}/expected/multi-root-error.txt" \
  /tmp/single-root-multi-root.out \
  "Multi-root validation output mismatch"

# Scenario 3: invalid zero-root hierarchy ownership
cp "${TEST_SCRIPT_DIR}/fixtures/zero-root.txt" "${TEST_DIR}/Requirements.md"
set +e
(cd "$TEST_DIR" && "$REQVIRE_BIN" validate > /tmp/single-root-zero-root.out 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: zero-root hierarchy should fail validation"
  cat /tmp/single-root-zero-root.out
  exit 1
fi

assert_file_matches \
  "${TEST_SCRIPT_DIR}/expected/zero-root-error.txt" \
  /tmp/single-root-zero-root.out \
  "Zero-root validation output mismatch"

exit 0
