#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Starting test..." > "${TEST_DIR}/test_results.log"

run_model() {
    local label="$1"
    shift

    echo "Running: reqvire model $*" >> "${TEST_DIR}/test_results.log"
    set +e
    OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model "$@" 2>&1)
    EXIT_CODE=$?
    set -e

    echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
    printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

    if [ $EXIT_CODE -ne 0 ]; then
        echo "FAILED ($label): model command exited with code $EXIT_CODE"
        echo "$OUTPUT"
        exit 1
    fi

    if ! jq . >/dev/null 2>&1 <<< "$OUTPUT"; then
        echo "FAILED ($label): output is not valid JSON"
        echo "$OUTPUT"
        exit 1
    fi
}

assert_json_matches() {
    local label="$1"
    local expected_file="$2"
    local actual_file="$3"

    local expected_json
    local actual_json
    expected_json=$(jq -S . "$expected_file")
    actual_json=$(jq -S . "$actual_file")

    if [ "$expected_json" != "$actual_json" ]; then
        echo "FAILED ($label): JSON output does not match expected"
        diff -u <(echo "$expected_json") <(echo "$actual_json") || true
        exit 1
    fi
}

run_model "default"
echo "$OUTPUT" | jq . > "${TEST_DIR}/actual_output.json"
assert_json_matches "default" "${TEST_SCRIPT_DIR}/expected/expected_output.json" "${TEST_DIR}/actual_output.json"

if jq -e '.. | objects | has("size_estimate")' >/dev/null 2>&1 <<< "$OUTPUT"; then
    echo "FAILED: model should omit size_estimate unless explicitly enabled"
    exit 1
fi

run_model "size estimates" --with-size-estimates

if ! jq -e '
  [.elements[]? | .. | objects | select(has("identifier") and has("name"))] as $elements
  | ($elements | length) > 0
  and all($elements[];
    (.size_estimate.content_bytes | type == "number")
    and (.size_estimate.rendered_context_bytes | type == "number")
    and (.size_estimate.estimated_tokens | type == "number")
  )
' >/dev/null 2>&1 <<< "$OUTPUT"; then
    echo "FAILED: --with-size-estimates should include size estimates on all element payloads"
    echo "$OUTPUT"
    exit 1
fi

run_model "filtered" --from="Model Diagram Generation"
echo "$OUTPUT" | jq . > "${TEST_DIR}/actual_filtered_output.json"
assert_json_matches "filtered" "${TEST_SCRIPT_DIR}/expected/expected_filtered_output.json" "${TEST_DIR}/actual_filtered_output.json"

run_model "reverse" --reverse
echo "$OUTPUT" | jq . > "${TEST_DIR}/actual_reverse_output.json"
assert_json_matches "reverse" "${TEST_SCRIPT_DIR}/expected/expected_reverse_output.json" "${TEST_DIR}/actual_reverse_output.json"

run_model "filter-type" --filter-type=test-verification

TYPE_FILTER=$(jq -r '.metadata.type_filter[0]' <<< "$OUTPUT")
if [ "$TYPE_FILTER" != "test-verification" ]; then
    echo "FAILED: expected type_filter to contain test-verification, got $TYPE_FILTER"
    exit 1
fi

WRONG_TYPE=$(jq '[.elements[].element_type] | map(select(. != "test-verification")) | length' <<< "$OUTPUT")
if [ "$WRONG_TYPE" -ne 0 ]; then
    echo "FAILED: filter-type should only return test-verification elements at top level"
    exit 1
fi

run_model "reverse filter-type" --reverse --filter-type=test-verification
echo "$OUTPUT" | jq . > "${TEST_DIR}/actual_reverse_filter_output.json"
assert_json_matches "reverse filter-type" "${TEST_SCRIPT_DIR}/expected/expected_reverse_filter_output.json" "${TEST_DIR}/actual_reverse_filter_output.json"

set +e
LEGACY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --json 2>&1)
LEGACY_EXIT=$?
set -e

if [ $LEGACY_EXIT -eq 0 ]; then
    echo "FAILED: removed --json flag should not be accepted by model"
    exit 1
fi

set +e
LEGACY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --mmd 2>&1)
LEGACY_EXIT=$?
set -e

if [ $LEGACY_EXIT -eq 0 ]; then
    echo "FAILED: removed --mmd flag should not be accepted by model"
    exit 1
fi

exit 0
