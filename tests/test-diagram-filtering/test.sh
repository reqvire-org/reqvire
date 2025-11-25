#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Diagram Relation Filtering
# ---------------------------------
# Acceptance Criteria:
# - System should render only forward relations to prevent duplicate arrows
# - System should include parent elements in diagrams even when they belong to different sections
# - System should apply direction-based rendering according to relation type registry
# - Generated diagrams should not contain both forward and backward relations for the same logical relationship
#
# Test Criteria:
# - Command exits with success (0) return code
# - Diagrams contain only forward relations (e.g., `derive` but not `derivedFrom`)
# - Bidirectional relationships appear as single arrows in their forward direction
# - Parent elements are included when child elements are in the section
# - No duplicate arrows exist for the same logical relationship
# - Arrow directions follow the semantic direction defined in relation type registry

# Helper function to compare files and show diff on failure
assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "❌ FAILED: $description"
    echo ""
    echo "If changes are intentional, update the expected file:"
    echo "  cp $actual $expected"
    exit 1
  fi
}

# Run reqvire to generate diagrams
echo "Running: reqvire generate-diagrams" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" generate-diagrams 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Check for basic success
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Diagram generation command returned error: $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Compare output against expected file
assert_file_matches \
  "${TEST_SCRIPT_DIR}/expected/RelationFilteringTest.md" \
  "$TEST_DIR/specifications/RelationFilteringTest.md" \
  "RelationFilteringTest.md after diagram generation does not match expected"

exit 0
