#!/usr/bin/env bash
set -uo pipefail

echo "Starting test..." > "${TEST_DIR}/test_results.log"

echo "Running: reqvire containment" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" containment 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: containment command failed"
  echo "$OUTPUT"
  exit 1
fi

if ! jq . >/dev/null 2>&1 <<< "$OUTPUT"; then
  echo "FAILED: containment output is not valid JSON"
  echo "$OUTPUT"
  exit 1
fi

if ! jq -e '
  tostring
  | contains("RootFile.md")
  and contains("Root Capability")
  and contains("Root System Requirement")
  and contains("Test Verification Element")
  and contains("Custom Type Element")
' >/dev/null 2>&1 <<< "$OUTPUT"; then
  echo "FAILED: containment JSON is missing expected folder/file/element content"
  echo "$OUTPUT"
  exit 1
fi

echo "Running: reqvire containment --short" >> "${TEST_DIR}/test_results.log"
set +e
SHORT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" containment --short 2>&1)
SHORT_EXIT=$?
set -e

if [ $SHORT_EXIT -ne 0 ]; then
  echo "FAILED: containment --short command failed"
  echo "$SHORT_OUTPUT"
  exit 1
fi

if ! jq . >/dev/null 2>&1 <<< "$SHORT_OUTPUT"; then
  echo "FAILED: containment --short output is not valid JSON"
  echo "$SHORT_OUTPUT"
  exit 1
fi

if ! jq -e 'tostring | contains("RootFile.md")' >/dev/null 2>&1 <<< "$SHORT_OUTPUT"; then
  echo "FAILED: containment --short JSON missing fixture file"
  exit 1
fi

set +e
FILE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" containment --output containment.json 2>&1)
FILE_EXIT=$?
set -e

if [ $FILE_EXIT -ne 0 ]; then
  echo "FAILED: containment --output exited with code $FILE_EXIT"
  echo "$FILE_OUTPUT"
  exit 1
fi

if ! grep -q "Output saved to" <<< "$FILE_OUTPUT"; then
  echo "FAILED: containment --output did not print confirmation"
  echo "$FILE_OUTPUT"
  exit 1
fi

if ! jq . "${TEST_DIR}/containment.json" >/dev/null 2>&1; then
  echo "FAILED: containment --output file is not valid JSON"
  exit 1
fi

if [ "$(jq -S . <<< "$OUTPUT")" != "$(jq -S . "${TEST_DIR}/containment.json")" ]; then
  echo "FAILED: containment --output file differs from stdout JSON"
  exit 1
fi

set +e
LEGACY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" containment --json 2>&1)
LEGACY_EXIT=$?
set -e

if [ $LEGACY_EXIT -eq 0 ]; then
  echo "FAILED: removed --json flag should not be accepted by containment"
  exit 1
fi

exit 0
