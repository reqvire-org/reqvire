#!/bin/bash

set -uo pipefail

assert_contains() {
  local file="$1"
  local expected="$2"
  local description="$3"
  if ! grep -q "$expected" "$file"; then
    echo "FAILED: $description"
    echo "Expected: $expected"
    cat "$file"
    exit 1
  fi
}

# Requirement upstream crosses to the owning feature and feature ancestors.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Leaf Requirement" > /tmp/collect-text-output.txt 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect requirement upstream returned error: $EXIT_CODE"
  cat /tmp/collect-text-output.txt
  exit 1
fi
assert_contains /tmp/collect-text-output.txt "The top product feature" "upstream should include parent feature"
assert_contains /tmp/collect-text-output.txt "The child product feature" "upstream should include owning feature"
assert_contains /tmp/collect-text-output.txt "The root requirement" "upstream should include root requirement"
assert_contains /tmp/collect-text-output.txt "The leaf requirement" "upstream should include starting requirement"

# JSON output remains machine readable and contains feature context.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Leaf Requirement" --json > /tmp/collect-json-output.json 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect requirement upstream JSON returned error: $EXIT_CODE"
  cat /tmp/collect-json-output.json
  exit 1
fi
if ! jq . /tmp/collect-json-output.json >/dev/null 2>&1; then
  echo "FAILED: invalid JSON output"
  cat /tmp/collect-json-output.json
  exit 1
fi
if [ "$(jq -r '.items[] | select(.name == "Product Feature") | .element_type' /tmp/collect-json-output.json)" != "feature" ]; then
  echo "FAILED: upstream JSON should include Product Feature as feature"
  cat /tmp/collect-json-output.json
  exit 1
fi

# Requirement downstream stays in the requirement hierarchy and does not cross back to features.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Root Requirement" --direction DOWNSTREAM > /tmp/collect-downstream-text.txt 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect requirement downstream returned error: $EXIT_CODE"
  cat /tmp/collect-downstream-text.txt
  exit 1
fi
assert_contains /tmp/collect-downstream-text.txt "The root requirement" "downstream should include root requirement"
assert_contains /tmp/collect-downstream-text.txt "The leaf requirement" "downstream should include leaf requirement"
if grep -q "The child product feature" /tmp/collect-downstream-text.txt; then
  echo "FAILED: requirement downstream should not include feature context"
  cat /tmp/collect-downstream-text.txt
  exit 1
fi

# Feature upstream stays in feature hierarchy only.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Child Feature" --direction UPSTREAM > /tmp/collect-feature-upstream.txt 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect feature upstream returned error: $EXIT_CODE"
  cat /tmp/collect-feature-upstream.txt
  exit 1
fi
assert_contains /tmp/collect-feature-upstream.txt "The top product feature" "feature upstream should include parent feature"
if grep -q "The root requirement" /tmp/collect-feature-upstream.txt; then
  echo "FAILED: feature upstream should not include specified requirements"
  cat /tmp/collect-feature-upstream.txt
  exit 1
fi

# Feature downstream follows feature hierarchy and specifiedBy into requirements.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Product Feature" --direction DOWNSTREAM --json > /tmp/collect-feature-downstream.json 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect feature downstream returned error: $EXIT_CODE"
  cat /tmp/collect-feature-downstream.json
  exit 1
fi
if ! jq . /tmp/collect-feature-downstream.json >/dev/null 2>&1; then
  echo "FAILED: invalid feature downstream JSON"
  cat /tmp/collect-feature-downstream.json
  exit 1
fi
for name in "Product Feature" "Child Feature" "Root Requirement" "Leaf Requirement" "Collect Ontology"; do
  if ! jq -e --arg name "$name" '.items[] | select(.name == $name)' /tmp/collect-feature-downstream.json >/dev/null; then
    echo "FAILED: feature downstream JSON missing ${name}"
    cat /tmp/collect-feature-downstream.json
    exit 1
  fi
done

# Error handling - element not found.
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Non Existent Element" 2>&1)
EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ] || ! echo "$OUTPUT" | grep -q "not found"; then
  echo "FAILED: missing element should return not found error"
  echo "$OUTPUT"
  exit 1
fi

# Error handling - non feature/requirement type.
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Test Verification" 2>&1)
EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ] || ! echo "$OUTPUT" | grep -q "not a feature or requirement type"; then
  echo "FAILED: verification collect should return type error"
  echo "$OUTPUT"
  exit 1
fi

# Invalid direction returns error.
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Leaf Requirement" --direction INVALID 2>&1)
EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ] || ! echo "$OUTPUT" | grep -qi "invalid direction"; then
  echo "FAILED: invalid direction should return error"
  echo "$OUTPUT"
  exit 1
fi

exit 0
