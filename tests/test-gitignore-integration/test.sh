#!/bin/bash
set -euo pipefail

# Test: File Exclusion Integration
# --------------------------------------
# Satisfies: specifications/Verifications/ValidationTests.md#file-exclusion-test
#
# Acceptance Criteria:
# - System SHALL always exclude reserved filenames from processing
# - System SHALL read exclusion patterns from root .gitignore file
# - System SHALL read exclusion patterns from root .reqvireignore file
# - System SHALL combine reserved filenames, .gitignore, and .reqvireignore patterns
# - Files matching .gitignore patterns SHALL be excluded from processing
# - Files matching .reqvireignore patterns SHALL be excluded from processing
# - Reserved filenames SHALL be excluded from processing even without ignore files
# - System SHALL use ONLY root .gitignore file, not nested .gitignore files
# - System SHALL use ONLY root .reqvireignore file, not nested .reqvireignore files
# - System SHALL correctly process files when .gitignore is absent
# - System SHALL correctly process files when .reqvireignore is absent
# - System SHALL correctly process files when both .gitignore and .reqvireignore are absent
# - Exclusion SHALL work across all commands (validate, summary, format, traces, etc.)
#
# Test Criteria:
# - Scenario 1: Files matching .gitignore patterns are NOT processed
# - Scenario 2: Combined exclusion from .gitignore AND .reqvireignore works
# - Scenario 3: Missing .reqvireignore file - only .gitignore and reserved exclusions applied
# - Scenario 4: Only root .gitignore and .reqvireignore are used - nested files ignored
# - Scenario 5: Missing .gitignore but .reqvireignore present - .reqvireignore and reserved exclusions applied
# - Scenario 6: Both files missing - reserved filenames still excluded

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

# Create .reqvireignore with Reqvire-specific patterns
cat > "${TEST_DIR}/.reqvireignore" << 'EOF'
# Reqvire-specific exclusions
README.md
archive/
EOF

# Update reqvire.yaml to minimal config (no excluded_filename_patterns)
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
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

# Create files matching .reqvireignore pattern
mkdir -p "${TEST_DIR}/specifications/archive"
cat > "${TEST_DIR}/specifications/archive/OldRequirements.md" << 'EOF'
# Old Requirements

### Old Requirement 001

This file is in archive/ folder excluded by .reqvireignore.

#### Relations
  * derivedFrom: ../ActiveRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/README.md" << 'EOF'
# README

### README Element

This README.md is excluded by .reqvireignore pattern.

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

# Verify files excluded by .reqvireignore are NOT processed
if echo "$OUTPUT" | grep -q "Old Requirement 001"; then
    echo "FAILED: archive/ file was processed (should be excluded by .reqvireignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if echo "$OUTPUT" | grep -q "README Element"; then
    echo "FAILED: README.md was processed (should be excluded by .reqvireignore)" >> "${TEST_DIR}/test_results.log"
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
# Scenario 3: Missing .reqvireignore handling
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 3: Missing .reqvireignore handling ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 2
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Remove .reqvireignore file (keep .gitignore)
rm -f "${TEST_DIR}/.reqvireignore"

# Update .gitignore with a simple exclusion pattern
cat > "${TEST_DIR}/.gitignore" << 'EOF'
# Excluded files
excluded-*.md
EOF

# Update reqvire.yaml to minimal config
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
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

This file matches excluded-*.md pattern in .gitignore.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: NormalRequirements.md#test-root
EOF

# Run reqvire summary without .reqvireignore
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Check that command succeeded gracefully
if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: Command failed when .reqvireignore is missing (should work gracefully)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify only .gitignore exclusions are applied
if echo "$OUTPUT" | grep -q "Excluded Requirement"; then
    echo "FAILED: excluded-*.md file was processed (should be excluded by .gitignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if ! echo "$OUTPUT" | grep -q "Normal Requirement 001"; then
    echo "FAILED: NormalRequirements.md should be processed" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 3 - System works gracefully without .reqvireignore" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Scenario 4: Nested ignore files are ignored
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 4: Nested ignore files are ignored ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 3
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Create root .gitignore with pattern
cat > "${TEST_DIR}/.gitignore" << 'EOF'
# Root gitignore pattern
root-excluded-*.md
EOF

# Create root .reqvireignore with pattern
cat > "${TEST_DIR}/.reqvireignore" << 'EOF'
# Root reqvireignore pattern
reqvire-excluded-*.md
EOF

# Update reqvire.yaml to minimal config
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
EOF

# Create nested .gitignore with different pattern
mkdir -p "${TEST_DIR}/specifications/subsystem"
cat > "${TEST_DIR}/specifications/subsystem/.gitignore" << 'EOF'
# Nested gitignore pattern (should be IGNORED)
nested-gitignore-excluded-*.md
EOF

