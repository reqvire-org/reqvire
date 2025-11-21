#!/bin/bash
set -euo pipefail

# Test: Attachments Feature
#
# Satisfies: specifications/System/AttachmentsVerifications.md
#
# Acceptance Criteria:
# - Attach command creates Attachments subsection and adds links
# - Detach command removes links and cleans up empty subsections
# - mv-attachment updates all references across elements
# - rm-attachment deletes file and detaches from all elements
# - Search filters correctly find elements by attachments
# - Validation detects missing attachment files
#
# Test Criteria:
# - Commands exit with code 0 on success
# - Files are modified as expected
# - Attachments are properly maintained

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "===================================="
echo "Attachments Feature Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Attach Command
# ==================================
echo "Test 1: Attach command..."

set +e
ATTACH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" attach "docs/SLA.txt" "Performance Requirement" 2>&1)
ATTACH_EXIT=$?
set -e

if [ $ATTACH_EXIT -ne 0 ]; then
  echo "FAILED: Attach command failed with exit code $ATTACH_EXIT"
  echo "$ATTACH_OUTPUT"
  exit 1
fi

# Verify Attachments subsection was created
if ! grep -q "#### Attachments" "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: Attachments subsection was not created"
  exit 1
fi

# Verify link was added with correct format
if ! grep -q '\[docs/SLA.txt\](docs/SLA.txt)' "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: Attachment link not added with correct format"
  exit 1
fi

# Verify model still validates
set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -ne 0 ]; then
  echo "FAILED: Model validation failed after attach"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "Attachment added successfully"
echo ""

# ==================================
# Test 2: Attach Idempotency
# ==================================
echo "Test 2: Attach idempotency (duplicate attach)..."

# Make backup
cp "$TEST_DIR/specifications/Requirements.md" "$TEST_DIR/requirements_backup.bak"

set +e
ATTACH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" attach "docs/SLA.txt" "Performance Requirement" 2>&1)
ATTACH_EXIT=$?
set -e

if [ $ATTACH_EXIT -ne 0 ]; then
  echo "FAILED: Duplicate attach command should succeed"
  echo "$ATTACH_OUTPUT"
  exit 1
fi

# Count occurrences of the link - should be exactly 1
LINK_COUNT=$(grep -c '\[docs/SLA.txt\](docs/SLA.txt)' "$TEST_DIR/specifications/Requirements.md" || true)
if [ "$LINK_COUNT" -ne 1 ]; then
  echo "FAILED: Duplicate attach created duplicate entry (found $LINK_COUNT occurrences)"
  exit 1
fi

echo "Attach idempotency verified"
echo ""

# ==================================
# Test 3: Multiple Attachments
# ==================================
echo "Test 3: Multiple attachments on same element..."

set +e
ATTACH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" attach "docs/benchmarks.txt" "Performance Requirement" 2>&1)
ATTACH_EXIT=$?
set -e

if [ $ATTACH_EXIT -ne 0 ]; then
  echo "FAILED: Second attach command failed"
  echo "$ATTACH_OUTPUT"
  exit 1
fi

# Verify both attachments exist
if ! grep -q '\[docs/SLA.txt\](docs/SLA.txt)' "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: First attachment was removed"
  exit 1
fi

if ! grep -q '\[docs/benchmarks.txt\](docs/benchmarks.txt)' "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: Second attachment was not added"
  exit 1
fi

echo "Multiple attachments verified"
echo ""

# ==================================
# Test 4: Many-to-Many Attachment
# ==================================
echo "Test 4: Same file attached to multiple elements..."

set +e
ATTACH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" attach "docs/SLA.txt" "Implementation Detail" 2>&1)
ATTACH_EXIT=$?
set -e

if [ $ATTACH_EXIT -ne 0 ]; then
  echo "FAILED: Attach to second element failed"
  echo "$ATTACH_OUTPUT"
  exit 1
fi

# Verify both elements have the attachment
SLA_COUNT=$(grep -c '\[docs/SLA.txt\](docs/SLA.txt)' "$TEST_DIR/specifications/Requirements.md" || true)
if [ "$SLA_COUNT" -ne 2 ]; then
  echo "FAILED: Expected SLA.txt attached to 2 elements, found $SLA_COUNT"
  exit 1
fi

echo "Many-to-many attachment verified"
echo ""

# ==================================
# Test 5: Detach Command
# ==================================
echo "Test 5: Detach command..."

set +e
DETACH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" detach "Performance Requirement" "docs/benchmarks.txt" 2>&1)
DETACH_EXIT=$?
set -e

if [ $DETACH_EXIT -ne 0 ]; then
  echo "FAILED: Detach command failed"
  echo "$DETACH_OUTPUT"
  exit 1
