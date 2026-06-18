#!/bin/bash
set -uo pipefail

# Test: ReusedContractContextEntry Scope Constraints
# -------------------------------------------------------------------
# Satisfies:
#   - system-model/Functional/Core/Verifications/ReusedContractContextVerifications.md#reused_contract_context-scope-constraints-test
#
# Acceptance Criteria:
# - Orphan contract (no define relations) reused_contract_context causes validation to fail
# - Defining requirement (has definedBy) cannot also reuse the contract
# - Descendant of defining requirement cannot reuse the contract
# - Ancestor of defining requirement cannot reuse the contract
# - Cross-subgraph reused_contract_context flow is one-directional at capability-root hierarchy level
# - Requirements in separate hierarchies can reuse the contract
# - Link command enforces same constraints
#
# Test Model Structure:
#
# Spec-1 defines User Req A:
#   User Req A (defining requirement - has definedBy: Spec-1)
#   ├── Req B (child - cannot reuse Spec-1)
#   │   └── Req C (grandchild - cannot reuse Spec-1)
#   │       └── Req C1 (great-grandchild - cannot reuse Spec-1)
#   └── Req D (sibling branch - cannot reuse Spec-1)
#
#   User Req X (separate branch - CAN reuse Spec-1) ✓
#   User Req Y → Req Y1 (separate branch - CAN reuse Spec-1) ✓
#
# Spec-2 defines Child With Contract:
#   Ancestor Req (ANCESTOR - cannot reuse Spec-2)
#   └── Child With Contract (defining - has definedBy: Spec-2)
#       └── Grandchild Req (descendant - cannot reuse Spec-2)
#
# Directionality case:
#   User Req X reuses Spec-1 owned by User Req A hierarchy
#   => User Req X hierarchy must not receive reused_contract_context to contracts it defines from User Req A hierarchy

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Helper function to compare output and show diff on failure
assert_output_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" <(echo "$actual"); then
    echo "❌ FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# ==================================
# Test 1: Valid Model Passes
# ==================================
# Base model has User Req X and Req Y1 (separate branches) reusesContract Spec-1

set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "❌ FAILED: Base model validation should pass"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

# ==================================
# Test 2: Orphan Contract ReusedContractContextEntry Fails
# ==================================
# Add reused_contract_context to orphan contract (no define relations)

cat > "$TEST_DIR/specifications/TestOrphanReusedContractContext.md" <<'EOF'
# Elements

### Req Reusing Contract Orphan

