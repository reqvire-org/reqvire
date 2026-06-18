#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test: Type Validation Error Messages
# --------------------------------------
# Satisfies: system-model/System/Core/Verifications/ValidationVerifications.md#type-validation-errors-test
#
# Acceptance Criteria:
# - Invalid element types in --filter-type show list of valid types
# - Error includes the custom type pattern hint "other-TYPENAME"
# - Custom type pattern "other-TYPENAME" is accepted as valid
#
# Test Criteria:
# - Commands with invalid types exit with non-zero code
# - Error output matches expected format with valid types list
# - Commands with valid custom types succeed

echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Helper function to strip timestamp from error output
# Input: "[2025-12-02T17:28:47Z ERROR reqvire] message"
# Output: "message"
strip_timestamp() {
  sed 's/^\[[0-9T:Z-]*[[:space:]]*ERROR[[:space:]]*reqvire\][[:space:]]*//'
}

# Helper function to compare outputs
assert_output_matches() {
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

# Test 1: Invalid element type in --filter-type for search
echo "Test 1: Invalid --filter-type error shows valid types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type="invalid-type" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Should fail with non-zero exit code
if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: Expected non-zero exit code for invalid type"
  exit 1
fi

# Compare error output against expected (strip timestamp for reproducibility)
echo "$OUTPUT" | strip_timestamp > "${TEST_DIR}/actual-search-invalid-type.txt"
assert_output_matches "${TEST_SCRIPT_DIR}/expected/search-invalid-type.txt" \
  "${TEST_DIR}/actual-search-invalid-type.txt" \
  "Search invalid type error does not match expected"

# Test 2: Invalid element type in --filter-type for model command
echo "Test 2: Invalid --filter-type in model shows valid types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --filter-type="not-a-real-type" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -eq 0 ]; then
  echo "FAILED: Expected non-zero exit code for invalid type in model"
  exit 1
fi

echo "$OUTPUT" | strip_timestamp > "${TEST_DIR}/actual-model-invalid-type.txt"
assert_output_matches "${TEST_SCRIPT_DIR}/expected/model-invalid-type.txt" \
  "${TEST_DIR}/actual-model-invalid-type.txt" \
  "Model invalid type error does not match expected"

# Test 3: Custom type pattern is accepted (should succeed with 0 elements)
echo "Test 3: other-TYPENAME pattern is accepted" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type="other-custom" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: other-custom should be accepted as valid type"
  echo "$OUTPUT"
  exit 1
fi

# Test 4: New standard types are accepted by search and model filters
for TYPE in state input-output formal-proof-verification verification-objective source semantic-contract capability ontology; do
  echo "Test 4: Valid --filter-type $TYPE is accepted by search" >> "${TEST_DIR}/test_results.log"
  set +e
  OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type="$TYPE" --json 2>&1)
  EXIT_CODE=$?
  set -e

  echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
  printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

  if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: $TYPE should be accepted as a valid search filter type"
    echo "$OUTPUT"
    exit 1
  fi

  echo "Test 4: Valid --filter-type $TYPE is accepted by model" >> "${TEST_DIR}/test_results.log"
  set +e
  OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --filter-type="$TYPE" --json 2>&1)
  EXIT_CODE=$?
  set -e

  echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
  printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

  if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: $TYPE should be accepted as a valid model filter type"
    echo "$OUTPUT"
    exit 1
  fi
done

exit 0
