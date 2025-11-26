#!/bin/bash
set -euo pipefail

# Create log file immediately to ensure it exists for runner
echo "Starting test..." > "${TEST_DIR}/test_results.log"

# Test: Subdirectory Auto-Detection Functionality
# ----------------------------------------------------
# Acceptance Criteria:
# - System should process only files within current directory when run from a subfolder
# - System should handle identifier normalization correctly within subdirectory context
# - System should generate validation errors for references to parent directories
# - System should work with validate, search, html, and other commands
#
# Test Criteria:
# - Validation should fail when parent directory references are detected
# - Commands run from subdirectory should process only submodule files
# - Identifier normalization should work correctly for paths within subdirectory
# - Commands should exit with success (0) return code when subdirectory processing works
# - Validation errors should clearly identify parent directory reference issues

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

# Test 1: Model search should fail with parent directory references
echo "Running: reqvire search (from submodule, should fail)" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" search 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results_validate.log"

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Model search should have failed due to parent directory references but succeeded"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check that validation error mentions missing relation target (due to parent directory reference)
if ! echo "$OUTPUT" | grep -q "Missing relation target.*specifications/MainRequirements.md"; then
  echo "❌ FAILED: Validation error should mention missing target for parent directory reference"
  echo "Output: $OUTPUT"
  exit 1
fi


# Replace the parent directory reference with a local reference for remaining tests
sed -i 's|derivedFrom: \[.*specifications/MainRequirements.md.*\].*|derivedFrom: [Submodule System](#submodule-system)|' "${TMP_DIR}/project-root/submodule/specifications/SubmoduleRequirements.md"

# Test 2: HTML export from submodule directory
echo "Running: reqvire export --output subdirectory-html" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" export --output subdirectory-html 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: HTML export from submodule directory failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
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

# Check that HTML content has correct paths (without submodule/ prefix)
SUBMODULE_HTML="${TMP_DIR}/project-root/submodule/subdirectory-html/specifications/SubmoduleRequirements.html"

# Verify paths don't have "submodule/" prefix in links
if grep -q "submodule/specifications" "$SUBMODULE_HTML"; then
  echo "❌ FAILED: HTML contains incorrect paths with 'submodule/' prefix"
  grep "submodule/specifications" "$SUBMODULE_HTML"
  exit 1
fi

# Check that mermaid diagrams use correct paths (if any exist)
if grep -q "click.*specifications/" "$SUBMODULE_HTML"; then
  # Mermaid click links should use specifications/, not submodule/specifications/
  if grep -q "click.*submodule/specifications" "$SUBMODULE_HTML"; then
    echo "❌ FAILED: Mermaid diagrams contain incorrect paths with 'submodule/' prefix"
    grep "click.*submodule/specifications" "$SUBMODULE_HTML"
    exit 1
  fi
fi

# Check index.html and traces.html for correct file paths
for artifact in "index.html" "traces.html"; do
  ARTIFACT_PATH="${TMP_DIR}/project-root/submodule/subdirectory-html/$artifact"
  if [ -f "$ARTIFACT_PATH" ]; then
    # File paths in artifacts should be specifications/, not submodule/specifications/
    if grep -q "submodule/specifications" "$ARTIFACT_PATH"; then
      echo "❌ FAILED: $artifact contains incorrect paths with 'submodule/' prefix"
      grep "submodule/specifications" "$ARTIFACT_PATH" | head -5
      exit 1
    fi
  fi
done

# Test 3: Format from submodule directory (preview mode - default)
echo "Running: reqvire format" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" format 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Format from submodule directory failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Test 4: Traces from submodule directory
echo "Running: reqvire traces" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" traces 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Traces from submodule directory failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Test 5: Generate diagrams from submodule directory
echo "Running: reqvire generate-diagrams" >> "${TEST_DIR}/test_results.log"
set +e
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" generate-diagrams 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Generate diagrams from submodule directory failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# Check that diagrams were only generated for submodule files
# The generate-diagrams command should only process files in the current subdirectory
if echo "$OUTPUT" | grep -q "specifications/MainRequirements.md"; then
  echo "❌ FAILED: Generate diagrams processed main requirements when it should only process submodule"
  echo "Output: $OUTPUT"
  exit 1
