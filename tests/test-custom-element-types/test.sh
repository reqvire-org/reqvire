#!/usr/bin/env bash
set -euo pipefail

# Test: Custom Element Type Tracking in Model Summary
# --------------------------------------
# Satisfies: TBD - specifications/Verifications/<file>.md#<verification-element>
#
# Acceptance Criteria:
# - Custom element types (non-standard types) are tracked and counted in summary report
# - Text output displays custom types as "Custom (type-name): count"
# - JSON output includes "custom_element_types" object with correct counts
# - Multiple custom types are sorted alphabetically in text output
# - Standard types (requirement, user-requirement, verification) are NOT counted as custom
# - Different custom types are tracked separately
# - When no custom types exist, the custom types section is not displayed in text output
#
# Test Criteria:
# - Commands exit with success (0) return code
# - Text summary displays custom types in correct format
# - JSON summary includes custom_element_types with correct counts
# - Custom types are alphabetically sorted in text output
# - Standard types are excluded from custom type counting

# Create log file
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test 1: JSON Output - Verify custom_element_types structure and counts
# Expected custom types: actor: 1, constraint: 1, moe: 2, use-case: 3
echo "Test 1: Verifying JSON output with custom element types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" summary --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_JSON" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: summary --json command exited with code $EXIT_CODE"
    echo "$OUTPUT_JSON"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT_JSON" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "FAILED: Output is not valid JSON"
    exit 1
fi

# Check that custom_element_types field exists
if ! echo "$OUTPUT_JSON" | jq -e '.global_counters.custom_element_types' >/dev/null 2>&1; then
    echo "FAILED: JSON missing 'custom_element_types' field in global_counters"
    exit 1
fi

# Verify actor count
ACTOR_COUNT=$(echo "$OUTPUT_JSON" | jq -r '.global_counters.custom_element_types.actor // 0')
if [ "$ACTOR_COUNT" -ne 1 ]; then
    echo "FAILED: Expected actor count: 1, got: $ACTOR_COUNT"
    exit 1
fi

# Verify constraint count
CONSTRAINT_COUNT=$(echo "$OUTPUT_JSON" | jq -r '.global_counters.custom_element_types.constraint // 0')
if [ "$CONSTRAINT_COUNT" -ne 1 ]; then
    echo "FAILED: Expected constraint count: 1, got: $CONSTRAINT_COUNT"
    exit 1
fi

# Verify moe count
MOE_COUNT=$(echo "$OUTPUT_JSON" | jq -r '.global_counters.custom_element_types.moe // 0')
if [ "$MOE_COUNT" -ne 2 ]; then
    echo "FAILED: Expected moe count: 2, got: $MOE_COUNT"
    exit 1
fi

# Verify use-case count
USE_CASE_COUNT=$(echo "$OUTPUT_JSON" | jq -r '.global_counters.custom_element_types["use-case"] // 0')
if [ "$USE_CASE_COUNT" -ne 3 ]; then
    echo "FAILED: Expected use-case count: 3, got: $USE_CASE_COUNT"
    exit 1
fi

# Verify total number of custom types tracked (should be 4)
CUSTOM_TYPES_COUNT=$(echo "$OUTPUT_JSON" | jq '.global_counters.custom_element_types | length')
if [ "$CUSTOM_TYPES_COUNT" -ne 4 ]; then
    echo "FAILED: Expected 4 different custom types, got: $CUSTOM_TYPES_COUNT"
    exit 1
fi

# Verify standard types are not in custom_element_types
if echo "$OUTPUT_JSON" | jq -e '.global_counters.custom_element_types.requirement' >/dev/null 2>&1; then
    echo "FAILED: Standard type 'requirement' should not be in custom_element_types"
    exit 1
fi

if echo "$OUTPUT_JSON" | jq -e '.global_counters.custom_element_types["user-requirement"]' >/dev/null 2>&1; then
    echo "FAILED: Standard type 'user-requirement' should not be in custom_element_types"
    exit 1
fi

if echo "$OUTPUT_JSON" | jq -e '.global_counters.custom_element_types.verification' >/dev/null 2>&1; then
    echo "FAILED: Standard type 'verification' should not be in custom_element_types"
    exit 1
fi

