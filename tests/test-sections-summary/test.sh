#!/usr/bin/env bash
set -euo pipefail

# Test: Sections Summary Command Functionality
# -------------------------------------------------------------------------------------
# Acceptance Criteria:
# - `reqvire model section-summary --json` produces valid JSON with sections information
# - `reqvire model section-summary` prints human-readable text summary
# - Both outputs include file paths, section names, section order indices, and section content
# - Individual elements (requirements, verifications) are excluded from output
# - Filter flags work independently and in combination
# - Section order indices preserve original document structure
# - JSON output includes section order information for document reconstruction
#
# Test Criteria:
# - Commands exit 0 on success
# - JSON output parses under jq
# - Text output shows files and sections without individual elements
# - Filters reduce section counts as expected
# - Section order is preserved and displayed

# Test 1: Base JSON sections summary
echo "Starting test..." > "${TEST_DIR}/test_results.log"

echo "Running: reqvire model section-summary --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model section-summary --json 2>&1)
EXIT_JSON=$?
set -e

echo "Exit code: $EXIT_JSON" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_JSON -ne 0 ]; then
  echo "❌ FAILED: base JSON sections summary exited $EXIT_JSON"
  cat "${TEST_DIR}/test_results.log"
  exit 1
fi

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if ! echo "$OUTPUT" | jq 'has("sections")' | grep -q true; then
  echo "❌ FAILED: JSON missing sections key"
  exit 1
fi

# Verify we have sections from the expected files
if ! echo "$OUTPUT" | jq '[.sections[] | select(.file == "specifications/Requirements.md")] | length > 0' | grep -q true; then
  echo "❌ FAILED: JSON missing sections from specifications/Requirements.md"
  exit 1
fi

if ! echo "$OUTPUT" | jq '[.sections[] | select(.file == "specifications/UserRequirements.md")] | length > 0' | grep -q true; then
  echo "❌ FAILED: JSON missing sections from specifications/UserRequirements.md"
  exit 1
fi

# Check that sections have required fields: file, name, content, section_order
REQUIREMENTS_SECTIONS=$(echo "$OUTPUT" | jq '[.sections[] | select(.file == "specifications/Requirements.md")] | length')
if [ "$REQUIREMENTS_SECTIONS" -lt 2 ]; then
  echo "❌ FAILED: Expected at least 2 sections from specifications/Requirements.md, got $REQUIREMENTS_SECTIONS"
  exit 1
fi

# Verify section structure includes all required fields
if ! echo "$OUTPUT" | jq '.sections[0] | has("file") and has("name") and has("content") and has("section_order")' | grep -q true; then
  echo "❌ FAILED: Section missing required fields"
  exit 1
fi

# Verify no individual elements are included (sections should only have file, name, content, section_order)
UNEXPECTED_FIELDS=$(echo "$OUTPUT" | jq '[.sections[] | keys[] | select(. != "file" and . != "name" and . != "content" and . != "section_order")] | length')
if [ "$UNEXPECTED_FIELDS" -gt 0 ]; then
  echo "❌ FAILED: JSON output contains unexpected fields beyond file, name, content, section_order"
  exit 1
fi

# Test 2: Base text sections summary
echo "Running: reqvire model section-summary" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_TEXT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model section-summary 2>&1)
EXIT_TEXT=$?
set -e

echo "Exit code: $EXIT_TEXT" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_TEXT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_TEXT -ne 0 ]; then
  echo "❌ FAILED: base text sections summary exited $EXIT_TEXT"
  exit 1
fi

# Check text output shows sections and files
if ! echo "$OUTPUT_TEXT" | grep -q "📋 Section: System Requirements"; then
  echo "❌ FAILED: Text output missing System Requirements section"
  exit 1
fi

if ! echo "$OUTPUT_TEXT" | grep -q "File: specifications/Requirements.md"; then
  echo "❌ FAILED: Text output missing specifications/Requirements.md"
  exit 1
fi

# Verify no individual elements are shown in text output
if echo "$OUTPUT_TEXT" | grep -q "🔹 Element:"; then
  echo "❌ FAILED: Text output should not contain individual elements"
  exit 1
fi

# Verify text output is section-oriented (section first, then file as field)
if ! echo "$OUTPUT_TEXT" | grep -B1 "File: specifications/Requirements.md" | grep -q "📋 Section:"; then
  echo "❌ FAILED: Text output should show sections first, then file as field"
  exit 1
fi

# Test 3: File filter
echo "Running: reqvire model section-summary --filter-file --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_FILTERED=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model section-summary --filter-file="**/Requirements.md" --json 2>&1)
EXIT_FILTERED=$?
set -e

echo "Exit code: $EXIT_FILTERED" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_FILTERED" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_FILTERED -ne 0 ]; then
  echo "❌ FAILED: file filter test exited $EXIT_FILTERED"
  echo "Output: $OUTPUT_FILTERED"
  exit 1
fi


# Should only have sections from Requirements.md (not UserRequirements.md)
USER_REQ_SECTIONS=$(echo "$OUTPUT_FILTERED" | jq '[.sections[] | select(.file | contains("UserRequirements.md"))] | length')
if [ "$USER_REQ_SECTIONS" -gt 0 ]; then
  echo "❌ FAILED: File filter should exclude UserRequirements.md"
  exit 1
fi

