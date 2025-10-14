#!/bin/bash
set -euo pipefail

# Test: Gitignore Integration
# --------------------------------------
# Satisfies: specifications/Verifications/ValidationTests.md#gitignore-integration-test
#
# Acceptance Criteria:
# - System SHALL read exclusion patterns from root .gitignore file
# - System SHALL combine .gitignore patterns with configured excluded_filename_patterns
# - Files matching .gitignore patterns SHALL be excluded from processing
# - System SHALL use ONLY root .gitignore file, not nested .gitignore files
# - System SHALL correctly process files when .gitignore is absent
# - Exclusion SHALL work across all commands (validate, summary, format, traces, etc.)
#
# Test Criteria:
# - Scenario 1: Files matching .gitignore patterns are NOT processed
# - Scenario 2: Combined exclusion from .gitignore AND reqvire.yaml works
# - Scenario 3: Missing .gitignore file - system works gracefully
# - Scenario 4: Only root .gitignore is used - nested .gitignore ignored

echo "Starting Gitignore Integration Test..." > "${TEST_DIR}/test_results.log"

# Track overall test result
OVERALL_RESULT=0

#############################################################################
# Scenario 1: Gitignore pattern exclusion
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 1: Gitignore pattern exclusion ===" >> "${TEST_DIR}/test_results.log"

# Create .gitignore with patterns
cat > "${TEST_DIR}/.gitignore" << 'EOF'
# Build artifacts
**/build/**

# Temporary files
temp-*.md

# Cache directories
cache/
EOF

# Create files that SHOULD be excluded by .gitignore
mkdir -p "${TEST_DIR}/specifications/build"
cat > "${TEST_DIR}/specifications/build/BuildRequirements.md" << 'EOF'
# Build Requirements

### Build Requirement 001

This file is in build/ folder and should be excluded by .gitignore.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: ../ValidRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/temp-draft.md" << 'EOF'
# Temporary Draft

### Temp Requirement 001

This is a temporary file matching temp-*.md pattern.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: ValidRequirements.md#test-root
EOF

mkdir -p "${TEST_DIR}/specifications/cache"
cat > "${TEST_DIR}/specifications/cache/CacheRequirements.md" << 'EOF'
# Cache Requirements

### Cache Requirement 001

This file is in cache/ folder and should be excluded.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: ../ValidRequirements.md#test-root
EOF

# Create a file that SHOULD be processed (not matching .gitignore)
cat > "${TEST_DIR}/specifications/ValidRequirements.md" << 'EOF'
# Valid Requirements

### Test Root

Root requirement for gitignore test.

### Valid Requirement 001

This file should be processed - not matching .gitignore patterns.

#### Relations
  * derivedFrom: #test-root
EOF

# Run reqvire summary and capture output
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Check that command succeeded
if [ $EXIT_CODE -ne 0 ]; then
    echo "Command failed with exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify gitignore-excluded files are NOT processed
if echo "$OUTPUT" | grep -q "Build Requirement 001"; then
    echo "FAILED: File in build/ folder was incorrectly processed (should be excluded by .gitignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if echo "$OUTPUT" | grep -q "Temp Requirement 001"; then
    echo "FAILED: temp-*.md file was incorrectly processed (should be excluded by .gitignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if echo "$OUTPUT" | grep -q "Cache Requirement 001"; then
    echo "FAILED: File in cache/ folder was incorrectly processed (should be excluded by .gitignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify non-excluded file IS processed
if ! echo "$OUTPUT" | grep -q "Valid Requirement 001"; then
    echo "FAILED: ValidRequirements.md should be processed but was not found" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 1 - Gitignore patterns correctly exclude files" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Scenario 2: Combined exclusion patterns
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 2: Combined exclusion patterns ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 1
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Update .gitignore with different pattern
cat > "${TEST_DIR}/.gitignore" << 'EOF'
# Draft documents
DRAFT*.md
EOF

# Update reqvire.yaml with excluded_filename_patterns
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
  excluded_filename_patterns:
    - "**/README.md"
    - "**/archive/**/*.md"
EOF

# Create root requirement file first
cat > "${TEST_DIR}/specifications/ActiveRequirements.md" << 'EOF'
# Active Requirements

### Test Root

Root requirement for gitignore test scenario 2.

### Active Requirement 001

This file should be processed.

#### Relations
  * derivedFrom: #test-root
EOF

# Create files matching .gitignore pattern
cat > "${TEST_DIR}/specifications/DRAFT-Feature.md" << 'EOF'
# Draft Feature

### Draft Requirement 001

This file matches DRAFT*.md pattern in .gitignore.

#### Relations
  * derivedFrom: ActiveRequirements.md#test-root
EOF

# Create files matching reqvire.yaml pattern
mkdir -p "${TEST_DIR}/specifications/archive"
cat > "${TEST_DIR}/specifications/archive/OldRequirements.md" << 'EOF'
# Old Requirements

### Old Requirement 001

This file is in archive/ folder excluded by reqvire.yaml.

#### Relations
  * derivedFrom: ../ActiveRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/README.md" << 'EOF'
# README

### README Element

This README.md is excluded by reqvire.yaml pattern.

