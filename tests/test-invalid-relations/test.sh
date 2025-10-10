#!/bin/bash

# Test: Two-Pass Validation of Invalid Relations using Validate Command
# -------------------------------------------------------------------
# This test validates the two-pass validation architecture using the validate command:
# - Pass 1: Element collection and local validation (parsing/format errors)
# - Pass 2: Graph construction and relation validation (relation errors)
# - Validate Command: Tests the validate command functionality with valid and invalid models
#
# The test runs separate scenarios to validate each pass independently using the validate command.

# Test 1: Pass 1 Errors (Parsing/Format Issues)

echo "Starting test..." > "${TEST_DIR}/test_results_pass1.log"

echo "Running: reqvire validate (pass1-errors)" >> "${TEST_DIR}/test_results_pass1.log"
set +e
OUTPUT_PASS1=$(cd "${TEST_DIR}/pass1-errors" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_PASS1=$?
set -e

echo "Exit code: $EXIT_CODE_PASS1" >> "${TEST_DIR}/test_results_pass1.log"
printf "%s\n" "$OUTPUT_PASS1" >> "${TEST_DIR}/test_results_pass1.log"

# Verify exit code indicates validation failure (non-zero)
if [ $EXIT_CODE_PASS1 -eq 0 ]; then
  echo "❌ FAILED: Pass 1 validation should have failed but returned success (0)"
  exit 1
fi

# Define Pass 1 errors we expect to see
EXPECTED_PASS1_ERRORS=(
  "Duplicate element:"
  "Invalid metadata format:"
  "Invalid relation format:"
  "Unsupported relation type:"
  "Duplicate subsection:"
  "Invalid header level in element"
)

MISSING_PASS1_ERRORS=()

for expected in "${EXPECTED_PASS1_ERRORS[@]}"; do
  if ! echo "$OUTPUT_PASS1" | grep -q "$expected"; then
    MISSING_PASS1_ERRORS+=("❌ MISSING Pass 1: $expected")
  fi
done

if [ ${#MISSING_PASS1_ERRORS[@]} -gt 0 ]; then
  echo "❌ FAILED: Missing expected Pass 1 validation errors!"
  for missing in "${MISSING_PASS1_ERRORS[@]}"; do
    echo "$missing"
  done
  echo ""
  echo "ACTUAL Pass 1 ERRORS:"
  echo "$OUTPUT_PASS1"
  exit 1
fi

# Test 2: Pass 2 Errors (Relation Validation Issues)

echo "Running: reqvire validate (pass2-errors)" >> "${TEST_DIR}/test_results_pass2.log"
set +e
OUTPUT_PASS2=$(cd "${TEST_DIR}/pass2-errors" && "$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_PASS2=$?
set -e

echo "Exit code: $EXIT_CODE_PASS2" >> "${TEST_DIR}/test_results_pass2.log"
printf "%s\n" "$OUTPUT_PASS2" >> "${TEST_DIR}/test_results_pass2.log"

# Verify exit code indicates validation failure (non-zero)
if [ $EXIT_CODE_PASS2 -eq 0 ]; then
  echo "❌ FAILED: Pass 2 validation should have failed but returned success (0)"
  exit 1
fi

# Define Pass 2 errors we expect to see
EXPECTED_PASS2_ERRORS=(
  "Missing relation target:"
  "Incompatible element types for relation:"
  "Circular dependency error:"
  "Non-test-verification element with satisfiedBy relation:"
)

MISSING_PASS2_ERRORS=()

for expected in "${EXPECTED_PASS2_ERRORS[@]}"; do
  if ! echo "$OUTPUT_PASS2" | grep -q "$expected"; then
    MISSING_PASS2_ERRORS+=("❌ MISSING Pass 2: $expected")
  fi
done

if [ ${#MISSING_PASS2_ERRORS[@]} -gt 0 ]; then
  echo "❌ FAILED: Missing expected Pass 2 validation errors!"
  for missing in "${MISSING_PASS2_ERRORS[@]}"; do
    echo "$missing"
  done
  echo ""
  echo "ACTUAL Pass 2 ERRORS:"
  echo "$OUTPUT_PASS2"
  exit 1
fi

# Test 3: Validate Command Tests

# Test 3.1: Validate command on valid model (create a simple valid model in the current test)
# Create a simple valid model for testing
mkdir -p "${TEST_DIR}/valid-test/specifications"
cat > "${TEST_DIR}/valid-test/specifications/ValidRequirements.md" << 'EOF'
# Valid Test Requirements

## Valid Requirement

This is a valid requirement.

#### Metadata
  * type: user-requirement

---

### Requirement with Valid Header Structure in Details

This requirement correctly uses level 5 headers inside Details subsection.

#### Details

##### This is valid - level 5 header inside Details

Level 5+ headers are allowed inside Details subsection.

###### Level 6 is also OK here

Content with multiple header levels.

#### Metadata
  * type: user-requirement
EOF

cd "${TEST_DIR}/valid-test"
echo "Running: reqvire validate (valid-test)" >> "${TEST_DIR}/test_results_pass2.log"
set +e
OUTPUT_VALID=$("$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_VALID=$?
set -e

echo "Exit code: $EXIT_CODE_VALID" >> "${TEST_DIR}/test_results_pass2.log"
printf "%s\n" "$OUTPUT_VALID" >> "${TEST_DIR}/test_results_pass2.log"

if [ $EXIT_CODE_VALID -ne 0 ]; then
  echo "❌ FAILED: Validate command should succeed on valid model but exited with code $EXIT_CODE_VALID"
  echo "Output: $OUTPUT_VALID"
  exit 1
fi

if ! echo "$OUTPUT_VALID" | grep -q "No validation issues found"; then
  echo "❌ FAILED: Validate command should output 'No validation issues found' for valid model"
  echo "Actual output: $OUTPUT_VALID"
  exit 1
fi

# Test 3.2: Validate command with --json on valid model
# Test 3.2: Validate command with --json flag on valid model
echo "Running: reqvire validate --json (valid-test)" >> "${TEST_DIR}/test_results_pass2.log"
set +e
OUTPUT_VALID_JSON=$("$REQVIRE_BIN" validate --json 2>&1)
EXIT_CODE_VALID_JSON=$?
set -e

echo "Exit code: $EXIT_CODE_VALID_JSON" >> "${TEST_DIR}/test_results_pass2.log"
printf "%s\n" "$OUTPUT_VALID_JSON" >> "${TEST_DIR}/test_results_pass2.log"

if [ $EXIT_CODE_VALID_JSON -ne 0 ]; then
  echo "❌ FAILED: Validate command with --json should succeed on valid model but exited with code $EXIT_CODE_VALID_JSON"
  echo "Output: $OUTPUT_VALID_JSON"
  exit 1
fi

# Check if output is valid JSON
if ! echo "$OUTPUT_VALID_JSON" | python3 -m json.tool > /dev/null 2>&1; then
  echo "❌ FAILED: Validate command --json output is not valid JSON"
  echo "Output: $OUTPUT_VALID_JSON"
  exit 1
fi

# Test 3.3: Validate command on invalid model (Pass 1 errors)
# Test 3.3: Validate command on invalid model (Pass 1 errors)
cd "${TEST_DIR}/pass1-errors"
echo "Running: reqvire validate (pass1-errors again)" >> "${TEST_DIR}/test_results_pass2.log"
set +e
OUTPUT_VALIDATE_PASS1=$("$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_VALIDATE_PASS1=$?
set -e

echo "Exit code: $EXIT_CODE_VALIDATE_PASS1" >> "${TEST_DIR}/test_results_pass2.log"
printf "%s\n" "$OUTPUT_VALIDATE_PASS1" >> "${TEST_DIR}/test_results_pass2.log"

# Validate command should exit with non-zero code for invalid model
if [ $EXIT_CODE_VALIDATE_PASS1 -eq 0 ]; then
  echo "❌ FAILED: Validate command should fail on invalid model but returned success (0)"
  echo "Output: $OUTPUT_VALIDATE_PASS1"
  exit 1
fi

# Check that validate command reports the expected validation errors
for expected in "${EXPECTED_PASS1_ERRORS[@]}"; do
  if ! echo "$OUTPUT_VALIDATE_PASS1" | grep -q "$expected"; then
    echo "❌ FAILED: Validate command missing expected Pass 1 error: $expected"
    echo "Output: $OUTPUT_VALIDATE_PASS1"
    exit 1
  fi
done

# Test 3.4: Validate command on invalid model (Pass 2 errors)
# Test 3.4: Validate command on invalid model (Pass 2 errors)
cd "${TEST_DIR}/pass2-errors"
echo "Running: reqvire validate (pass2-errors again)" >> "${TEST_DIR}/test_results_pass2.log"
set +e
OUTPUT_VALIDATE_PASS2=$("$REQVIRE_BIN" validate 2>&1)
EXIT_CODE_VALIDATE_PASS2=$?
set -e

echo "Exit code: $EXIT_CODE_VALIDATE_PASS2" >> "${TEST_DIR}/test_results_pass2.log"
printf "%s\n" "$OUTPUT_VALIDATE_PASS2" >> "${TEST_DIR}/test_results_pass2.log"

# Validate command should exit with non-zero code for invalid model
if [ $EXIT_CODE_VALIDATE_PASS2 -eq 0 ]; then
  echo "❌ FAILED: Validate command should fail on invalid model but returned success (0)"
  echo "Output: $OUTPUT_VALIDATE_PASS2"
  exit 1
fi

# Check that validate command reports the expected validation errors
for expected in "${EXPECTED_PASS2_ERRORS[@]}"; do
  if ! echo "$OUTPUT_VALIDATE_PASS2" | grep -q "$expected"; then
    echo "❌ FAILED: Validate command missing expected Pass 2 error: $expected"
    echo "Output: $OUTPUT_VALIDATE_PASS2"
    exit 1
  fi
done

# Test 3.5: Verify validate command does not modify files
# Test 3.5: Verify validate command does not modify files
cd "${TEST_DIR}/valid-test"

# Get hash of a file before validation
if [ -f "specifications/ValidRequirements.md" ]; then
  BEFORE_HASH=$(sha256sum "specifications/ValidRequirements.md")
else
  echo "❌ FAILED: Test file specifications/ValidRequirements.md not found"
  exit 1
fi

# Run validate command
"$REQVIRE_BIN" validate > /dev/null 2>&1

# Get hash after validation
AFTER_HASH=$(sha256sum "specifications/ValidRequirements.md")

if [ "$BEFORE_HASH" != "$AFTER_HASH" ]; then
  echo "❌ FAILED: Validate command modified files during validation"
  echo "Before: $BEFORE_HASH"
  echo "After: $AFTER_HASH"
  exit 1
fi


exit 0