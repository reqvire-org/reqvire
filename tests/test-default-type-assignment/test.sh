#!/bin/bash
set -euo pipefail

# Test: Default Element Type Assignment Test
# --------------------------------------
# Satisfies: specifications/Verifications/ValidationTests.md#default-element-type-assignment-test
#
# Acceptance Criteria:
# - System shall assign type 'requirement' to elements without explicit type metadata
# - Default type assignment shall be location-independent (same behavior for all directories)
# - System shall NOT use file location to determine element type
# - System shall NOT support user_requirements_root_folder configuration parameter
# - System shall allow explicit type specification via Metadata subsection
# - System shall respect explicit type metadata when present
#
# Test Criteria:
# - Command exits with success (0) return code
# - All elements without type metadata have type 'requirement' regardless of location
# - Elements with explicit type metadata use the specified type
# - Location independence is verified across root, specifications/, and nested directories

echo "Starting Default Element Type Assignment Test..." > "${TEST_DIR}/test_results.log"

# Track overall test result
OVERALL_RESULT=0

#############################################################################
# Scenario 1: Default type assignment - location independent
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 1: Default type assignment - location independent ===" >> "${TEST_DIR}/test_results.log"

# Create elements WITHOUT type metadata in various locations
# 1. Root of specifications folder
cat > "${TEST_DIR}/specifications/RootRequirements.md" << 'EOF'
# Root Requirements

### Root User Requirement

This is a root user requirement that all others derive from.

#### Metadata
  * type: user-requirement

### Root Requirement Without Type

This element has NO type metadata and is in the specifications root.

#### Relations
  * derivedFrom: #root-user-requirement

### Root Requirement With User Type

This element has explicit user-requirement type metadata.

#### Metadata
  * type: user-requirement
EOF

# 2. Subdirectory specifications/root/
cat > "${TEST_DIR}/specifications/root/SubfolderRequirements.md" << 'EOF'
# Subfolder Requirements

### Subfolder Requirement Without Type

This element has NO type metadata and is in a subfolder.

#### Relations
  * derivedFrom: ../RootRequirements.md#root-user-requirement

### Subfolder Requirement With Verification Type

This element has explicit verification type metadata.

#### Metadata
  * type: verification

#### Relations
  * verify: ../RootRequirements.md#root-user-requirement
EOF

# 3. Deeper nested directory specifications/nested/deeper/
cat > "${TEST_DIR}/specifications/nested/deeper/NestedRequirements.md" << 'EOF'
# Nested Requirements

### Nested Requirement Without Type

This element has NO type metadata and is deeply nested.

#### Relations
  * derivedFrom: ../../RootRequirements.md#root-user-requirement

### Nested Requirement With Test Type

This element has explicit test-verification type metadata.

#### Metadata
  * type: test-verification

#### Relations
  * verify: ../../RootRequirements.md#root-user-requirement
EOF

# Run reqvire summary --json to extract element types
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" summary --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

# Check that command succeeded
if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: Command failed with exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Parse JSON and verify types for elements WITHOUT metadata
# Root element without type should be 'requirement'
ROOT_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/RootRequirements.md"].sections[].elements[] | select(.name == "Root Requirement Without Type") | .type')
if [ "$ROOT_TYPE" != "requirement" ]; then
    echo "FAILED: Root element without type metadata should be 'requirement', got '$ROOT_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Root element without type metadata has type 'requirement'" >> "${TEST_DIR}/test_results.log"
fi

# Subfolder element without type should be 'requirement'
SUBFOLDER_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/root/SubfolderRequirements.md"].sections[].elements[] | select(.name == "Subfolder Requirement Without Type") | .type')
if [ "$SUBFOLDER_TYPE" != "requirement" ]; then
    echo "FAILED: Subfolder element without type metadata should be 'requirement', got '$SUBFOLDER_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Subfolder element without type metadata has type 'requirement'" >> "${TEST_DIR}/test_results.log"
fi

# Nested element without type should be 'requirement'
NESTED_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/nested/deeper/NestedRequirements.md"].sections[].elements[] | select(.name == "Nested Requirement Without Type") | .type')
if [ "$NESTED_TYPE" != "requirement" ]; then
    echo "FAILED: Nested element without type metadata should be 'requirement', got '$NESTED_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Nested element without type metadata has type 'requirement'" >> "${TEST_DIR}/test_results.log"
fi

