#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# Test 1: text output
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: submodels command exited with code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

printf "%s\n" "$OUTPUT" > "$TEST_DIR/output/submodels.actual.md"
assert_file_matches \
  "$TEST_SCRIPT_DIR/expected/expected_output.md" \
  "$TEST_DIR/output/submodels.actual.md" \
  "Submodels text output mismatch"

# Test 2: JSON stdout
set +e
JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --json 2>&1)
JSON_EXIT=$?
set -e

if [ $JSON_EXIT -ne 0 ]; then
  echo "FAILED: submodels --json command exited with code $JSON_EXIT"
  echo "$JSON_OUTPUT"
  exit 1
fi

printf "%s\n" "$JSON_OUTPUT" > "$TEST_DIR/output/submodels.actual.json"
if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.actual.json"); then
  echo "FAILED: Submodels JSON output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_output.json"
  exit 1
fi

# Test 3: JSON file output
set +e
FILE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --json --output output/submodels.json 2>&1)
FILE_EXIT=$?
set -e

if [ $FILE_EXIT -ne 0 ]; then
  echo "FAILED: submodels --json --output command exited with code $FILE_EXIT"
  echo "$FILE_OUTPUT"
  exit 1
fi

if ! echo "$FILE_OUTPUT" | grep -q "Output saved to output/submodels.json"; then
  echo "FAILED: missing output confirmation message"
  echo "$FILE_OUTPUT"
  exit 1
fi

if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.json"); then
  echo "FAILED: Submodels JSON file output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_output.json"
  exit 1
fi

# Test 4: --from text output
set +e
FROM_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Root One" 2>&1)
FROM_EXIT=$?
set -e

if [ $FROM_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from command exited with code $FROM_EXIT"
  echo "$FROM_OUTPUT"
  exit 1
fi

printf "%s\n" "$FROM_OUTPUT" > "$TEST_DIR/output/submodels.from-root-one.actual.md"
assert_file_matches \
  "$TEST_SCRIPT_DIR/expected/expected_from_root_one_output.md" \
  "$TEST_DIR/output/submodels.from-root-one.actual.md" \
  "Submodels --from text output mismatch"

# Test 5: --from JSON output
set +e
FROM_JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Root One" --json 2>&1)
FROM_JSON_EXIT=$?
set -e

if [ $FROM_JSON_EXIT -ne 0 ]; then
  echo "FAILED: submodels --from --json command exited with code $FROM_JSON_EXIT"
  echo "$FROM_JSON_OUTPUT"
  exit 1
fi

printf "%s\n" "$FROM_JSON_OUTPUT" > "$TEST_DIR/output/submodels.from-root-one.actual.json"
if ! diff -u \
  <(jq -S . "$TEST_SCRIPT_DIR/expected/expected_from_root_one_output.json") \
  <(jq -S . "$TEST_DIR/output/submodels.from-root-one.actual.json"); then
  echo "FAILED: Submodels --from JSON output mismatch"
  echo ""
  echo "If changes are intentional, update $TEST_SCRIPT_DIR/expected/expected_from_root_one_output.json"
  exit 1
fi

# Test 6: --from missing root should fail
set +e
MISSING_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" submodels --from "Missing Root" 2>&1)
MISSING_EXIT=$?
set -e

if [ $MISSING_EXIT -eq 0 ]; then
  echo "FAILED: submodels --from Missing Root should fail"
  exit 1
fi

if ! echo "$MISSING_OUTPUT" | grep -qi "Submodel root 'Missing Root' not found"; then
  echo "FAILED: missing-root error message mismatch"
  echo "$MISSING_OUTPUT"
  exit 1
fi

exit 0
