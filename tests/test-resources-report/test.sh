#!/usr/bin/env bash
set -uo pipefail

echo "Starting test..." > "${TEST_DIR}/test_results.log"

echo "Running: reqvire resources" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" resources 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: resources command exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

if ! jq . >/dev/null 2>&1 <<< "$OUTPUT"; then
    echo "FAILED: resources output is not valid JSON"
    echo "$OUTPUT"
    exit 1
fi

if ! jq -e 'has("relations") and has("contract_bindings") and has("summary")' >/dev/null 2>&1 <<< "$OUTPUT"; then
    echo "FAILED: resources JSON missing required top-level fields"
    exit 1
fi

if ! jq -e '
  (.summary.total_relation_files == 4)
  and (.summary.total_contract_bindings_files == 0)
  and (.summary.total_relation_references > 0)
  and (.contract_bindings | length == 0)
' >/dev/null 2>&1 <<< "$OUTPUT"; then
    echo "FAILED: resources summary values are not as expected"
    echo "$OUTPUT"
    exit 1
fi

if ! jq -e '
  all(.relations[];
    has("file_path")
    and has("references")
    and all(.references[];
      has("relation_type")
      and has("element_id")
      and has("element_name")
    )
  )
' >/dev/null 2>&1 <<< "$OUTPUT"; then
    echo "FAILED: resources relation entries do not match JSON schema"
    exit 1
fi

set +e
FILE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" resources --output resources.json 2>&1)
FILE_EXIT=$?
set -e

if [ $FILE_EXIT -ne 0 ]; then
    echo "FAILED: resources --output exited with code $FILE_EXIT"
    echo "$FILE_OUTPUT"
    exit 1
fi

if ! grep -q "Output saved to" <<< "$FILE_OUTPUT"; then
    echo "FAILED: resources --output did not print confirmation"
    echo "$FILE_OUTPUT"
    exit 1
fi

if ! jq . "${TEST_DIR}/resources.json" >/dev/null 2>&1; then
    echo "FAILED: resources --output file is not valid JSON"
    exit 1
fi

if [ "$(jq -S . <<< "$OUTPUT")" != "$(jq -S . "${TEST_DIR}/resources.json")" ]; then
    echo "FAILED: resources --output file differs from stdout JSON"
    exit 1
fi

set +e
LEGACY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" resources --json 2>&1)
LEGACY_EXIT=$?
set -e

if [ $LEGACY_EXIT -eq 0 ]; then
    echo "FAILED: removed --json flag should not be accepted by resources"
    exit 1
fi

exit 0
