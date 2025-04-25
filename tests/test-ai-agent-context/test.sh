#!/bin/bash

# Test: AI Agent Context Generation
# ----------------------------------------------------
# Acceptance Criteria:
# - System should provide comprehensive context for AI agents
# - Context should include information about Reqvire usage and methodology
# - Context should be accessible via a dedicated command
#
# Test Criteria:
# - Command exits with success (0) return code
# - Context information is comprehensive and usable by AI agents
# - Command output is properly formatted for AI consumption
#

# Create output directory if it doesn't exist
mkdir -p "${TEST_DIR}/output"

# Run reqvire with --ai-context flag to generate AI context
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "${TEST_DIR}/reqvire.yaml" --llm-context 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# Verify exit code indicates success
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: AI context command failed with exit code $EXIT_CODE"
  exit 1
fi

# Check that output contains essential Reqvire concepts
if ! echo "$OUTPUT" | grep -q "Requirements"; then
  echo "❌ FAILED: AI context missing information about Requirements"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Relations"; then
  echo "❌ FAILED: AI context missing information about Relations"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Verification"; then
  echo "❌ FAILED: AI context missing information about Verification"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Metadata"; then
  echo "❌ FAILED: AI context missing information about Metadata"
  exit 1
fi

# Check that output is in proper markdown format
if ! echo "$OUTPUT" | grep -q "^#"; then
  echo "❌ FAILED: AI context not formatted in markdown with headers"
  exit 1
fi

echo "✅ PASSED: AI agent context test"
exit 0