REQ_SECTIONS=$(echo "$OUTPUT_FILTERED" | jq '[.sections[] | select(.file | endswith("Requirements.md"))] | length')
if [ "$REQ_SECTIONS" -eq 0 ]; then
  echo "❌ FAILED: File filter should include Requirements.md"
  exit 1
fi

# Test 4: Section filter
echo "Running: reqvire model section-summary --filter-section --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_SECTION_FILTERED=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model section-summary --filter-section="System*" --json 2>&1)
EXIT_SECTION_FILTERED=$?
set -e

echo "Exit code: $EXIT_SECTION_FILTERED" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_SECTION_FILTERED" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_SECTION_FILTERED -ne 0 ]; then
  echo "❌ FAILED: section filter test exited $EXIT_SECTION_FILTERED"
  echo "Output: $OUTPUT_SECTION_FILTERED"
  exit 1
fi


# Should have sections matching "System*"
SYSTEM_SECTIONS=$(echo "$OUTPUT_SECTION_FILTERED" | jq '[.sections[] | select(.name | startswith("System"))] | length')
if [ "$SYSTEM_SECTIONS" -eq 0 ]; then
  echo "❌ FAILED: Section filter should find sections starting with 'System'"
  exit 1
fi

# Test 5: Content filter
echo "Running: reqvire model section-summary --filter-content --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_CONTENT_FILTERED=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model section-summary --filter-content="MUST" --json 2>&1)
EXIT_CONTENT_FILTERED=$?
set -e

echo "Exit code: $EXIT_CONTENT_FILTERED" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_CONTENT_FILTERED" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CONTENT_FILTERED -ne 0 ]; then
  echo "❌ FAILED: content filter test exited $EXIT_CONTENT_FILTERED"
  echo "Output: $OUTPUT_CONTENT_FILTERED"
  exit 1
fi


# Should have sections containing "MUST"
MUST_SECTIONS=$(echo "$OUTPUT_CONTENT_FILTERED" | jq '[.sections[] | select(.content | test("MUST"))] | length')
if [ "$MUST_SECTIONS" -eq 0 ]; then
  echo "❌ FAILED: Content filter should find sections containing 'MUST'"
  exit 1
fi

# Test 6: Combined filters
echo "Running: reqvire model section-summary --filter-file --filter-section --json" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT_COMBINED=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" model section-summary --filter-file="**/Requirements.md" --filter-section="System*" --json 2>&1)
EXIT_COMBINED=$?
set -e

echo "Exit code: $EXIT_COMBINED" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT_COMBINED" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_COMBINED -ne 0 ]; then
  echo "❌ FAILED: combined filters test exited $EXIT_COMBINED"
  echo "Output: $OUTPUT_COMBINED"
  exit 1
fi


# Should only have sections from Requirements.md with System sections
COMBINED_USER_SECTIONS=$(echo "$OUTPUT_COMBINED" | jq '[.sections[] | select(.file | contains("UserRequirements.md"))] | length')
if [ "$COMBINED_USER_SECTIONS" -gt 0 ]; then
  echo "❌ FAILED: Combined filter should exclude UserRequirements.md"
  exit 1
fi

COMBINED_SECTIONS=$(echo "$OUTPUT_COMBINED" | jq '[.sections[] | select(.name | startswith("System"))] | length')
if [ "$COMBINED_SECTIONS" -eq 0 ]; then
  echo "❌ FAILED: Combined filter should find System sections"
  exit 1
fi

# Test 7: Section order preservation
# Verify that sections are properly ordered by file and section_order
FIRST_FILE_ORDERS=$(echo "$OUTPUT" | jq -r '[.sections[] | select(.file == "specifications/Requirements.md") | .section_order] | @csv')
if [ "$FIRST_FILE_ORDERS" != "0,1" ]; then
  echo "❌ FAILED: Section order should be preserved within each file, got: $FIRST_FILE_ORDERS"
  echo "$OUTPUT"
  exit 1
fi

# Test 8: Auto-injected Requirements section
# Verify that when elements appear before first ## section, a default "Requirements" section is created
EARLY_ELEMENT_SECTIONS=$(echo "$OUTPUT" | jq -r '[.sections[] | select(.file == "specifications/EarlyElement.md") | .name] | @csv')
if ! echo "$EARLY_ELEMENT_SECTIONS" | grep -q "Requirements"; then
  echo "❌ FAILED: Auto-injected Requirements section not found in EarlyElement.md, got: $EARLY_ELEMENT_SECTIONS"
  exit 1
fi

# Verify Requirements section has order 0
EARLY_REQUIREMENTS_ORDER=$(echo "$OUTPUT" | jq -r '.sections[] | select(.file == "specifications/EarlyElement.md" and .name == "Requirements") | .section_order')
if [ "$EARLY_REQUIREMENTS_ORDER" != "0" ]; then
  echo "❌ FAILED: Auto-injected Requirements section should have order 0, got: $EARLY_REQUIREMENTS_ORDER"
  exit 1
fi

# Verify First Explicit Section has order 1
EARLY_FIRST_SECTION_ORDER=$(echo "$OUTPUT" | jq -r '.sections[] | select(.file == "specifications/EarlyElement.md" and .name == "First Explicit Section") | .section_order')
if [ "$EARLY_FIRST_SECTION_ORDER" != "1" ]; then
  echo "❌ FAILED: First Explicit Section should have order 1, got: $EARLY_FIRST_SECTION_ORDER"
  exit 1
fi

exit 0