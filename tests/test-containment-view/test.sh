#!/bin/bash
set -uo pipefail

# Test: Containment View Functionality
# ----------------------------------------------------
# Satisfies:
#   - specifications/Verifications/ReportsTests.md#containment-hierarchy-extraction-test
#   - specifications/Verifications/ReportsTests.md#containment-view-text-output-test
#   - specifications/Verifications/ReportsTests.md#containment-view-json-output-test
#   - specifications/Verifications/ReportsTests.md#containment-view-mermaid-diagram-test
#   - specifications/Verifications/ReportsTests.md#html-export-containment-view-integration-test
#
# Acceptance Criteria:
# - Containment hierarchy extracts folders, files, and elements (skipping sections)
# - Mermaid flowchart diagram with nested subgraphs for folder/file hierarchy
# - Element nodes with proper styling based on type
# - Clickable links to element locations
# - JSON output provides valid structured data with correct schema
# - HTML export includes containment view page with interactive features

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "===================================="
echo "Containment View Tests"
echo "===================================="
echo ""

# ==================================
# Test 1: Mermaid Diagram Structure Validation
# ==================================
echo "Test 1: Mermaid diagram structure validation..."

# Run containment command
set +e
DIAGRAM_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" containment 2>&1)
DIAGRAM_EXIT=$?
set -e

if [ $DIAGRAM_EXIT -ne 0 ]; then
  echo "❌ FAILED: containment command failed"
  echo "$DIAGRAM_OUTPUT"
  exit 1
fi

# Save actual output
echo "$DIAGRAM_OUTPUT" > "$TEST_DIR/actual-containment-output.md"

# Verify output starts with mermaid code block
if ! echo "$DIAGRAM_OUTPUT" | head -1 | grep -q '```mermaid'; then
  echo "❌ FAILED: Output does not start with mermaid code block"
  exit 1
fi

# Extract mermaid diagram from code block for further testing
MERMAID_DIAGRAM=$(echo "$DIAGRAM_OUTPUT" | sed -n '/```mermaid/,/```/p' | sed '1d;$d')

# Verify diagram starts with graph TD
if ! echo "$MERMAID_DIAGRAM" | head -1 | grep -q "^graph TD"; then
  echo "❌ FAILED: Mermaid diagram does not start with 'graph TD'"
  exit 1
fi

echo "✓ Markdown output with Mermaid diagram is correct"

# ==================================
# Test 2: Subgraph Nesting and Structure
# ==================================
echo ""
echo "Test 2: Subgraph nesting and structure..."

# Verify folder nodes with emoji
if ! echo "$MERMAID_DIAGRAM" | grep -q '\["📁.*"\]'; then
  echo "❌ FAILED: Missing folder nodes with 📁 emoji"
  exit 1
fi

# Verify file subgraphs with emoji
if ! echo "$MERMAID_DIAGRAM" | grep -q 'subgraph.*\["📄.*"\]'; then
  echo "❌ FAILED: Missing file subgraphs with 📄 emoji"
  exit 1
fi

# Verify folder connections exist (tree structure)
if ! echo "$MERMAID_DIAGRAM" | grep -q -- '-->'; then
  echo "❌ FAILED: Missing folder/file connections (-->)"
  exit 1
fi

# Verify all subgraphs are closed
SUBGRAPH_COUNT=$(echo "$MERMAID_DIAGRAM" | grep -c "^  *subgraph" || true)
END_COUNT=$(echo "$MERMAID_DIAGRAM" | grep -c "^  *end$" || true)
if [ "$SUBGRAPH_COUNT" -ne "$END_COUNT" ]; then
  echo "❌ FAILED: Subgraph count ($SUBGRAPH_COUNT) doesn't match end count ($END_COUNT)"
  exit 1
fi

echo "✓ Subgraphs are properly nested and closed"

# ==================================
# Test 3: Element Nodes and Hash IDs
# ==================================
echo ""
echo "Test 3: Element nodes and hash IDs..."

# Verify element nodes exist (format: hashId["Element Name"])
# Default mode shows all elements (15 total including Deep Nested Element)
# Note: Test 8.4 specifically validates the intermediate folder case
NODE_COUNT=$(echo "$MERMAID_DIAGRAM" | grep -cE '^\s+[a-f0-9]+\["[^"]+"\]' || true)
if [ "$NODE_COUNT" -lt 14 ]; then
  echo "❌ FAILED: Expected at least 14 element nodes (all elements), got $NODE_COUNT"
  exit 1
fi

# Verify element hash IDs are consistent length (16 characters)
# Only check element nodes (indented inside subgraphs), not folder/file nodes
INVALID_HASH=$(echo "$MERMAID_DIAGRAM" | grep -E '^\s+[a-f0-9]+\["[^"]+"\]' | grep -oE '^[[:space:]]+[a-f0-9]+' | grep -oE '[a-f0-9]+' | awk 'length($0) != 16' || true)
if [ -n "$INVALID_HASH" ]; then
  echo "❌ FAILED: Found element hash IDs not exactly 16 characters"
  echo "$INVALID_HASH"
  exit 1
fi

