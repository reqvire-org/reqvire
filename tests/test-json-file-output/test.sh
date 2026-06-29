#!/bin/bash
set -uo pipefail

# Test: JSON File Output Option (all commands)
# Verifies --output <FILE> writes JSON to file for every CLI command
# that emits JSON.
#
# Acceptance Criteria:
# - --json --output writes JSON to file for commands with selectable JSON output
# - --output writes JSON to file directly for JSON-only commands
# - Confirmation message printed to stdout
# - File contains valid JSON
# - File content matches direct stdout JSON output (spot-checked)
# - --output without JSON selection is an error only for commands with non-JSON modes

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASS_COUNT=0

# Helper: test a command with --output and verify the result.
# Pass "true" when the command still requires --json to select JSON output.
test_json_output() {
  local test_name="$1"
  local output_file="$2"
  local needs_json_flag="$3"
  shift 3
  # remaining args are the command (without --output)

  set +e
  if [ "$needs_json_flag" = "true" ]; then
    OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" "$@" --json --output "$output_file" 2>&1)
  else
    OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" "$@" --output "$output_file" 2>&1)
  fi
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

# Helper: verify file content matches direct stdout JSON output
verify_content_matches() {
  local test_name="$1"
  local output_file="$2"
  local needs_json_flag="$3"
  shift 3

  set +e
  if [ "$needs_json_flag" = "true" ]; then
    DIRECT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" "$@" --json 2>&1)
  else
    DIRECT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" "$@" 2>&1)
  fi
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

test_json_output "validate" "validate.json" true validate
verify_content_matches "validate" "validate.json" true validate

test_json_output "format" "format.json" true format

test_json_output "search" "search.json" true search
verify_content_matches "search" "search.json" true search

test_json_output "traces" "traces.json" false traces

test_json_output "coverage" "coverage.json" true coverage

test_json_output "model" "model.json" false model

test_json_output "lint" "lint.json" true lint

test_json_output "containment" "containment.json" false containment

test_json_output "resources" "resources.json" false resources

test_json_output "collect" "collect.json" true collect "Test Requirement Beta"

# ========================================
# Change impact (needs git history)
# ========================================

cd "$TEST_DIR"
echo "" >> specifications/Requirements.md
git add -A > /dev/null 2>&1 && git commit -m "test change" > /dev/null 2>&1

test_json_output "change-impact" "change-impact.json" true change-impact

# ========================================
# CRUD commands (with --dry-run)
# ========================================

test_json_output "add" "add.json" true add "specifications/Requirements.md" \
  --content "### Added Element

A new element for testing.

#### Metadata
  * type: requirement
" --dry-run

test_json_output "rm" "rm.json" true rm "Test Verification Beta" --dry-run

test_json_output "mv" "mv.json" true mv "Test Verification Beta" "specifications/Other.md" --dry-run

test_json_output "rename" "rename.json" true rename "Test Verification Beta" "Renamed Verification" --dry-run

test_json_output "merge" "merge.json" true merge "Test Verification Alpha" "Test Verification Beta" --dry-run

test_json_output "mv-file" "mv-file.json" true mv-file "specifications/Requirements.md" "specifications/Moved.md" --dry-run

test_json_output "link" "link.json" true link "Test Requirement Beta" satisfiedBy "https://example.com/evidence" --dry-run

test_json_output "unlink" "unlink.json" true unlink "Test Verification Alpha" "Test Requirement Alpha" --dry-run

test_json_output "relink" "relink.json" true relink "Test Verification Alpha" verify "Test Requirement Alpha" "Test Requirement Beta" --dry-run

test_json_output "mv-asset" "mv-asset.json" true mv-asset "docs/asset.txt" "docs/asset-renamed.txt" --dry-run

test_json_output "rm-asset" "rm-asset.json" true rm-asset "docs/asset.txt" --dry-run

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

test_json_output "overwrite" "overwrite.json" true validate

if grep -q "old content" "$TEST_DIR/overwrite.json"; then
  echo "❌ FAILED: File was not overwritten"
  exit 1
fi

exit 0
