#!/usr/bin/env bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Verification Traces Filter Options Test
# --------------------------------------
# Satisfies: specifications/Verifications/ReportsTests.md#traces-filter-options-test
#
# Acceptance Criteria:
# - System shall provide CLI command `traces` that generates upward trace trees from verifications
# - Command shall output to stdout in Markdown format with embedded Mermaid diagrams by default
# - Command shall support `--json` flag for structured JSON output without diagrams
# - Mermaid diagrams shall show verification element as root with arrows following relation semantics
# - Directly verified requirements shall be marked/highlighted in diagrams using CSS classes
# - System shall traverse all upward parent relations to reach root requirements
# - System shall merge multiple verification paths into single tree per verification
# - System shall support `--verification-id=<id>` filter for specific verification element
# - System shall support `--filter-name=<regex>` for filtering by verification name pattern
# - System shall support `--filter-type=<type>` for filtering by verification type
# - Multiple filters shall be combinable using AND logic
# - JSON output shall include verification ID, directly verified requirements, and complete trace tree structure
#
# Test Criteria:
# - Commands exit with success (0) return code
# - Markdown output matches expected structure with files, sections, and verification traces
# - JSON output matches expected structure with files, sections, and verification data
# - Filters correctly restrict output to matching verifications

# Test 1: Basic Markdown Output - Compare against expected file
echo "Running: reqvire traces" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces 2>&1)
EXIT_CODE=$?
set -e

# Save output for debugging (always do this)
echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces command exited with code $EXIT_CODE"
    echo "$OUTPUT"
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

# Test 2: JSON Output - Compare against expected file
echo "Running: reqvire traces --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces --json command exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Output is not valid JSON"
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

# Test 3: Specific Verification Filter
echo "Running: reqvire traces --filter-id" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --filter-id="specifications/Verifications/Tests.md#oauth-flow-test" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces with --filter-id exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Should contain only OAuth Flow Test verification
if ! grep -q "OAuth Flow Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Missing specified verification 'OAuth Flow Test'"
    exit 1
fi

# Should NOT contain other verifications
if grep -q "Session Timeout Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should exclude 'Session Timeout Test'"
    exit 1
fi

if grep -q "Encryption Coverage Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should exclude 'Encryption Coverage Test'"
    exit 1
fi

# Test 4: Name Pattern Filter
echo "Running: reqvire traces --filter-name" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --filter-name=".*Coverage.*" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces with --filter-name exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Should contain verifications matching pattern
if ! grep -q "Encryption Coverage Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should include 'Encryption Coverage Test'"
    exit 1
fi

if ! grep -q "Coverage Calculation Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should include 'Coverage Calculation Test'"
    exit 1
fi

# Should NOT contain non-matching verifications
if grep -q "OAuth Flow Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should exclude 'OAuth Flow Test' (doesn't match pattern)"
    exit 1
fi

# Test 5: Type Filter
echo "Running: reqvire traces --filter-type" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --filter-type="test-verification" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces with --filter-type exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Should contain test-verification elements
if ! grep -q "OAuth Flow Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should include test-verification 'OAuth Flow Test'"
    exit 1
fi

# Should NOT contain other verification types
if grep -q "Security Analysis" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should exclude analysis-verification 'Security Analysis'"
    exit 1
fi

if grep -q "Code Inspection" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should exclude inspection-verification 'Code Inspection'"
    exit 1
fi

# Test 6: Combined Filters
echo "Running: reqvire traces --filter-type --filter-name" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --filter-type="test-verification" --filter-name=".*Test" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces with combined filters exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Should contain verifications matching ALL criteria (test-verification AND name ends with "Test")
if ! grep -q "OAuth Flow Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Combined filters should include 'OAuth Flow Test' (matches both)"
    exit 1
fi

# "Security Analysis" should be excluded (not test-verification)
if grep -q "Security Analysis" <<< "$OUTPUT"; then
    echo "❌ FAILED: Combined filters should exclude 'Security Analysis' (wrong type)"
    exit 1
fi

# Verify count - should have exactly 4 test-verification elements ending with "Test"
TEST_COUNT=$(echo "$OUTPUT" | grep -c '^#### ' || true)
# We have: OAuth Flow Test, Session Timeout Test, Encryption Coverage Test, Coverage Calculation Test
# That's 4 test-verification elements with "Test" in name
if [ "$TEST_COUNT" -ne 4 ]; then
    echo "❌ FAILED: Combined filters should return exactly 4 verifications, got $TEST_COUNT"
    exit 1
fi

# Test 7: From-Folder Option - Basic usage
# Create the from-folder directory so canonicalize works
mkdir -p "${TEST_DIR}/specifications"

echo "Running: reqvire traces --from-folder=specifications" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --from-folder=specifications 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces --from-folder=specifications exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Check that output contains click handlers
if ! grep -q 'click .* ".*' <<< "$OUTPUT"; then
    echo "❌ FAILED: Output should contain click handlers with paths"
    exit 1
fi

# When from-folder=specifications, links should NOT have "specifications/" prefix
# They should be relative to the specifications folder (e.g., "Verifications/Tests.md" not "specifications/Verifications/Tests.md")
if grep -q 'click .* "specifications/' <<< "$OUTPUT"; then
    echo "❌ FAILED: Links should be relative to specifications folder (should not contain 'specifications/' prefix)"
    exit 1
