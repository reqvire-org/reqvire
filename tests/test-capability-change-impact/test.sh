#!/bin/bash
set -uo pipefail

echo "Starting capability change impact test..." > "${TEST_DIR}/test_results.log"

create_workspace() {
  local name="$1"
  local work_dir="${TEST_DIR}/${name}"
  mkdir -p "$work_dir"
  cp "${TEST_DIR}/Requirements.md" "${TEST_DIR}/payload_impl.txt" "${TEST_DIR}/payload_test.txt" "$work_dir/"
  (
    cd "$work_dir" &&
      git init >/dev/null 2>&1 &&
      git config user.email test@example.com &&
      git config user.name "Test User" &&
      git remote remove origin >/dev/null 2>&1 || true &&
      git remote add origin https://dummy.example.com/dummy-repo.git &&
      git add . &&
      git commit -m init >/dev/null 2>&1
  )
  printf "%s\n" "$work_dir"
}

run_change_impact_json() {
  local work_dir="$1"
  local json_raw
  set +e
  json_raw=$(cd "$work_dir" && "$REQVIRE_BIN" change-impact --json 2>&1)
  local status=$?
  set -e

  echo "Workspace: $work_dir" >> "${TEST_DIR}/test_results.log"
  echo "Exit code: $status" >> "${TEST_DIR}/test_results.log"
  printf "%s\n" "$json_raw" >> "${TEST_DIR}/test_results.log"

  if [ $status -ne 0 ]; then
    echo "FAILED: change-impact --json failed in $work_dir"
    echo "$json_raw"
    exit 1
  fi

  local json_output
  json_output=$(echo "$json_raw" | grep -v "Warning:" | grep -A 1000 "^{")
  if ! echo "$json_output" | jq . >/dev/null 2>&1; then
    echo "FAILED: change-impact output is not valid JSON"
    echo "$json_raw"
    exit 1
  fi
  printf "%s\n" "$json_output"
}

assert_changed_element() {
  local json="$1"
  local name="$2"
  local count
  count=$(echo "$json" | jq --arg name "$name" '[.changed[] | select(.name == $name)] | length')
  if [ "$count" -ne 1 ]; then
    echo "FAILED: expected changed element '$name'"
    echo "$json" | jq '.changed'
    exit 1
  fi
}

assert_tree_contains() {
  local tree="$1"
  local name="$2"
  local message="$3"
  if ! echo "$tree" | jq -e --arg name "$name" '.. | objects | select(.name? == $name)' >/dev/null; then
    echo "FAILED: $message"
    echo "$tree"
    exit 1
  fi
}

CAPABILITY_WORK=$(create_workspace "capability-change")
sed -i 's/Product capability./Product capability changed./' "${CAPABILITY_WORK}/Requirements.md"
CAPABILITY_JSON=$(run_change_impact_json "$CAPABILITY_WORK")
assert_changed_element "$CAPABILITY_JSON" "Product Capability"
CAPABILITY_TREE=$(echo "$CAPABILITY_JSON" | jq '.changed[] | select(.name == "Product Capability") | .change_impact_tree')
assert_tree_contains "$CAPABILITY_TREE" "Payload Requirement" "Product Capability change should impact specified Payload Requirement through specifiedBy"
assert_tree_contains "$CAPABILITY_TREE" "Payload Verification" "Product Capability change should propagate through specified requirement to verification"

REQUIREMENT_WORK=$(create_workspace "requirement-change")
sed -i 's/The system shall produce payloads conforming to the product semantic contract./The system shall produce payloads conforming to the product semantic contract safely./' "${REQUIREMENT_WORK}/Requirements.md"
REQUIREMENT_JSON=$(run_change_impact_json "$REQUIREMENT_WORK")
assert_changed_element "$REQUIREMENT_JSON" "Payload Requirement"
REQUIREMENT_TREE=$(echo "$REQUIREMENT_JSON" | jq '.changed[] | select(.name == "Payload Requirement") | .change_impact_tree')
assert_tree_contains "$REQUIREMENT_TREE" "Payload Shape Contract" "Payload Requirement change should flag Payload Shape Contract consistency review through constrainedBy"
assert_tree_contains "$REQUIREMENT_TREE" "Payload Verification" "Payload Requirement change should invalidate Payload Verification through verifiedBy"

SHAPE_CONTRACT_WORK=$(create_workspace "shape-contract-change")
sed -i 's/sh:minCount 1/sh:minCount 2/' "${SHAPE_CONTRACT_WORK}/Requirements.md"
SHAPE_CONTRACT_JSON=$(run_change_impact_json "$SHAPE_CONTRACT_WORK")
assert_changed_element "$SHAPE_CONTRACT_JSON" "Payload Shape Contract"
SHAPE_CONTRACT_TREE=$(echo "$SHAPE_CONTRACT_JSON" | jq '.changed[] | select(.name == "Payload Shape Contract") | .change_impact_tree')
assert_tree_contains "$SHAPE_CONTRACT_TREE" "Payload Requirement" "Payload Shape Contract change should impact owning Payload Requirement through constrain"
assert_tree_contains "$SHAPE_CONTRACT_TREE" "Payload Verification" "Payload Shape Contract change should propagate from requirement to verification"
if echo "$SHAPE_CONTRACT_TREE" | jq -e '.. | objects | select(.name? == "Contract Only Ontology")' >/dev/null; then
  echo "FAILED: Payload Shape Contract change should not propagate back to used ontology"
  echo "$SHAPE_CONTRACT_TREE"
  exit 1
fi

CONTRACT_ONTOLOGY_WORK=$(create_workspace "contract-ontology-change")
sed -i 's/contract:Initial/contract:Changed/' "${CONTRACT_ONTOLOGY_WORK}/Requirements.md"
CONTRACT_ONTOLOGY_JSON=$(run_change_impact_json "$CONTRACT_ONTOLOGY_WORK")
assert_changed_element "$CONTRACT_ONTOLOGY_JSON" "Contract Only Ontology"
CONTRACT_ONTOLOGY_TREE=$(echo "$CONTRACT_ONTOLOGY_JSON" | jq '.changed[] | select(.name == "Contract Only Ontology") | .change_impact_tree')
assert_tree_contains "$CONTRACT_ONTOLOGY_TREE" "Payload Shape Contract" "Contract Only Ontology change should impact Payload Shape Contract through usedBy"
assert_tree_contains "$CONTRACT_ONTOLOGY_TREE" "Payload Requirement" "Contract Only Ontology change should propagate through Payload Shape Contract to Payload Requirement"
assert_tree_contains "$CONTRACT_ONTOLOGY_TREE" "Payload Verification" "Contract Only Ontology change should propagate to downstream Payload Verification"

exit 0
