#!/usr/bin/env bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Model Command Verification
# --------------------------------------
# Satisfies: specifications/Verifications/ReportsTests.md#model-command-verification
#
# Acceptance Criteria:
# - `reqvire model` generates markdown with complete model diagram showing all elements
# - `reqvire model --root-id=<id>` generates diagram with only forward-related elements from root
# - `reqvire model --json` generates valid JSON structure with all model data
# - `reqvire model --root-id=<id> --json` generates filtered JSON with forward-related elements
# - Filtered diagrams include only elements reachable via forward relations
# - JSON output can be parsed and contains expected fields (folders, relations)
#
# Test Criteria:
# - Commands exit with success (0) return code
# - Markdown output matches expected structure with model diagram
# - JSON output matches expected structure with folders, files, sections, elements, relations
# - Filters correctly restrict output to forward-reachable elements only

# Test 1: Full Model Markdown Output - Compare against expected file
echo "Running: reqvire model" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model 2>&1)
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
if ! grep -q "# Model Diagram Report" <<< "$OUTPUT"; then
    echo "❌ FAILED: Output missing '# Model Diagram Report' header"
    exit 1
fi

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
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model --json 2>&1)
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
if ! echo "$OUTPUT" | jq -e 'has("folders")' >/dev/null 2>&1; then
    echo "❌ FAILED: JSON missing 'folders' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq -e 'has("relations")' >/dev/null 2>&1; then
    echo "❌ FAILED: JSON missing 'relations' field"
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
echo "Running: reqvire model --root-id (filtered)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model \
    --root-id="specifications/SystemRequirements.md#model-diagram-generation" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: model --root-id exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Should contain the root element
if ! grep -q "Model Diagram Generation" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing root element 'Model Diagram Generation'"
    exit 1
fi

# Should contain forward-related elements (derive children)
if ! grep -q "Model Filtering Capability" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing forward-related element 'Model Filtering Capability'"
    exit 1
fi

if ! grep -q "Forward Relation Traversal" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing forward-related element 'Forward Relation Traversal'"
    exit 1
fi

if ! grep -q "JSON Output Format" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing forward-related element 'JSON Output Format'"
    exit 1
fi

# Should contain verification (verifiedBy forward relation)
if ! grep -q "Model Generation Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing verification 'Model Generation Test' (verifiedBy relation)"
    exit 1
fi

# Should NOT contain parent (backward relation)
if grep -q "Model Structure Exploration" <<< "$OUTPUT"; then
    echo "❌ FAILED: Should not include parent 'Model Structure Exploration' (backward derivedFrom)"
    exit 1
fi

# Should NOT contain sibling with no relation
if grep -q "Markdown Output Format" <<< "$OUTPUT"; then
    echo "❌ FAILED: Should not include unrelated sibling 'Markdown Output Format'"
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
echo "Running: reqvire model --root-id --json (filtered)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model \
    --root-id="specifications/SystemRequirements.md#model-diagram-generation" --json 2>&1)
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

# Test 5: JSON Structure Validation
echo "Validating JSON structure fields" >> "${TEST_DIR}/test_results.log"

# Load the full model JSON
FULL_JSON=$(cat "${TEST_DIR}/actual_output.json")

# Validate folder structure
if ! echo "$FULL_JSON" | jq -e '.folders[0] | has("name")' >/dev/null 2>&1; then
    echo "❌ FAILED: Folder missing 'name' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.folders[0] | has("path")' >/dev/null 2>&1; then
    echo "❌ FAILED: Folder missing 'path' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.folders[0] | has("files")' >/dev/null 2>&1; then
    echo "❌ FAILED: Folder missing 'files' field"
    exit 1
fi

# Validate file structure
if ! echo "$FULL_JSON" | jq -e '.folders[0].files[0] | has("name")' >/dev/null 2>&1; then
    echo "❌ FAILED: File missing 'name' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.folders[0].files[0] | has("path")' >/dev/null 2>&1; then
    echo "❌ FAILED: File missing 'path' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.folders[0].files[0] | has("sections")' >/dev/null 2>&1; then
    echo "❌ FAILED: File missing 'sections' field"
    exit 1
fi

# Validate section structure
if ! echo "$FULL_JSON" | jq -e '.folders[0].files[0].sections[0] | has("name")' >/dev/null 2>&1; then
    echo "❌ FAILED: Section missing 'name' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.folders[0].files[0].sections[0] | has("elements")' >/dev/null 2>&1; then
    echo "❌ FAILED: Section missing 'elements' field"
    exit 1
fi

# Validate element structure
if ! echo "$FULL_JSON" | jq -e '.folders[0].files[0].sections[0].elements[0] | has("identifier")' >/dev/null 2>&1; then
    echo "❌ FAILED: Element missing 'identifier' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.folders[0].files[0].sections[0].elements[0] | has("name")' >/dev/null 2>&1; then
    echo "❌ FAILED: Element missing 'name' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.folders[0].files[0].sections[0].elements[0] | has("element_type")' >/dev/null 2>&1; then
    echo "❌ FAILED: Element missing 'element_type' field"
    exit 1
fi

# Validate relation structure
if ! echo "$FULL_JSON" | jq -e '.relations[0] | has("source_id")' >/dev/null 2>&1; then
    echo "❌ FAILED: Relation missing 'source_id' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.relations[0] | has("target_id")' >/dev/null 2>&1; then
    echo "❌ FAILED: Relation missing 'target_id' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.relations[0] | has("relation_type")' >/dev/null 2>&1; then
    echo "❌ FAILED: Relation missing 'relation_type' field"
    exit 1
fi

if ! echo "$FULL_JSON" | jq -e '.relations[0] | has("is_external")' >/dev/null 2>&1; then
    echo "❌ FAILED: Relation missing 'is_external' field"
    exit 1
fi

exit 0
