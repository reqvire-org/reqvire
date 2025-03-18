#!/bin/bash

# Test: Whitespace Linting Functionality
# --------------------------------------
# Acceptance Criteria:
# - System should detect excess whitespace after headers
# - System should fix excess whitespace in linting mode
# - The output should show before/after changes
#
# Test Criteria:
# - Command exits with success (0) return code
# - Output shows whitespace being fixed
# - Output should contain diff-style formatting
#




OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --lint --dry-run 2>&1)
EXIT_CODE=$?

printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

#echo "$OUTPUT"

ISSUES=$(awk '/# [^:]+:/' "${TEST_DIR}/test_results.log")
ISSUE_COUNTS=$(echo "$ISSUES" | awk -F': ' '{counts[$1]++} END {for (type in counts) print counts[type], type}')

EXPECTED_ISSUES=$(cat <<EOF
1 # Nonlink identifier
1 # Inconsistent reserved subsections
2 # Inconsistent newlines
2 # Excess whitespace
1 # Missing separator
EOF
)

# Remove ANSI color codes
ISSUE_COUNTS=$(echo "$ISSUE_COUNTS" | sed $'s/\x1b\[[0-9;]*m//g')
EXPECTED_ISSUES=$(echo "$EXPECTED_ISSUES" | sed $'s/\x1b\[[0-9;]*m//g')

# Normalize output (trim spaces, remove newlines, fix Windows endings)
ISSUE_COUNTS=$(echo "$ISSUE_COUNTS" | awk 'NF' | tr -d '\r')
EXPECTED_ISSUES=$(echo "$EXPECTED_ISSUES" | awk 'NF' | tr -d '\r')

# Verify results
if ! diff -wB <(echo "$ISSUE_COUNTS" | sort) <(echo "$EXPECTED_ISSUES" | sort) > /dev/null; then
    echo "❌ Issue counts do NOT match expected results!"
    echo "🔍 Expected:"
    echo "$EXPECTED_ISSUES"
    echo "🔎 Found:"
    echo "$ISSUE_COUNTS"
    exit 1
fi

# Run linting with changes applied
echo "Running linting with changes applied..."
OUTPUT=$(cd "$TEST_DIR" && "$REQFLOW_BIN" --config "${TEST_DIR}/reqflow.yaml"  --lint 2>&1)
EXIT_CODE=$?



printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

if [ $EXIT_CODE -ne 0 ]; then
  echo "FAILED: Lint command returned error: $EXIT_CODE"
  exit 1
fi


EXPECTED_CONTENT="# Test Requirements Document

### Element Header

Content

#### Subsection Header

More content


---


### Requirement
 
More content

#### Relations
  * refine: [Element Header](#element-header)"



# Remove ANSI color codes

GOTTEN_CONTENT=$(cat "$TEST_DIR/Requirements.md")

#echo "$GOTTEN_CONTENT"

if ! diff <(echo "$EXPECTED_CONTENT") <(echo "$GOTTEN_CONTENT") > /dev/null; then
  echo "FAILED: Lint fix didn't work as expected."
  diff -u <(echo "$EXPECTED_CONTENT") <(echo "$GOTTEN_CONTENT")
  exit 1
fi

exit 0