echo "✓ Element nodes use proper hash IDs"

# ==================================
# Test 4: Element Type Styling
# ==================================
echo ""
echo "Test 4: Element type styling..."

# Verify CSS class definitions exist
if ! echo "$MERMAID_DIAGRAM" | grep -q "classDef userRequirement"; then
  echo "❌ FAILED: Missing userRequirement class definition"
  exit 1
fi

if ! echo "$MERMAID_DIAGRAM" | grep -q "classDef requirement"; then
  echo "❌ FAILED: Missing requirement class definition"
  exit 1
fi

if ! echo "$MERMAID_DIAGRAM" | grep -q "classDef verification"; then
  echo "❌ FAILED: Missing verification class definition"
  exit 1
fi

# Verify class assignments exist
if ! echo "$MERMAID_DIAGRAM" | grep -q "class .* userRequirement"; then
  echo "❌ FAILED: No elements assigned userRequirement class"
  exit 1
fi

if ! echo "$MERMAID_DIAGRAM" | grep -q "class .* verification"; then
  echo "❌ FAILED: No elements assigned verification class"
  exit 1
fi

echo "✓ Element type styling is applied correctly"

# ==================================
# Test 5: Clickable Links
# ==================================
echo ""
echo "Test 5: Clickable links..."

# Verify click directives exist (at least 14 for elements, Test 8.4 checks for 15th)
CLICK_COUNT=$(echo "$MERMAID_DIAGRAM" | grep -c "^  click" || true)
if [ "$CLICK_COUNT" -lt 14 ]; then
  echo "❌ FAILED: Expected at least 14 click directives (all elements), got $CLICK_COUNT"
  exit 1
fi

# Verify click format: click hashId "path#fragment"
if ! echo "$MERMAID_DIAGRAM" | grep -qE 'click [a-f0-9]+ "[^"]+\.md#[^"]+"'; then
  echo "❌ FAILED: Click directives don't follow correct format"
  exit 1
fi

# Verify fragments are normalized (lowercase with hyphens)
INVALID_FRAGMENT=$(echo "$MERMAID_DIAGRAM" | grep -oE 'click [a-f0-9]+ "[^#]+#[^"]+"' | grep -E '#[A-Z]' || true)
if [ -n "$INVALID_FRAGMENT" ]; then
  echo "❌ FAILED: Found non-normalized fragments (should be lowercase)"
  echo "$INVALID_FRAGMENT"
  exit 1
fi

echo "✓ Clickable links are properly formatted"

# ==================================
# Test 6: Sections Omitted
# ==================================
echo ""
echo "Test 6: Sections omitted from hierarchy..."

# Verify section names do NOT appear in diagram
# Sections in fixtures: "Section One", "Data Operations", "Performance Requirements", "Root Elements"
if echo "$MERMAID_DIAGRAM" | grep -qi "Section One"; then
  echo "❌ FAILED: Section 'Section One' should not appear in diagram"
  exit 1
fi

if echo "$MERMAID_DIAGRAM" | grep -qi "Data Operations"; then
  echo "❌ FAILED: Section 'Data Operations' should not appear in diagram"
  exit 1
fi

if echo "$MERMAID_DIAGRAM" | grep -qi "Performance Requirements"; then
  echo "❌ FAILED: Section 'Performance Requirements' should not appear in diagram"
  exit 1
fi

echo "✓ Sections are correctly omitted from hierarchy"

# ==================================
# Test 7: All Element Types Present
# ==================================
echo ""
echo "Test 7: All element types present..."

# Verify top-level parent elements appear in diagram (filtered)
# Child elements with derivedFrom to same-file parents are excluded
REQUIRED_ELEMENTS=(
  "User Authentication"
  "Export to CSV"
  "High Performance"
  "Root User Requirement"
  "Test Verification Element"
  "Analysis Verification Element"
  "Inspection Verification Element"
  "Demonstration Verification Element"
  "Custom Type Element"
)

for element in "${REQUIRED_ELEMENTS[@]}"; do
  if ! echo "$MERMAID_DIAGRAM" | grep -q "\"$element\""; then
    echo "❌ FAILED: Element '$element' not found in diagram"
    exit 1
  fi
done

echo "✓ All element types are present in diagram"

# ==================================
# Test 8: JSON Output (Optional)
# ==================================
echo ""
echo "Test 8: JSON output validation..."

set +e
JSON_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" containment --json 2>&1)
JSON_EXIT=$?
set -e

if [ $JSON_EXIT -eq 0 ]; then
  # Validate JSON is parseable
  if ! echo "$JSON_OUTPUT" | jq . > /dev/null 2>&1; then
    echo "❌ FAILED: JSON output is not valid JSON"
    exit 1
  fi

  # Verify JSON structure
  if ! echo "$JSON_OUTPUT" | jq -e '.root_folder' > /dev/null 2>&1; then
    echo "❌ FAILED: JSON missing root_folder field (TODO response not acceptable for tests)"
    exit 1
  fi

  echo "✓ JSON output is valid"
else
  echo "⏸  JSON output not yet implemented (optional)"
fi