# Verify elements WITH explicit type metadata use the specified type
ROOT_USER_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/RootRequirements.md"].sections[].elements[] | select(.name == "Root Requirement With User Type") | .type')
if [ "$ROOT_USER_TYPE" != "user-requirement" ]; then
    echo "FAILED: Element with explicit user-requirement type should be 'user-requirement', got '$ROOT_USER_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Element with explicit type metadata has type 'user-requirement'" >> "${TEST_DIR}/test_results.log"
fi

SUBFOLDER_VERIF_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/root/SubfolderRequirements.md"].sections[].elements[] | select(.name == "Subfolder Requirement With Verification Type") | .type')
if [ "$SUBFOLDER_VERIF_TYPE" != "test-verification" ]; then
    echo "FAILED: Element with explicit verification type should default to 'test-verification', got '$SUBFOLDER_VERIF_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Element with explicit type metadata has type 'test-verification' (default for verification)" >> "${TEST_DIR}/test_results.log"
fi

NESTED_TEST_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/nested/deeper/NestedRequirements.md"].sections[].elements[] | select(.name == "Nested Requirement With Test Type") | .type')
if [ "$NESTED_TEST_TYPE" != "test-verification" ]; then
    echo "FAILED: Element with explicit test-verification type should be 'test-verification', got '$NESTED_TEST_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Element with explicit type metadata has type 'test-verification'" >> "${TEST_DIR}/test_results.log"
fi

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 1 - Default type assignment is location-independent" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Scenario 2: All standard element types supported
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 2: All standard element types supported ===" >> "${TEST_DIR}/test_results.log"

# Clean up files from Scenario 1
rm -rf "${TEST_DIR}/specifications"
mkdir -p "${TEST_DIR}/specifications"

# Create elements with all supported explicit types
cat > "${TEST_DIR}/specifications/AllTypes.md" << 'EOF'
# All Element Types

### Root User Requirement

This is the root requirement for testing all types.

#### Metadata
  * type: user-requirement

### Default Requirement

No type metadata - should default to requirement.

#### Relations
  * derivedFrom: #root-user-requirement

### Explicit Requirement

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #root-user-requirement

### User Requirement

#### Metadata
  * type: user-requirement

### Verification

#### Metadata
  * type: verification

#### Relations
  * verify: #root-user-requirement

### Test Verification

#### Metadata
  * type: test-verification

#### Relations
  * verify: #root-user-requirement

### Analysis Verification

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: #root-user-requirement

### Inspection Verification

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: #root-user-requirement

### Demonstration Verification

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: #root-user-requirement

### Other Type

#### Metadata
  * type: other

#### Relations
  * trace: #root-user-requirement
EOF

# Run reqvire summary --json
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" summary --json 2>&1)
EXIT_CODE=$?
set -e

echo "Exit code: $EXIT_CODE" >> "${TEST_DIR}/test_results.log"
printf "%s\n" "$OUTPUT" >> "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
    echo "FAILED: Command failed when processing all element types" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
fi

# Verify each type
TYPES_TO_CHECK=(
    "Default Requirement:requirement"
    "Explicit Requirement:requirement"
    "User Requirement:user-requirement"
    "Verification:test-verification"
    "Test Verification:test-verification"
    "Analysis Verification:analysis-verification"
    "Inspection Verification:inspection-verification"
    "Demonstration Verification:demonstration-verification"
    "Other Type:other"
)

for type_check in "${TYPES_TO_CHECK[@]}"; do
    ELEMENT_NAME="${type_check%:*}"
    EXPECTED_TYPE="${type_check#*:}"

    ACTUAL_TYPE=$(echo "$OUTPUT" | jq -r --arg name "$ELEMENT_NAME" '.files["specifications/AllTypes.md"].sections[].elements[] | select(.name == $name) | .type')

    if [ "$ACTUAL_TYPE" != "$EXPECTED_TYPE" ]; then
        echo "FAILED: Element '$ELEMENT_NAME' should have type '$EXPECTED_TYPE', got '$ACTUAL_TYPE'" >> "${TEST_DIR}/test_results.log"
        OVERALL_RESULT=1
    else
        echo "PASSED: Element '$ELEMENT_NAME' has correct type '$EXPECTED_TYPE'" >> "${TEST_DIR}/test_results.log"
    fi
done

if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: Scenario 2 - All standard element types are supported" >> "${TEST_DIR}/test_results.log"
fi

#############################################################################
# Final Result
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
if [ $OVERALL_RESULT -eq 0 ]; then
    echo "PASSED: All default type assignment scenarios passed" >> "${TEST_DIR}/test_results.log"
    exit 0
else
    echo "FAILED: One or more default type assignment scenarios failed" >> "${TEST_DIR}/test_results.log"
    cat "${TEST_DIR}/test_results.log"
    exit 1
fi
