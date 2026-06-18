### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---

#!/bin/bash
set -uo pipefail

# Test: Requirement Governance Metadata
# ------------------------------------
# Satisfies:
#   - requirements/Functional/Core/Verifications/ParsingVerifications.md#requirement-governance-metadata-verification
#   - requirements/Functional/Operations/Verifications/FormattingVerifications.md#requirement-governance-metadata-formatting-verification
#
# Acceptance Criteria:
# - Requirement governance metadata is exposed as effective model evidence.
# - Effective values distinguish explicit, inherited, and default sources.
# - Invalid status, priority, and risk enum values are rejected.
# - Non-requirement elements cannot author requirement governance metadata.
# - Formatting does not persist inherited or default governance metadata.

FAILURES=0

fail() {
  echo "FAILED: $1"
  FAILURES=$((FAILURES + 1))
}

assert_eq() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if [ "$actual" != "$expected" ]; then
    fail "$description (expected '$expected', got '$actual')"
  fi
}

json_value() {
  local json="$1"
  local element_name="$2"
  local path="$3"

  echo "$json" | jq -r \
    --arg name "$element_name" \
    ".files[\"specifications/Requirements.md\"].elements[] | select(.name == \$name) | ${path}"
}

assert_validation_rejects_file() {
  local file_path="$1"
  local file_content="$2"
  local expected_pattern="$3"
  local description="$4"

  printf "%s\n" "$file_content" > "$file_path"

  set +e
  OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
  EXIT_CODE=$?

  if [ $EXIT_CODE -eq 0 ]; then
    fail "$description should fail validation"
  elif ! echo "$OUTPUT" | grep -Eiq "$expected_pattern"; then
    fail "$description should mention '$expected_pattern' in diagnostics. Output: $OUTPUT"
  fi

  rm -f "$file_path"
}

echo "Test 1: Initial fixture validates"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
  fail "initial governance metadata fixture should validate. Output: $OUTPUT"
fi

echo "Test 2: Effective governance metadata is exposed with source information"
set +e
SEARCH_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json 2>&1)
SEARCH_EXIT=$?

if [ $SEARCH_EXIT -ne 0 ]; then
  fail "search --json should succeed. Output: $SEARCH_JSON"
else
  echo "$SEARCH_JSON" > "$TEST_DIR/search-output.json"

  assert_eq "approved" "$(json_value "$SEARCH_JSON" "Root Requirement" ".governance_metadata.status.value")" "root status value"
  assert_eq "explicit" "$(json_value "$SEARCH_JSON" "Root Requirement" ".governance_metadata.status.source")" "root status source"
  assert_eq "false" "$(json_value "$SEARCH_JSON" "Root Requirement" ".governance_metadata.status | has(\"source_identifier\")")" "root explicit status omits source identifier"
  assert_eq "high" "$(json_value "$SEARCH_JSON" "Root Requirement" ".governance_metadata.priority.value")" "root priority value"
  assert_eq "medium" "$(json_value "$SEARCH_JSON" "Root Requirement" ".governance_metadata.risk.value")" "root risk value"
  assert_eq "Platform Team" "$(json_value "$SEARCH_JSON" "Root Requirement" ".governance_metadata.owner.value")" "root owner value"

  assert_eq "approved" "$(json_value "$SEARCH_JSON" "Child Inherits Governance" ".governance_metadata.status.value")" "child inherited status value"
  assert_eq "inherited" "$(json_value "$SEARCH_JSON" "Child Inherits Governance" ".governance_metadata.status.source")" "child inherited status source"
  assert_eq "specifications/Requirements.md#root-requirement" "$(json_value "$SEARCH_JSON" "Child Inherits Governance" ".governance_metadata.status.source_identifier")" "child inherited status source identifier"
  assert_eq "high" "$(json_value "$SEARCH_JSON" "Child Inherits Governance" ".governance_metadata.priority.value")" "child inherited priority value"
  assert_eq "Platform Team" "$(json_value "$SEARCH_JSON" "Child Inherits Governance" ".governance_metadata.owner.value")" "child inherited owner value"

  assert_eq "approved" "$(json_value "$SEARCH_JSON" "Independent Requirement" ".governance_metadata.status.value")" "independent default status value"
  assert_eq "default" "$(json_value "$SEARCH_JSON" "Independent Requirement" ".governance_metadata.status.source")" "independent default status source"
  assert_eq "false" "$(json_value "$SEARCH_JSON" "Independent Requirement" ".governance_metadata.status | has(\"source_identifier\")")" "independent default status omits source identifier"
  assert_eq "medium" "$(json_value "$SEARCH_JSON" "Independent Requirement" ".governance_metadata.priority.value")" "independent default priority value"
  assert_eq "low" "$(json_value "$SEARCH_JSON" "Independent Requirement" ".governance_metadata.risk.value")" "independent default risk value"
  assert_eq "" "$(json_value "$SEARCH_JSON" "Independent Requirement" ".governance_metadata.owner.value")" "independent default owner value"

  assert_eq "review" "$(json_value "$SEARCH_JSON" "Child Overrides Governance" ".governance_metadata.status.value")" "override child status value"
  assert_eq "explicit" "$(json_value "$SEARCH_JSON" "Child Overrides Governance" ".governance_metadata.status.source")" "override child status source"
  assert_eq "critical" "$(json_value "$SEARCH_JSON" "Child Overrides Governance" ".governance_metadata.risk.value")" "override child risk value"
  assert_eq "explicit" "$(json_value "$SEARCH_JSON" "Child Overrides Governance" ".governance_metadata.risk.source")" "override child risk source"
  assert_eq "high" "$(json_value "$SEARCH_JSON" "Child Overrides Governance" ".governance_metadata.priority.value")" "override child inherited priority value"
  assert_eq "inherited" "$(json_value "$SEARCH_JSON" "Child Overrides Governance" ".governance_metadata.priority.source")" "override child inherited priority source"

  assert_eq "5" "$(echo "$SEARCH_JSON" | jq -r '.global_counters.total_governance_metadata.status.approved')" "summary approved status count"
  assert_eq "1" "$(echo "$SEARCH_JSON" | jq -r '.global_counters.total_governance_metadata.status.review')" "summary review status count"
  assert_eq "3" "$(echo "$SEARCH_JSON" | jq -r '.global_counters.total_governance_metadata.priority.high')" "summary high priority count"
  assert_eq "3" "$(echo "$SEARCH_JSON" | jq -r '.global_counters.total_governance_metadata.risk.low')" "summary low risk count"
  assert_eq "3" "$(echo "$SEARCH_JSON" | jq -r '.global_counters.total_governance_metadata.owner["Platform Team"]')" "summary owner count"
  assert_eq "3" "$(echo "$SEARCH_JSON" | jq -r '.global_counters.total_governance_metadata.owner.unassigned')" "summary unassigned owner count"
