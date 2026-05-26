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
# - System shall NOT support location-based requirement root folder configuration parameter
# - System shall allow explicit type specification via Metadata subsection
# - System shall respect explicit type metadata when present
#
# Test Criteria:
# - Command exits with success (0) return code
# - All elements without type metadata have type 'requirement' regardless of location
# - Elements with explicit type metadata use the specified type
# - Location independence is verified across root, specifications/, and nested directories

echo "Starting Default Element Type Assignment Test..." > "${TEST_DIR}/test_results.log"

# Identifier normalization uses repository-relative paths.
(
    cd "${TEST_DIR}" &&
    git init >/dev/null 2>&1 &&
    git config user.email test@example.com &&
    git config user.name "Test User"
)

# Track overall test result
OVERALL_RESULT=0

#############################################################################
# Scenario 1: Default type assignment - location independent
#############################################################################
echo "" >> "${TEST_DIR}/test_results.log"
echo "=== Scenario 1: Default type assignment - location independent ===" >> "${TEST_DIR}/test_results.log"

# Create test directory structure
mkdir -p "${TEST_DIR}/specifications/root"
mkdir -p "${TEST_DIR}/specifications/nested/deeper"

# Create elements WITHOUT type metadata in various locations
# 1. Root of specifications folder
cat > "${TEST_DIR}/specifications/RootRequirements.md" << 'EOF'
# Elements

### Root Capability

This is a root capability that specifies root requirements.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: #root-requirement-without-type

### Root Requirement Without Type

This element has NO type metadata and is in the specifications root.

#### Relations
  * specify: #root-capability

### Root Capability With Explicit Type

This element has explicit capability type metadata.

#### Metadata
  * type: capability
EOF

# 2. Subdirectory specifications/root/
cat > "${TEST_DIR}/specifications/root/SubfolderRequirements.md" << 'EOF'
# Elements

### Subfolder Requirement Without Type

This element has NO type metadata and is in a subfolder.

#### Relations
  * specify: ../RootRequirements.md#root-capability

### Subfolder Requirement With Verification Type

This element has explicit verification type metadata.

#### Metadata
  * type: verification

#### Relations
  * verify: ../RootRequirements.md#root-requirement-without-type
EOF

# 3. Deeper nested directory specifications/nested/deeper/
cat > "${TEST_DIR}/specifications/nested/deeper/NestedRequirements.md" << 'EOF'
# Elements

### Nested Requirement Without Type

This element has NO type metadata and is deeply nested.

#### Relations
  * specify: ../../RootRequirements.md#root-capability

### Nested Requirement With Test Type

This element has explicit test-verification type metadata.

#### Metadata
  * type: test-verification

#### Relations
  * verify: ../../RootRequirements.md#root-requirement-without-type
EOF

# Run reqvire search --json to extract element types
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" search --json 2>&1)
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
ROOT_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/RootRequirements.md"].elements[] | select(.name == "Root Requirement Without Type") | .type')
if [ "$ROOT_TYPE" != "requirement" ]; then
    echo "FAILED: Root element without type metadata should be 'requirement', got '$ROOT_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Root element without type metadata has type 'requirement'" >> "${TEST_DIR}/test_results.log"
fi

# Subfolder element without type should be 'requirement'
SUBFOLDER_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/root/SubfolderRequirements.md"].elements[] | select(.name == "Subfolder Requirement Without Type") | .type')
if [ "$SUBFOLDER_TYPE" != "requirement" ]; then
    echo "FAILED: Subfolder element without type metadata should be 'requirement', got '$SUBFOLDER_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Subfolder element without type metadata has type 'requirement'" >> "${TEST_DIR}/test_results.log"
fi

# Nested element without type should be 'requirement'
NESTED_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/nested/deeper/NestedRequirements.md"].elements[] | select(.name == "Nested Requirement Without Type") | .type')
if [ "$NESTED_TYPE" != "requirement" ]; then
    echo "FAILED: Nested element without type metadata should be 'requirement', got '$NESTED_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Nested element without type metadata has type 'requirement'" >> "${TEST_DIR}/test_results.log"
fi

