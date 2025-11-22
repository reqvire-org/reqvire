#!/usr/bin/env bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Model Command Verification
# --------------------------------------
# Satisfies: specifications/Verifications/ReportsTests.md#model-command-verification
#
# Acceptance Criteria:
# - `reqvire model` generates model-centric output showing root requirements with nested relations
# - `reqvire model --from=<name>` generates nested structure from specified element
# - `reqvire model --json` generates valid JSON with nested element structure
# - `reqvire model --from=<name> --json` generates filtered JSON from specified starting point
# - Default mode filters to root requirements (no hierarchical parent relations)
# - Relations contain full target element details recursively
#
# Test Criteria:
# - Commands exit with success (0) return code
# - Markdown output matches expected structure with model-centric view
# - JSON output matches expected structure with elements, metadata, and nested relations
# - Filters correctly restrict output to forward-reachable elements only

# Test 1: Full Model Markdown Output - Compare against expected file
echo "Running: reqvire model" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model 2>&1)
EXIT_CODE=$?
set -e

# Save output for debugging (always do this)
echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model command exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Basic validation before comparing with expected
if ! grep -q '```mermaid' <<< "$OUTPUT"; then
    echo "❌ FAILED: Output missing mermaid diagram block"
    exit 1
fi

if ! grep -q 'graph LR' <<< "$OUTPUT"; then
    echo "❌ FAILED: Mermaid diagram missing 'graph LR' declaration"
    exit 1
fi

# Save actual output for comparison
echo "$OUTPUT" > "${TEST_DIR}/actual_output.md"

# Compare with expected output
if ! diff -u "${TEST_DIR}/expected_output.md" "${TEST_DIR}/actual_output.md"; then
    echo "❌ FAILED: Markdown output does not match expected format"
    echo "Expected: ${TEST_DIR}/expected_output.md"
    echo "Actual: ${TEST_DIR}/actual_output.md"
    diff -u "${TEST_DIR}/expected_output.md" "${TEST_DIR}/actual_output.md"
    exit 1
fi

# Test 2: Full Model JSON Output - Compare against expected file
echo "Running: reqvire model --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --json command exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Output is not valid JSON"
    exit 1
fi

# Validate required JSON fields
if ! echo "$OUTPUT" | jq -e 'has("elements")' >/dev/null 2>&1; then
    echo "❌ FAILED: JSON missing 'elements' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq -e 'has("metadata")' >/dev/null 2>&1; then
    echo "❌ FAILED: JSON missing 'metadata' field"
    exit 1
fi

# Save actual JSON output
echo "$OUTPUT" | jq '.' > "${TEST_DIR}/actual_output.json"

# Compare JSON outputs using jq (to handle formatting differences)
EXPECTED_JSON=$(jq -S '.' "${TEST_DIR}/expected_output.json")
ACTUAL_JSON=$(jq -S '.' "${TEST_DIR}/actual_output.json")

if [ "$EXPECTED_JSON" != "$ACTUAL_JSON" ]; then
    echo "❌ FAILED: JSON output does not match expected format"
    echo "Expected: ${TEST_DIR}/expected_output.json"
    echo "Actual: ${TEST_DIR}/actual_output.json"
    diff -u <(echo "$EXPECTED_JSON") <(echo "$ACTUAL_JSON") || true
    exit 1
fi

# Test 3: Filtered Model Markdown Output - Starting from "Model Diagram Generation"
# Forward relations from this element:
# - derive -> "Model Filtering Capability" -> "Forward Relation Traversal"
# - derive -> "JSON Output Format"
# - verifiedBy -> "Model Generation Test"
# Should NOT include:
# - "Model Structure Exploration" (parent, backward derivedFrom)
# - "Markdown Output Format" (sibling, no relation)
echo "Running: reqvire model --from (filtered)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model \
    --from="Model Diagram Generation" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --from exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Save actual filtered output
echo "$OUTPUT" > "${TEST_DIR}/actual_filtered_output.md"

# Compare with expected filtered output
if ! diff -u "${TEST_DIR}/expected_filtered_output.md" "${TEST_DIR}/actual_filtered_output.md"; then
    echo "❌ FAILED: Filtered markdown output does not match expected format"
    echo "Expected: ${TEST_DIR}/expected_filtered_output.md"
    echo "Actual: ${TEST_DIR}/actual_filtered_output.md"
    diff -u "${TEST_DIR}/expected_filtered_output.md" "${TEST_DIR}/actual_filtered_output.md"
    exit 1
fi

# Test 4: Filtered Model JSON Output
echo "Running: reqvire model --from --json (filtered)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model \
    --from="Model Diagram Generation" --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --root-id --json exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Filtered output is not valid JSON"
    exit 1
fi

# Save actual filtered JSON output
echo "$OUTPUT" | jq '.' > "${TEST_DIR}/actual_filtered_output.json"

# Compare with expected filtered JSON
EXPECTED_FILTERED_JSON=$(jq -S '.' "${TEST_DIR}/expected_filtered_output.json")
ACTUAL_FILTERED_JSON=$(jq -S '.' "${TEST_DIR}/actual_filtered_output.json")

if [ "$EXPECTED_FILTERED_JSON" != "$ACTUAL_FILTERED_JSON" ]; then
    echo "❌ FAILED: Filtered JSON output does not match expected format"
    echo "Expected: ${TEST_DIR}/expected_filtered_output.json"
    echo "Actual: ${TEST_DIR}/actual_filtered_output.json"
    diff -u <(echo "$EXPECTED_FILTERED_JSON") <(echo "$ACTUAL_FILTERED_JSON") || true
    exit 1
fi

exit 0
