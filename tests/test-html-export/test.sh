#!/bin/bash
set -uo pipefail  # NOTE: Do NOT use -e, it causes silent failures with diff

# Get the directory where this test script is located
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: HTML Export
# ----------------------------------------------------
# Acceptance Criteria:
# - System should export specifications to HTML format
# - HTML files should be saved in the designated output location
# - HTML output should maintain the structure and content of the original specifications
# - SpecificationIndex.md should be renamed to index.html in output
# - Links in diagrams and text must be converted to use .html instead of .md
# - Paths in HTML files should maintain the original relative structure
# - System should work in environments without Git repositories
# - The .git directory should not be exported
# - index.html should be the SPA Explorer shell and Project Store host
#
# Test Criteria:
# - Command exits with success (0) return code
# - HTML files are generated at the expected location with .html extension
# - SpecificationIndex.md is converted to index.html
# - HTML content preserves the structure and information from the source files
# - Links in HTML files use .html extension instead of .md
# - Mermaid click links are properly converted from .md to .html
# - Both GitHub-style URLs and direct file paths in mermaid click links are handled correctly
# - Paths should not have duplicated folder names (e.g., specifications/specifications)
# - The .git directory is not present in export output
# - Project Store seed and canonical route markers are present in index.html
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

# Check that output message indicates HTML files were generated
if ! echo "$OUTPUT" | grep -q "Total Markdown files exported: [0-9]"; then
  echo "❌ FAILED: Output message doesn't indicate successful HTML conversion"
  exit 1
fi

# Count the number of Markdown files in the specifications folder
MD_FILE_COUNT=$(find "${TEST_DIR}/specifications" -name "*.md" | wc -l)

# Count the number of HTML files in the output folder
HTML_FILE_COUNT=$(find "${TEST_DIR}/output" -name "*.html" | wc -l)

# Verify that at least one HTML file was generated
if [ $HTML_FILE_COUNT -eq 0 ]; then
  echo "❌ FAILED: No HTML files were generated"
  exit 1
fi

# Verify HTML files match the number of Markdown files
if [ $HTML_FILE_COUNT -lt $MD_FILE_COUNT ]; then
  echo "❌ FAILED: Number of HTML files ($HTML_FILE_COUNT) is less than Markdown files ($MD_FILE_COUNT)"
  exit 1
fi

# Check if README.md was converted to index.html
if [ -f "${TEST_DIR}/SpecificationIndex.md" ] && [ ! -f "${TEST_DIR}/output/index.html" ]; then
  echo "❌ FAILED: SpecificationIndex.md was not converted to index.html"
  exit 1
fi

if [ -f "${TEST_DIR}/output/index.html" ]; then
  # index.html is the compiled React Explorer SPA bundle with the seed injected.
  if ! grep -q '<div id="root"></div>' "${TEST_DIR}/output/index.html"; then
    echo "❌ FAILED: index.html must be the Reqvire Explorer SPA bundle (mount point)"
    exit 1
  fi

  if ! grep -q "assets/explorer.js" "${TEST_DIR}/output/index.html" \
     || [ ! -f "${TEST_DIR}/output/assets/explorer.js" ] \
     || [ ! -f "${TEST_DIR}/output/assets/explorer.css" ]; then
    echo "❌ FAILED: index.html must reference the compiled Explorer bundle assets"
    exit 1
  fi

  if ! grep -q "reqvireProjectStore" "${TEST_DIR}/output/index.html"; then
    echo "❌ FAILED: index.html must seed the browser-local Project Store"
    exit 1
  fi

  # The legacy store.rs runtime must not be embedded in the exported bundle.
  if grep -q 'id="reqvire-explorer-runtime"' "${TEST_DIR}/output/index.html" \
     || grep -q "ReqvireExplorerStore" "${TEST_DIR}/output/index.html"; then
    echo "❌ FAILED: index.html must not embed the legacy store.rs Explorer runtime"
    exit 1
  fi

  INDEX_FILE="${TEST_DIR}/output/index.html" node - <<'NODE'
const fs = require('fs');
const html = fs.readFileSync(process.env.INDEX_FILE, 'utf8');
const projectStoreScript = html.match(/<script[^>]*id=["']reqvire-project-store["'][^>]*>([\s\S]*?)<\/script>/);
if (!projectStoreScript) {
  console.error('index.html missing reqvire-project-store seed script');
  process.exit(1);
}
try {
  new Function(projectStoreScript[1]);
} catch (error) {
  console.error(`reqvire-project-store seed must parse as JavaScript: ${error.message}`);
  process.exit(1);
}
NODE
fi

# Helper function to compare files and show diff on failure
assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "❌ FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# Check basic HTML structure generation - compare against expected files
if [ -f "${TEST_DIR}/output/specifications/TestRequirements.html" ]; then
  assert_file_matches "${TEST_SCRIPT_DIR}/expected/TestRequirements.html" \
    "${TEST_DIR}/output/specifications/TestRequirements.html" \
    "TestRequirements.html does not match expected output"
fi

# Explicit diagram assertions (beyond full-file golden match)
# - auto-generated marker present
# - expected rendered relationship labels present
# - reverse/opposite labels not rendered in diagram output for this fixture
if [ -f "${TEST_DIR}/output/specifications/TestRequirements.html" ]; then
  TEST_REQUIREMENTS_HTML="${TEST_DIR}/output/specifications/TestRequirements.html"

  if ! grep -q "REQVIRE-AUTOGENERATED-DIAGRAM" "$TEST_REQUIREMENTS_HTML"; then
    echo "❌ FAILED: Diagram marker REQVIRE-AUTOGENERATED-DIAGRAM missing"
    exit 1
  fi

  if ! grep -q "|deriveReqT|" "$TEST_REQUIREMENTS_HTML"; then
    echo "❌ FAILED: Expected deriveReqT relationship label not found in diagram output"
    exit 1
  fi

  if ! grep -q "|verifiedBy|" "$TEST_REQUIREMENTS_HTML"; then
    echo "❌ FAILED: Expected verifiedBy relationship label not found in diagram output"
    exit 1
  fi

  if grep -q "|derivedFrom|" "$TEST_REQUIREMENTS_HTML"; then
    echo "❌ FAILED: Diagram should not render reverse relation label derivedFrom for this fixture"
    exit 1
  fi

  if grep -q "|verify|" "$TEST_REQUIREMENTS_HTML"; then
    echo "❌ FAILED: Diagram should not render reverse relation label verify for this fixture"
    exit 1
  fi
fi

# Check MixedLinkTypes.html if it exists
if [ -f "${TEST_DIR}/output/specifications/Subfolder/MixedLinkTypes.html" ]; then
  assert_file_matches "${TEST_SCRIPT_DIR}/expected/Subfolder/MixedLinkTypes.html" \
    "${TEST_DIR}/output/specifications/Subfolder/MixedLinkTypes.html" \
    "MixedLinkTypes.html does not match expected output"
fi

# ============================================================
# Exclusion Tests
# Verify that .git directory is not exported
# ============================================================

# Check that .git directory is NOT exported
if [ -d "${TEST_DIR}/output/.git" ]; then
  echo "❌ FAILED: .git directory should not be exported"
  exit 1
fi

exit 0