# Create nested .reqvireignore with different pattern
cat > "${TEST_DIR}/specifications/subsystem/.reqvireignore" << 'EOF'
# Nested reqvireignore pattern (should be IGNORED)
nested-reqvire-excluded-*.md
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

# Create files matching root .reqvireignore pattern
cat > "${TEST_DIR}/specifications/reqvire-excluded-file.md" << 'EOF'
# Reqvire Excluded

### Reqvire Excluded Requirement

This matches root .reqvireignore pattern and should be excluded.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: subsystem/normal-file.md#test-root
EOF

# Create file matching nested .gitignore pattern (should NOT be excluded)
cat > "${TEST_DIR}/specifications/subsystem/nested-gitignore-excluded-file.md" << 'EOF'
# Nested Gitignore Pattern File

### Nested Gitignore Requirement

This matches nested .gitignore pattern but nested .gitignore should be ignored.
This file SHOULD be processed.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: normal-file.md#test-root
EOF

# Create file matching nested .reqvireignore pattern (should NOT be excluded)
cat > "${TEST_DIR}/specifications/subsystem/nested-reqvire-excluded-file.md" << 'EOF'
# Nested Reqvireignore Pattern File

### Nested Reqvireignore Requirement

This matches nested .reqvireignore pattern but nested .reqvireignore should be ignored.
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

# Verify file matching root .reqvireignore is excluded
if echo "$OUTPUT" | grep -q "Reqvire Excluded Requirement"; then
    echo "FAILED: reqvire-excluded-*.md file was processed (should be excluded by root .reqvireignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify file matching nested .gitignore IS processed (nested .gitignore should be ignored)
if ! echo "$OUTPUT" | grep -q "Nested Gitignore Requirement"; then
    echo "FAILED: nested-gitignore-excluded-*.md file was not processed (nested .gitignore should be ignored)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify file matching nested .reqvireignore IS processed (nested .reqvireignore should be ignored)
if ! echo "$OUTPUT" | grep -q "Nested Reqvireignore Requirement"; then
    echo "FAILED: nested-reqvire-excluded-*.md file was not processed (nested .reqvireignore should be ignored)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify normal file is processed
if ! echo "$OUTPUT" | grep -q "Normal Subsystem Requirement"; then
    echo "FAILED: normal-file.md should be processed" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 4 - Only root ignore files are used" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Scenario 5: Missing .gitignore but .reqvireignore present
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 5: Missing .gitignore but .reqvireignore present ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 4
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Remove .gitignore (keep .reqvireignore)
rm -f "${TEST_DIR}/.gitignore"

# Create .reqvireignore with pattern
cat > "${TEST_DIR}/.reqvireignore" << 'EOF'
# Reqvire-specific exclusions
reqvire-only-excluded-*.md
EOF

# Update reqvire.yaml to minimal config
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
EOF

# Create files
cat > "${TEST_DIR}/specifications/ProcessedRequirements.md" << 'EOF'
# Processed Requirements

### Test Root

Root requirement for gitignore test scenario 5.

#### Metadata
  * type: user-requirement

### Processed Requirement 001

This file should be processed.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #test-root
EOF

cat > "${TEST_DIR}/specifications/reqvire-only-excluded-test.md" << 'EOF'
# Reqvire Only Excluded

### Reqvire Only Excluded Requirement

This file matches reqvire-only-excluded-*.md pattern in .reqvireignore.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: ProcessedRequirements.md#test-root
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

# Verify only .reqvireignore exclusions are applied
if echo "$OUTPUT" | grep -q "Reqvire Only Excluded Requirement"; then
    echo "FAILED: reqvire-only-excluded-*.md file was processed (should be excluded by .reqvireignore)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if ! echo "$OUTPUT" | grep -q "Processed Requirement 001"; then
    echo "FAILED: ProcessedRequirements.md should be processed" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 5 - System works gracefully with only .reqvireignore" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Scenario 6: Both files missing - reserved filenames still excluded
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 6: Both files missing - reserved filenames still excluded ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 5
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Remove both .gitignore and .reqvireignore
rm -f "${TEST_DIR}/.gitignore"
rm -f "${TEST_DIR}/.reqvireignore"

# Update reqvire.yaml to minimal config
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
EOF

# Create files that would match typical ignore patterns
cat > "${TEST_DIR}/specifications/AllRequirements.md" << 'EOF'
# All Requirements

### Test Root

Root requirement for gitignore test scenario 6.

#### Metadata
  * type: user-requirement

### All Requirements 001

This file should be processed.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #test-root
EOF

cat > "${TEST_DIR}/specifications/README.md" << 'EOF'
# README

### README Requirement

This is a reserved filename and should ALWAYS be excluded.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: AllRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/DRAFT-test.md" << 'EOF'
# Draft Test

