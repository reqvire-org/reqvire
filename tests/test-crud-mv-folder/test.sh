#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "===================================="
echo "Move Folder Operation Test"
echo "===================================="
echo ""

fail() {
  echo "❌ FAILED: $1"
  if [ -n "${2:-}" ] && [ -f "$2" ]; then
    cat "$2"
  fi
  exit 1
}

assert_file_contains() {
  local file="$1"
  local pattern="$2"
  local message="$3"
  if ! grep -Fq "$pattern" "$file"; then
    fail "$message" "$file"
  fi
}

echo "Test 1: dry-run previews recursive folder move without changing files..."
set +e
DRY_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-folder --dry-run "specifications/Shared" "specifications/Renamed" 2>&1)
DRY_EXIT=$?
set -e

if [ $DRY_EXIT -ne 0 ]; then
  echo "$DRY_OUTPUT"
  fail "mv-folder --dry-run failed"
fi

if [ ! -d "$TEST_DIR/specifications/Shared" ]; then
  fail "dry-run removed source folder"
fi
if [ -e "$TEST_DIR/specifications/Renamed" ]; then
  fail "dry-run created target folder"
fi
if ! echo "$DRY_OUTPUT" | grep -Fq "specifications/Shared/Contracts.md"; then
  echo "$DRY_OUTPUT"
  fail "dry-run output did not include moved source file"
fi

echo "Test 2: invalid folder moves are rejected without filesystem changes..."
set +e
MISSING_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-folder "specifications/Missing" "specifications/New" 2>&1)
MISSING_EXIT=$?
FILE_SOURCE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-folder "specifications/Outside.md" "specifications/New" 2>&1)
FILE_SOURCE_EXIT=$?
EXISTING_TARGET_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-folder "specifications/Shared" "specifications/Existing" 2>&1)
EXISTING_TARGET_EXIT=$?
DESCENDANT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-folder "specifications/Shared" "specifications/Shared/Nested" 2>&1)
DESCENDANT_EXIT=$?
set -e

[ $MISSING_EXIT -ne 0 ] || fail "missing source folder should fail"
[ $FILE_SOURCE_EXIT -ne 0 ] || fail "file source should fail"
[ $EXISTING_TARGET_EXIT -ne 0 ] || fail "existing target folder should fail"
[ $DESCENDANT_EXIT -ne 0 ] || fail "moving folder into its own descendant should fail"
[ -d "$TEST_DIR/specifications/Shared" ] || fail "invalid moves changed source folder"
[ ! -e "$TEST_DIR/specifications/New" ] || fail "invalid moves created unexpected target"

echo "Test 3: JSON execution moves folder and rewrites model references..."
set +e
JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-folder --json "specifications/Shared" "specifications/Renamed" 2>&1)
JSON_EXIT=$?
set -e

if [ $JSON_EXIT -ne 0 ]; then
  echo "$JSON_OUTPUT"
  fail "mv-folder --json failed"
fi

echo "$JSON_OUTPUT" > "$TEST_DIR/output/mv-folder.json"

if ! jq -e '.operation == "move" and .dry_run == false' "$TEST_DIR/output/mv-folder.json" >/dev/null; then
  fail "mv-folder JSON output missing move operation metadata" "$TEST_DIR/output/mv-folder.json"
fi

[ ! -d "$TEST_DIR/specifications/Shared" ] || fail "source folder still exists after move"
[ -f "$TEST_DIR/specifications/Renamed/Contracts.md" ] || fail "target contract file missing"
[ -f "$TEST_DIR/specifications/Renamed/Concepts.md" ] || fail "target concepts file missing"
[ -f "$TEST_DIR/specifications/Renamed/assets/evidence.txt" ] || fail "non-model asset missing after folder move"

assert_file_contains "$TEST_DIR/specifications/Outside.md" "Renamed/Contracts.md#shared-payload-specification" "contract_bindings target was not updated"
assert_file_contains "$TEST_DIR/specifications/Outside.md" "Renamed/Concepts.md#traceability" "Concept References target was not updated"
assert_file_contains "$TEST_DIR/specifications/Outside.md" "Renamed/assets/evidence.txt" "InternalPath satisfiedBy target was not updated"
assert_file_contains "$TEST_DIR/specifications/Renamed/Contracts.md" "Shared Requirement" "moved contract file content was not preserved"
assert_file_contains "$TEST_DIR/specifications/Renamed/Concepts.md" "Traceability" "moved concept file content was not preserved"

set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "$VALIDATION_OUTPUT"
  fail "model validation failed after mv-folder"
fi

echo "✅ Move folder operation test passed"
