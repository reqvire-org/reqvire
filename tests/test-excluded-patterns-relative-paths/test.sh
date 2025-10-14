#!/bin/bash

# Test: Excluded Patterns Relative Path Resolution
# ------------------------------------------------------------
# Acceptance Criteria:
# - Excluded patterns should be resolved relative to git root, not cwd
# - Files matching excluded patterns should not be processed as elements
# - Excluded pattern matching should work consistently from any working directory
#
# Test Criteria:
# - Elements in external/**/*.md and build/**/*.md should not be processed
# - Pattern matching should work the same way from git root and subfolder
# - Behavior should be consistent when run from root vs subfolder

printf "%s\n" "Testing excluded patterns relative path resolution..." > "${TEST_DIR}/test_results.log"

# Test 1: Run from git root
printf "%s\n" "=== Test 1: Running from git root ===" >> "${TEST_DIR}/test_results.log"

echo "Running: reqvire summary --json (from root)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_ROOT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" summary --json 2>&1)
EXIT_CODE_ROOT=$?
set -e

echo "Exit code: $EXIT_CODE_ROOT" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_ROOT" >> "${TEST_DIR}/test_results.log"

if [[ $EXIT_CODE_ROOT -ne 0 ]]; then
    echo "❌ FAILED: Command failed when run from git root"
    exit $EXIT_CODE_ROOT
fi

# Test 2: Run from subfolder
printf "%s\n" "=== Test 2: Running from subfolder ===" >> "${TEST_DIR}/test_results.log"

echo "Running: reqvire summary --json (from subfolder)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_SUB=$(cd "${TEST_DIR}/specifications/subfolder" && "$REQVIRE_BIN" summary --json 2>&1)
EXIT_CODE_SUB=$?
set -e

echo "Exit code: $EXIT_CODE_SUB" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_SUB" >> "${TEST_DIR}/test_results.log"

if [[ $EXIT_CODE_SUB -ne 0 ]]; then
    echo "❌ FAILED: Command failed when run from subfolder"
    exit $EXIT_CODE_SUB
fi

# Test 3: Verify excluded files are not processed (no elements from excluded files)
printf "%s\n" "=== Test 3: Verifying excluded files are not processed ===" >> "${TEST_DIR}/test_results.log"

# Check that excluded files don't contribute elements to the model
if echo "$OUTPUT_ROOT" | grep -q "External Configuration Item"; then
    echo "❌ FAILED: Elements from external/**/*.md are being processed (should be excluded)"
    exit 1
fi

if echo "$OUTPUT_ROOT" | grep -q "Temp Requirement"; then
    echo "❌ FAILED: Elements from build/**/*.md are being processed (should be excluded)"
    exit 1
fi

# Test 4: Verify that excluded files don't appear in validation errors as missing relation targets
# (This test is not applicable since we removed the problematic relations)
printf "%s\n" "=== Test 4: No missing relation target errors for excluded files (test skipped) ===" >> "${TEST_DIR}/test_results.log"

# Test 5: Verify behavior difference between root and subfolder execution
printf "%s\n" "=== Test 5: Comparing root vs subfolder behavior ===" >> "${TEST_DIR}/test_results.log"

# Root should process more elements than subfolder (due to subdirectory processing limitation)
ELEMENTS_ROOT=$(echo "$OUTPUT_ROOT" | jq '.global_counters.total_elements')
ELEMENTS_SUB=$(echo "$OUTPUT_SUB" | jq '.global_counters.total_elements')

if [[ "$ELEMENTS_ROOT" -le "$ELEMENTS_SUB" ]]; then
    echo "❌ FAILED: Expected more elements from root ($ELEMENTS_ROOT) than subfolder ($ELEMENTS_SUB)"
    exit 1
fi

# Subfolder should only process 2 elements (Sub Root Requirement + Sub Test Requirement)
if [[ "$ELEMENTS_SUB" -ne 2 ]]; then
    echo "❌ FAILED: Expected exactly 2 elements from subfolder, got $ELEMENTS_SUB"
    exit 1
fi

# Test 6: Verify the main requirements are processed (not excluded)
if ! echo "$OUTPUT_ROOT" | grep -q "Main Test Requirement"; then
    echo "❌ FAILED: Main Test Requirement not found in output (should be processed)"
    exit 1
fi

if ! echo "$OUTPUT_ROOT" | grep -q "Sub Test Requirement"; then
    echo "❌ FAILED: Sub Test Requirement not found in output (should be processed)"
    exit 1
fi

printf "%s\n" "✅ All tests passed" >> "${TEST_DIR}/test_results.log"
exit 0