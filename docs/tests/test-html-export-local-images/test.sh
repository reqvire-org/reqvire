#!/bin/bash
set -uo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: HTML Export Local Linked Files
# ----------------------------------------------------
# Acceptance Criteria:
# - Exported HTML shall preserve local linked-file paths
# - Referenced local files shall be copied into export output
# - Asset paths shall remain asset paths, not rewritten to .html
#
# Test Criteria:
# - Command exits with success (0) return code
# - HTML file contains expected href and img src paths
# - Referenced linked files exist in output
#

echo "Running: reqvire export --output output" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" export --output output 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: HTML export command failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

HTML_FILE="${TEST_DIR}/output/specifications/Requirements.html"
IMAGE_FILE="${TEST_DIR}/output/specifications/images/local-diagram.svg"
DOC_FILE="${TEST_DIR}/output/specifications/docs/reference.txt"

if [ ! -f "$HTML_FILE" ]; then
  echo "❌ FAILED: Expected HTML file not found: $HTML_FILE"
  exit 1
fi

if ! grep -q 'img src="images/local-diagram.svg"' "$HTML_FILE"; then
  echo "❌ FAILED: Exported HTML does not preserve the local image src path"
  grep -n "img src" "$HTML_FILE" || true
  exit 1
fi

if grep -q 'images/local-diagram.html' "$HTML_FILE"; then
  echo "❌ FAILED: Image path was incorrectly rewritten to .html"
  exit 1
fi

if [ ! -f "$IMAGE_FILE" ]; then
  echo "❌ FAILED: Referenced local image file was not copied to output: $IMAGE_FILE"
  find "${TEST_DIR}/output" -type f | sort
  exit 1
fi

if ! grep -q 'href="docs/reference.txt"' "$HTML_FILE"; then
  echo "❌ FAILED: Exported HTML does not preserve the local file href path"
  grep -n "reference.txt\\|href=" "$HTML_FILE" || true
  exit 1
fi

if grep -q 'docs/reference.html' "$HTML_FILE"; then
  echo "❌ FAILED: Local file path was incorrectly rewritten to .html"
  exit 1
fi

if [ ! -f "$DOC_FILE" ]; then
  echo "❌ FAILED: Referenced local file was not copied to output: $DOC_FILE"
  find "${TEST_DIR}/output" -type f | sort
  exit 1
fi

exit 0