# Test 2: Text Output - Verify custom types display format and alphabetical sorting
echo "Test 2: Verifying text output with custom element types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_TEXT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_TEXT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: summary command exited with code $EXIT_CODE"
    echo "$OUTPUT_TEXT"
    exit 1
fi

# Check that custom types are displayed with correct format
if ! echo "$OUTPUT_TEXT" | grep -q "^Custom (actor): 1$"; then
    echo "FAILED: Text output missing or incorrect format for 'Custom (actor): 1'"
    echo "Output:"
    echo "$OUTPUT_TEXT"
    exit 1
fi

if ! echo "$OUTPUT_TEXT" | grep -q "^Custom (constraint): 1$"; then
    echo "FAILED: Text output missing or incorrect format for 'Custom (constraint): 1'"
    exit 1
fi

if ! echo "$OUTPUT_TEXT" | grep -q "^Custom (moe): 2$"; then
    echo "FAILED: Text output missing or incorrect format for 'Custom (moe): 2'"
    exit 1
fi

if ! echo "$OUTPUT_TEXT" | grep -q "^Custom (use-case): 3$"; then
    echo "FAILED: Text output missing or incorrect format for 'Custom (use-case): 3'"
    exit 1
fi

# Test 3: Alphabetical Sorting in Text Output
echo "Test 3: Verifying alphabetical sorting of custom types" >> "${TEST_DIR}/test_results.log"

# Extract custom types section and verify ordering
CUSTOM_SECTION=$(echo "$OUTPUT_TEXT" | grep "^Custom (" || true)
if [ -z "$CUSTOM_SECTION" ]; then
    echo "FAILED: No custom types found in text output"
    exit 1
fi

# Check that actor comes before constraint, constraint before moe, moe before use-case
ACTOR_LINE=$(echo "$CUSTOM_SECTION" | grep -n "actor" | cut -d: -f1)
CONSTRAINT_LINE=$(echo "$CUSTOM_SECTION" | grep -n "constraint" | cut -d: -f1)
MOE_LINE=$(echo "$CUSTOM_SECTION" | grep -n "moe" | cut -d: -f1)
USE_CASE_LINE=$(echo "$CUSTOM_SECTION" | grep -n "use-case" | cut -d: -f1)

if [ "$ACTOR_LINE" -gt "$CONSTRAINT_LINE" ] || \
   [ "$CONSTRAINT_LINE" -gt "$MOE_LINE" ] || \
   [ "$MOE_LINE" -gt "$USE_CASE_LINE" ]; then
    echo "FAILED: Custom types are not alphabetically sorted"
    echo "Order found:"
    echo "$CUSTOM_SECTION"
    exit 1
fi

# Test 4: Standard Type Counts - Verify they exist and are correct
echo "Test 4: Verifying standard type counts" >> "${TEST_DIR}/test_results.log"

# Check that standard types are counted correctly in separate fields
TOTAL_REQUIREMENTS=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_requirements_system')
if [ "$TOTAL_REQUIREMENTS" -ne 2 ]; then
    echo "FAILED: Expected 2 system requirements, got: $TOTAL_REQUIREMENTS"
    exit 1
fi

TOTAL_USER_REQS=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_requirements_user')
if [ "$TOTAL_USER_REQS" -ne 2 ]; then
    echo "FAILED: Expected 2 user requirements, got: $TOTAL_USER_REQS"
    exit 1
fi

TOTAL_VERIFICATIONS=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_verifications_test')
if [ "$TOTAL_VERIFICATIONS" -ne 1 ]; then
    echo "FAILED: Expected 1 verification, got: $TOTAL_VERIFICATIONS"
    exit 1
fi

# Test 5: Total Elements Count - Should include both standard and custom types
echo "Test 5: Verifying total elements count" >> "${TEST_DIR}/test_results.log"

TOTAL_ELEMENTS=$(echo "$OUTPUT_JSON" | jq '.global_counters.total_elements')
# 2 requirements + 2 user-requirements + 1 verification + 7 custom types = 12
if [ "$TOTAL_ELEMENTS" -ne 12 ]; then
    echo "FAILED: Expected 12 total elements (2+2+1+7), got: $TOTAL_ELEMENTS"
    exit 1
fi

