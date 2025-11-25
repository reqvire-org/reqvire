#!/bin/bash
set -euo pipefail

# Test: Refinement Element Types
# ----------------------------------------------------
# Satisfies:
#   - specifications/System/Core/Verifications/ParsingVerifications.md#refinement-element-type-parsing-test
#   - specifications/System/Core/Verifications/ParsingVerifications.md#refinement-relations-rejection-test
#
# Acceptance Criteria:
# - Refinement element types (constraint, behavior, specification) are parsed correctly
# - Refinement element types are displayed correctly in JSON output
# - Search filtering by Refinement types works correctly
# - Validation fails when Refinement elements have Relations subsection
#
# Test Criteria:
# - Parse constraint type from metadata
# - Parse behavior type from metadata
# - Parse specification type from metadata
# - Filter elements by Refinement types
# - Reject Refinement elements with Relations

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "===================================="
echo "Refinement Element Type Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Constraint Type Parsing
# ==================================
echo "Test 1: Constraint type parsing..."

# Remove invalid file for first test
rm -f "$TEST_DIR/specifications/InvalidRefinement.md"

# Validate the model
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "FAILED: Model validation failed"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

# Get JSON output
set +e
FULL_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json 2>&1)
JSON_EXIT=$?
set -e

if [ $JSON_EXIT -ne 0 ]; then
  echo "FAILED: Search command failed"
  echo "$FULL_JSON"
  exit 1
fi

# Save for debugging
echo "$FULL_JSON" > "$TEST_DIR/actual-output.json"

# Check constraint type
CONSTRAINT_TYPE=$(echo "$FULL_JSON" | jq -r '.files | to_entries[] | .value.elements[] | select(.name == "Test Constraint Element") | .type')
if [ "$CONSTRAINT_TYPE" != "constraint" ]; then
  echo "FAILED: Constraint type not parsed correctly"
  echo "  Expected: constraint"
  echo "  Actual: $CONSTRAINT_TYPE"
  exit 1
fi

# ==================================
# Test 2: Behavior Type Parsing
# ==================================
echo "Test 2: Behavior type parsing..."

BEHAVIOR_TYPE=$(echo "$FULL_JSON" | jq -r '.files | to_entries[] | .value.elements[] | select(.name == "Test Behavior Element") | .type')
if [ "$BEHAVIOR_TYPE" != "behavior" ]; then
  echo "FAILED: Behavior type not parsed correctly"
  echo "  Expected: behavior"
  echo "  Actual: $BEHAVIOR_TYPE"
  exit 1
fi

# ==================================
# Test 3: Specification Type Parsing
# ==================================
echo "Test 3: Specification type parsing..."

SPEC_TYPE=$(echo "$FULL_JSON" | jq -r '.files | to_entries[] | .value.elements[] | select(.name == "Test Specification Element") | .type')
if [ "$SPEC_TYPE" != "specification" ]; then
  echo "FAILED: Specification type not parsed correctly"
  echo "  Expected: specification"
  echo "  Actual: $SPEC_TYPE"
  exit 1
fi

# ==================================
# Test 4: Filter by Constraint Type
# ==================================
echo "Test 4: Search filtering by constraint type..."

set +e
CONSTRAINT_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type="constraint" --json 2>&1)
FILTER_EXIT=$?
set -e

if [ $FILTER_EXIT -ne 0 ]; then
  echo "FAILED: Filter by constraint type failed"
  echo "$CONSTRAINT_JSON"
  exit 1
fi

# Verify only constraint elements returned
CONSTRAINT_COUNT=$(echo "$CONSTRAINT_JSON" | jq '[.files | to_entries[] | .value.elements[]] | length')
if [ "$CONSTRAINT_COUNT" -ne 1 ]; then
  echo "FAILED: Filter by constraint returned wrong count"
  echo "  Expected: 1"
  echo "  Actual: $CONSTRAINT_COUNT"
  exit 1
fi

# ==================================
# Test 5: Filter by Behavior Type
# ==================================
echo "Test 5: Search filtering by behavior type..."

set +e
BEHAVIOR_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type="behavior" --json 2>&1)
FILTER_EXIT=$?
set -e

if [ $FILTER_EXIT -ne 0 ]; then
  echo "FAILED: Filter by behavior type failed"
  echo "$BEHAVIOR_JSON"
  exit 1
fi

# Verify only behavior elements returned
BEHAVIOR_COUNT=$(echo "$BEHAVIOR_JSON" | jq '[.files | to_entries[] | .value.elements[]] | length')
if [ "$BEHAVIOR_COUNT" -ne 1 ]; then
  echo "FAILED: Filter by behavior returned wrong count"
  echo "  Expected: 1"
  echo "  Actual: $BEHAVIOR_COUNT"
  exit 1
fi

# ==================================
# Test 6: Filter by Specification Type
# ==================================
echo "Test 6: Search filtering by specification type..."

set +e
SPEC_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-type="specification" --json 2>&1)
FILTER_EXIT=$?
set -e

if [ $FILTER_EXIT -ne 0 ]; then
  echo "FAILED: Filter by specification type failed"
  echo "$SPEC_JSON"
  exit 1
fi

