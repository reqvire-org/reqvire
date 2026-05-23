#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Model Command Verification
# --------------------------------------
# Satisfies: specifications/Verifications/ReportsTests.md#model-command-verification
#
# Acceptance Criteria:
# - `reqvire model` generates model-centric output showing ontology roots and feature roots with nested relations
# - `reqvire model --from=<name>` generates nested structure from specified element
# - `reqvire model --json` generates valid JSON with nested element structure
# - `reqvire model --from=<name> --json` generates filtered JSON from specified starting point
# - Default mode filters to ontology roots and feature roots
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

if ! grep -q 'graph TD' <<< "$OUTPUT"; then
    echo "❌ FAILED: Mermaid diagram missing 'graph TD' declaration"
    exit 1
fi

# Save actual output for comparison
echo "$OUTPUT" > "${TEST_DIR}/actual_output.md"

# Compare with expected output
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected_output.md" "${TEST_DIR}/actual_output.md"; then
    echo "❌ FAILED: Markdown output does not match expected format"
    echo "Expected: ${TEST_DIR}/expected_output.md"
    echo "Actual: ${TEST_DIR}/actual_output.md"
    diff -u "${TEST_SCRIPT_DIR}/expected/expected_output.md" "${TEST_DIR}/actual_output.md"
    exit 1
fi

# Test 1b: Pure Mermaid Output
echo "Running: reqvire model --mmd" >> "${TEST_DIR}/test_results.log"
set +e
MMD_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --mmd 2>&1)
MMD_EXIT_CODE=$?
set -e

echo "Exit code: $MMD_EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$MMD_OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $MMD_EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --mmd command exited with code $MMD_EXIT_CODE"
    echo "$MMD_OUTPUT"
    exit 1
fi

if ! grep -q '^graph TD' <<< "$MMD_OUTPUT"; then
    echo "❌ FAILED: model --mmd output missing Mermaid graph declaration"
    echo "$MMD_OUTPUT"
    exit 1
fi

if grep -q '```' <<< "$MMD_OUTPUT"; then
    echo "❌ FAILED: model --mmd output should not contain Markdown fences"
    echo "$MMD_OUTPUT"
    exit 1
fi

if ! grep -q 'Model Command Ontology' <<< "$MMD_OUTPUT"; then
    echo "❌ FAILED: model --mmd output should include ontology roots"
    echo "$MMD_OUTPUT"
    exit 1
fi

if ! grep -q -- '-->|attaches|' <<< "$MMD_OUTPUT"; then
    echo "❌ FAILED: model --mmd output should include attachment edges"
    echo "$MMD_OUTPUT"
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
EXPECTED_JSON=$(jq -S '.' "${TEST_SCRIPT_DIR}/expected/expected_output.json")
ACTUAL_JSON=$(jq -S '.' "${TEST_DIR}/actual_output.json")

if [ "$EXPECTED_JSON" != "$ACTUAL_JSON" ]; then
    echo "❌ FAILED: JSON output does not match expected format"
    echo "Expected: ${TEST_DIR}/expected_output.json"
    echo "Actual: ${TEST_DIR}/actual_output.json"
    diff -u <(echo "$EXPECTED_JSON") <(echo "$ACTUAL_JSON") || true
    exit 1
fi

if echo "$OUTPUT" | jq -e '.. | objects | has("size_estimate")' >/dev/null 2>&1; then
    echo "❌ FAILED: model --json should omit size_estimate unless explicitly enabled"
    exit 1
fi

# Test 2b: Size estimates are JSON-only and opt-in
echo "Running: reqvire model --with-size-estimates without --json" >> "${TEST_DIR}/test_results.log"
set +e
SIZE_ERROR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --with-size-estimates 2>&1)
SIZE_ERROR_EXIT_CODE=$?
set -e

if [ $SIZE_ERROR_EXIT_CODE -eq 0 ]; then
    echo "❌ FAILED: model --with-size-estimates without --json should fail"
    echo "$SIZE_ERROR_OUTPUT"
    exit 1
fi

if ! grep -q -- "--with-size-estimates requires --json" <<< "$SIZE_ERROR_OUTPUT"; then
    echo "❌ FAILED: size-estimate JSON-only diagnostic missing"
    echo "$SIZE_ERROR_OUTPUT"
    exit 1
fi

echo "Running: reqvire model --json --with-size-estimates" >> "${TEST_DIR}/test_results.log"
set +e
SIZE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --json --with-size-estimates 2>&1)
SIZE_EXIT_CODE=$?
set -e