A requirement that tries to reuse an orphan contract.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Orphan-Spec](Contracts.md#orphan-spec)

#### Relations
  * specify: [User Req X Capability](Requirements.md#user-req-x-capability)
---

EOF

set +e
ORPHAN_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
ORPHAN_EXIT=$?
set -e

if [ $ORPHAN_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for orphan contract reused_contract_context"
  exit 1
fi

# Sanitize output
SANITIZED_ORPHAN=$(echo "$ORPHAN_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/orphan-reused-context-error.txt" \
  "$SANITIZED_ORPHAN" \
  "Orphan contract reused_contract_context error message does not match expected"

rm -f "$TEST_DIR/specifications/TestOrphanReusedContractContext.md"

# ==================================
# Test 3: Defining Requirement ReusedContractContextEntry Fails (same as definedBy)
# ==================================
# User Req A has definedBy: Spec-1, so it cannot ALSO reuse Spec-1

cat > "$TEST_DIR/specifications/TestDefiningReusedContractContext.md" <<'EOF'
# Elements

### User Req A With ReusedContractContextEntry

User requirement that has definedBy AND reused_contract_context to same contract.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Spec-1](Contracts.md#spec-1)

#### Relations
  * specify: [User Req A Capability](Requirements.md#user-req-a-capability)
  * definedBy: [Spec-1](Contracts.md#spec-1)
---

EOF

set +e
DEFINING_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
DEFINING_EXIT=$?
set -e

if [ $DEFINING_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for defining requirement reused_contract_context"
  exit 1
fi

SANITIZED_DEFINING=$(echo "$DEFINING_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/defining-reused-context-error.txt" \
  "$SANITIZED_DEFINING" \
  "Defining requirement reused_contract_context error message does not match expected"

rm -f "$TEST_DIR/specifications/TestDefiningReusedContractContext.md"

# ==================================
# Test 4: Child Requirement ReusedContractContextEntry Fails (descendant)
# ==================================
# Req B is child of User Req A, so cannot reuse Spec-1

cat > "$TEST_DIR/specifications/TestChildReusedContractContext.md" <<'EOF'
# Elements

### Child Reusing Contract Spec

Child of defining requirement tries to reuse the contract.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Spec-1](Contracts.md#spec-1)

#### Relations
  * derivedFrom: [Req B](Requirements.md#req-b)
---

EOF

set +e
CHILD_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
CHILD_EXIT=$?
set -e

if [ $CHILD_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for child requirement reused_contract_context"
  exit 1
fi

SANITIZED_CHILD=$(echo "$CHILD_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/child-reused-context-error.txt" \
  "$SANITIZED_CHILD" \
  "Child reused_contract_context error message does not match expected"

rm -f "$TEST_DIR/specifications/TestChildReusedContractContext.md"

# ==================================
# Test 5: Grandchild Requirement ReusedContractContextEntry Fails (deep descendant)
# ==================================
# Req C is grandchild of User Req A, so cannot reuse Spec-1

cat > "$TEST_DIR/specifications/TestGrandchildReusedContractContext.md" <<'EOF'
# Elements

### Grandchild Reusing Contract Spec

Grandchild of defining requirement tries to reuse the contract.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Spec-1](Contracts.md#spec-1)

#### Relations
  * derivedFrom: [Req C](Requirements.md#req-c)
---

EOF

set +e
GRANDCHILD_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
GRANDCHILD_EXIT=$?
set -e

if [ $GRANDCHILD_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for grandchild requirement reused_contract_context"
  exit 1
fi

SANITIZED_GRANDCHILD=$(echo "$GRANDCHILD_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/grandchild-reused-context-error.txt" \
  "$SANITIZED_GRANDCHILD" \
  "Grandchild reused_contract_context error message does not match expected"

rm -f "$TEST_DIR/specifications/TestGrandchildReusedContractContext.md"

# ==================================
# Test 6: Ancestor Requirement ReusedContractContextEntry Fails
# ==================================
# Ancestor Req is parent of Child With Contract, so cannot reuse Spec-2

cat > "$TEST_DIR/specifications/TestAncestorReusedContractContext.md" <<'EOF'
# Elements

### Ancestor Reusing Contract Spec

Ancestor of defining requirement tries to reuse the contract.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Spec-2](Contracts.md#spec-2)

#### Relations
  * specify: [Ancestor Req Capability](Requirements.md#ancestor-req-capability)
  * derive: [Ancestor Req](Requirements.md#ancestor-req)
---

EOF

set +e
ANCESTOR_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
ANCESTOR_EXIT=$?
set -e

if [ $ANCESTOR_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for ancestor requirement reused_contract_context"
  exit 1
fi

SANITIZED_ANCESTOR=$(echo "$ANCESTOR_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/ancestor-reused-context-error.txt" \
  "$SANITIZED_ANCESTOR" \
  "Ancestor reused_contract_context error message does not match expected"

rm -f "$TEST_DIR/specifications/TestAncestorReusedContractContext.md"

# ==================================
# Test 7: Reverse Direction ReusedContractContextEntry Fails
# ==================================

cat > "$TEST_DIR/specifications/TestReverseDirectionReusedContractContext.md" <<'EOF'
# Elements

### Req D Reverse Flow ReusedContractContextEntry

Requirement in User Req A hierarchy tries to reuse a contract owned by User Req X.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Spec-X](Contracts.md#spec-x)

#### Relations
  * derivedFrom: [Req D](Requirements.md#req-d)
---

EOF

set +e
REVERSE_DIRECTION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
REVERSE_DIRECTION_EXIT=$?
set -e

if [ $REVERSE_DIRECTION_EXIT -eq 0 ]; then
  echo "❌ FAILED: Validation should fail for reverse-direction subgraph reused_contract_context flow"
  exit 1
fi

SANITIZED_REVERSE_DIRECTION=$(echo "$REVERSE_DIRECTION_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/reverse-direction-reused-context-error.txt" \
  "$SANITIZED_REVERSE_DIRECTION" \
  "Reverse-direction reused_contract_context error message does not match expected"

rm -f "$TEST_DIR/specifications/TestReverseDirectionReusedContractContext.md"

# ==================================
# Test 8: Link Command Enforces Direction Constraint
# ==================================

set +e
LINK_DIRECTION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Req D" reusesContract "Contracts.md#spec-x" 2>&1)
LINK_DIRECTION_EXIT=$?
set -e

if [ $LINK_DIRECTION_EXIT -eq 0 ]; then
  echo "❌ FAILED: Link command should fail for reverse-direction subgraph reused_contract_context"
  exit 1
fi

SANITIZED_LINK_DIRECTION=$(echo "$LINK_DIRECTION_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g' | sed 's|\[.*ERROR reqvire\] |error: |g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/link-direction-error.txt" \
  "$SANITIZED_LINK_DIRECTION" \
  "Link direction constraint error message does not match expected"

# ==================================
# Test 9: Merge Command Enforces Direction Constraint
# ==================================

cat > "$TEST_DIR/specifications/TestMergeDirection.md" <<'EOF'
# Elements

### Merge Direction Source

Source element with reused_contract_context to Spec-X.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Spec-X](Contracts.md#spec-x)

#### Relations
  * derivedFrom: [User Req Y](Requirements.md#user-req-y)
---

EOF

set +e
MERGE_DIRECTION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Req D" "Merge Direction Source" 2>&1)
MERGE_DIRECTION_EXIT=$?
set -e

if [ $MERGE_DIRECTION_EXIT -eq 0 ]; then
  echo "❌ FAILED: Merge command should fail when merged reused_contract_context would reverse subgraph flow"
  exit 1
fi

SANITIZED_MERGE_DIRECTION=$(echo "$MERGE_DIRECTION_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g' | sed 's|\[.*ERROR reqvire\] |error: |g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/merge-direction-error.txt" \
  "$SANITIZED_MERGE_DIRECTION" \
  "Merge direction constraint error message does not match expected"

rm -f "$TEST_DIR/specifications/TestMergeDirection.md"

# ==================================
# Test 10: Link Command Enforces Orphan Constraint
# ==================================

set +e
LINK_ORPHAN_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "User Req X" reusesContract "Contracts.md#orphan-spec" 2>&1)
LINK_ORPHAN_EXIT=$?
set -e

if [ $LINK_ORPHAN_EXIT -eq 0 ]; then
  echo "❌ FAILED: Link command should fail for orphan contract"
  exit 1
fi

SANITIZED_LINK_ORPHAN=$(echo "$LINK_ORPHAN_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g' | sed 's|\[.*ERROR reqvire\] |error: |g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/link-orphan-error.txt" \
  "$SANITIZED_LINK_ORPHAN" \
  "Link orphan contract error message does not match expected"

# ==================================
# Test 11: Link Command Enforces Hierarchy Constraint
# ==================================

set +e
LINK_HIERARCHY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Req B" reusesContract "Contracts.md#spec-1" 2>&1)
LINK_HIERARCHY_EXIT=$?
set -e

if [ $LINK_HIERARCHY_EXIT -eq 0 ]; then
  echo "❌ FAILED: Link command should fail for same hierarchy reused_contract_context"
  exit 1
fi

SANITIZED_LINK_HIERARCHY=$(echo "$LINK_HIERARCHY_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g' | sed 's|\[.*ERROR reqvire\] |error: |g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/link-hierarchy-error.txt" \
  "$SANITIZED_LINK_HIERARCHY" \
  "Link same hierarchy error message does not match expected"

# ==================================
# Test 12: Merge Command Enforces Hierarchy Constraint
# ==================================
# Create a source element with reused_contract_context that would violate hierarchy for target

cat > "$TEST_DIR/specifications/TestMergeSource.md" <<'EOF'
# Elements

### Merge Source Element

Source element with reused_contract_context to Spec-1.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Spec-1](Contracts.md#spec-1)

#### Relations
  * derivedFrom: [User Req Y](Requirements.md#user-req-y)
---

EOF

set +e
MERGE_HIERARCHY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "Req B" "Merge Source Element" 2>&1)
MERGE_HIERARCHY_EXIT=$?
set -e

if [ $MERGE_HIERARCHY_EXIT -eq 0 ]; then
  echo "❌ FAILED: Merge command should fail when source has reused_contract_context violating hierarchy"
  exit 1
fi

SANITIZED_MERGE_HIERARCHY=$(echo "$MERGE_HIERARCHY_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g' | sed 's|\[.*ERROR reqvire\] |error: |g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/merge-hierarchy-error.txt" \
  "$SANITIZED_MERGE_HIERARCHY" \
  "Merge hierarchy constraint error message does not match expected"

rm -f "$TEST_DIR/specifications/TestMergeSource.md"

# ==================================
# Test 13: Merge Command Enforces Orphan Constraint
# ==================================
# Create a source element with reused_contract_context to orphan contract

cat > "$TEST_DIR/specifications/TestMergeOrphan.md" <<'EOF'
# Elements

### Merge Orphan Source

Source element with reused_contract_context to orphan contract.

#### Metadata
  * type: requirement

#### Reused Contract Context
  * [Orphan-Spec](Contracts.md#orphan-spec)

#### Relations
  * derivedFrom: [User Req Y](Requirements.md#user-req-y)
---

EOF

set +e
MERGE_ORPHAN_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" merge "User Req X" "Merge Orphan Source" 2>&1)
MERGE_ORPHAN_EXIT=$?
set -e

if [ $MERGE_ORPHAN_EXIT -eq 0 ]; then
  echo "❌ FAILED: Merge command should fail when source has orphan contract reused_contract_context"
  exit 1
fi

SANITIZED_MERGE_ORPHAN=$(echo "$MERGE_ORPHAN_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g' | sed 's|\[.*ERROR reqvire\] |error: |g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/merge-orphan-error.txt" \
  "$SANITIZED_MERGE_ORPHAN" \
  "Merge orphan constraint error message does not match expected"

rm -f "$TEST_DIR/specifications/TestMergeOrphan.md"

exit 0
