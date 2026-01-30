#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Impact Scope Summary in Change Impact Report
# ---------------------------------------------------
# Acceptance Criteria:
# - When two sibling requirements change, their parent appears as scope root
# - When a requirement is deleted, its parent (if still in model) appears in scope
# - Single changed element without siblings remains as-is in scope
# - Impact scope section appears between Changed Elements and Invalidated Verifications
# - Both text and JSON outputs include impact scope data

# Modify both children of Branch A (siblings) -> should merge into Branch A
sed -i 's/The system shall implement leaf A1 feature./The system shall implement leaf A1 feature with updates./' "${TEST_DIR}/Requirements.md"
sed -i 's/The system shall implement leaf A2 feature./The system shall implement leaf A2 feature with updates./' "${TEST_DIR}/Requirements.md"

# Delete Leaf B1 -> its parent Branch B should appear in scope
sed -i '/^### Leaf B1$/,/^---$/d' "${TEST_DIR}/Requirements.md"

# Modify standalone req -> should remain as-is (no sibling to merge)
sed -i 's/The system shall provide standalone functionality./The system shall provide standalone functionality with updates./' "${TEST_DIR}/Requirements.md"

# Test 1: Run change impact detection (text output)
set +e
OUTPUT=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: Change impact detection failed with exit code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Sanitize output (remove blob URLs for deterministic comparison)
GOTTEN_CONTENT=$(echo "$OUTPUT" | grep -v "INFO  reqvire::config" | grep -v "Warning: Element")
SANITIZED_OUTPUT=$(echo "$GOTTEN_CONTENT" | sed -E 's#https://[^ )]+/blob/(HEAD|[a-f0-9]{7,40})/##g')

if ! diff -u "${TEST_SCRIPT_DIR}/expected/change-impact-report.txt" <(echo "$SANITIZED_OUTPUT"); then
  echo "FAILED: Text output not matching expected content."
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/change-impact-report.txt"
  exit 1
fi

# Test 2: Run change impact detection (JSON output)
set +e
JSON_RAW=$(cd "${TEST_DIR}" && "${REQVIRE_BIN}" change-impact --json 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: Change impact detection with JSON output failed with exit code $EXIT_CODE"
    echo "$JSON_RAW"
    exit 1
fi

# Extract JSON (skip warnings)
JSON_OUTPUT=$(echo "$JSON_RAW" | grep -v "Warning:" | grep -A 1000 "^{")

if ! echo "$JSON_OUTPUT" | jq . >/dev/null 2>&1; then
    echo "FAILED: Output is not valid JSON"
    exit 1
fi

# Verify impact_scope exists and has expected entries
SCOPE_COUNT=$(echo "$JSON_OUTPUT" | jq '.impact_scope | length')
if [ "$SCOPE_COUNT" -ne 3 ]; then
    echo "FAILED: Expected 3 impact scope entries, got $SCOPE_COUNT"
    echo "$JSON_OUTPUT" | jq '.impact_scope'
    exit 1
fi

# Verify scope names (sorted by element_id, which is deterministic)
SCOPE_NAMES=$(echo "$JSON_OUTPUT" | jq -r '.impact_scope[].name')
EXPECTED_NAMES=$(printf "Branch A\nBranch B\nStandalone Req")
if [ "$SCOPE_NAMES" != "$EXPECTED_NAMES" ]; then
    echo "FAILED: Expected scope names 'Branch A, Branch B, Standalone Req', got: $SCOPE_NAMES"
    exit 1
fi

exit 0