### Draft Requirement

This file should be processed (not a reserved filename).

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: AllRequirements.md#test-root
EOF

# Run reqvire summary without any ignore files
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Check that command succeeded gracefully
if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: Command failed when both ignore files are missing (should work gracefully)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify non-reserved files are processed
if ! echo "$OUTPUT" | grep -q "All Requirements 001"; then
    echo "FAILED: AllRequirements.md should be processed" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if ! echo "$OUTPUT" | grep -q "Draft Requirement"; then
    echo "FAILED: DRAFT-test.md should be processed (no ignore files present)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify reserved filename is ALWAYS excluded (even without ignore files)
if echo "$OUTPUT" | grep -q "README Requirement"; then
    echo "FAILED: README.md was processed (reserved filenames should ALWAYS be excluded)" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 6 - Reserved filenames excluded even without ignore files" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Scenario 7: Reserved filenames always excluded
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 7: Reserved filenames always excluded ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 6
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Remove both .gitignore and .reqvireignore to test reserved filenames alone
rm -f "${TEST_DIR}/.gitignore"
rm -f "${TEST_DIR}/.reqvireignore"

# Update reqvire.yaml to minimal config
cat > "${TEST_DIR}/reqvire.yaml" << 'EOF'
paths:
  user_requirements_root_folder: "specifications"
  output_folder: "./output"
EOF

# Create valid requirements file
cat > "${TEST_DIR}/specifications/ValidRequirements.md" << 'EOF'
# Valid Requirements

### Test Root

Root requirement for reserved filenames test.

#### Metadata
  * type: user-requirement

### Valid Requirement 001

This file should be processed.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #test-root
EOF

# Create reserved documentation files
cat > "${TEST_DIR}/specifications/README.md" << 'EOF'
# README
### README Requirement
Reserved filename - should be excluded.
#### Relations
  * derivedFrom: ValidRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/CHANGELOG.md" << 'EOF'
# CHANGELOG
### Changelog Requirement
Reserved filename - should be excluded.
#### Relations
  * derivedFrom: ValidRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/CONTRIBUTING.md" << 'EOF'
# CONTRIBUTING
### Contributing Requirement
Reserved filename - should be excluded.
#### Relations
  * derivedFrom: ValidRequirements.md#test-root
EOF

# Create reserved AI instruction files
cat > "${TEST_DIR}/specifications/CLAUDE.md" << 'EOF'
# CLAUDE
### Claude Requirement
Reserved AI instruction file - should be excluded.
#### Relations
  * derivedFrom: ValidRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/AGENT.md" << 'EOF'
# AGENT
### Agent Requirement
Reserved AI instruction file - should be excluded.
#### Relations
  * derivedFrom: ValidRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/CURSOR.md" << 'EOF'
# CURSOR
### Cursor Requirement
Reserved AI instruction file - should be excluded.
#### Relations
  * derivedFrom: ValidRequirements.md#test-root
EOF

cat > "${TEST_DIR}/specifications/COPILOT.md" << 'EOF'
# COPILOT
### Copilot Requirement
Reserved AI instruction file - should be excluded.
#### Relations
  * derivedFrom: ValidRequirements.md#test-root
EOF

# Run reqvire summary
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" summary 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Check that command succeeded gracefully
if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: Command failed when processing reserved filenames" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify valid file is processed
if ! echo "$OUTPUT" | grep -q "Valid Requirement 001"; then
    echo "FAILED: ValidRequirements.md should be processed" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify reserved documentation files are NOT processed
RESERVED_DOCS=("README Requirement" "Changelog Requirement" "Contributing Requirement")
for doc in "${RESERVED_DOCS[@]}"; do
    if echo "$OUTPUT" | grep -q "$doc"; then
        echo "FAILED: Reserved documentation file was processed: $doc" >> "${TEST_DIR}/test_results.log"
        OVERALL_RESULT=1
    fi
done

# Verify reserved AI instruction files are NOT processed
RESERVED_AI=("Claude Requirement" "Agent Requirement" "Cursor Requirement" "Copilot Requirement")
for ai in "${RESERVED_AI[@]}"; do
    if echo "$OUTPUT" | grep -q "$ai"; then
        echo "FAILED: Reserved AI instruction file was processed: $ai" >> "${TEST_DIR}/test_results.log"
        OVERALL_RESULT=1
    fi
done

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 7 - All reserved filenames properly excluded" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Final Result
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
if [ $OVERALL_RESULT -eq 0 ]; then
    echo "✅ PASSED: All file exclusion scenarios passed" >> "${TEST_DIR}/test_results.log"
    exit 0
else
    echo "❌ FAILED: One or more file exclusion scenarios failed" >> "${TEST_DIR}/test_results.log"
    cat "${TEST_DIR}/test_results.log"
    exit 1
fi
