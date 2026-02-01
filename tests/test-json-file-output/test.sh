#!/bin/bash
set -uo pipefail

# Test: JSON File Output Option (all commands)
# Verifies --output <FILE> writes JSON to file when used with --json
# for every CLI command that supports --json.
#
# Acceptance Criteria:
# - --json --output writes JSON to file for all 17 commands
# - Confirmation message printed to stdout
# - File contains valid JSON
# - File content matches direct --json stdout output (spot-checked)
# - --output without --json is an error

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASS_COUNT=0

# Helper: test a command with --json --output and verify the result
test_json_output() {
  local test_name="$1"
  local output_file="$2"
  shift 2
  # remaining args are the command (without --json --output)

  set +e
  OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" "$@" --json --output "$output_file" 2>&1)
  EXIT_CODE=$?
  set -e

  if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ FAILED ($test_name): Command returned $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
  fi

  if ! echo "$OUTPUT" | grep -q "Output saved to"; then
    echo "❌ FAILED ($test_name): Missing confirmation message"
    echo "$OUTPUT"
    exit 1
  fi

  if [ ! -f "$TEST_DIR/$output_file" ]; then
    echo "❌ FAILED ($test_name): Output file not created"
    exit 1
  fi

  if ! jq . "$TEST_DIR/$output_file" > /dev/null 2>&1; then
    echo "❌ FAILED ($test_name): Output file is not valid JSON"
    cat "$TEST_DIR/$output_file"
    exit 1
  fi

  PASS_COUNT=$((PASS_COUNT + 1))
}

# Helper: verify file content matches direct --json stdout output
verify_content_matches() {
  local test_name="$1"
  local output_file="$2"
  shift 2

  set +e
  DIRECT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" "$@" --json 2>&1)
  set -e

  FILE_CONTENT=$(cat "$TEST_DIR/$output_file")
  if [ "$DIRECT_OUTPUT" != "$FILE_CONTENT" ]; then
    echo "❌ FAILED ($test_name): File content differs from stdout JSON output"
    exit 1
  fi
}

# ========================================
# Read-only commands
# ========================================

test_json_output "validate" "validate.json" validate
verify_content_matches "validate" "validate.json" validate

test_json_output "format" "format.json" format

test_json_output "search" "search.json" search
verify_content_matches "search" "search.json" search

test_json_output "traces" "traces.json" traces

test_json_output "coverage" "coverage.json" coverage

test_json_output "model" "model.json" model

test_json_output "lint" "lint.json" lint

test_json_output "containment" "containment.json" containment

test_json_output "resources" "resources.json" resources

test_json_output "collect" "collect.json" collect "Test Requirement Beta"

# ========================================
# Change impact (needs git history)
# ========================================

cd "$TEST_DIR"
echo "" >> specifications/Requirements.md
git add -A > /dev/null 2>&1 && git commit -m "test change" > /dev/null 2>&1

test_json_output "change-impact" "change-impact.json" change-impact

# ========================================
# CRUD commands (with --dry-run)
# ========================================

test_json_output "add" "add.json" add "specifications/Requirements.md" \
  --content "### Added Element

A new element for testing.

#### Metadata
  * type: user-requirement
" --dry-run

test_json_output "rm" "rm.json" rm "Test Verification Beta" --dry-run

test_json_output "mv" "mv.json" mv "Test Verification Beta" "specifications/Other.md" --dry-run

test_json_output "rename" "rename.json" rename "Test Verification Beta" "Renamed Verification" --dry-run

test_json_output "merge" "merge.json" merge "Test Verification Alpha" "Test Verification Beta" --dry-run

test_json_output "mv-file" "mv-file.json" mv-file "specifications/Requirements.md" "specifications/Moved.md" --dry-run

# ========================================
# Error case: --output without --json
# ========================================

set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate --output output.json 2>&1)
EXIT_CODE=$?
set -e

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Expected non-zero exit for --output without --json"
  exit 1
fi

PASS_COUNT=$((PASS_COUNT + 1))

# ========================================
# Overwrite test
# ========================================

echo "old content" > "$TEST_DIR/overwrite.json"

test_json_output "overwrite" "overwrite.json" validate

if grep -q "old content" "$TEST_DIR/overwrite.json"; then
  echo "❌ FAILED: File was not overwritten"
  exit 1
fi

exit 0
