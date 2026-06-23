#!/bin/bash
set -uo pipefail

# Test: ContractBindingEntry Scope Constraints
# -------------------------------------------------------------------
# Satisfies:
#   - system-model/Functional/Core/Verifications/ContractBindingVerifications.md#contract_bindings-scope-constraints-test
#
# Acceptance Criteria:
# - Orphan contract (no define relations) contract_bindings causes validation to fail
# - Defining requirement (has definedBy) cannot also bind the contract
# - Descendant of defining requirement cannot bind the contract
# - Ancestor of defining requirement cannot bind the contract
# - Cross-subgraph contract_bindings flow is one-directional at capability-root hierarchy level
# - Requirements in separate hierarchies can bind the contract
# - Link command enforces same constraints
#
# Test Model Structure:
#
# Spec-1 defines User Req A:
#   User Req A (defining requirement - has definedBy: Spec-1)
#   ├── Req B (child - cannot bind Spec-1)
#   │   └── Req C (grandchild - cannot bind Spec-1)
#   │       └── Req C1 (great-grandchild - cannot bind Spec-1)
#   └── Req D (sibling branch - cannot bind Spec-1)
#
#   User Req X (separate branch - CAN reuse Spec-1) ✓
#   User Req Y → Req Y1 (separate branch - CAN reuse Spec-1) ✓
#
# Spec-2 defines Child With Contract:
#   Ancestor Req (ANCESTOR - cannot bind Spec-2)
#   └── Child With Contract (defining - has definedBy: Spec-2)
#       └── Grandchild Req (descendant - cannot bind Spec-2)
#
# Directionality case:
#   User Req X reuses Spec-1 owned by User Req A hierarchy
#   => User Req X hierarchy must not receive contract_bindings to contracts it defines from User Req A hierarchy

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
# Base model has User Req X and Req Y1 (separate branches) bindContract Spec-1

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
# Test 2: Orphan Contract ContractBindingEntry Fails
# ==================================
# Add contract_bindings to orphan contract (no define relations)

cat > "$TEST_DIR/specifications/TestOrphanContractBinding.md" <<'EOF'
# Elements

### Req Reusing Contract Orphan

A requirement that tries to reuse an orphan contract.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Validation should fail for orphan contract contract_bindings"
  exit 1
fi