fi

# Verify benchmarks.txt was removed
if grep -q '\[docs/benchmarks.txt\](docs/benchmarks.txt)' "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: Attachment was not removed after detach"
  exit 1
fi

# Verify SLA.txt still exists on Performance Requirement
# (We need to check the specific element - use enough lines to capture all sections)
if ! grep -A 20 "### Performance Requirement" "$TEST_DIR/specifications/Requirements.md" | grep -q '\[docs/SLA.txt\](docs/SLA.txt)'; then
  echo "FAILED: Other attachment was incorrectly removed"
  exit 1
fi

echo "Detach command verified"
echo ""

# ==================================
# Test 6: Detach from One Element
# ==================================
echo "Test 6: Detach from one element doesn't affect others..."

set +e
DETACH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" detach "Performance Requirement" "docs/SLA.txt" 2>&1)
DETACH_EXIT=$?
set -e

if [ $DETACH_EXIT -ne 0 ]; then
  echo "FAILED: Detach from one element failed"
  echo "$DETACH_OUTPUT"
  exit 1
fi

# Verify SLA.txt was removed from Performance Requirement but still on Implementation Detail
if grep -A 20 "### Performance Requirement" "$TEST_DIR/specifications/Requirements.md" | grep -B 20 "### Implementation Detail" | grep -q '\[docs/SLA.txt\](docs/SLA.txt)'; then
  echo "FAILED: SLA.txt was not removed from Performance Requirement"
  exit 1
fi

if ! grep -A 20 "### Implementation Detail" "$TEST_DIR/specifications/Requirements.md" | grep -q '\[docs/SLA.txt\](docs/SLA.txt)'; then
  echo "FAILED: SLA.txt was incorrectly removed from Implementation Detail"
  exit 1
fi

echo "Detach isolation verified"
echo ""

# ==================================
# Test 7: Empty Subsection Cleanup
# ==================================
echo "Test 7: Empty Attachments subsection cleanup..."

# Performance Requirement should no longer have Attachments subsection
# since we detached all its attachments

# Count Attachments subsections - should be 1 (only on Implementation Detail)
ATTACHMENTS_COUNT=$(grep -c "#### Attachments" "$TEST_DIR/specifications/Requirements.md" || true)
if [ "$ATTACHMENTS_COUNT" -ne 1 ]; then
  echo "FAILED: Expected 1 Attachments subsection, found $ATTACHMENTS_COUNT"
  cat "$TEST_DIR/specifications/Requirements.md"
  exit 1
fi

echo "Empty subsection cleanup verified"
echo ""

# ==================================
# Test 8: Search Filters
# ==================================
echo "Test 8: Search filters for attachments..."

# Re-attach for search tests
set +e
cd "$TEST_DIR" && "$REQVIRE_BIN" attach "docs/SLA.txt" "Performance Requirement" 2>&1 > /dev/null
set -e

# Test --has-attachments filter
set +e
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --has-attachments --short 2>&1)
SEARCH_EXIT=$?
set -e

if [ $SEARCH_EXIT -ne 0 ]; then
  echo "FAILED: Search with --has-attachments failed"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

# Should find Performance Requirement and Implementation Detail, not No Attachments Requirement
if ! echo "$SEARCH_OUTPUT" | grep -q "Performance Requirement"; then
  echo "FAILED: Search did not find Performance Requirement"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

if ! echo "$SEARCH_OUTPUT" | grep -q "Implementation Detail"; then
  echo "FAILED: Search did not find Implementation Detail"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

if echo "$SEARCH_OUTPUT" | grep -q "No Attachments Requirement"; then
  echo "FAILED: Search incorrectly found No Attachments Requirement"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

echo "Search filter --has-attachments verified"

# Test --filter-attachment with glob
set +e
SEARCH_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" search --filter-attachment "*.txt" --short 2>&1)
SEARCH_EXIT=$?
set -e

if [ $SEARCH_EXIT -ne 0 ]; then
  echo "FAILED: Search with --filter-attachment failed"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

if ! echo "$SEARCH_OUTPUT" | grep -q "Performance Requirement"; then
  echo "FAILED: Pattern *.txt did not match SLA.txt"
  echo "$SEARCH_OUTPUT"
  exit 1
fi

echo "Search filter --filter-attachment verified"
echo ""

# ==================================
# Test 9: mv-attachment Command
# ==================================
echo "Test 9: mv-attachment command..."

# Create new location
mkdir -p "$TEST_DIR/documents"

set +e
MV_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" mv-attachment "docs/SLA.txt" "documents/SLA.txt" 2>&1)
MV_EXIT=$?
set -e

