#!/bin/bash

# Test: Comprehensive Validation of Invalid Relation Types and Formats
# -----------------------------------------------------------------------
# Acceptance Criteria:
# - System should detect and report Duplicate element
# - System should detect and report Invalid metadata format
# - System should detect and report Invalid relation format
# - System should detect and report Unsupported relation type
# - System should detect and report Duplicate subsection
# - System should detect and report Incompatible element types for relation
# - System should detect and report Missing relation target
# - System should detect and report Circular dependency error
# - System should detect and report Missing parent relation
#
# Test Criteria:
# - Command exits with 0 error code but outputs expected validation errors
# - Error output contains specific error messages for each type of invalid relation
#



OUTPUT=$(cd "${TEST_DIR}" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" validate --json 2>&1)
EXIT_CODE=$?


printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# Verify exit code indicates doesn't indicate error 
if ! [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Validation should have failed but returned success (0)"
  exit 1
fi

# Check for specific error messages - be explicit about what we expect vs get

# Define specific errors we expect to see
EXPECTED_ERRORS=(
  "Circular dependency error:"
  "Duplicate element:"
  "Duplicate subsection:"
  "Invalid metadata format:"
  "Invalid relation format:"
  "Missing parent relation:"
  "Unsupported relation type:"
  # Incompatible element types - we expect 3 instances
  "Incompatible.*requirement-with-incompactible-element.*Requirement.*requirement-with-invalid-relation-type.*Requirement"
  "Incompatible.*requirement-satisfiedby-requirement-invalid.*Requirement.*valid-requirement.*Requirement"
  "Incompatible.*verification-satisfiedby-verification-invalid.*Verification.*valid-verification-with-correct-satisfiedby.*Verification"
  # Missing relation targets - we expect 4 instances
  "Missing.*requirement-with-missing-target.*NonExistentElement.md#missing-element"
  "Missing.*requirement-with-missing-verifiedby-target.*NonExistentVerification.md#missing-verification"
  "Missing.*verification-with-missing-satisfiedby-target.*non-existing-test-script.sh"
  "Missing.*requirement-with-missing-satisfiedby-file.*non-existing-implementation.py"
)

MISSING_ERRORS=()

for expected in "${EXPECTED_ERRORS[@]}"; do
  if ! echo "$OUTPUT" | grep -q "$expected"; then
    MISSING_ERRORS+=("❌ MISSING: $expected")
  fi
done

if [ ${#MISSING_ERRORS[@]} -gt 0 ]; then
  echo "❌ FAILED: Missing expected validation errors!"
  for missing in "${MISSING_ERRORS[@]}"; do
    echo "$missing"
  done
  echo ""
  echo "ACTUAL ERRORS:"
  echo "$OUTPUT" | jq -r '.errors[]'
  exit 1
fi


exit 0
