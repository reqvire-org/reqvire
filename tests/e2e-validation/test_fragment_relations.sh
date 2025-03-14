#!/bin/bash

# Test: Validation of Same-File Fragment Relations
# -----------------------------------------------
# Acceptance Criteria:
# - System should correctly validate relations to fragments within the same file
# - System should not report errors for valid fragment references
#
# Test Criteria:
# - Command exits with success (zero) return code
# - No error output about missing relation targets when using #fragment references
#
# Test Implementation for Verification:
# - Tests requirements with fragment-only references like "#fragment-id"
# - Tests fragments referenced by proper element ID

# Setup

TEST_NAME="Same-File Fragment Relations Validation"
TEST_DIR="../fixtures/test-fragment-relations"
REQFLOW_BIN="${REQFLOW_BIN:-$(cd ../../ && pwd)/target/debug/reqflow}"

echo "======== $TEST_NAME ========"
echo "Test directory: $TEST_DIR"

# Run validation with increased logging level for debugging
echo "Running validation command..."
OUTPUT=$(cd "$TEST_DIR" && RUST_LOG=debug "$REQFLOW_BIN" --validate-relations --verbose 2>&1)
EXIT_CODE=$?

# Verify results
echo "Checking results..."

# Verify exit code indicates success (0)
if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Validation should have succeeded but returned error ($EXIT_CODE)"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "Exit code $EXIT_CODE indicates validation succeeded as expected"

# Check that output doesn't contain error messages about missing fragment targets
MISSING_FRAGMENT_PATTERN="Referenced file not found: #NOTIF-IMPL"
if echo "$OUTPUT" | grep -q "$MISSING_FRAGMENT_PATTERN"; then
  echo "FAILED: Found unexpected error about missing fragment target"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check for kebab-case fragment reference by element name (reliability-requirements)
MISSING_KEBAB_PATTERN="Missing relation target: .*#reliability-requirements"
if echo "$OUTPUT" | grep -q "$MISSING_KEBAB_PATTERN"; then
  echo "FAILED: Found unexpected error about missing kebab-case fragment target"
  echo "Output: $OUTPUT"
  exit 1
fi

# Check for fragment reference by element ID (#NOTIF-ARCH-001)
MISSING_ELEMENT_ID_PATTERN="Referenced file not found: #NOTIF-ARCH-001"
if echo "$OUTPUT" | grep -q "$MISSING_ELEMENT_ID_PATTERN"; then
  echo "FAILED: Found unexpected error about missing element ID fragment target"
  echo "Output: $OUTPUT"
  exit 1
fi

# Verify that successful validation message is displayed
if ! echo "$OUTPUT" | grep -q "Relations validation passed\|All validations passed"; then
  echo "FAILED: Missing success message for relation validation"
  echo "Output: $OUTPUT"
  exit 1
fi

echo "SUCCESS: All same-file fragment references were correctly validated"
exit 0
