#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Starting test..." > "${TEST_DIR}/test_results.log"

run_traces() {
    local label="$1"
    shift

    echo "Running: reqvire traces $*" >> "${TEST_DIR}/test_results.log"
    set +e
    OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" traces "$@" 2>&1)
    EXIT_CODE=$?
    set -e

    echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
    printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

    if [ $EXIT_CODE -ne 0 ]; then
        echo "FAILED ($label): traces command exited with code $EXIT_CODE"
        echo "$OUTPUT"
        exit 1
    fi

    if ! jq . >/dev/null 2>&1 <<< "$OUTPUT"; then
        echo "FAILED ($label): output is not valid JSON"
        echo "$OUTPUT"
        exit 1
    fi
}

verification_count() {
    jq '[.files[]?.verifications[]?] | length' <<< "$1"
}

has_verification() {
    local name="$1"
    jq -e --arg name "$name" 'any(.files[]?.verifications[]?; .name == $name)' >/dev/null 2>&1 <<< "$OUTPUT"
}

assert_missing_verification() {
    local name="$1"
    if has_verification "$name"; then
        echo "FAILED: filter should exclude $name"
        echo "$OUTPUT"
        exit 1
    fi
}

assert_present_verification() {
    local name="$1"
    if ! has_verification "$name"; then
        echo "FAILED: filter should include $name"
        echo "$OUTPUT"
        exit 1
    fi
}

run_traces "default"
echo "$OUTPUT" | jq . > "${TEST_DIR}/actual_output.json"

EXPECTED_JSON=$(jq -S . "${TEST_SCRIPT_DIR}/expected/expected_output.json")
ACTUAL_JSON=$(jq -S . "${TEST_DIR}/actual_output.json")
if [ "$EXPECTED_JSON" != "$ACTUAL_JSON" ]; then
    echo "FAILED: traces JSON output does not match expected"
    diff -u <(echo "$EXPECTED_JSON") <(echo "$ACTUAL_JSON") || true
    exit 1
fi

run_traces "filter-id" --filter-id="specifications/Verifications/Tests.md#oauth-flow-test"
assert_present_verification "OAuth Flow Test"
assert_missing_verification "Session Timeout Test"
assert_missing_verification "Encryption Coverage Test"

run_traces "filter-name" --filter-name=".*Coverage.*"
assert_present_verification "Encryption Coverage Test"
assert_present_verification "Coverage Calculation Test"
assert_missing_verification "OAuth Flow Test"

run_traces "filter-type" --filter-type="test-verification"
assert_present_verification "OAuth Flow Test"
assert_missing_verification "Security Analysis"
assert_missing_verification "Code Inspection"

run_traces "combined filters" --filter-type="test-verification" --filter-name=".*Test"
assert_present_verification "OAuth Flow Test"
assert_missing_verification "Security Analysis"

TEST_COUNT=$(verification_count "$OUTPUT")
if [ "$TEST_COUNT" -ne 4 ]; then
    echo "FAILED: combined filters should return exactly 4 verifications, got $TEST_COUNT"
    echo "$OUTPUT"
    exit 1
fi

run_traces "redundant relations"
echo "$OUTPUT" | jq . > "${TEST_DIR}/actual_redundant_output.json"
if [ -f "${TEST_SCRIPT_DIR}/expected/expected_redundant_output.json" ]; then
    EXPECTED_JSON=$(jq -S . "${TEST_SCRIPT_DIR}/expected/expected_redundant_output.json")
    ACTUAL_JSON=$(jq -S . "${TEST_DIR}/actual_redundant_output.json")

    if [ "$EXPECTED_JSON" != "$ACTUAL_JSON" ]; then
        echo "FAILED: redundant relations JSON output does not match expected"
        diff -u <(echo "$EXPECTED_JSON") <(echo "$ACTUAL_JSON") || true
        exit 1
    fi
fi

set +e
FILE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" traces --output traces.json 2>&1)
FILE_EXIT=$?
set -e

if [ $FILE_EXIT -ne 0 ]; then
    echo "FAILED: traces --output exited with code $FILE_EXIT"
    echo "$FILE_OUTPUT"
    exit 1
fi

if ! grep -q "Output saved to" <<< "$FILE_OUTPUT"; then
    echo "FAILED: traces --output did not print confirmation"
    echo "$FILE_OUTPUT"
    exit 1
fi

if ! jq . "${TEST_DIR}/traces.json" >/dev/null 2>&1; then
    echo "FAILED: traces --output file is not valid JSON"
    exit 1
fi

set +e
LEGACY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" traces --json 2>&1)
LEGACY_EXIT=$?
set -e

if [ $LEGACY_EXIT -eq 0 ]; then
    echo "FAILED: removed --json flag should not be accepted by traces"
    exit 1
fi

set +e
LEGACY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" traces --from-folder=specifications 2>&1)
LEGACY_EXIT=$?
set -e

if [ $LEGACY_EXIT -eq 0 ]; then
    echo "FAILED: removed --from-folder flag should not be accepted by traces"
    exit 1
fi

exit 0
