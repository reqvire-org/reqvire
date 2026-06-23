#!/bin/bash
set -uo pipefail  # NOTE: Do NOT use -e, it causes silent failures with diff

# Test: Add Command Contextual Error Messages
# Acceptance Criteria:
# - When format parsing fails, error message includes example of correctly formatted element
# - Error messages provide clear guidance on expected format
#
# Test Criteria:
# - Command exits with error (non-zero) for invalid input
# - Error output contains example element markdown
# - Error output contains specific format guidance
# - Example includes all subsections: Details, Metadata, Relations, Contract Bindings

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test 1: Missing ### header
echo "Test 1: Missing ### header..."
OUTPUT=$(cd "$TEST_DIR" && echo "This is invalid markdown without a header" | "$REQVIRE_BIN" add system-model/test.md 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Expected error for missing header"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Example of correctly formatted element"; then
  echo "❌ FAILED: Error message missing example"
  echo "$OUTPUT"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "### Element Name"; then
  echo "❌ FAILED: Example missing element header"
  exit 1
fi

echo "✅ Test 1 passed"

# Test 2: Multiple ### headers
echo "Test 2: Multiple ### headers..."
OUTPUT=$(cd "$TEST_DIR" && cat <<'EOF' | "$REQVIRE_BIN" add system-model/test.md 2>&1
### First Header

Description

### Second Header

More description
EOF
)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Expected error for multiple headers"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Example of correctly formatted element"; then
  echo "❌ FAILED: Error message missing example for multiple headers"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "#### Metadata"; then
  echo "❌ FAILED: Example missing Metadata subsection"
  exit 1
fi

echo "✅ Test 2 passed"

# Test 3: Invalid metadata format
echo "Test 3: Invalid metadata format..."
OUTPUT=$(cd "$TEST_DIR" && cat <<'EOF' | "$REQVIRE_BIN" add system-model/test.md 2>&1
### Test Element

Description

#### Metadata
type: requirement
EOF
)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Expected error for invalid metadata"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Expected format.*key: value"; then
  echo "❌ FAILED: Error message missing format guidance for metadata"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Example of correctly formatted element"; then
  echo "❌ FAILED: Error message missing example for metadata error"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "  \* type: requirement"; then
  echo "❌ FAILED: Example missing correct metadata format"
  exit 1
fi

echo "✅ Test 3 passed"

# Test 4: Invalid relation format
echo "Test 4: Invalid relation format..."
OUTPUT=$(cd "$TEST_DIR" && cat <<'EOF' | "$REQVIRE_BIN" add system-model/test.md 2>&1
### Test Element

Description

#### Relations
bad relation format
EOF
)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Expected error for invalid relation"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Expected format.*relationType:.*Text.*link"; then
  echo "❌ FAILED: Error message missing format guidance for relations"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Example of correctly formatted element"; then
  echo "❌ FAILED: Error message missing example for relation error"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "#### Relations"; then
  echo "❌ FAILED: Example missing Relations subsection"
  exit 1
fi

echo "✅ Test 4 passed"

# Test 5: Invalid contract_bindings format
echo "Test 5: Invalid contract_bindings format..."
OUTPUT=$(cd "$TEST_DIR" && cat <<'EOF' | "$REQVIRE_BIN" add system-model/test.md 2>&1
### Test Element

Description

#### Contract Bindings
invalid contract_bindings
EOF
)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Expected error for invalid contract_bindings"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Expected format.*Text.*link"; then
  echo "❌ FAILED: Error message missing format guidance for contract_bindings"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Example of correctly formatted element"; then
  echo "❌ FAILED: Error message missing example for contract_bindings error"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "#### Contract Bindings"; then
  echo "❌ FAILED: Example missing Contract Bindings subsection"
  exit 1
fi

echo "✅ Test 5 passed"

# Test 6: Example completeness - verify all subsections are in example
echo "Test 6: Example completeness..."
OUTPUT=$(cd "$TEST_DIR" && echo "invalid" | "$REQVIRE_BIN" add system-model/test.md 2>&1)

if ! echo "$OUTPUT" | grep -q "#### Details"; then
  echo "❌ FAILED: Example missing Details subsection"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "derivedFrom:"; then
  echo "❌ FAILED: Example missing derivedFrom relation example"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "satisfiedBy:"; then
  echo "❌ FAILED: Example missing satisfiedBy relation example"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "\-\-\-"; then
  echo "❌ FAILED: Example missing separator"
  exit 1
fi

echo "✅ Test 6 passed"

echo "✅ All tests passed"
exit 0