fi

# Check that links are correct format (like "Verifications/Tests.md" or "SystemRequirements.md")
if ! grep -q 'click .* "Verifications/Tests\.md' <<< "$OUTPUT"; then
    echo "❌ FAILED: Links should be relative to specifications folder (e.g., Verifications/Tests.md)"
    exit 1
fi

# Test 8: From-Folder with nested path
# Create the from-folder directory so canonicalize works
mkdir -p "${TEST_DIR}/output/verification/traces"

echo "Running: reqvire traces --from-folder=output/verification/traces" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --from-folder=output/verification/traces 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces --from-folder with nested path exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Debug: Show actual click handlers generated
echo "DEBUG: Sample click handlers from output:"
echo "$OUTPUT" | grep "click" | head -5

# Check for correct relative path depth (should have ../../../ for three levels)
if ! grep -q '\.\./\.\./\.\./specifications' <<< "$OUTPUT"; then
    echo "❌ FAILED: Links should navigate up three levels (should contain ../../../specifications)"
    echo "DEBUG: Actual links found:"
    echo "$OUTPUT" | grep "click" | head -3
    exit 1
fi

# Test 9: From-Folder with JSON output (should not affect JSON structure)
# Create the from-folder directory so canonicalize works
mkdir -p "${TEST_DIR}/docs/reports"

echo "Running: reqvire traces --from-folder=docs/reports --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --from-folder=docs/reports --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces --from-folder --json exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Output with --from-folder --json is not valid JSON"
    exit 1
fi

# JSON identifiers should still be absolute (from git root), not affected by from-folder
# Check that identifiers in JSON don't contain ../../ patterns
if echo "$OUTPUT" | jq -e '.files | to_entries[] | .value.sections | to_entries[] | .value.verifications[].identifier' | grep -q '\.\./'; then
    echo "❌ FAILED: JSON identifiers should remain absolute, not relative"
    exit 1
fi

# Test 10: From-Folder combined with filter
# Create the from-folder directory so canonicalize works
mkdir -p "${TEST_DIR}/docs/reports"

echo "Running: reqvire traces --from-folder=docs/reports --filter-type=test-verification" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --from-folder=docs/reports --filter-type="test-verification" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces --from-folder with filter exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Should still have relative links
if ! grep -q '\.\./\.\./specifications' <<< "$OUTPUT"; then
    echo "❌ FAILED: Links should still be relative when using --from-folder with filters"
    exit 1
fi

# Should contain test-verification elements
if ! grep -q "OAuth Flow Test" <<< "$OUTPUT"; then
    echo "❌ FAILED: Filter should work correctly with --from-folder option"
    exit 1
fi

# Test 11: From-Folder with / (special case for git root)
echo "Running: reqvire traces --from-folder=/" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces \
    --from-folder=/ 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces --from-folder=/ exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# When from-folder=/, links should remain as git-root-relative (with "specifications/" prefix)
if ! grep -q 'click .* "specifications/' <<< "$OUTPUT"; then
    echo "❌ FAILED: With --from-folder=/, links should be git-root-relative (contain 'specifications/' prefix)"
    exit 1
fi

# Should match the basic output format (Test 1)
if ! grep -q 'click .* "specifications/Verifications/Tests\.md' <<< "$OUTPUT"; then
    echo "❌ FAILED: With --from-folder=/, links should be same as basic output"
    exit 1
fi

# Test 12: Redundant Relations - Compare markdown output with expected
echo "Running: reqvire traces (checking for redundant relations)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces command exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Save actual output
echo "$OUTPUT" > "${TEST_DIR}/actual_redundant_output.md"

# Compare with expected output if it exists
if [ -f "${TEST_DIR}/expected_redundant_output.md" ]; then
    if ! diff -u "${TEST_DIR}/expected_redundant_output.md" "${TEST_DIR}/actual_redundant_output.md" > "${TEST_DIR}/diff_redundant.txt" 2>&1; then
        echo "❌ FAILED: Redundant relations markdown output does not match expected"
        echo "Diff (expected vs actual):"
        cat "${TEST_DIR}/diff_redundant.txt"
        exit 1
    fi
fi

# Test 13: Redundant Relations - Compare JSON output with expected
echo "Running: reqvire traces --json (checking redundant_relations field)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" traces --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: traces --json command exited with code $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "❌ FAILED: Output is not valid JSON"
    exit 1
fi

# Save actual JSON output (formatted)
echo "$OUTPUT" | jq '.' > "${TEST_DIR}/actual_redundant_output.json"

# Compare JSON outputs if expected exists
if [ -f "${TEST_DIR}/expected_redundant_output.json" ]; then
    EXPECTED_JSON=$(jq -S '.' "${TEST_DIR}/expected_redundant_output.json")
    ACTUAL_JSON=$(jq -S '.' "${TEST_DIR}/actual_redundant_output.json")

    if [ "$EXPECTED_JSON" != "$ACTUAL_JSON" ]; then
        echo "❌ FAILED: Redundant relations JSON output does not match expected"
        echo "Diff (expected vs actual):"
        diff -u <(echo "$EXPECTED_JSON") <(echo "$ACTUAL_JSON") || true
        exit 1
    fi
fi

exit 0
