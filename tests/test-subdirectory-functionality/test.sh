#!/bin/bash

# Test: Subdirectory Auto-Detection Functionality
# ----------------------------------------------------
# Acceptance Criteria:
# - System should process only files within current directory when run from a subfolder
# - System should handle identifier normalization correctly within subdirectory context
# - System should validate cross-references correctly even when they point outside subdirectory
# - System should work with validate, model-summary, html, and other commands
#
# Test Criteria:
# - Commands run from subdirectory should process only submodule files
# - Identifier normalization should work correctly for paths within and outside subdirectory
# - Commands should exit with success (0) return code when subdirectory processing works
# - Commands should fail gracefully if identifier normalization fails

# Create a unique temporary directory
TMP_DIR=$(mktemp -d -t reqvire-subdirectory-test-XXXXXX)
cp -a "${TEST_DIR}/." "${TMP_DIR}/"
mkdir -p "${TMP_DIR}/output"

# Create simple git repository to test changes
cd "${TMP_DIR}/project-root"
git init > /dev/null 2>&1
git config --local user.email "test@example.com" > /dev/null 2>&1 
git config --local user.name "Test User" > /dev/null 2>&1
git remote add origin 'https://dummy.example.com/dummy-repo.git' > /dev/null 2>&1
git add . > /dev/null 2>&1
git commit -m "Initial test structure" > /dev/null 2>&1

# Test 1: Model summary from submodule directory - should focus only on submodule
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" model-summary 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_validate.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Model summary from submodule directory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that model summary only processed submodule files (files under 'submodule/' path)
# The output should not contain file paths outside the submodule directory for processed files
if echo "$OUTPUT" | grep -q "Main Requirement One"; then
  echo "❌ FAILED: Model summary processed main requirements when it should only process submodule"
  echo "Output: $OUTPUT"
  exit 1
fi

# Should include submodule requirements
if ! echo "$OUTPUT" | grep -q "Submodule Requirement One"; then
  echo "❌ FAILED: Model summary did not include submodule requirements"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test 2: HTML export from submodule directory
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" html --output subdirectory-html 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: HTML export from submodule directory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that HTML was generated only for submodule
if [ -f "${TMP_DIR}/project-root/submodule/subdirectory-html/specifications/MainRequirements.html" ]; then
  echo "❌ FAILED: HTML export included main requirements when it should only process submodule"
  exit 1
fi

if [ ! -f "${TMP_DIR}/project-root/submodule/subdirectory-html/specifications/SubmoduleRequirements.html" ]; then
  echo "❌ FAILED: HTML export did not create submodule requirements file"
  exit 1
fi

# Test 4: Lint from submodule directory
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" lint --dry-run 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Lint from submodule directory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test 5: Traces from submodule directory
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" traces 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Traces from submodule directory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test 6: Generate diagrams from submodule directory
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" generate-diagrams 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Generate diagrams from submodule directory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that diagrams were only generated for submodule files
# The generate-diagrams command should only process files in the current subdirectory
if echo "$OUTPUT" | grep -q "specifications/MainRequirements.md"; then
  echo "❌ FAILED: Generate diagrams processed main requirements when it should only process submodule"
  echo "Output: $OUTPUT"
  exit 1
fi

# Test 7: Generate index from submodule directory
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" generate-index 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Generate index from submodule directory failed with exit code $EXIT_CODE"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that index generation only processed submodule files
# The generate-index command should only process files in the current subdirectory
if echo "$OUTPUT" | grep -q "specifications/MainRequirements.md"; then
  echo "❌ FAILED: Generate index processed main requirements when it should only process submodule"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that index was generated in the submodule directory, not repo root
if [ -f "${TMP_DIR}/project-root/SpecificationIndex.md" ]; then
  echo "❌ FAILED: Generate index created file in repo root instead of submodule directory"
  exit 1
fi

if [ ! -f "${TMP_DIR}/project-root/submodule/SpecificationIndex.md" ]; then
  echo "❌ FAILED: Generate index did not create file in submodule directory"
  exit 1
fi

exit 0
