#!/bin/bash
set -uo pipefail

echo "Starting feature change impact test..." > "${TEST_DIR}/test_results.log"

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

FEATURE_WORK=$(create_workspace "feature-change")
sed -i 's/Product feature./Product feature changed./' "${FEATURE_WORK}/Requirements.md"
FEATURE_JSON=$(run_change_impact_json "$FEATURE_WORK")
assert_changed_element "$FEATURE_JSON" "Product Feature"
FEATURE_TREE=$(echo "$FEATURE_JSON" | jq '.changed[] | select(.name == "Product Feature") | .change_impact_tree')
assert_tree_contains "$FEATURE_TREE" "Payload Requirement" "Product Feature change should impact specified Payload Requirement through specifiedBy"
assert_tree_contains "$FEATURE_TREE" "Payload Verification" "Product Feature change should propagate through specified requirement to verification"

REQUIREMENT_WORK=$(create_workspace "requirement-change")
sed -i 's/The system shall produce payloads conforming to the product semantic contract./The system shall produce payloads conforming to the product semantic contract safely./' "${REQUIREMENT_WORK}/Requirements.md"
REQUIREMENT_JSON=$(run_change_impact_json "$REQUIREMENT_WORK")
assert_changed_element "$REQUIREMENT_JSON" "Payload Requirement"
REQUIREMENT_TREE=$(echo "$REQUIREMENT_JSON" | jq '.changed[] | select(.name == "Payload Requirement") | .change_impact_tree')
assert_tree_contains "$REQUIREMENT_TREE" "Payload Verification" "Payload Requirement change should invalidate Payload Verification through verifiedBy"

SHAPE_CONTRACT_WORK=$(create_workspace "shape-contract-change")
sed -i 's/sh:minCount 1/sh:minCount 2/' "${SHAPE_CONTRACT_WORK}/Requirements.md"
SHAPE_CONTRACT_JSON=$(run_change_impact_json "$SHAPE_CONTRACT_WORK")
assert_changed_element "$SHAPE_CONTRACT_JSON" "Payload Shape Contract"
SHAPE_CONTRACT_TREE=$(echo "$SHAPE_CONTRACT_JSON" | jq '.changed[] | select(.name == "Payload Shape Contract") | .change_impact_tree')
assert_tree_contains "$SHAPE_CONTRACT_TREE" "Payload Requirement" "Payload Shape Contract change should impact owning Payload Requirement through refine"
assert_tree_contains "$SHAPE_CONTRACT_TREE" "Payload Verification" "Payload Shape Contract change should propagate from requirement to verification"

ATTACHED_ONTOLOGY_WORK=$(create_workspace "attached-ontology-change")
sed -i 's/shared:Initial/shared:Changed/' "${ATTACHED_ONTOLOGY_WORK}/Requirements.md"
ATTACHED_ONTOLOGY_JSON=$(run_change_impact_json "$ATTACHED_ONTOLOGY_WORK")
assert_changed_element "$ATTACHED_ONTOLOGY_JSON" "Product Feature"
ATTACHED_CHANGED=$(echo "$ATTACHED_ONTOLOGY_JSON" | jq -r '.changed[] | select(.name == "Product Feature") | .changed_attachments[]?')
if ! echo "$ATTACHED_CHANGED" | grep -q "shared-ontology"; then
  echo "FAILED: Shared Ontology content change should mark Product Feature attachment as changed"
  echo "$ATTACHED_ONTOLOGY_JSON" | jq '.changed[] | select(.name == "Product Feature")'
  exit 1
fi
ATTACHED_TREE=$(echo "$ATTACHED_ONTOLOGY_JSON" | jq '.changed[] | select(.name == "Product Feature") | .change_impact_tree')
assert_tree_contains "$ATTACHED_TREE" "Payload Requirement" "Attached ontology change should propagate from attaching feature to specified requirement"

exit 0
