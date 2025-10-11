#!/usr/bin/env bash
set -euo pipefail

# Test: CLI Help Structure Verification
# --------------------------------------
# Satisfies: specifications/Verifications/Misc.md#cli-help-structure-verification
#
# Acceptance Criteria:
# - Running reqvire (without arguments) displays the main help output as default command
# - Main help output matches the expected structure with all commands and options flattened
#
# Test Criteria:
# - Command exits with success (0) return code
# - Output matches expected.txt exactly

# Get the directory where this test script is located (not the temp directory)
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Run reqvire without arguments to get help output
set +e
ACTUAL_OUTPUT=$("$REQVIRE_BIN" 2>&1)
EXIT_CODE=$?
set -e

# Check exit code is 0
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: reqvire (no args) should exit with code 0, got $EXIT_CODE"
  exit 1
fi

# Compare with expected output
if ! diff -u "${TEST_SCRIPT_DIR}/expected.txt" <(echo "$ACTUAL_OUTPUT"); then
  echo "❌ FAILED: CLI help output does not match expected format"
  echo ""
  echo "Differences shown above (expected vs actual)"
  echo ""
  echo "If the changes are intentional, update tests/test-cli-help-structure/expected.txt"
  exit 1
fi

echo "✅ PASSED: CLI help structure verification"
exit 0
