#!/usr/bin/env bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test: Multi-Type Search Filter
# --------------------------------------
# Satisfies: requirements/System/Output/Verifications/ReportingVerifications.md#multi-type-search-filter-test
#
# Acceptance Criteria:
# - System shall support comma-separated element types in --filter-type flag
# - Single type queries work as before (backward compatibility)
# - Multi-type queries return elements matching ANY specified type (OR logic)
# - Invalid types in list produce clear error messages
# - Comma-separated parsing handles whitespace correctly
# - Combined filters work correctly
# - JSON output is valid
#
# Test Criteria:
# - All tests exit with expected exit codes
# - Output contains expected elements
# - JSON output is valid and structured correctly
# - Error messages are clear for invalid types

echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test 1: Single type (backward compatibility)
echo "Test 1: Single type filter" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type requirement --short 2>&1)
EXIT_CODE=$?
set -e

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Single type search failed with exit code $EXIT_CODE"
    exit 1
fi

# Should find exactly 4 requirements
COUNT=$(echo "$OUTPUT" | grep -c "^\[requirement\]" || true)
if [ "$COUNT" -ne 4 ]; then
    echo "❌ FAILED: Expected 4 requirements, found $COUNT"
    exit 1
fi

# Test 2: Two comma-separated types
echo "Test 2: Two comma-separated types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type requirement,feature --short 2>&1)
EXIT_CODE=$?
set -e

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Two type search failed with exit code $EXIT_CODE"
    exit 1
fi

# Should find 5 elements (4 requirements + 1 feature)
REQ_COUNT=$(echo "$OUTPUT" | grep -c "^\[requirement\]" || true)
FEATURE_COUNT=$(echo "$OUTPUT" | grep -c "^\[feature\]" || true)
TOTAL=$((REQ_COUNT + FEATURE_COUNT))

if [ "$TOTAL" -ne 5 ]; then
    echo "❌ FAILED: Expected 5 elements (4 req + 1 feature), found $TOTAL"
    exit 1
fi

# Test 3: Three types
echo "Test 3: Three comma-separated types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type requirement,test-verification,behavior --short 2>&1)
EXIT_CODE=$?
set -e

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Three type search failed with exit code $EXIT_CODE"
    exit 1
fi

# Should find 6 elements (4 requirements + 1 test-verification + 1 behavior)
REQ_COUNT=$(echo "$OUTPUT" | grep -c "^\[requirement\]" || true)
VER_COUNT=$(echo "$OUTPUT" | grep -c "^\[test-verification\]" || true)
BEH_COUNT=$(echo "$OUTPUT" | grep -c "^\[behavior\]" || true)
TOTAL=$((REQ_COUNT + VER_COUNT + BEH_COUNT))

if [ "$TOTAL" -ne 6 ]; then
    echo "❌ FAILED: Expected 6 elements (4 req + 1 ver + 1 beh), found $TOTAL"
    exit 1
fi

# Test 4: Custom type in list
echo "Test 4: Custom type in list" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type requirement,other-custom-type --short 2>&1)
EXIT_CODE=$?
set -e

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Custom type search failed with exit code $EXIT_CODE"
    exit 1
fi

# Should find 5 elements (4 requirements + 1 custom)
# Custom types are displayed without the "other-" prefix
if ! echo "$OUTPUT" | grep -q "^\[custom-type\]"; then
    echo "❌ FAILED: Custom type not found in results"
    exit 1
fi

# Test 5: Invalid type in list (should error)
echo "Test 5: Invalid type in list" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type requirement,invalid-type --short 2>&1)
EXIT_CODE=$?
set -e

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -eq 0 ]; then
    echo "❌ FAILED: Expected error for invalid type, got success"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "Invalid element type"; then
    echo "❌ FAILED: Expected 'Invalid element type' error message"
    exit 1
fi

# Test 6: Multiple types with other filters
echo "Test 6: Multiple types combined with name filter" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type requirement,behavior --filter-name "One" --short 2>&1)
EXIT_CODE=$?
set -e

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Combined filter search failed with exit code $EXIT_CODE"
    exit 1
fi

# Should find 2 elements (Test Requirement One + Test Behavior One)
COUNT=$(echo "$OUTPUT" | grep -c "One" || true)
if [ "$COUNT" -ne 3 ]; then
    echo "❌ FAILED: Expected 3 elements with 'One' in name, found $COUNT"
    exit 1
fi

# Test 7: JSON output with multiple types
echo "Test 7: JSON output with multiple types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type requirement,feature --json 2>&1)
EXIT_CODE=$?
set -e

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: JSON search failed with exit code $EXIT_CODE"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Output is not valid JSON"
    exit 1
fi

# Check for required structure
if ! echo "$OUTPUT" | jq 'has("files")' | grep -q true; then
    echo "❌ FAILED: JSON missing 'files' field"
    exit 1
fi

# Count elements in JSON - should be 5 total
TOTAL=$(echo "$OUTPUT" | jq '[.files[] | .elements[]] | length')
if [ "$TOTAL" -ne 5 ]; then
    echo "❌ FAILED: Expected 5 elements in JSON, found $TOTAL"
    exit 1
fi

# Test 8: Whitespace handling
echo "Test 8: Whitespace handling in comma-separated types" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type "requirement ,  feature , behavior" --short 2>&1)
EXIT_CODE=$?
set -e

printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: Whitespace handling failed with exit code $EXIT_CODE"
    exit 1
fi

# Should find 6 elements despite extra whitespace
REQ_COUNT=$(echo "$OUTPUT" | grep -c "^\[requirement\]" || true)
FEATURE_COUNT=$(echo "$OUTPUT" | grep -c "^\[feature\]" || true)
BEH_COUNT=$(echo "$OUTPUT" | grep -c "^\[behavior\]" || true)
TOTAL=$((REQ_COUNT + FEATURE_COUNT + BEH_COUNT))

if [ "$TOTAL" -ne 6 ]; then
    echo "❌ FAILED: Expected 6 elements with whitespace handling, found $TOTAL"
    exit 1
fi

exit 0
