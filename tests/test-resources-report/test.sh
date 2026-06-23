#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test: Resources Report
# --------------------------------------
# Satisfies: specifications/System/Output/Verifications/ReportingVerifications.md#resources-report-verification
#
# Acceptance Criteria:
# - System shall provide a CLI command `resources` that lists files referenced by the model
# - Command shall support `--json` flag for JSON output format
# - Resources report shall have two sections: Relations and Contract Bindings
# - Relations section lists files from InternalPath relations such as satisfiedBy
# - Contract Bindings section reports file targets from contract_bindings (empty when only identifier contract_bindings are used)
# - Files shall be sorted alphabetically by path
# - References within each file shall be sorted by relation type, then by element identifier
# - Text output shall include markdown links to referencing elements
# - JSON output shall include file_path, references array with element details
#
# Test Criteria:
# - Command exits with success (0) return code
# - Text output matches expected format with both Relations and Contract Bindings sections
# - JSON output is valid and contains required fields

# Test 1: Basic Resources Report (Text Output)
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
    exit 1
fi

# Compare output with expected output
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected_output.md" <(echo "$OUTPUT"); then
    echo "FAILED: Resources report output does not match expected output"
    echo ""
    echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected_output.md"
    exit 1
fi

# Test 2: JSON Resources Report
echo "Running: reqvire resources --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" resources --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: resources --json command exited with code $EXIT_CODE"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "FAILED: Output is not valid JSON"
    exit 1
fi

# Check for required top-level fields
if ! echo "$OUTPUT" | jq 'has("relations")' | grep -q true; then
    echo "FAILED: JSON missing 'relations' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq 'has("contract_bindings")' | grep -q true; then
    echo "FAILED: JSON missing 'contract_bindings' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq 'has("summary")' | grep -q true; then
    echo "FAILED: JSON missing 'summary' field"
    exit 1
fi

# Check summary fields
if ! echo "$OUTPUT" | jq '.summary | has("total_relation_files")' | grep -q true; then
    echo "FAILED: JSON missing 'summary.total_relation_files' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("total_contract_bindings_files")' | grep -q true; then
    echo "FAILED: JSON missing 'summary.total_contract_bindings_files' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("total_relation_references")' | grep -q true; then
    echo "FAILED: JSON missing 'summary.total_relation_references' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.summary | has("total_contract_bindings_references")' | grep -q true; then
    echo "FAILED: JSON missing 'summary.total_contract_bindings_references' field"
    exit 1
fi

# Test 3: Verify relations count
RELATION_FILES=$(echo "$OUTPUT" | jq '.summary.total_relation_files')
if [ "$RELATION_FILES" -ne 4 ]; then
    echo "FAILED: Expected 4 relation files, got $RELATION_FILES"
    exit 1
fi

# Test 4: Verify contract_bindings count
ATTACHMENT_FILES=$(echo "$OUTPUT" | jq '.summary.total_contract_bindings_files')
if [ "$ATTACHMENT_FILES" -ne 0 ]; then
    echo "FAILED: Expected 0 contract_bindings files, got $ATTACHMENT_FILES"
    exit 1
fi

# Test 5: Verify first relation has required fields
if ! echo "$OUTPUT" | jq '.relations[0] | has("file_path")' | grep -q true; then
    echo "FAILED: Relations entry missing 'file_path' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.relations[0] | has("references")' | grep -q true; then
    echo "FAILED: Relations entry missing 'references' field"
    exit 1
fi

# Test 6: Verify reference has required fields
if ! echo "$OUTPUT" | jq '.relations[0].references[0] | has("relation_type")' | grep -q true; then
    echo "FAILED: Reference missing 'relation_type' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.relations[0].references[0] | has("element_id")' | grep -q true; then
    echo "FAILED: Reference missing 'element_id' field"
    exit 1
fi

if ! echo "$OUTPUT" | jq '.relations[0].references[0] | has("element_name")' | grep -q true; then
    echo "FAILED: Reference missing 'element_name' field"
    exit 1
fi

# Test 7: Verify contract_bindings list is empty for identifier-only contract_bindings fixtures
ATTACHMENT_LIST_SIZE=$(echo "$OUTPUT" | jq '.contract_bindings | length')
if [ "$ATTACHMENT_LIST_SIZE" -ne 0 ]; then
    echo "FAILED: Expected empty contract_bindings list, got size: $ATTACHMENT_LIST_SIZE"
    exit 1
fi

exit 0
