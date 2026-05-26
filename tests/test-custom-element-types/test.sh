#!/usr/bin/env bash
set -euo pipefail

# Test: Custom Element Type Tracking in Model Summary
# --------------------------------------
# Satisfies: TBD - specifications/Verifications/<file>.md#<verification-element>
#
# Acceptance Criteria:
# - Custom element types (non-standard types) are tracked and counted in search report
# - Text output displays custom types under "📋 Other Types:" section
# - JSON output includes "total_other_types" object with correct counts
# - Multiple custom types are sorted alphabetically in output
# - Standard types (capability, requirement, verification) are NOT counted as custom
# - Different custom types are tracked separately
# - When no custom types exist, the other types section is not displayed
#
# Test Criteria:
# - Commands exit with success (0) return code
# - Text search displays custom types in correct format
# - JSON search includes total_other_types with correct counts
# - Standard types are excluded from custom type counting

# Create log file
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test 1: JSON Output - Verify total_other_types structure and counts
# Expected custom types: actor: 1, rule: 1, moe: 2, use-case: 3
echo "Test 1: Verifying JSON output with custom element types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_JSON" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: search --json command exited with code $EXIT_CODE"
    echo "$OUTPUT_JSON"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT_JSON" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "FAILED: Output is not valid JSON"
    exit 1
fi

# Check that total_other_types field exists
if ! echo "$OUTPUT_JSON" | jq -e '.global_counters.total_other_types' >/dev/null 2>&1; then
    echo "FAILED: JSON missing 'total_other_types' field in global_counters"
    exit 1
fi

# Verify actor count
ACTOR_COUNT=$(echo "$OUTPUT_JSON" | jq -r '.global_counters.total_other_types.actor // 0')
if [ "$ACTOR_COUNT" -ne 1 ]; then
    echo "FAILED: Expected actor count: 1, got: $ACTOR_COUNT"
    exit 1
fi

# Verify rule count
RULE_COUNT=$(echo "$OUTPUT_JSON" | jq -r '.global_counters.total_other_types.rule // 0')
if [ "$RULE_COUNT" -ne 1 ]; then
    echo "FAILED: Expected rule count: 1, got: $RULE_COUNT"
    exit 1
fi

# Verify moe count
MOE_COUNT=$(echo "$OUTPUT_JSON" | jq -r '.global_counters.total_other_types.moe // 0')
if [ "$MOE_COUNT" -ne 2 ]; then
    echo "FAILED: Expected moe count: 2, got: $MOE_COUNT"
    exit 1
fi

# Verify use-case count
USE_CASE_COUNT=$(echo "$OUTPUT_JSON" | jq -r '.global_counters.total_other_types["use-case"] // 0')
if [ "$USE_CASE_COUNT" -ne 3 ]; then
    echo "FAILED: Expected use-case count: 3, got: $USE_CASE_COUNT"
    exit 1
fi

# Verify total number of custom types tracked (should be 4)
CUSTOM_TYPES_COUNT=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_other_types | length')
if [ "$CUSTOM_TYPES_COUNT" -ne 4 ]; then
    echo "FAILED: Expected 4 different custom types, got: $CUSTOM_TYPES_COUNT"
    exit 1
fi

# Verify standard types are not in total_other_types
if echo "$OUTPUT_JSON" | jq -e '.global_counters.total_other_types.requirement' >/dev/null 2>&1; then
    echo "FAILED: Standard type 'requirement' should not be in total_other_types"
    exit 1
fi

if echo "$OUTPUT_JSON" | jq -e '.global_counters.total_other_types.capability' >/dev/null 2>&1; then
    echo "FAILED: Standard type 'capability' should not be in total_other_types"
    exit 1
fi

# Test 2: Text Output - Verify custom types display format
echo "Test 2: Verifying text output with custom element types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_TEXT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_TEXT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: search command exited with code $EXIT_CODE"
    echo "$OUTPUT_TEXT"
    exit 1
fi

# Check that custom types section exists
if ! echo "$OUTPUT_TEXT" | grep -q "📋 Other Types:"; then
    echo "FAILED: Text output missing '📋 Other Types:' section"
    echo "Output:"
    echo "$OUTPUT_TEXT"
    exit 1
fi

# Check individual custom type counts
if ! echo "$OUTPUT_TEXT" | grep -q "actor: 1"; then
    echo "FAILED: Text output missing 'actor: 1'"
    exit 1
fi

if ! echo "$OUTPUT_TEXT" | grep -q "rule: 1"; then
    echo "FAILED: Text output missing 'rule: 1'"
    exit 1
fi

if ! echo "$OUTPUT_TEXT" | grep -q "moe: 2"; then
    echo "FAILED: Text output missing 'moe: 2'"
    exit 1
fi

if ! echo "$OUTPUT_TEXT" | grep -q "use-case: 3"; then
    echo "FAILED: Text output missing 'use-case: 3'"
    exit 1
fi

# Test 3: Standard Type Counts - Verify they exist and are correct
echo "Test 3: Verifying standard type counts in new format" >> "${TEST_DIR}/test_results.log"

# Check that standard types are counted correctly in nested maps
TOTAL_REQUIREMENTS=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_requirements_types["system-requirement"]')
if [ "$TOTAL_REQUIREMENTS" -ne 2 ]; then
    echo "FAILED: Expected 2 system requirements, got: $TOTAL_REQUIREMENTS"
    exit 1
fi