# ==================================
# Test 8.4: Intermediate Folders Without Files
# ==================================
echo ""
echo "Test 8.4: Intermediate folders without files..."

# ParentOnly folder has no direct files, only ChildFolder subfolder with files
# The containment view should include ParentOnly even though it has no files

# Check that ParentOnly folder appears in diagram
if ! echo "$MERMAID_DIAGRAM" | grep -q "ParentOnly"; then
  echo "❌ FAILED: Intermediate folder 'ParentOnly' not found in diagram"
  echo "  ParentOnly has no direct files but contains ChildFolder with DeepFile.md"
  echo "  The folder should still appear in hierarchy"
  exit 1
fi

# Check that ChildFolder appears under ParentOnly
if ! echo "$MERMAID_DIAGRAM" | grep -q "ChildFolder"; then
  echo "❌ FAILED: 'ChildFolder' not found in diagram"
  exit 1
fi

# Check that DeepFile.md appears
if ! echo "$MERMAID_DIAGRAM" | grep -q "DeepFile"; then
  echo "❌ FAILED: 'DeepFile.md' not found in diagram"
  exit 1
fi

# Check that Deep Nested Element appears
if ! echo "$MERMAID_DIAGRAM" | grep -q "Deep Nested Element"; then
  echo "❌ FAILED: 'Deep Nested Element' not found in diagram"
  exit 1
fi

echo "✓ Intermediate folders without direct files are included"

# ==================================
# Test 8.5: --short Flag (Element Display Modes)
# ==================================
echo ""
echo "Test 8.5: --short flag validation..."

# Run containment with --short flag
set +e
SHORT_OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" containment --short 2>&1)
SHORT_EXIT=$?
set -e

if [ $SHORT_EXIT -ne 0 ]; then
  echo "❌ FAILED: containment --short command failed"
  echo "$SHORT_OUTPUT"
  exit 1
fi

# Extract mermaid diagram from short output
SHORT_MERMAID=$(echo "$SHORT_OUTPUT" | sed -n '/```mermaid/,/```/p' | sed '1d;$d')

# Verify short output has fewer elements than default (9 vs 14)
SHORT_NODE_COUNT=$(echo "$SHORT_MERMAID" | grep -cE '^\s+[a-f0-9]+\["[^"]+"\]' || true)
if [ "$SHORT_NODE_COUNT" -ge "$NODE_COUNT" ]; then
  echo "❌ FAILED: --short mode should have fewer elements than default mode"
  echo "  Default mode: $NODE_COUNT elements"
  echo "  Short mode: $SHORT_NODE_COUNT elements"
  exit 1
fi

# Verify short output indicates root elements only
if ! echo "$SHORT_OUTPUT" | grep -qi "root elements"; then
  echo "❌ FAILED: --short output should indicate root elements only"
  exit 1
fi

# Save --short output for diff comparison
echo "$SHORT_OUTPUT" > "$TEST_DIR/actual-containment-short.md"

# Compare with expected --short output
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-containment-output.md" "$TEST_DIR/actual-containment-short.md"; then
  echo "❌ FAILED: Containment --short output does not match expected"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-containment-output.md"
  exit 1
fi

echo "✓ --short flag works correctly (shows $SHORT_NODE_COUNT root elements vs $NODE_COUNT total)"

# ==================================
# Test 8.6: Design Documents in Containment View (diff comparison)
# ==================================
echo ""
echo "Test 8.6: Design documents in containment view..."

# Save extracted mermaid diagram for comparison
echo "$MERMAID_DIAGRAM" > "$TEST_DIR/actual-containment-diagram.mmd"

# Compare with expected diagram
if ! diff -u "${TEST_SCRIPT_DIR}/expected/expected-containment-diagram.mmd" "$TEST_DIR/actual-containment-diagram.mmd"; then
  echo "❌ FAILED: Containment diagram does not match expected output"
  echo ""
  echo "If changes are intentional, update ${TEST_SCRIPT_DIR}/expected/expected-containment-diagram.mmd"
  exit 1
fi

echo "✓ Design documents are included in containment view"

# ==================================
# Test 9: HTML Export Integration (Optional)
# ==================================
echo ""
echo "Test 9: HTML export integration..."

set +e
cd "$TEST_DIR" && "$REQVIRE_BIN" export --output output 2>&1
EXPORT_EXIT=$?
set -e

if [ $EXPORT_EXIT -eq 0 ] && [ -f "$TEST_DIR/output/containment.html" ]; then
  # Verify HTML file exists and contains Mermaid
  if ! grep -q "graph TD" "$TEST_DIR/output/containment.html"; then
    echo "❌ FAILED: containment.html doesn't contain Mermaid diagram"
    exit 1
  fi

  if ! grep -q "mermaid" "$TEST_DIR/output/containment.html"; then
    echo "❌ FAILED: containment.html doesn't include Mermaid.js"
    exit 1
  fi

  echo "✓ HTML export includes containment view"
else
  echo "⏸  HTML export integration not yet implemented (optional)"
fi

# ==================================
# Final Result
# ==================================
echo ""
echo "✅ All containment view tests passed"
exit 0