# Test 6: Empty Custom Types Case - Create a test with no custom types
echo "Test 6: Verifying behavior with no custom types" >> "${TEST_DIR}/test_results.log"

# Create a temporary directory with only standard types
TEMP_NO_CUSTOM="${TEST_DIR}/test_no_custom"
mkdir -p "${TEMP_NO_CUSTOM}/specifications"

cat > "${TEMP_NO_CUSTOM}/specifications/StandardOnly.md" << 'EOF'
# Standard Types Only

## Requirements

### User Requirement

This is a user requirement.

#### Metadata
* type: user-requirement

### Standard Requirement

This is a standard requirement derived from User Requirement.

#### Metadata
* type: requirement

#### Relations
* derivedFrom: #user-requirement
EOF

cat > "${TEMP_NO_CUSTOM}/reqvire.yaml" << 'EOF'
specifications_path: specifications/
output_path: output/
EOF

# Initialize git repo for the test
cd "${TEMP_NO_CUSTOM}"
git init > /dev/null 2>&1
git add . > /dev/null 2>&1
git commit -m "Initial commit" > /dev/null 2>&1
cd - > /dev/null 2>&1

# Run summary on directory with no custom types
set +e
OUTPUT_NO_CUSTOM_JSON=$(cd "${TEMP_NO_CUSTOM}" && "$REQVIRE_BIN" summary --json 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: summary --json (no custom types) exited with code $EXIT_CODE"
    exit 1
fi

# Check that custom_element_types is empty or absent in JSON
CUSTOM_TYPES_EMPTY=$(echo "$OUTPUT_NO_CUSTOM_JSON" | jq '.global_counters.custom_element_types // {} | length')
if [ "$CUSTOM_TYPES_EMPTY" -ne 0 ]; then
    echo "FAILED: Expected no custom types, but found: $CUSTOM_TYPES_EMPTY"
    echo "$OUTPUT_NO_CUSTOM_JSON" | jq '.global_counters.custom_element_types'
    exit 1
fi

# Run summary in text format and verify no "Custom (" lines appear
set +e
OUTPUT_NO_CUSTOM_TEXT=$(cd "${TEMP_NO_CUSTOM}" && "$REQVIRE_BIN" summary 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: summary (no custom types) exited with code $EXIT_CODE"
    exit 1
fi

# Verify no "Custom (" lines in output
if echo "$OUTPUT_NO_CUSTOM_TEXT" | grep -q "^Custom ("; then
    echo "FAILED: Text output should not show custom types when none exist"
    echo "Found:"
    echo "$OUTPUT_NO_CUSTOM_TEXT" | grep "^Custom ("
    exit 1
fi

# Clean up temporary test directory
rm -rf "${TEMP_NO_CUSTOM}"

# Test 7: Filter Interaction - Verify custom type counts with filters
echo "Test 7: Verifying custom type counts with filter-type" >> "${TEST_DIR}/test_results.log"

# Filter to show only use-case custom type elements
set +e
OUTPUT_FILTERED_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" summary --json --filter-type="use-case" 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: summary --json --filter-type=use-case exited with code $EXIT_CODE"
    exit 1
fi

# Total elements should be 3 (only use-case elements)
FILTERED_TOTAL=$(echo "$OUTPUT_FILTERED_JSON" | jq '.global_counters.total_elements')
if [ "$FILTERED_TOTAL" -ne 3 ]; then
    echo "FAILED: Expected 3 elements with filter-type=use-case, got: $FILTERED_TOTAL"
    exit 1
fi

# Custom types should only show use-case
FILTERED_CUSTOM_COUNT=$(echo "$OUTPUT_FILTERED_JSON" | jq '.global_counters.custom_element_types | length')
if [ "$FILTERED_CUSTOM_COUNT" -ne 1 ]; then
    echo "FAILED: Expected only 1 custom type with filter, got: $FILTERED_CUSTOM_COUNT"
    exit 1
fi

FILTERED_USE_CASE=$(echo "$OUTPUT_FILTERED_JSON" | jq '.global_counters.custom_element_types["use-case"]')
if [ "$FILTERED_USE_CASE" -ne 3 ]; then
    echo "FAILED: Expected 3 use-case elements with filter, got: $FILTERED_USE_CASE"
    exit 1
fi

echo "PASSED: Custom element type tracking tests"
exit 0
