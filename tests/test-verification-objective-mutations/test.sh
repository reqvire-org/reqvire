#!/bin/bash
set -uo pipefail

# Test: Verification objective mutations
#
# Proves verification-objective participates in verification-family hierarchy
# mutations while remaining separate from concrete verification semantics.

assert_success() {
  local description="$1"
  shift

  set +e
  OUTPUT=$("$@" 2>&1)
  EXIT_CODE=$?
  set -e

  if [ "$EXIT_CODE" -ne 0 ]; then
    echo "FAILED: $description"
    echo "$OUTPUT"
    exit 1
  fi
}

assert_failure_contains() {
  local description="$1"
  local expected="$2"
  shift 2

  set +e
  OUTPUT=$("$@" 2>&1)
  EXIT_CODE=$?
  set -e

  if [ "$EXIT_CODE" -eq 0 ]; then
    echo "FAILED: $description should fail"
    exit 1
  fi

  if ! echo "$OUTPUT" | grep -qi "$expected"; then
    echo "FAILED: $description should mention '$expected'"
    echo "$OUTPUT"
    exit 1
  fi
}

assert_file_contains() {
  local file="$1"
  local expected="$2"
  local description="$3"

  if ! grep -Fq "$expected" "$file"; then
    echo "FAILED: $description"
    echo "Expected to find: $expected"
    echo "In file: $file"
    sed -n '1,220p' "$file"
    exit 1
  fi
}

assert_file_not_contains() {
  local file="$1"
  local unexpected="$2"
  local description="$3"

  if grep -Fq "$unexpected" "$file"; then
    echo "FAILED: $description"
    echo "Unexpectedly found: $unexpected"
    echo "In file: $file"
    sed -n '1,220p' "$file"
    exit 1
  fi
}

cd "$TEST_DIR" || exit 1

echo "Test 1: Initial model validates"
assert_success "initial validation" "$REQVIRE_BIN" validate

echo "Test 2: Link objective hierarchy"
assert_success "link objective hierarchy" "$REQVIRE_BIN" link "Second Verification Objective" derivedFrom "Verification Objective"
assert_success "validation after objective link" "$REQVIRE_BIN" validate
assert_file_contains \
  "$TEST_DIR/specifications/VerificationModel.md" \
  "  * derivedFrom: [Verification Objective](#verification-objective)" \
  "linked objective hierarchy should be written"

echo "Test 3: Unlink objective hierarchy"
assert_success "unlink objective hierarchy" "$REQVIRE_BIN" unlink "Second Verification Objective" "Verification Objective"
assert_success "validation after objective unlink" "$REQVIRE_BIN" validate

SECOND_BLOCK=$(sed -n '/### Second Verification Objective/,/---/p' "$TEST_DIR/specifications/VerificationModel.md")
if echo "$SECOND_BLOCK" | grep -Fq "derivedFrom"; then
  echo "FAILED: objective unlink should remove derivedFrom from second objective"
  echo "$SECOND_BLOCK"
  exit 1
fi

echo "Test 4: Relink concrete verification from one objective to another"
assert_success \
  "relink concrete verification objective parent" \
  "$REQVIRE_BIN" relink "Concrete Verification" derivedFrom "Verification Objective" "Second Verification Objective"
assert_success "validation after concrete relink" "$REQVIRE_BIN" validate
assert_file_contains \
  "$TEST_DIR/specifications/VerificationModel.md" \
  "  * derivedFrom: [Second Verification Objective](#second-verification-objective)" \
  "concrete verification should point to second objective"

echo "Test 5: Move objective and update incoming concrete verification relation"
assert_success "move objective to new file" "$REQVIRE_BIN" mv "Second Verification Objective" "specifications/VerificationPlans.md"
assert_success "validation after objective move" "$REQVIRE_BIN" validate
assert_file_contains \
  "$TEST_DIR/specifications/VerificationModel.md" \
  "  * derivedFrom: [Second Verification Objective](VerificationPlans.md#second-verification-objective)" \
  "incoming concrete verification relation should be updated after objective move"
assert_file_contains \
  "$TEST_DIR/specifications/VerificationPlans.md" \
  "### Second Verification Objective" \
  "moved objective should exist in target file"

echo "Test 6: Merge objective into objective succeeds"
assert_success "merge objective into objective" "$REQVIRE_BIN" merge "Objective Merge Target" "Objective Merge Source"
assert_success "validation after objective merge" "$REQVIRE_BIN" validate
SEARCH_OUTPUT=$("$REQVIRE_BIN" search --filter-name="Objective Merge Source" --short 2>&1)
if echo "$SEARCH_OUTPUT" | grep -Fq "Objective Merge Source"; then
  echo "FAILED: merged objective source should be removed"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

echo "Test 7: Merge objective with concrete verification fails"
assert_failure_contains \
  "merge objective with concrete verification" \
  "type mismatch" \
  "$REQVIRE_BIN" merge "Objective Merge Target" "Concrete Merge Candidate"
assert_success "validation after failed mixed merge" "$REQVIRE_BIN" validate

echo "Test 8: Objective cannot verify requirement"
assert_failure_contains \
  "objective verify relation" \
  "verification-objective" \
  "$REQVIRE_BIN" link "Objective Merge Target" verify "System Requirement"
assert_success "validation after failed objective verify" "$REQVIRE_BIN" validate

echo "Test 9: Requirement cannot be verifiedBy objective"
assert_failure_contains \
  "requirement verifiedBy objective" \
  "verification-objective" \
  "$REQVIRE_BIN" link "System Requirement" verifiedBy "Objective Merge Target"
assert_success "validation after failed verifiedBy objective" "$REQVIRE_BIN" validate

echo "Test 10: Objective cannot carry evidence"
assert_failure_contains \
  "objective satisfiedBy evidence" \
  "verification-objective" \
  "$REQVIRE_BIN" link "Objective Merge Target" satisfiedBy "specifications/test.sh"
assert_success "validation after failed objective evidence link" "$REQVIRE_BIN" validate

echo "Test 11: Dry-run move does not mutate objective file"
BACKUP_DIR="/tmp/reqvire-verification-objective-mutations-$$"
mkdir -p "$BACKUP_DIR"
cp "$TEST_DIR/specifications/VerificationPlans.md" "$BACKUP_DIR/VerificationPlans.before.md"
assert_success "dry-run objective move" "$REQVIRE_BIN" mv "Second Verification Objective" "specifications/DryRunTarget.md" --dry-run
if ! cmp -s "$TEST_DIR/specifications/VerificationPlans.md" "$BACKUP_DIR/VerificationPlans.before.md"; then
  echo "FAILED: dry-run objective move should not mutate source file"
  diff -u "$BACKUP_DIR/VerificationPlans.before.md" "$TEST_DIR/specifications/VerificationPlans.md"
  rm -rf "$BACKUP_DIR"
  exit 1
fi

if [ -f "$TEST_DIR/specifications/DryRunTarget.md" ]; then
  echo "FAILED: dry-run objective move should not create target file"
  rm -rf "$BACKUP_DIR"
  exit 1
fi

rm -rf "$BACKUP_DIR"

echo "All verification objective mutation tests passed"
