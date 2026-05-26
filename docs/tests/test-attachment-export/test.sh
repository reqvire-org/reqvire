#!/bin/bash
set -uo pipefail

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Attachment Export
# ----------------------------------------------------
# Acceptance Criteria:
# - System shall copy all attachment files during HTML export
# - Attachment files shall preserve their relative paths in output
# - Index view shall show attachment links under elements with 📎 icon
# - Diagrams shall display attachment filenames in element boxes
#
# Test Criteria:
# - Attachment files exist in output directory at correct paths
# - index.html contains attachment links with 📎 icon
# - Mermaid diagrams contain attachment labels with <br/> formatting
#

# Generate HTML
echo "Running: reqvire export --output output" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" export --output output 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Verify exit code indicates success
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: HTML export command failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# ============================================================
# Test 1: Attachment files are exported
# ============================================================

# Check that attachment file was copied to output (may be converted to HTML)
if [ ! -f "${TEST_DIR}/output/docs/DesignSpec.md" ] && [ ! -f "${TEST_DIR}/output/docs/DesignSpec.html" ]; then
  echo "❌ FAILED: Attachment file 'docs/DesignSpec.md' or 'docs/DesignSpec.html' was not exported"
  echo "Contents of output directory:"
  find "${TEST_DIR}/output" -type f
  exit 1
fi

# ============================================================
# Test 2: Index view shows attachment links
# ============================================================

# Check that index.html exists
if [ ! -f "${TEST_DIR}/output/index.html" ]; then
  echo "❌ FAILED: index.html was not generated"
  exit 1
fi

# Check for paperclip icon in index
if ! grep -q "📎" "${TEST_DIR}/output/index.html"; then
  echo "❌ FAILED: Index does not contain attachment links with 📎 icon"
  echo "Searching for attachment links in index.html..."
  grep -i "attachment\|DesignSpec" "${TEST_DIR}/output/index.html" || true
  exit 1
fi

# Check that attachment filename appears in index (may be converted to .html)
if ! grep -q "DesignSpec" "${TEST_DIR}/output/index.html"; then
  echo "❌ FAILED: Index does not contain attachment filename 'DesignSpec'"
  exit 1
fi

# ============================================================
# Test 3: Diagrams show attachments in element boxes
# ============================================================

# Find HTML file with diagrams
SPEC_HTML="${TEST_DIR}/output/specifications/Requirements.html"
if [ ! -f "$SPEC_HTML" ]; then
  echo "❌ FAILED: Specification HTML file not found: $SPEC_HTML"
  exit 1
fi

# Check that mermaid diagram contains attachment with line break
# The attachment should appear in the element node label with format:
# Element Name<br/>📎 DesignSpec.md

# Extract expected content (trimmed)
EXPECTED_PATTERN=$(cat "${TEST_SCRIPT_DIR}/expected/expected-diagram-content.txt" | head -1)

# Check for the expected pattern in the HTML file
if ! grep -qF "$EXPECTED_PATTERN" "$SPEC_HTML"; then
  echo "❌ FAILED: Diagram does not contain expected attachment format"
  echo "Expected pattern: $EXPECTED_PATTERN"
  echo ""
  echo "Searching for related content in $SPEC_HTML..."
  grep -o 'System Capability Requirement[^"]*' "$SPEC_HTML" | head -5 || true
  exit 1
fi

# Verify paperclip icon is in the diagram context
if ! grep -q '📎.*DesignSpec' "$SPEC_HTML"; then
  echo "❌ FAILED: Diagram does not show 📎 icon with attachment filename"
  exit 1
fi

exit 0