# Sanitize output
SANITIZED_ORPHAN=$(echo "$ORPHAN_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/orphan-contract-binding-error.txt" \
  "$SANITIZED_ORPHAN" \
  "Orphan contract contract_bindings error message does not match expected"

rm -f "$TEST_DIR/specifications/TestOrphanContractBinding.md"

# ==================================
# Test 3: Defining Requirement ContractBindingEntry Fails (same as definedBy)
# ==================================
# User Req A has definedBy: Spec-1, so it cannot ALSO reuse Spec-1

cat > "$TEST_DIR/specifications/TestDefiningContractBinding.md" <<'EOF'
# Elements

### User Req A With ContractBindingEntry

User requirement that has definedBy AND contract_bindings to same contract.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Validation should fail for defining requirement contract_bindings"
  exit 1
fi

SANITIZED_DEFINING=$(echo "$DEFINING_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/defining-contract-binding-error.txt" \
  "$SANITIZED_DEFINING" \
  "Defining requirement contract_bindings error message does not match expected"

rm -f "$TEST_DIR/specifications/TestDefiningContractBinding.md"

# ==================================
# Test 4: Child Requirement ContractBindingEntry Fails (descendant)
# ==================================
# Req B is child of User Req A, so cannot bind Spec-1

cat > "$TEST_DIR/specifications/TestChildContractBinding.md" <<'EOF'
# Elements

### Child Reusing Contract Spec

Child of defining requirement tries to reuse the contract.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Validation should fail for child requirement contract_bindings"
  exit 1
fi

SANITIZED_CHILD=$(echo "$CHILD_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/child-contract-binding-error.txt" \
  "$SANITIZED_CHILD" \
  "Child contract_bindings error message does not match expected"

rm -f "$TEST_DIR/specifications/TestChildContractBinding.md"

# ==================================
# Test 5: Grandchild Requirement ContractBindingEntry Fails (deep descendant)
# ==================================
# Req C is grandchild of User Req A, so cannot bind Spec-1

cat > "$TEST_DIR/specifications/TestGrandchildContractBinding.md" <<'EOF'
# Elements

### Grandchild Reusing Contract Spec

Grandchild of defining requirement tries to reuse the contract.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Validation should fail for grandchild requirement contract_bindings"
  exit 1
fi

SANITIZED_GRANDCHILD=$(echo "$GRANDCHILD_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/grandchild-contract-binding-error.txt" \
  "$SANITIZED_GRANDCHILD" \
  "Grandchild contract_bindings error message does not match expected"

rm -f "$TEST_DIR/specifications/TestGrandchildContractBinding.md"

# ==================================
# Test 6: Ancestor Requirement ContractBindingEntry Fails
# ==================================
# Ancestor Req is parent of Child With Contract, so cannot bind Spec-2

cat > "$TEST_DIR/specifications/TestAncestorContractBinding.md" <<'EOF'
# Elements

### Ancestor Reusing Contract Spec

Ancestor of defining requirement tries to reuse the contract.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Validation should fail for ancestor requirement contract_bindings"
  exit 1
fi

SANITIZED_ANCESTOR=$(echo "$ANCESTOR_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/ancestor-contract-binding-error.txt" \
  "$SANITIZED_ANCESTOR" \
  "Ancestor contract_bindings error message does not match expected"

rm -f "$TEST_DIR/specifications/TestAncestorContractBinding.md"

# ==================================
# Test 7: Reverse Direction ContractBindingEntry Fails
# ==================================

cat > "$TEST_DIR/specifications/TestReverseDirectionContractBinding.md" <<'EOF'
# Elements

### Req D Reverse Flow ContractBindingEntry

Requirement in User Req A hierarchy tries to bind a contract owned by User Req X.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Validation should fail for reverse-direction subgraph contract_bindings flow"
  exit 1
fi

SANITIZED_REVERSE_DIRECTION=$(echo "$REVERSE_DIRECTION_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/reverse-direction-contract-binding-error.txt" \
  "$SANITIZED_REVERSE_DIRECTION" \
  "Reverse-direction contract_bindings error message does not match expected"

rm -f "$TEST_DIR/specifications/TestReverseDirectionContractBinding.md"

# ==================================
# Test 8: Link Command Enforces Direction Constraint
# ==================================

set +e
LINK_DIRECTION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Req D" bindContract "Contracts.md#spec-x" 2>&1)
LINK_DIRECTION_EXIT=$?
set -e

if [ $LINK_DIRECTION_EXIT -eq 0 ]; then
  echo "❌ FAILED: Link command should fail for reverse-direction subgraph contract_bindings"
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

Source element with contract_bindings to Spec-X.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Merge command should fail when merged contract_bindings would reverse subgraph flow"
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
LINK_ORPHAN_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "User Req X" bindContract "Contracts.md#orphan-spec" 2>&1)
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
LINK_HIERARCHY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" link "Req B" bindContract "Contracts.md#spec-1" 2>&1)
LINK_HIERARCHY_EXIT=$?
set -e

if [ $LINK_HIERARCHY_EXIT -eq 0 ]; then
  echo "❌ FAILED: Link command should fail for same hierarchy contract_bindings"
  exit 1
fi

SANITIZED_LINK_HIERARCHY=$(echo "$LINK_HIERARCHY_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g' | sed 's|\[.*ERROR reqvire\] |error: |g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/link-hierarchy-error.txt" \
  "$SANITIZED_LINK_HIERARCHY" \
  "Link same hierarchy error message does not match expected"

# ==================================
# Test 12: Merge Command Enforces Hierarchy Constraint
# ==================================
# Create a source element with contract_bindings that would violate hierarchy for target

cat > "$TEST_DIR/specifications/TestMergeSource.md" <<'EOF'
# Elements

### Merge Source Element

Source element with contract_bindings to Spec-1.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Merge command should fail when source has contract_bindings violating hierarchy"
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
# Create a source element with contract_bindings to orphan contract

cat > "$TEST_DIR/specifications/TestMergeOrphan.md" <<'EOF'
# Elements

### Merge Orphan Source

Source element with contract_bindings to orphan contract.

#### Metadata
  * type: requirement

#### Contract Bindings
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
  echo "❌ FAILED: Merge command should fail when source has orphan contract contract_bindings"
  exit 1
fi

SANITIZED_MERGE_ORPHAN=$(echo "$MERGE_ORPHAN_OUTPUT" | sed "s|${TEST_DIR}/||g" | sed 's|/tmp/reqvire-test-[^/]*/||g' | sed 's|\[.*ERROR reqvire\] |error: |g')

assert_output_matches "${TEST_SCRIPT_DIR}/expected/merge-orphan-error.txt" \
  "$SANITIZED_MERGE_ORPHAN" \
  "Merge orphan constraint error message does not match expected"

rm -f "$TEST_DIR/specifications/TestMergeOrphan.md"

exit 0
