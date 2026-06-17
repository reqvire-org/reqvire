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

# Requirement upstream crosses to the owning capability and capability ancestors.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Leaf Requirement" > /tmp/collect-text-output.txt 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect requirement upstream returned error: $EXIT_CODE"
  cat /tmp/collect-text-output.txt
  exit 1
fi
assert_contains /tmp/collect-text-output.txt "The top product capability" "upstream should include parent capability"
assert_contains /tmp/collect-text-output.txt "The child product capability" "upstream should include owning capability"
assert_contains /tmp/collect-text-output.txt "The root requirement" "upstream should include root requirement"
assert_contains /tmp/collect-text-output.txt "The leaf requirement" "upstream should include starting requirement"

# JSON output remains machine readable and contains capability context.
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
if [ "$(jq -r '.items[] | select(.name == "Product Capability") | .element_type' /tmp/collect-json-output.json)" != "capability" ]; then
  echo "FAILED: upstream JSON should include Product Capability as capability"
  cat /tmp/collect-json-output.json
  exit 1
fi

# Requirement downstream stays in the requirement hierarchy and does not cross back to capabilities.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Root Requirement" --direction DOWNSTREAM > /tmp/collect-downstream-text.txt 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect requirement downstream returned error: $EXIT_CODE"
  cat /tmp/collect-downstream-text.txt
  exit 1
fi
assert_contains /tmp/collect-downstream-text.txt "The root requirement" "downstream should include root requirement"
assert_contains /tmp/collect-downstream-text.txt "The leaf requirement" "downstream should include leaf requirement"
if grep -q "The child product capability" /tmp/collect-downstream-text.txt; then
  echo "FAILED: requirement downstream should not include capability context"
  cat /tmp/collect-downstream-text.txt
  exit 1
fi

# Capability upstream stays in capability hierarchy only.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Child Capability" --direction UPSTREAM > /tmp/collect-capability-upstream.txt 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect capability upstream returned error: $EXIT_CODE"
  cat /tmp/collect-capability-upstream.txt
  exit 1
fi
assert_contains /tmp/collect-capability-upstream.txt "The top product capability" "capability upstream should include parent capability"
if grep -q "The root requirement" /tmp/collect-capability-upstream.txt; then
  echo "FAILED: capability upstream should not include specified requirements"
  cat /tmp/collect-capability-upstream.txt
  exit 1
fi

# Capability downstream follows capability hierarchy and specifiedBy into requirements.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Product Capability" --direction DOWNSTREAM --json > /tmp/collect-capability-downstream.json 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect capability downstream returned error: $EXIT_CODE"
  cat /tmp/collect-capability-downstream.json
  exit 1
fi
if ! jq . /tmp/collect-capability-downstream.json >/dev/null 2>&1; then
  echo "FAILED: invalid capability downstream JSON"
  cat /tmp/collect-capability-downstream.json
  exit 1
fi
for name in "Product Capability" "Child Capability" "Root Requirement" "Leaf Requirement" "Collect Ontology"; do
  if ! jq -e --arg name "$name" '.items[] | select(.name == $name)' /tmp/collect-capability-downstream.json >/dev/null; then
    echo "FAILED: capability downstream JSON missing ${name}"
    cat /tmp/collect-capability-downstream.json
    exit 1
  fi
done

# Ontology upstream follows ontology hierarchy.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Collect Child Ontology" --direction UPSTREAM --json > /tmp/collect-ontology-upstream.json 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect ontology upstream returned error: $EXIT_CODE"
  cat /tmp/collect-ontology-upstream.json
  exit 1
fi
for name in "Collect Ontology" "Collect Child Ontology"; do
  if ! jq -e --arg name "$name" '.items[] | select(.name == $name and .element_type == "ontology")' /tmp/collect-ontology-upstream.json >/dev/null; then
    echo "FAILED: ontology upstream JSON missing ${name}"
    cat /tmp/collect-ontology-upstream.json
    exit 1
  fi
done

# Ontology downstream follows child ontology and semantic contracts that use the ontology context.
cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Collect Ontology" --direction DOWNSTREAM --json > /tmp/collect-ontology-downstream.json 2>&1
EXIT_CODE=$?
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: collect ontology downstream returned error: $EXIT_CODE"
  cat /tmp/collect-ontology-downstream.json
  exit 1
fi
for name in "Collect Ontology" "Collect Child Ontology" "Collect Shape Contract"; do
  if ! jq -e --arg name "$name" '.items[] | select(.name == $name)' /tmp/collect-ontology-downstream.json >/dev/null; then
    echo "FAILED: ontology downstream JSON missing ${name}"
    cat /tmp/collect-ontology-downstream.json
    exit 1
  fi
done
if ! jq -e '.items[] | select(.name == "Collect Shape Contract" and .element_type == "semantic-contract")' /tmp/collect-ontology-downstream.json >/dev/null; then
  echo "FAILED: ontology downstream JSON should include semantic-contract users"
  cat /tmp/collect-ontology-downstream.json
  exit 1
fi

# Error handling - element not found.
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Non Existent Element" 2>&1)
EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ] || ! echo "$OUTPUT" | grep -q "not found"; then
  echo "FAILED: missing element should return not found error"
  echo "$OUTPUT"
  exit 1
fi

# Error handling - non capability/requirement type.
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" collect "Test Verification" 2>&1)
EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ] || ! echo "$OUTPUT" | grep -q "not a capability, requirement, or ontology type"; then
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
