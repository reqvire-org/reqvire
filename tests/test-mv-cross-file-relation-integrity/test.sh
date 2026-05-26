#!/bin/bash
set -uo pipefail

# Test: Cross-File Relation Integrity After mv
# -----------------------------------------------
# Regression test for update_relation_identifiers bug (graph_registry.rs line 2538):
#
# BUG: When element A moves to the same file as element B (which has a
# derivedFrom relation to A), the relation identifier is not updated.
# Part 1 of update_relation_identifiers detects that both elements are
# now in the same file and does "keep as-is" - but "as-is" means the
# OLD cross-file path (e.g. ../Requirements.md#capability-a) which is now
# stale. The relation should become a same-file fragment (#capability-a).
#
# Additionally, Part 2 of update_relation_identifiers fails to update
# the moved element's own outgoing relations because it looks up bare
# fragments against full-identifier keys in target_file_paths.
#
# Acceptance Criteria:
# - After mv: cross-file relations pointing to moved element are updated
# - After mv: same-file relations are converted to fragment-only references
# - After mv: moved element's own outgoing cross-file relations are correct
# - Model validates after each mv operation (no missing relation targets)
#
# Test Criteria:
# - mv command exits with success (0)
# - File contents match expected output (diff comparison)
# - Model validation passes after the move

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# ==================================
# Test 1: Move element into same file as referring element
# ==================================
# Setup:
#   - Capability A is in specifications/Requirements.md
#   - Sub Capability is in specifications/SubDir/SubRequirements.md
#   - Sub Capability has derivedFrom: ../Requirements.md#capability-a (cross-file)
#
# Action: Move Capability A to specifications/SubDir/SubRequirements.md
#
# Expected: Sub Capability's derivedFrom should become #capability-a (same-file)
#           because both elements are now in the same file.
#
# BUG: The relation stays as ../Requirements.md#capability-a (stale cross-file ref)
#      causing "Missing relation target" validation error.

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Capability A" "specifications/SubDir/SubRequirements.md" 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: mv command failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Check file contents match expected
assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-move-requirements.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "Requirements.md after moving Capability A out"

assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-move-subrequirements.md" \
  "$TEST_DIR/specifications/SubDir/SubRequirements.md" \
  "SubRequirements.md after Capability A moved in (cross-file ref should become same-file)"

assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-move-tests.md" \
  "$TEST_DIR/specifications/Verifications/Tests.md" \
  "Tests.md verify relation should point to new location"

# CRITICAL: Model must validate - no missing relation targets
set +e
VALIDATE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATE_EXIT=$?
set -e

if [ $VALIDATE_EXIT -ne 0 ]; then
  echo "FAILED: Model validation failed after mv"
  echo "$VALIDATE_OUTPUT"
  exit 1
fi

exit 0