#### Relations
  * derivedFrom: ActiveRequirements.md#test-root
EOF

# Run reqvire summary
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "Command failed with exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify files excluded by .gitignore are NOT processed
if echo "$OUTPUT" | grep -q "Draft Requirement 001"; then
    echo "FAILED: DRAFT*.md file was processed (should be excluded by .gitignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify files excluded by reqvire.yaml are NOT processed
if echo "$OUTPUT" | grep -q "Old Requirement 001"; then
    echo "FAILED: archive/ file was processed (should be excluded by reqvire.yaml)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if echo "$OUTPUT" | grep -q "README Element"; then
    echo "FAILED: README.md was processed (should be excluded by reqvire.yaml)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify non-excluded file IS processed
if ! echo "$OUTPUT" | grep -q "Active Requirement 001"; then
    echo "FAILED: ActiveRequirements.md should be processed but was not found" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 2 - Combined exclusion patterns work correctly" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Scenario 3: Missing .gitignore handling
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 3: Missing .gitignore handling ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 2
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Remove .gitignore file
rm -f "${TEST_DIR}/.gitignore"

# Update reqvire.yaml with a simple exclusion pattern
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
  excluded_filename_patterns:
    - "**/excluded-*.md"
EOF

# Create files
cat > "${TEST_DIR}/specifications/NormalRequirements.md" << 'EOF'
# Normal Requirements

### Test Root

Root requirement for gitignore test scenario 3.

#### Metadata
  * type: user-requirement

### Normal Requirement 001

This file should be processed.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #test-root
EOF

cat > "${TEST_DIR}/specifications/excluded-test.md" << 'EOF'
# Excluded Test

### Excluded Requirement

This file matches excluded-*.md pattern in reqvire.yaml.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: NormalRequirements.md#test-root
EOF

# Run reqvire summary without .gitignore
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Check that command succeeded gracefully
if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: Command failed when .gitignore is missing (should work gracefully)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify only reqvire.yaml exclusions are applied
if echo "$OUTPUT" | grep -q "Excluded Requirement"; then
    echo "FAILED: excluded-*.md file was processed (should be excluded by reqvire.yaml)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if ! echo "$OUTPUT" | grep -q "Normal Requirement 001"; then
    echo "FAILED: NormalRequirements.md should be processed" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 3 - System works gracefully without .gitignore" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Scenario 4: Nested .gitignore ignored
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 4: Nested .gitignore ignored ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 3
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Create root .gitignore with pattern
cat > "${TEST_DIR}/.gitignore" << 'EOF'
# Root gitignore pattern
root-excluded-*.md
EOF

# Update reqvire.yaml to ensure no exclusion of test files
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
  excluded_filename_patterns: []
EOF

# Create nested .gitignore with different pattern
mkdir -p "${TEST_DIR}/specifications/subsystem"
cat > "${TEST_DIR}/specifications/subsystem/.gitignore" << 'EOF'
# Nested gitignore pattern (should be IGNORED)
nested-excluded-*.md
EOF

# Create normal file that should be processed - with root requirement
cat > "${TEST_DIR}/specifications/subsystem/normal-file.md" << 'EOF'
# Normal File

### Test Root

Root requirement for gitignore test scenario 4.

#### Metadata
  * type: user-requirement

### Normal Subsystem Requirement

This file should be processed normally.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #test-root
EOF

# Create files matching root .gitignore pattern
cat > "${TEST_DIR}/specifications/root-excluded-file.md" << 'EOF'
# Root Excluded

### Root Excluded Requirement

This matches root .gitignore pattern and should be excluded.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: subsystem/normal-file.md#test-root
EOF

# Create file matching nested .gitignore pattern (should NOT be excluded)
cat > "${TEST_DIR}/specifications/subsystem/nested-excluded-file.md" << 'EOF'
# Nested Pattern File

### Nested Requirement

This matches nested .gitignore pattern but nested .gitignore should be ignored.
This file SHOULD be processed.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: normal-file.md#test-root
EOF

# Run reqvire summary
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "Command failed with exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify file matching root .gitignore is excluded
if echo "$OUTPUT" | grep -q "Root Excluded Requirement"; then
    echo "FAILED: root-excluded-*.md file was processed (should be excluded by root .gitignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify file matching nested .gitignore IS processed (nested .gitignore should be ignored)
if ! echo "$OUTPUT" | grep -q "Nested Requirement"; then
    echo "FAILED: nested-excluded-*.md file was not processed (nested .gitignore should be ignored)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify normal file is processed
if ! echo "$OUTPUT" | grep -q "Normal Subsystem Requirement"; then
    echo "FAILED: normal-file.md should be processed" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 4 - Only root .gitignore is used" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Final Result
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
if [ $OVERALL_RESULT -eq 0 ]; then
    echo "✅ PASSED: All gitignore integration scenarios passed" >> "${TEST_DIR}/test_results.log"
    exit 0
else
    echo "❌ FAILED: One or more gitignore integration scenarios failed" >> "${TEST_DIR}/test_results.log"
    cat "${TEST_DIR}/test_results.log"
    exit 1
fi