fi

echo "Test 2.1: Text search summary includes governance metadata counters"
set +e
SEARCH_TEXT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search 2>&1)
SEARCH_TEXT_EXIT=$?

if [ $SEARCH_TEXT_EXIT -ne 0 ]; then
  fail "search text should succeed. Output: $SEARCH_TEXT"
else
  echo "$SEARCH_TEXT" > "$TEST_DIR/search-output.txt"
  if ! echo "$SEARCH_TEXT" | grep -q "Requirement Governance Metadata"; then
    fail "search text summary should include governance metadata section"
  fi
  if ! echo "$SEARCH_TEXT" | grep -q "    approved: 5"; then
    fail "search text summary should include approved status count"
  fi
  if ! echo "$SEARCH_TEXT" | grep -q "    high: 3"; then
    fail "search text summary should include high priority count"
  fi
  if ! echo "$SEARCH_TEXT" | grep -q "    Platform Team: 3"; then
    fail "search text summary should include owner count"
  fi
  if ! echo "$SEARCH_TEXT" | grep -q "    unassigned: 3"; then
    fail "search text summary should include unassigned owner count"
  fi
fi

echo "Test 3: Governance metadata filters use effective values"
assert_search_names() {
  local args="$1"
  local expected_names="$2"
  local description="$3"

  set +e
  FILTER_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json $args 2>&1)
  FILTER_EXIT=$?

  if [ $FILTER_EXIT -ne 0 ]; then
    fail "$description should succeed. Output: $FILTER_JSON"
    return
  fi

  ACTUAL_NAMES=$(echo "$FILTER_JSON" | jq -r '[.files | to_entries[] | .value.elements[].name] | sort | join("|")')
  assert_eq "$expected_names" "$ACTUAL_NAMES" "$description"
}

assert_search_rejects() {
  local args="$1"
  local expected_pattern="$2"
  local description="$3"

  set +e
  OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search $args 2>&1)
  EXIT_CODE=$?

  if [ $EXIT_CODE -eq 0 ]; then
    fail "$description should fail"
  elif ! echo "$OUTPUT" | grep -Eiq "$expected_pattern"; then
    fail "$description should mention '$expected_pattern'. Output: $OUTPUT"
  fi
}