if [ $SIZE_EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --json --with-size-estimates exited with code $SIZE_EXIT_CODE"
    echo "$SIZE_OUTPUT"
    exit 1
fi

echo "$SIZE_OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: size-estimate output is not valid JSON"
    exit 1
fi

if ! echo "$SIZE_OUTPUT" | jq -e '
  [.elements[]? | .. | objects | select(has("identifier") and has("name"))] as $elements
  | ($elements | length) > 0
  and all($elements[];
    (.size_estimate.content_bytes | type == "number")
    and (.size_estimate.rendered_context_bytes | type == "number")
    and (.size_estimate.estimated_tokens | type == "number")
  )
' >/dev/null 2>&1; then
    echo "❌ FAILED: model --json --with-size-estimates should include size estimates on all element payloads"
    echo "$SIZE_OUTPUT"
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
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected_filtered_output.md" "${TEST_DIR}/actual_filtered_output.md"; then
    echo "❌ FAILED: Filtered markdown output does not match expected format"
    echo "Expected: ${TEST_DIR}/expected_filtered_output.md"
    echo "Actual: ${TEST_DIR}/actual_filtered_output.md"
    diff -u "${TEST_SCRIPT_DIR}/expected/expected_filtered_output.md" "${TEST_DIR}/actual_filtered_output.md"
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
EXPECTED_FILTERED_JSON=$(jq -S '.' "${TEST_SCRIPT_DIR}/expected/expected_filtered_output.json")
ACTUAL_FILTERED_JSON=$(jq -S '.' "${TEST_DIR}/actual_filtered_output.json")

if [ "$EXPECTED_FILTERED_JSON" != "$ACTUAL_FILTERED_JSON" ]; then
    echo "❌ FAILED: Filtered JSON output does not match expected format"
    echo "Expected: ${TEST_DIR}/expected_filtered_output.json"
    echo "Actual: ${TEST_DIR}/actual_filtered_output.json"
    diff -u <(echo "$EXPECTED_FILTERED_JSON") <(echo "$ACTUAL_FILTERED_JSON") || true
    exit 1
fi

# Test 5: Reverse Model Output - Compare against expected file
echo "Running: reqvire model --reverse --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --reverse --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --reverse --json exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Reverse output is not valid JSON"
    exit 1
fi

# Save actual output for comparison
echo "$OUTPUT" | jq '.' > "${TEST_DIR}/actual_reverse_output.json"

# Compare JSON outputs using jq (to handle formatting differences)
EXPECTED_REVERSE_JSON=$(jq -S '.' "${TEST_SCRIPT_DIR}/expected/expected_reverse_output.json")
ACTUAL_REVERSE_JSON=$(jq -S '.' "${TEST_DIR}/actual_reverse_output.json")

if [ "$EXPECTED_REVERSE_JSON" != "$ACTUAL_REVERSE_JSON" ]; then
    echo "❌ FAILED: Reverse JSON output does not match expected format"
    echo "Expected: ${TEST_SCRIPT_DIR}/expected/expected_reverse_output.json"
    echo "Actual: ${TEST_DIR}/actual_reverse_output.json"
    diff -u <(echo "$EXPECTED_REVERSE_JSON") <(echo "$ACTUAL_REVERSE_JSON") || true
    exit 1
fi

# Test 5b: Reverse Model Markdown Output - Compare against expected file
echo "Running: reqvire model --reverse (markdown)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --reverse 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --reverse exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Save actual output for comparison
echo "$OUTPUT" > "${TEST_DIR}/actual_reverse_output.md"

# Compare markdown outputs
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected_reverse_output.md" "${TEST_DIR}/actual_reverse_output.md"; then
    echo "❌ FAILED: Reverse markdown output does not match expected format"
    exit 1
fi

# Test 6: Filter Type Output
echo "Running: reqvire model --filter-type=test-verification --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --filter-type=test-verification --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --filter-type exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Filter-type output is not valid JSON"
    exit 1
fi

# Verify type_filter is set
TYPE_FILTER=$(echo "$OUTPUT" | jq -r '.metadata.type_filter[0]')
if [ "$TYPE_FILTER" != "test-verification" ]; then
    echo "❌ FAILED: Expected type_filter to contain 'test-verification', got '$TYPE_FILTER'"
    exit 1
fi

# Verify all top-level elements are test-verification type
WRONG_TYPE=$(echo "$OUTPUT" | jq '[.elements[].element_type] | map(select(. != "test-verification")) | length')
if [ "$WRONG_TYPE" -ne 0 ]; then
    echo "❌ FAILED: Filter-type should only return test-verification elements at top level"
    exit 1
fi

# Test 7: Reverse + Filter Type (traces-like output) - Compare against expected file
echo "Running: reqvire model --reverse --filter-type=test-verification --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --reverse --filter-type=test-verification --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --reverse --filter-type exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Reverse+filter-type output is not valid JSON"
    exit 1
fi

# Save actual output for comparison
echo "$OUTPUT" | jq '.' > "${TEST_DIR}/actual_reverse_filter_output.json"

# Compare JSON outputs using jq (to handle formatting differences)
EXPECTED_REVERSE_FILTER_JSON=$(jq -S '.' "${TEST_SCRIPT_DIR}/expected/expected_reverse_filter_output.json")
ACTUAL_REVERSE_FILTER_JSON=$(jq -S '.' "${TEST_DIR}/actual_reverse_filter_output.json")

if [ "$EXPECTED_REVERSE_FILTER_JSON" != "$ACTUAL_REVERSE_FILTER_JSON" ]; then
    echo "❌ FAILED: Reverse+filter-type JSON output does not match expected format"
    echo "Expected: ${TEST_SCRIPT_DIR}/expected/expected_reverse_filter_output.json"
    echo "Actual: ${TEST_DIR}/actual_reverse_filter_output.json"
    diff -u <(echo "$EXPECTED_REVERSE_FILTER_JSON") <(echo "$ACTUAL_REVERSE_FILTER_JSON") || true
    exit 1
fi

# Test 7b: Reverse + Filter Type Markdown Output - Compare against expected file
echo "Running: reqvire model --reverse --filter-type=test-verification (markdown)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model --reverse --filter-type=test-verification 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --reverse --filter-type (markdown) exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Save actual output for comparison
echo "$OUTPUT" > "${TEST_DIR}/actual_reverse_filter_output.md"

# Compare markdown outputs
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected_reverse_filter_output.md" "${TEST_DIR}/actual_reverse_filter_output.md"; then
    echo "❌ FAILED: Reverse+filter-type markdown output does not match expected format"
    exit 1
fi

exit 0