fi

# Note: Index generation is tested in Test 2 (export command generates index.html)

# Test 6: CRUD mv command from submodule directory (move to different file in subdirectory)
echo "Running: reqvire mv (move element to different file within subdirectory)" >> "${TEST_DIR}/test_results.log"

# Create a new target file in the submodule (just a header, no elements needed - pages are tracked)
cat > "${TMP_DIR}/project-root/submodule/specifications/OtherRequirements.md" <<'EOF'
# Elements

This file will receive the moved element.
EOF

# Add to git so reqvire can find it
cd "${TMP_DIR}/project-root" && git add submodule/specifications/OtherRequirements.md && git commit -m "Add other requirements" >/dev/null 2>&1

set +e
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" mv "Submodule System" "specifications/OtherRequirements.md" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: mv command from submodule directory failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# According to requirement: "CRUD commands resolve paths relative to current working directory"
# When running from submodule/, the path "specifications/OtherRequirements.md" should be resolved
# relative to current working directory (submodule/), so it should resolve to:
# submodule/specifications/OtherRequirements.md (relative to git root)
# Verify element was moved to the file within submodule directory
if ! grep -q "### Submodule System" "${TMP_DIR}/project-root/submodule/specifications/OtherRequirements.md"; then
  echo "❌ FAILED: Element was not moved to the correct file (should be submodule/specifications/OtherRequirements.md)"
  # Debug: show where file was actually created
  echo "Checking if file was incorrectly created at git root:"
  if [ -f "${TMP_DIR}/project-root/specifications/OtherRequirements.md" ]; then
    echo "  File exists at git root (WRONG): specifications/OtherRequirements.md"
    grep "### Submodule System" "${TMP_DIR}/project-root/specifications/OtherRequirements.md" || true
  fi
  exit 1
fi

# Verify element was removed from original file
if grep -q "### Submodule System" "${TMP_DIR}/project-root/submodule/specifications/SubmoduleRequirements.md"; then
  echo "❌ FAILED: Element was not removed from original file"
  exit 1
fi

# Test 7: CRUD mv-file command from submodule directory
echo "Running: reqvire mv-file (move entire file within subdirectory)" >> "${TEST_DIR}/test_results.log"

# Create a new directory structure
mkdir -p "${TMP_DIR}/project-root/submodule/specs"

set +e
OUTPUT=$(cd "${TMP_DIR}/project-root/submodule" && "$REQVIRE_BIN" mv-file "specifications/OtherRequirements.md" "specs/Renamed.md" 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: mv-file command from submodule directory failed with exit code $EXIT_CODE"
  echo "$OUTPUT"
  exit 1
fi

# According to requirement: "CRUD commands resolve paths relative to current working directory"
# When running from submodule/, paths should be resolved relative to submodule/
# Source: submodule/specifications/OtherRequirements.md
# Target: submodule/specs/Renamed.md

# Verify source file was removed
if [ -f "${TMP_DIR}/project-root/submodule/specifications/OtherRequirements.md" ]; then
  echo "❌ FAILED: Source file was not removed after mv-file"
  exit 1
fi

# Verify target file was created in subdirectory
if [ ! -f "${TMP_DIR}/project-root/submodule/specs/Renamed.md" ]; then
  echo "❌ FAILED: Target file was not created by mv-file at submodule/specs/Renamed.md"
  # Debug: check if file was incorrectly created at git root
  if [ -f "${TMP_DIR}/project-root/specs/Renamed.md" ]; then
    echo "  File exists at git root (WRONG): specs/Renamed.md"
  fi
  exit 1
fi

# Verify element content was preserved
if ! grep -q "### Submodule System" "${TMP_DIR}/project-root/submodule/specs/Renamed.md"; then
  echo "❌ FAILED: Element was not preserved in moved file"
  exit 1
fi

# Verify cross-file reference was updated (if any exist)
# Since we moved OtherRequirements.md which contained "Submodule System",
# and SubmoduleRequirements.md originally had a self-reference that we changed,
# we don't have cross-file references to check in this test

exit 0
