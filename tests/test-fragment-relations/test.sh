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


OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --validate 2>&1)
EXIT_CODE=$?


printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"


# Check that output doesn't contain error messages about missing fragment targets
MISSING_FRAGMENT_PATTERN=" Failed to normalize identifier for 'NOTIF-IMPL-"
if echo "$OUTPUT" | grep -q "$MISSING_FRAGMENT_PATTERN"; then
  echo "FAILED: Found unexpected error about missing fragment target"
  exit 1
fi

# Check for kebab-case fragment reference by element name (reliability-requirements)
MISSING_KEBAB_PATTERN="Missing relation target: .*#reliability-requirements"
if echo "$OUTPUT" | grep -q "$MISSING_KEBAB_PATTERN"; then
  echo "FAILED: Found unexpected error about missing kebab-case fragment target"
  exit 1
fi

# Check for fragment reference by element ID (#NOTIF-ARCH-001)
MISSING_ELEMENT_ID_PATTERN="Referenced file not found: #NOTIF-ARCH-001"
if echo "$OUTPUT" | grep -q "$MISSING_ELEMENT_ID_PATTERN"; then
  echo "FAILED: Found unexpected error about missing element ID fragment target"
  exit 1
fi

# Verify that successful validation message is displayed
if ! echo "$OUTPUT" | grep -q "Validation completed successfully with no errors."; then
  echo "FAILED: Missing success message for relation validation"
  exit 1
fi

exit 0
