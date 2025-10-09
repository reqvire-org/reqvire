#!/usr/bin/env bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Verification Traces Filter Options Test
# --------------------------------------
# Satisfies: specifications/Verifications/ReportsTests.md#verification-traces-filter-options-test
#
# Acceptance Criteria:
# - System shall provide CLI command `verification-traces` that generates upward trace trees from verifications
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
echo "Running: reqvire verification-traces" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" verification-traces 2>&1)
EXIT_CODE=$?
set -e

# Save output for debugging (always do this)
echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: verification-traces command exited with code $EXIT_CODE"
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
echo "Running: reqvire verification-traces --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" verification-traces --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: verification-traces --json command exited with code $EXIT_CODE"
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
echo "Running: reqvire verification-traces --filter-id" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" verification-traces \
    --filter-id="specifications/Verifications/Tests.md#oauth-flow-test" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: verification-traces with --filter-id exited with code $EXIT_CODE"
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
echo "Running: reqvire verification-traces --filter-name" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" verification-traces \
    --filter-name=".*Coverage.*" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: verification-traces with --filter-name exited with code $EXIT_CODE"
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
echo "Running: reqvire verification-traces --filter-type" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" verification-traces \
    --filter-type="test-verification" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: verification-traces with --filter-type exited with code $EXIT_CODE"
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
echo "Running: reqvire verification-traces --filter-type --filter-name" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" verification-traces \
    --filter-type="test-verification" --filter-name=".*Test" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED: verification-traces with combined filters exited with code $EXIT_CODE"
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

exit 0