if [ $MV_EXIT -ne 0 ]; then
  echo "FAILED: mv-attachment command failed"
  echo "$MV_OUTPUT"
  exit 1
fi

# Verify old path is gone from all elements
if grep -q '\[docs/SLA.txt\](docs/SLA.txt)' "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: Old path still exists after mv-attachment"
  exit 1
fi

# Verify new path exists in all elements that had the attachment
if ! grep -q '\[documents/SLA.txt\](documents/SLA.txt)' "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: New path not found after mv-attachment"
  exit 1
fi

# Verify file was moved
if [ -f "$TEST_DIR/docs/SLA.txt" ]; then
  echo "FAILED: Old file still exists"
  exit 1
fi

if [ ! -f "$TEST_DIR/documents/SLA.txt" ]; then
  echo "FAILED: File was not moved to new location"
  exit 1
fi

echo "mv-attachment verified"
echo ""

# ==================================
# Test 10: rm-attachment Command
# ==================================
echo "Test 10: rm-attachment command..."

set +e
RM_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" rm-attachment "documents/SLA.txt" 2>&1)
RM_EXIT=$?
set -e

if [ $RM_EXIT -ne 0 ]; then
  echo "FAILED: rm-attachment command failed"
  echo "$RM_OUTPUT"
  exit 1
fi

# Verify file was deleted
if [ -f "$TEST_DIR/documents/SLA.txt" ]; then
  echo "FAILED: File was not deleted"
  exit 1
fi

# Verify attachment was removed from all elements
if grep -q '\[documents/SLA.txt\](documents/SLA.txt)' "$TEST_DIR/specifications/Requirements.md"; then
  echo "FAILED: Attachment reference still exists after rm-attachment"
  exit 1
fi

# Verify empty Attachments subsections were cleaned up
# No elements should have Attachments subsections now
ATTACHMENTS_COUNT=$(grep -c "#### Attachments" "$TEST_DIR/specifications/Requirements.md" || true)
if [ "$ATTACHMENTS_COUNT" -ne 0 ]; then
  echo "FAILED: Empty Attachments subsections not cleaned up after rm-attachment"
  exit 1
fi

echo "rm-attachment verified"
echo ""

# ==================================
# Test 11: Validation - Missing Attachment
# ==================================
echo "Test 11: Validation detects missing attachment files..."

# Manually add attachment that doesn't exist
cat >> "$TEST_DIR/specifications/Requirements.md" << 'EOF'

### Test Missing Attachment

This element has a missing attachment.

#### Attachments
* [missing/file.txt](missing/file.txt)

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
---

EOF

set +e
VALIDATION_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" validate 2>&1)
VALIDATION_EXIT=$?
set -e

if [ $VALIDATION_EXIT -eq 0 ]; then
  echo "FAILED: Validation should fail for missing attachment file"
  exit 1
fi

if ! echo "$VALIDATION_OUTPUT" | grep -qi "missing\|not found\|attachment"; then
  echo "FAILED: Validation error should mention missing attachment"
  echo "$VALIDATION_OUTPUT"
  exit 1
fi

echo "Missing attachment validation verified"
echo ""

# ==================================
# Test 12: Dry-run Mode
# ==================================
echo "Test 12: Dry-run mode..."

# Clean up Test 11's invalid element before continuing
# Remove the Test Missing Attachment element that was added
sed -i '/### Test Missing Attachment/,/^---$/d' "$TEST_DIR/specifications/Requirements.md"

# Create a fresh requirement for dry-run test
cat > "$TEST_DIR/specifications/DryRunTest.md" << 'EOF'
# Dry Run Test

## Requirements

### Dry Run Element

Test element for dry-run.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: Requirements.md#system-requirements
---

EOF

# Make backup
cp "$TEST_DIR/specifications/DryRunTest.md" "$TEST_DIR/dryrun_backup.bak"

set +e
DRYRUN_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" attach "docs/benchmarks.txt" "Dry Run Element" --dry-run 2>&1)
DRYRUN_EXIT=$?
set -e

if [ $DRYRUN_EXIT -ne 0 ]; then
  echo "FAILED: Dry-run attach failed"
  echo "$DRYRUN_OUTPUT"
  exit 1
fi

# Verify file was NOT modified
if ! cmp -s "$TEST_DIR/specifications/DryRunTest.md" "$TEST_DIR/dryrun_backup.bak"; then
  echo "FAILED: Dry-run mode modified the file"
  diff "$TEST_DIR/dryrun_backup.bak" "$TEST_DIR/specifications/DryRunTest.md"
  exit 1
fi

echo "Dry-run mode verified"
echo ""

echo "===================================="
echo "All Attachments tests passed"
echo "===================================="
exit 0
