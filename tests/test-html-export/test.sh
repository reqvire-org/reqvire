#!/bin/bash

# Test: HTML Export
# ----------------------------------------------------
# Acceptance Criteria:
# - System should export specifications to HTML format
# - HTML files should be saved in the designated output location
# - HTML output should maintain the structure and content of the original specifications
# - README.md should be renamed to index.html in output
# - Links in diagrams and text must be converted to use .html instead of .md
#
# Test Criteria:
# - Command exits with success (0) return code
# - HTML files are generated at the expected location with .html extension
# - README.md is converted to index.html
# - HTML content preserves the structure and information from the source files
# - Links in HTML files use .html extension instead of .md
#

# Create output directory if it doesn't exist
mkdir -p "${TEST_DIR}/output"

# First generate diagrams
DIAGRAM_OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml" --generate-diagrams 2>&1)
DIAGRAM_EXIT_CODE=$?

if [ $DIAGRAM_EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Diagram generation failed with exit code $DIAGRAM_EXIT_CODE"
  exit 1
fi

# Now run reqflow with --html flag
OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml" --html 2>&1)
EXIT_CODE=$?

# Save output for inspection
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# Verify exit code indicates success
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: HTML export command failed with exit code $EXIT_CODE"
  exit 1
fi

# Check that output message indicates HTML files were generated
if ! echo "$OUTPUT" | grep -q "markdown files converted to HTML"; then
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
if [ -f "${TEST_DIR}/specifications/README.md" ] && [ ! -f "${TEST_DIR}/output/index.html" ]; then
  echo "❌ FAILED: README.md was not converted to index.html"
  exit 1
fi

# Check basic HTML structure generation
FIRST_HTML=$(find "${TEST_DIR}/output" -name "*.html" -not -name "index.html" | head -n 1)
if [ -n "$FIRST_HTML" ]; then
  # Check for presence of HTML structure
  if ! grep -q "<html" "$FIRST_HTML"; then
    echo "❌ FAILED: Generated file doesn't contain basic HTML structure"
    exit 1
  fi
  
  # Check for proper HTML conversion of Markdown headers
  if ! grep -q "<h1\|<h2\|<h3" "$FIRST_HTML"; then
    echo "❌ FAILED: HTML doesn't contain heading tags"
    exit 1
  fi
  
  # Check for presence of README.md to index.html conversion in links
  if grep -q "README.md" "$FIRST_HTML"; then
    echo "❌ FAILED: HTML file contains unconverted README.md references"
    exit 1
  fi
  
  # Check for presence of mermaid diagrams in HTML files
  if grep -q "mermaid" "$FIRST_HTML"; then
    
    # Verify that no raw .md references exist in the HTML
    if grep -q "\.md\"" "$FIRST_HTML"; then
      echo "❌ FAILED: HTML file contains unconverted .md links"
      exit 1
    fi
    
    echo "✓ HTML file link format validation passed"
  fi
fi

echo "✅ PASSED: HTML export test"
exit 0