TOTAL_CAPABILITYS=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_requirements_types.capability')
if [ "$TOTAL_CAPABILITYS" -ne 3 ]; then
    echo "FAILED: Expected 3 capabilities, got: $TOTAL_CAPABILITYS"
    exit 1
fi

TOTAL_VERIFICATIONS=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_verifications_types["test-verification"]')
if [ "$TOTAL_VERIFICATIONS" -ne 1 ]; then
    echo "FAILED: Expected 1 verification, got: $TOTAL_VERIFICATIONS"
    exit 1
fi

# Test 4: Total Elements Count - Should include both standard and custom types
echo "Test 4: Verifying total elements count" >> "${TEST_DIR}/test_results.log"

TOTAL_ELEMENTS=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_elements')
# 3 capabilities + 2 requirements + 1 verification + 7 custom types = 13
if [ "$TOTAL_ELEMENTS" -ne 13 ]; then
    echo "FAILED: Expected 13 total elements (1+4+1+7), got: $TOTAL_ELEMENTS"
    exit 1
fi

# Test 5: Empty Custom Types Case - Create a test with no custom types
echo "Test 5: Verifying behavior with no custom types" >> "${TEST_DIR}/test_results.log"

# Create a temporary directory with only standard types
TEMP_NO_CUSTOM="${TEST_DIR}/test_no_custom"
mkdir -p "${TEMP_NO_CUSTOM}/specifications"

cat > "${TEMP_NO_CUSTOM}/specifications/StandardOnly.md" << 'EOF'
# Elements


### Capability

This is a capability.

#### Metadata
* type: capability

### Standard Requirement

This is a standard requirement derived from Capability.

#### Metadata
* type: requirement

#### Relations
* specify: #capability
EOF

cat > "${TEMP_NO_CUSTOM}/reqvire.yaml" << 'EOF'
specifications_path: specifications/
output_path: output/
EOF

# Initialize git repo for the test
cd "${TEMP_NO_CUSTOM}"
git init > /dev/null 2>&1
git config user.email "test@example.com" > /dev/null 2>&1
git config user.name "Test User" > /dev/null 2>&1
git add . > /dev/null 2>&1
git commit -m "Initial commit" > /dev/null 2>&1
cd - > /dev/null 2>&1

# Run search on directory with no custom types
set +e
OUTPUT_NO_CUSTOM_JSON=$(cd "${TEMP_NO_CUSTOM}" && "$REQVIRE_BIN" search --json 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: search --json (no custom types) exited with code $EXIT_CODE"
    exit 1
fi

# Check that total_other_types is empty or absent in JSON
CUSTOM_TYPES_EMPTY=$(echo "$OUTPUT_NO_CUSTOM_JSON" | jq '.global_counters.total_other_types // {} | length')
if [ "$CUSTOM_TYPES_EMPTY" -ne 0 ]; then
    echo "FAILED: Expected no custom types, but found: $CUSTOM_TYPES_EMPTY"
    echo "$OUTPUT_NO_CUSTOM_JSON" | jq '.global_counters.total_other_types'
    exit 1
fi

# Run search in text format and verify no "Other Types:" section appears
set +e
OUTPUT_NO_CUSTOM_TEXT=$(cd "${TEMP_NO_CUSTOM}" && "$REQVIRE_BIN" search 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: search (no custom types) exited with code $EXIT_CODE"
    exit 1
fi

# Verify no "Other Types:" section in output
if echo "$OUTPUT_NO_CUSTOM_TEXT" | grep -q "📋 Other Types:"; then
    echo "FAILED: Text output should not show Other Types section when none exist"
    echo "Found:"
    echo "$OUTPUT_NO_CUSTOM_TEXT" | grep "Other Types" -A5
    exit 1
fi

# Clean up temporary test directory
rm -rf "${TEMP_NO_CUSTOM}"

# Test 6: Filter Interaction - Verify custom type counts with filters
# Custom types must be filtered using the pattern "other-TYPENAME"
echo "Test 6: Verifying custom type counts with filter-type" >> "${TEST_DIR}/test_results.log"

# Filter to show only use-case custom type elements using "other-use-case" pattern
set +e
OUTPUT_FILTERED_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json --filter-type="other-use-case" 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: search --json --filter-type=other-use-case exited with code $EXIT_CODE"
    exit 1
fi

# Total elements should be 3 (only use-case elements)
FILTERED_TOTAL=$(echo "$OUTPUT_FILTERED_JSON" | jq '.global_counters.total_elements')
if [ "$FILTERED_TOTAL" -ne 3 ]; then
    echo "FAILED: Expected 3 elements with filter-type=other-use-case, got: $FILTERED_TOTAL"
    exit 1
fi

# Custom types should only show use-case
FILTERED_CUSTOM_COUNT=$(echo "$OUTPUT_FILTERED_JSON" | jq '.global_counters.total_other_types | length')
if [ "$FILTERED_CUSTOM_COUNT" -ne 1 ]; then
    echo "FAILED: Expected only 1 custom type with filter, got: $FILTERED_CUSTOM_COUNT"
    exit 1
fi

FILTERED_USE_CASE=$(echo "$OUTPUT_FILTERED_JSON" | jq '.global_counters.total_other_types["use-case"]')
if [ "$FILTERED_USE_CASE" -ne 3 ]; then
    echo "FAILED: Expected 3 use-case elements with filter, got: $FILTERED_USE_CASE"
    exit 1
fi

echo "PASSED: Custom element type tracking tests"
exit 0
