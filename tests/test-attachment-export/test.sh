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
# Test 2: SPA Project Store contains attachment links
# ============================================================
# Check that index.html (SPA bundle) exists and standalone containment is absent.
if [ ! -f "${TEST_DIR}/output/index.html" ]; then
  echo "❌ FAILED: index.html was not generated"
  exit 1
fi
CONTAINMENT_ENTRY="${TEST_DIR}/output/containment"'.html'
if [ -f "$CONTAINMENT_ENTRY" ]; then
  echo "❌ FAILED: standalone containment page must not be generated"
  exit 1
fi

INDEX_FILE="${TEST_DIR}/output/index.html" node - <<'NODE'
const fs = require('fs');
const html = fs.readFileSync(process.env.INDEX_FILE, 'utf8');
const match = html.match(/(?:const|let|var)\s+reqvireProjectStore\s*=\s*(\{[\s\S]*?\});\s*<\/script>/);
if (!match) {
  console.error('❌ FAILED: index.html missing Project Store seed');
  process.exit(1);
}
const store = JSON.parse(match[1]);
const serialized = JSON.stringify({
  attachments: store.attachments,
  elements: store.elements,
  relations: store.relations,
  resources: store.resources,
});
if (!serialized.includes('Design Spec Contract') || !serialized.includes('docs/DesignSpec.md')) {
  console.error('❌ FAILED: Project Store does not contain attachment evidence for Design Spec Contract');
  process.exit(1);
}
NODE

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
# Element Name<br/>📎 Design Spec Contract

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
# Visible diagram labels should stay compact; full identifiers remain link targets or structured data.
if ! grep -q '📎.*Design Spec Contract' "$SPEC_HTML"; then
  echo "❌ FAILED: Diagram does not show 📎 icon with attached refinement display name"
  exit 1
fi

if grep -q '📎.*docs/DesignSpec.md#design-spec-contract' "$SPEC_HTML"; then
  echo "❌ FAILED: Diagram visible label should not expose full attached refinement identifier"
  exit 1
fi

exit 0
