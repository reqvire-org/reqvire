#!/bin/bash

# Test: Subdirectory Flag Functionality
# ----------------------------------------------------
# Acceptance Criteria:
# - System should process only files within specified subdirectory when --subdirectory flag is used
# - System should handle identifier normalization correctly within subdirectory context
# - System should validate cross-references correctly even when they point outside subdirectory
# - System should work with validate, model-summary, html, and other commands
#
# Test Criteria:
# - Commands with --subdirectory flag should process only submodule files
# - Identifier normalization should work correctly for paths within and outside subdirectory
# - Commands should exit with success (0) return code when subdirectory processing works
# - Commands should fail gracefully if identifier normalization fails

# Setup: Create a git repository to simulate real environment
cd "${TEST_DIR}/project-root"
git init > /dev/null 2>&1
git config --local user.email "test@example.com" > /dev/null 2>&1
git config --local user.name "Test User" > /dev/null 2>&1
git add . > /dev/null 2>&1
git commit -m "Initial test structure" > /dev/null 2>&1

# Test 1: Validate with subdirectory flag - should focus only on submodule
echo "Testing validate command with --subdirectory flag..."
OUTPUT=$(cd "${TEST_DIR}/project-root" && "$REQVIRE_BIN" --config "../reqvire.yaml" --subdirectory submodule validate 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_validate.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Validate with --subdirectory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that validation mentions only submodule files
if echo "$OUTPUT" | grep -q "MainRequirements.md"; then
  echo "❌ FAILED: Validation processed main requirements when it should only process submodule"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test 2: Model summary with subdirectory flag
echo "Testing model-summary command with --subdirectory flag..."
OUTPUT=$(cd "${TEST_DIR}/project-root" && "$REQVIRE_BIN" --config "../reqvire.yaml" --subdirectory submodule model-summary 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_model_summary.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Model summary with --subdirectory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that model summary shows only submodule elements
if echo "$OUTPUT" | grep -q "Main Requirement One"; then
  echo "❌ FAILED: Model summary included main requirements when it should only process submodule"
  echo "Output: $OUTPUT"
  exit 1
fi

# Should include submodule requirements
if ! echo "$OUTPUT" | grep -q "Submodule Requirement One"; then
  echo "❌ FAILED: Model summary did not include submodule requirements"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test 3: HTML export with subdirectory flag
echo "Testing HTML export with --subdirectory flag..."
OUTPUT=$(cd "${TEST_DIR}/project-root" && "$REQVIRE_BIN" --config "../reqvire.yaml" --subdirectory submodule html --output subdirectory-html 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_html.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: HTML export with --subdirectory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that HTML was generated only for submodule
if [ -f "${TEST_DIR}/project-root/subdirectory-html/specifications/MainRequirements.html" ]; then
  echo "❌ FAILED: HTML export included main requirements when it should only process submodule"
  exit 1
fi

if [ ! -f "${TEST_DIR}/project-root/subdirectory-html/submodule/specifications/SubmoduleRequirements.html" ]; then
  echo "❌ FAILED: HTML export did not create submodule requirements file"
  exit 1
fi

# Test 4: Lint with subdirectory flag
echo "Testing lint command with --subdirectory flag..."
OUTPUT=$(cd "${TEST_DIR}/project-root" && "$REQVIRE_BIN" --config "../reqvire.yaml" --subdirectory submodule lint --dry-run 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_lint.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Lint with --subdirectory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test 5: Traces with subdirectory flag
echo "Testing traces command with --subdirectory flag..."
OUTPUT=$(cd "${TEST_DIR}/project-root" && "$REQVIRE_BIN" --config "../reqvire.yaml" --subdirectory submodule traces 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_traces.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Traces with --subdirectory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "✅ All subdirectory functionality tests passed"
exit 0