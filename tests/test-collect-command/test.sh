#!/bin/bash

# Test: Collect Command Verification
# Acceptance Criteria:
# - Collect command aggregates content from requirement chain via derivedFrom
# - Output includes source citations
# - JSON output has correct structure
# - Error handling for non-existent and non-requirement elements

set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Helper function to compare files and show diff on failure
assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "❌ FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# Test 1: Basic text output for leaf requirement
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Leaf Requirement" > /tmp/collect-text-output.txt 2>&1
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Collect command returned error: $EXIT_CODE"
  cat /tmp/collect-text-output.txt
  exit 1
fi

assert_file_matches "${TEST_SCRIPT_DIR}/expected/text-output.txt" \
  /tmp/collect-text-output.txt \
  "Text output does not match expected"

# Test 2: JSON output structure
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Leaf Requirement" --json > /tmp/collect-json-output.json 2>&1
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: JSON collect command returned error: $EXIT_CODE"
  cat /tmp/collect-json-output.json
  exit 1
fi

# Verify valid JSON
if ! jq . /tmp/collect-json-output.json >/dev/null 2>&1; then
  echo "❌ FAILED: Invalid JSON output"
  cat /tmp/collect-json-output.json
  exit 1
fi

assert_file_matches "${TEST_SCRIPT_DIR}/expected/json-output.json" \
  /tmp/collect-json-output.json \
  "JSON output does not match expected"

# Test 3: Error handling - element not found
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Non Existent Element" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Should have returned error for non-existent element"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "not found"; then
  echo "❌ FAILED: Error message should mention 'not found'"
  echo "$OUTPUT"
  exit 1
fi

# Test 4: Error handling - non-requirement type
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Test Verification" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Should have returned error for verification element"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "not a requirement type"; then
  echo "❌ FAILED: Error message should mention 'not a requirement type'"
  echo "$OUTPUT"
  exit 1
fi

exit 0