assert_search_names "--filter-status=approved" "Child Inherits Governance|Default Governance Root|Independent Requirement|Root Requirement|Test Capability Test Requirement Governance Metadata Specifications Requirements Md" "filter by inherited and default status"
assert_search_names "--filter-status=review" "Child Overrides Governance" "filter by explicit review status"
assert_search_names "--filter-priority=high" "Child Inherits Governance|Child Overrides Governance|Root Requirement" "filter by inherited priority"
assert_search_names "--filter-risk=critical" "Child Overrides Governance" "filter by explicit critical risk"
assert_search_names "--filter-owner=Platform.*" "Child Inherits Governance|Child Overrides Governance|Root Requirement" "filter by inherited owner regex"
assert_search_rejects "--filter-status=blocked" "status|draft|review|approved" "invalid status filter"
assert_search_rejects "--filter-priority=urgent" "priority|low|medium|high|critical" "invalid priority filter"
assert_search_rejects "--filter-risk=severe" "risk|low|medium|high|critical" "invalid risk filter"

echo "Test 4: Invalid enum values are rejected"
assert_validation_rejects_file \
  "$TEST_DIR/specifications/InvalidGovernance.md" \
  '# Elements

### Invalid Status Requirement

This requirement uses an invalid governance status.

#### Metadata
  * type: requirement
  * status: blocked

#### Relations
  * derivedFrom: [Root Requirement](Requirements.md#root-requirement)
---' \
  'status|draft|review|approved' \
  "invalid status"

assert_validation_rejects_file \
  "$TEST_DIR/specifications/InvalidGovernance.md" \
  '# Elements

### Invalid Priority Requirement

This requirement uses an invalid governance priority.

#### Metadata
  * type: requirement
  * priority: urgent

#### Relations
  * derivedFrom: [Root Requirement](Requirements.md#root-requirement)
---' \
  'priority|low|medium|high|critical' \
  "invalid priority"

assert_validation_rejects_file \
  "$TEST_DIR/specifications/InvalidGovernance.md" \
  '# Elements

### Invalid Risk Requirement

This requirement uses an invalid governance risk.

#### Metadata
  * type: requirement
  * risk: severe

#### Relations
  * derivedFrom: [Root Requirement](Requirements.md#root-requirement)
---' \
  'risk|low|medium|high|critical' \
  "invalid risk"

echo "Test 5: Non-requirement elements cannot author governance metadata"
assert_validation_rejects_file \
  "$TEST_DIR/specifications/InvalidContractGovernance.md" \
  '# Elements

### Invalid Contract Governance

This contract illegally declares governance metadata.

#### Metadata
  * type: specification
  * owner: Platform Team

#### Relations
  * define: [Root Requirement](Requirements.md#root-requirement)
---' \
  'governance|owner|requirement' \
  "contract governance metadata"

assert_validation_rejects_file \
  "$TEST_DIR/specifications/InvalidVerificationGovernance.md" \
  '# Elements

### Invalid Verification Governance

This verification illegally declares requirement governance metadata.

#### Metadata
  * type: test-verification
  * priority: high

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Root Requirement](Requirements.md#root-requirement)
---' \
  'governance|priority|requirement' \
  "verification governance metadata"

echo "Test 6: Formatting does not persist inherited/default governance metadata"
set +e
FORMAT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" format --fix 2>&1)
FORMAT_EXIT=$?

if [ $FORMAT_EXIT -ne 0 ]; then
  fail "format --fix should succeed. Output: $FORMAT_OUTPUT"
else
  CHILD_BLOCK=$(awk '/^### Child Inherits Governance$/,/^---$/' "$TEST_DIR/specifications/Requirements.md")
  if echo "$CHILD_BLOCK" | grep -Eq '  \* (status|priority|risk|owner):'; then
    fail "format should not insert inherited governance metadata into child requirement"
  fi

  INDEPENDENT_BLOCK=$(awk '/^### Independent Requirement$/,/^---$/' "$TEST_DIR/specifications/Requirements.md")
  if echo "$INDEPENDENT_BLOCK" | grep -Eq '  \* (status|priority|risk|owner):'; then
    fail "format should not insert default governance metadata into requirement"
  fi

  CONTRACT_BLOCK=$(awk '/^### Contract$/,/^---$/' "$TEST_DIR/specifications/Requirements.md")
  if echo "$CONTRACT_BLOCK" | grep -Eq '  \* (status|priority|risk|owner):'; then
    fail "format should not insert governance metadata into contract element"
  fi
fi

if [ $FAILURES -ne 0 ]; then
  echo "$FAILURES governance metadata check(s) failed"
  exit 1
fi

echo "Requirement governance metadata tests passed"
exit 0