# Verify only specification elements returned
SPEC_COUNT=$(echo "$SPEC_JSON" | jq '[.files | to_entries[] | .value.elements[]] | length')
if [ "$SPEC_COUNT" -ne 1 ]; then
  echo "FAILED: Filter by specification returned wrong count"
  echo "  Expected: 1"
  echo "  Actual: $SPEC_COUNT"
  exit 1
fi

# ==================================
# Test 7: Refinement With Relations Rejection
# ==================================
echo "Test 7: Validation rejects Refinement with Relations..."

# Restore invalid file
cat > "$TEST_DIR/specifications/InvalidRefinement.md" <<'EOF'
# Elements

### Invalid Constraint With Relations

This constraint element has relations which is not allowed.

#### Metadata
  * type: constraint

#### Relations
  * derivedFrom: [Parent Requirement](Requirements.md#parent-requirement)
---
EOF

# Validation should fail
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -eq 0 ]; then
  echo "FAILED: Validation should have failed for Refinement with Relations"
  exit 1
fi

# Verify error message mentions Refinement or constraint
if ! echo "$VALIDATION_OUTPUT" | grep -qi "constraint\|refinement\|relations"; then
  echo "FAILED: Error message should mention constraint/refinement/relations issue"
  echo "Output: $VALIDATION_OUTPUT"
  exit 1
fi

# ==================================
# Test 8: Attachment Identifier to Refinement Element
# ==================================
echo "Test 8: Attachment identifier to Refinement element..."

# Remove invalid file for this test
rm -f "$TEST_DIR/specifications/InvalidRefinement.md"

# Verify attachment parsing
set +e
FULL_JSON=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --json 2>&1)
JSON_EXIT=$?
set -e

if [ $JSON_EXIT -ne 0 ]; then
  echo "FAILED: Search command failed"
  echo "$FULL_JSON"
  exit 1
fi

# Check attachment exists on "Requirement With Refinement Attachment"
ATTACHMENTS=$(echo "$FULL_JSON" | jq -r '.files | to_entries[] | .value.elements[] | select(.name == "Requirement With Refinement Attachment") | .attachments')
if [ "$ATTACHMENTS" == "null" ] || [ "$ATTACHMENTS" == "[]" ]; then
  echo "FAILED: Attachment not found on Requirement With Refinement Attachment"
  exit 1
fi

# ==================================
# Test 9: Rename Refinement Element Updates Attachment Identifiers
# ==================================
echo "Test 9: Rename Refinement element updates attachment identifiers..."

# Rename the constraint element
set +e
RENAME_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rename "Test Constraint Element" "Renamed Constraint" 2>&1)
RENAME_EXIT=$?
set -e

if [ $RENAME_EXIT -ne 0 ]; then
  echo "FAILED: Rename command failed"
  echo "$RENAME_OUTPUT"
  exit 1
fi

# Verify the attachment identifier was updated
FILE_CONTENT=$(cat "$TEST_DIR/specifications/Requirements.md")
if ! echo "$FILE_CONTENT" | grep -q "#renamed-constraint"; then
  echo "FAILED: Attachment identifier not updated after rename"
  echo "File content:"
  echo "$FILE_CONTENT"
  exit 1
fi

# Validate the model after rename
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "FAILED: Model validation failed after rename"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

# Rename back for subsequent tests
set +e
RENAME_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rename "Renamed Constraint" "Test Constraint Element" 2>&1)
RENAME_EXIT=$?
set -e

if [ $RENAME_EXIT -ne 0 ]; then
  echo "FAILED: Rename back failed"
  echo "$RENAME_OUTPUT"
  exit 1
fi

# ==================================
# Test 10: Move Refinement Element Updates Attachment Identifiers
# ==================================
echo "Test 10: Move Refinement element updates attachment identifiers..."

# Create a new file to move to
cat > "$TEST_DIR/specifications/Refinements.md" <<'EOF'
# Refinements
EOF

# Move the constraint element to a new file
set +e
MOVE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Test Constraint Element" specifications/Refinements.md 2>&1)
MOVE_EXIT=$?
set -e

if [ $MOVE_EXIT -ne 0 ]; then
  echo "FAILED: Move command failed"
  echo "$MOVE_OUTPUT"
  exit 1
fi

# Verify the attachment identifier was updated with the new file path
# Note: The markdown output uses relative paths, so we check for Refinements.md (not specifications/Refinements.md)
FILE_CONTENT=$(cat "$TEST_DIR/specifications/Requirements.md")
if ! echo "$FILE_CONTENT" | grep -q "Refinements.md#test-constraint-element"; then
  echo "FAILED: Attachment identifier not updated after move"
  echo "File content:"
  echo "$FILE_CONTENT"
  exit 1
fi

# Validate the model after move
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "FAILED: Model validation failed after move"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

# Move back for cleanup
set +e
MOVE_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv "Test Constraint Element" specifications/Requirements.md 2>&1)
MOVE_EXIT=$?
set -e

if [ $MOVE_EXIT -ne 0 ]; then
  echo "FAILED: Move back failed"
  echo "$MOVE_OUTPUT"
  exit 1
fi

# Remove the temporary file
rm -f "$TEST_DIR/specifications/Refinements.md"

# ==================================
# Final Result
# ==================================
echo ""
echo "All Refinement element type tests passed"
exit 0
