#!/bin/bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Traceability Matrix Generation
# ----------------------------------------------------
# Verifies requirements:
# - Traceability Matrix Builder Implementation
# - CLI Traces Flag
# - CLI Traces SVG Flag
# - JSON Matrix Output
# - Hierarchical Matrix Format

# Test 1: Generate markdown traceability matrix

MATRIX_MD="${TEST_DIR}/output/matrix.md"
echo "Running: reqvire matrix" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" matrix 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
echo "$OUTPUT" > "$MATRIX_MD"

# Verify exit code indicates success
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Traceability matrix generation failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Check for required headers
if ! grep -q "# Traceability Matrix" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing main title in markdown output"
  exit 1
fi

if ! grep -q "## Relation Types Used" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing relation types section in markdown output"
  exit 1
fi

if ! grep -q "## Legend" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing legend section in markdown output"
  exit 1
fi

# Check for basic structural elements and requirements
if ! grep -q "## Root Requirement" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing root requirement sections in markdown output"
  exit 1
fi

# Check for verification indicators
if ! grep -q "✅" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing verification check marks in markdown output"
  exit 1
fi

# Check for unverified indicators 
if ! grep -q "❌" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing unverified indicators in markdown output"
  exit 1
fi

# Check for hierarchy indicators (indented child requirements)
if ! grep -q "↳" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing hierarchy indicators for child requirements"
  exit 1
fi

# Check for deeper hierarchy indicators
if ! grep -q "__↳" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing hierarchy indicators for grandchild requirements"
  exit 1
fi

# Verify table structure
if ! grep -q "| Requirement | Verified |" "$MATRIX_MD"; then
  echo "❌ FAILED: Missing table header with Requirement and Verified columns"
  exit 1
fi

# Test 1.1: Validate Verification Roll-up Strategy in Markdown
# Root Requirement Alpha (all children verified → ✅)
if ! grep "Root Requirement Alpha" "$MATRIX_MD" | grep -q "✅"; then
  echo "❌ FAILED: Root Requirement Alpha should be verified (roll-up: all children verified)"
  exit 1
fi

# Parent With Unverified Child (has direct verification BUT child unverified → ❌)
if grep "Parent With Unverified Child" "$MATRIX_MD" | grep -q "✅"; then
  echo "❌ FAILED: Parent With Unverified Child should be unverified (roll-up: not all children verified)"
  exit 1
fi


# Test 2: Generate JSON traceability matrix
MATRIX_JSON="${TEST_DIR}/output/matrix.json"
echo "Running: reqvire matrix --json" >> "${TEST_DIR}/test_results.log"
set +e
JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" matrix --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$JSON_OUTPUT" >> "${TEST_DIR}/test_results.log"
echo "$JSON_OUTPUT" > "$MATRIX_JSON"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: JSON matrix generation failed with exit code $EXIT_CODE"
  echo "$JSON_OUTPUT"
  exit 1
fi

# Verify JSON output is valid and has expected fields
if ! command -v jq &> /dev/null; then
  echo "⚠️ WARNING: jq not available, skipping JSON validation"
else
  # Check if valid JSON
  if ! jq . "$MATRIX_JSON" > /dev/null 2>&1; then
    echo "❌ FAILED: Invalid JSON format in output"
    exit 1
  fi

  # Check for required JSON fields
  if ! jq -e '.metadata' "$MATRIX_JSON" > /dev/null 2>&1; then
    echo "❌ FAILED: Missing metadata field in JSON output"
    exit 1
  fi
  
  if ! jq -e '.sources' "$MATRIX_JSON" > /dev/null 2>&1; then
    echo "❌ FAILED: Missing sources field in JSON output"
    exit 1
  fi
  
  if ! jq -e '.targets' "$MATRIX_JSON" > /dev/null 2>&1; then
    echo "❌ FAILED: Missing targets field in JSON output"
    exit 1
  fi
  
  if ! jq -e '.matrix' "$MATRIX_JSON" > /dev/null 2>&1; then
    echo "❌ FAILED: Missing matrix field in JSON output"
    exit 1
  fi
  
  if ! jq -e '.verificationStatus' "$MATRIX_JSON" > /dev/null 2>&1; then
    echo "❌ FAILED: Missing verificationStatus field in JSON output"
    exit 1
  fi
fi


# Test 3: Generate SVG traceability matrix
MATRIX_SVG="${TEST_DIR}/output/matrix.svg"
echo "Running: reqvire matrix --svg" >> "${TEST_DIR}/test_results.log"
set +e
SVG_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" matrix --svg 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$SVG_OUTPUT" >> "${TEST_DIR}/test_results.log"
echo "$SVG_OUTPUT" > "$MATRIX_SVG"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: SVG matrix generation failed with exit code $EXIT_CODE"
  echo "$SVG_OUTPUT"
  exit 1
fi

# Verify basic SVG format
if ! grep -q "<svg" "$MATRIX_SVG"; then
  echo "❌ FAILED: Missing SVG root element in output"
  exit 1
fi

# Check for at least some SVG elements we expect to find
if ! grep -q "<\(rect\|text\|g\)" "$MATRIX_SVG"; then
  echo "❌ FAILED: Missing expected SVG elements in output"
  exit 1
fi

# Test 3.1: Validate SVG output matches expected with verification roll-up strategy
EXPECTED_SVG="${TEST_DIR}/expected.svg"

if ! diff -q "$MATRIX_SVG" "$EXPECTED_SVG" > /dev/null 2>&1; then
  echo "❌ FAILED: SVG output does not match expected output"
  echo ""
  echo "Differences:"
  diff "$MATRIX_SVG" "$EXPECTED_SVG" || true
  echo ""
  echo "The SVG matrix should implement verification roll-up strategy:"
  echo "- Parent With Unverified Child should be ❌ (has unverified child)"
  echo "- Root Requirement Alpha should be ✅ (all children verified)"
  echo "- Root Requirement Beta should be ✅ (all children verified)"
  echo "- Unverified Requirement should be ❌ (no verification)"
  exit 1
fi



# Test 4: Check for conflicts
echo "Running: reqvire matrix --json --svg" >> "${TEST_DIR}/test_results.log"
set +e
CONFLICT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN"  --config "${TEST_DIR}/reqvire.yaml" matrix --json --svg 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$CONFLICT_OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Conflict between --json and --svg flags was not detected"
  echo "$CONFLICT_OUTPUT"
  exit 1
fi


exit 0
