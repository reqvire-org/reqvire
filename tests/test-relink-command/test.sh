#!/bin/bash
set -euo pipefail

echo "===================================="
echo "Relink Command Tests"
echo "===================================="

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: $description"
    exit 1
  fi
}

assert_output_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: $description"
    exit 1
  fi
}

mkdir -p "$TEST_DIR/output"

# Test 1: dry-run should show deterministic summary and keep files unchanged
set +e
DRY_RUN_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" relink "Child Requirement" derivedFrom "Old Parent" "New Parent" --dry-run 2>&1)
DRY_RUN_EXIT=$?
set -e

if [ $DRY_RUN_EXIT -ne 0 ]; then
  echo "$DRY_RUN_OUTPUT"
  echo "FAILED: relink --dry-run should succeed"
  exit 1
fi

printf "%s\n" "$DRY_RUN_OUTPUT" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | grep -E "^(Updated element:|Dry run - no files modified)" \
  > "$TEST_DIR/output/01-dry-run-summary.actual.txt"

assert_output_matches \
  "$TEST_SCRIPT_DIR/expected/01-dry-run-summary.txt" \
  "$TEST_DIR/output/01-dry-run-summary.actual.txt" \
  "Dry-run summary output mismatch"

assert_file_matches \
  "$TEST_SCRIPT_DIR/specifications/Requirements.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "Dry-run should not modify model file"

# Test 2: apply relink should update relation target
cd "$TEST_DIR" && "$REQVIRE_BIN" relink "Child Requirement" derivedFrom "Old Parent" "New Parent" > /dev/null 2>&1

assert_file_matches \
  "$TEST_SCRIPT_DIR/expected/02-after-relink.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "Relinked file content does not match expected"

# Test 3: relation-type mismatch must fail with deterministic error
set +e
MISMATCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" relink "Child Requirement" verify "New Parent" "Old Parent" 2>&1)
MISMATCH_EXIT=$?
set -e

if [ $MISMATCH_EXIT -eq 0 ]; then
  echo "FAILED: relink with relation-type mismatch should fail"
  exit 1
fi

printf "%s\n" "$MISMATCH_OUTPUT" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | sed -E 's/^\[[^]]+\][[:space:]]*//' \
  | grep -E "Relation mismatch:" \
  | tail -n 1 \
  > "$TEST_DIR/output/03-relation-mismatch.actual.txt"

assert_output_matches \
  "$TEST_SCRIPT_DIR/expected/03-relation-mismatch.txt" \
  "$TEST_DIR/output/03-relation-mismatch.actual.txt" \
  "Mismatch error output differs from expected"

# Test 4: relink that creates multi-root ownership must fail and keep files unchanged
cat > "$TEST_DIR/specifications/Requirements.md" << 'EOF'
# Elements

### Root A

#### Metadata
  * type: user-requirement
---

### Root B

#### Metadata
  * type: user-requirement
---

### Parent A

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root A](#root-a)
---

### Parent B

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root B](#root-b)
---

### Parent A2

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root A](#root-a)
---

### Child

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent A](#parent-a)
  * derivedFrom: [Parent A2](#parent-a2)
---
EOF

BEFORE_MULTIROOT="$(mktemp /tmp/reqvire-relink-before4-XXXXXX.md)"
cp "$TEST_DIR/specifications/Requirements.md" "$BEFORE_MULTIROOT"

set +e
MULTIROOT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" relink "Child" derivedFrom "Parent A2" "Parent B" 2>&1)
MULTIROOT_EXIT=$?
set -e

if [ $MULTIROOT_EXIT -eq 0 ]; then
  echo "FAILED: relink should fail deterministically with single-root ownership violation"
  exit 1
fi

printf "%s\n" "$MULTIROOT_OUTPUT" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | sed -E 's/^\[[^]]+\][[:space:]]*//' \
  | grep -E "Single-root hierarchy ownership violation" \
  > "$TEST_DIR/output/04-relink-multiroot-error.actual.txt"

assert_output_matches \
  "$TEST_SCRIPT_DIR/expected/04-relink-multiroot-error.txt" \
  "$TEST_DIR/output/04-relink-multiroot-error.actual.txt" \
  "Deterministic single-root error output mismatch for relink"

assert_file_matches \
  "$BEFORE_MULTIROOT" \
  "$TEST_DIR/specifications/Requirements.md" \
  "Failed relink must not modify file"

rm -f "$BEFORE_MULTIROOT"

echo "All relink command tests passed"