# Verify elements WITH explicit type metadata use the specified type
ROOT_CAPABILITY_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/RootRequirements.md"].elements[] | select(.name == "Root Capability With Explicit Type") | .type')
if [ "$ROOT_CAPABILITY_TYPE" != "capability" ]; then
    echo "FAILED: Element with explicit capability type should be 'capability', got '$ROOT_CAPABILITY_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Element with explicit type metadata has type 'capability'" >> "${TEST_DIR}/test_results.log"
fi

SUBFOLDER_VERIF_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/root/SubfolderRequirements.md"].elements[] | select(.name == "Subfolder Requirement With Verification Type") | .type')
if [ "$SUBFOLDER_VERIF_TYPE" != "test-verification" ]; then
    echo "FAILED: Element with explicit verification type should default to 'test-verification', got '$SUBFOLDER_VERIF_TYPE'" >> "${TEST_DIR}/test_results.log"
    OVERALL_RESULT=1
else
    echo "PASSED: Element with explicit type metadata has type 'test-verification' (default for verification)" >> "${TEST_DIR}/test_results.log"
fi

NESTED_TEST_TYPE=$(echo "$OUTPUT" | jq -r '.files["specifications/nested/deeper/NestedRequirements.md"].elements[] | select(.name == "Nested Requirement With Test Type") | .type')
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
# Elements

### Root Capability

This is the root requirement for testing all types.

#### Metadata
  * type: capability

#### Attachments
  * [Ontology Element](#ontology-element)

### Default Requirement

No type metadata - should default to requirement.

#### Relations
  * specify: #root-capability

### Explicit Requirement

#### Metadata
  * type: requirement

#### Relations
  * specify: #root-capability
  * refinedBy: [State Refinement](#state-refinement)
  * refinedBy: [Input Output Refinement](#input-output-refinement)
  * refinedBy: [Semantic Contract Refinement](#semantic-contract-refinement)

### Capability

#### Metadata
  * type: capability

#### Attachments
  * [Ontology Element](#ontology-element)

### Source Refinement

#### Metadata
  * type: source

#### Relations
  * refine: #root-capability

### Semantic Contract Refinement

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: #explicit-requirement

#### Shapes
```turtle
@prefix reqvire: <urn:reqvire:test#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

reqvire:DefaultTypeShape
  a sh:NodeShape ;
  sh:targetClass reqvire:DefaultTypeContract .
```

### Ontology Element

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix reqvire: <urn:reqvire:test#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:DefaultTypeContract a owl:Class .
```

### Verification

#### Metadata
  * type: verification

#### Relations
  * verify: #explicit-requirement

### Test Verification

#### Metadata
  * type: test-verification

#### Relations
  * verify: #explicit-requirement

### Formal Proof Verification

#### Metadata
  * type: formal-proof-verification

#### Relations
  * verify: #explicit-requirement
  * satisfiedBy: [proof-report.txt](proof-report.txt)

### Analysis Verification

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: #explicit-requirement

### Inspection Verification

#### Metadata
  * type: inspection-verification

#### Relations
  * verify: #explicit-requirement

### Demonstration Verification

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: #explicit-requirement

### State Refinement

#### Metadata
  * type: state

#### Relations
  * refine: #explicit-requirement

### Input Output Refinement

#### Metadata
  * type: input-output

#### Relations
  * refine: #explicit-requirement

### Other Type

#### Metadata
  * type: other-custom

#### Relations
  * trace: #explicit-requirement
EOF

printf "formal proof evidence placeholder\n" > "${TEST_DIR}/specifications/proof-report.txt"

# Run reqvire search --json
set +e
OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" search --json 2>&1)
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
    "Capability:capability"
    "Ontology Element:ontology"
    "Source Refinement:source"
    "Semantic Contract Refinement:semantic-contract"
    "Verification:test-verification"
    "Test Verification:test-verification"
    "Formal Proof Verification:formal-proof-verification"
    "Analysis Verification:analysis-verification"
    "Inspection Verification:inspection-verification"
    "Demonstration Verification:demonstration-verification"
    "State Refinement:state"
    "Input Output Refinement:input-output"
    "Other Type:custom"
)

for type_check in "${TYPES_TO_CHECK[@]}"; do
    ELEMENT_NAME="${type_check%:*}"
    EXPECTED_TYPE="${type_check#*:}"

    ACTUAL_TYPE=$(echo "$OUTPUT" | jq -r --arg name "$ELEMENT_NAME" '.files["specifications/AllTypes.md"].elements[] | select(.name == $name) | .type')

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
