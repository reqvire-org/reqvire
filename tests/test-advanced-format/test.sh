#!/bin/bash

# Test: Format Command Requirements Verification
# This test verifies the format command requirements from SystemRequirements and UserRequirements

#set -e

# Use the test directory provided by run_tests.sh
cd "$TEST_DIR"

# No need to backup files - we're already in a temporary directory

# Use the reqvire binary provided by test runner
REQVIRE="$REQVIRE_BIN"

# Test 1: Requirement - System shall provide format command with dry-run as default
echo "Starting test..." > test_results.log

echo "Running: reqvire format (preview mode - default)" >> test_results.log
set +e
"$REQVIRE" format > dry_run_output.txt 2>&1
DRY_RUN_EXIT_CODE=$?
set -e

echo "Exit code: $DRY_RUN_EXIT_CODE" >> test_results.log
cat dry_run_output.txt >> test_results.log

if [ $DRY_RUN_EXIT_CODE -ne 0 ]; then
    echo "FAIL: Format command failed with exit code $DRY_RUN_EXIT_CODE"
    exit 1
fi

# Test 1.1: Verify dry-run shows expected changes
# Remove ANSI color codes for easier text matching
sed 's/\x1b\[[0-9;]*m//g' dry_run_output.txt > dry_run_clean.txt

# Test 1.2: Compare diff output with expected diff file (file order is now deterministic)
if [ -f expected_diff.txt ]; then
    if ! diff -u expected_diff.txt dry_run_clean.txt > /dev/null; then
        echo "FAIL: Diff output does not match expected diff output"
        echo "Differences (first 80 lines):"
        diff -u expected_diff.txt dry_run_clean.txt | head -80
        exit 1
    fi
fi

EXPECTED_LINK_CONVERSION="\[MOE_UA\](MOEs.md#moe_ua)"
EXPECTED_SIMPLE_ID_CONVERSION="\[Requirements Processing\](SystemRequirements/Requirements.md#requirements-processing)"
EXPECTED_SEPARATOR_ADDITION="\+.*---"

# Test absolute path conversion from subfolder to root
EXPECTED_SUBFOLDER_TO_ROOT="\[Managing MBSE Models\](../UserStories.md#managing-mbse-models)"
# Test absolute path conversion from subfolder to another subfolder
EXPECTED_SUBFOLDER_TO_SUBFOLDER="\[Format Test\](../Verifications/Tests.md#format-test)"
# Test absolute path conversion to internal file (from SystemRequirements subfolder)
EXPECTED_ABSOLUTE_INTERNAL_PATH="../core/src/parser.rs"
# Test absolute path from verification in subfolder to rs file (from Verifications subfolder)
EXPECTED_VERIFICATION_ABSOLUTE_PATH="../core/src/element.rs"

if ! grep -q "$EXPECTED_LINK_CONVERSION" dry_run_clean.txt; then
    echo "FAIL: Dry run does not show absolute link conversion to relative" >> test_results.log
    echo "Expected: $EXPECTED_LINK_CONVERSION" >> test_results.log
    echo "Dry run output:" >> test_results.log
    cat dry_run_clean.txt >> test_results.log
    exit 1
fi

if ! grep -q "$EXPECTED_SIMPLE_ID_CONVERSION" dry_run_clean.txt; then
    echo "FAIL: Dry run does not show simple identifier conversion to markdown link"
    exit 1
fi

if ! grep -q "$EXPECTED_SEPARATOR_ADDITION" dry_run_clean.txt; then
    echo "FAIL: Dry run does not show separator addition"
    exit 1
fi

if ! grep -q "$EXPECTED_SUBFOLDER_TO_ROOT" dry_run_clean.txt; then
    echo "FAIL: Dry run does not show absolute path conversion from subfolder to root"
    exit 1
fi

if ! grep -q "$EXPECTED_SUBFOLDER_TO_SUBFOLDER" dry_run_clean.txt; then
    echo "FAIL: Dry run does not show absolute path conversion from subfolder to subfolder"
    exit 1
fi

if ! grep -q "$EXPECTED_ABSOLUTE_INTERNAL_PATH" dry_run_clean.txt; then
    echo "FAIL: Dry run does not show absolute internal path conversion"
    exit 1
fi

if ! grep -q "$EXPECTED_VERIFICATION_ABSOLUTE_PATH" dry_run_clean.txt; then
    echo "FAIL: Dry run does not show verification absolute path conversion"
    exit 1
fi

# Test 2: Apply formatting changes with --fix flag
echo "Running: reqvire format --fix" >> test_results.log
set +e
"$REQVIRE" format --fix > format_output.txt 2>&1
FORMAT_EXIT_CODE=$?
set -e

echo "Exit code: $FORMAT_EXIT_CODE" >> test_results.log
cat format_output.txt >> test_results.log

if [ $FORMAT_EXIT_CODE -ne 0 ]; then
    echo "FAIL: Format command execution failed with exit code $FORMAT_EXIT_CODE"
    exit 1
fi

# Test 2.1: Verify that format command shows diff output when applying changes
# If files were changed, we should see diff output; if no changes, we should see "No files needed formatting"
if [ $FORMAT_EXIT_CODE -eq 0 ]; then
    if grep -q "Formatted [0-9]* file(s)" format_output.txt; then
        # Changes were made - should show diff output
        if ! grep -q "UserStories.md\|SystemRequirements\|Verifications" format_output.txt; then
            echo "FAIL: Format command should show file names when applying changes"
            cat format_output.txt
            exit 1
        fi
    elif ! grep -q "No files needed formatting" format_output.txt; then
        echo "FAIL: Format command should indicate if no formatting was needed"
        cat format_output.txt
        exit 1
    fi
fi

# Test 3: Verify expected changes were applied with exact content matching

# Test 3.1: Check that absolute links were converted to relative
EXPECTED_RELATIVE_LINK="\[MOE_UA\](MOEs.md#moe_ua)"
if ! grep -q "$EXPECTED_RELATIVE_LINK" UserStories.md; then
    echo "FAIL: Absolute links not converted to relative"
    exit 1
fi

# Test 3.2: Check that simple identifiers were converted to markdown links with readable names
EXPECTED_MARKDOWN_LINK="\[Requirements Processing\](SystemRequirements/Requirements.md#requirements-processing)"
if ! grep -q "$EXPECTED_MARKDOWN_LINK" UserStories.md; then
    echo "FAIL: Simple identifiers not converted to markdown links with readable names"
    exit 1
fi

# Test 3.3: Check that separators were added where missing
if ! grep -A 4 "derive: \[Requirements Processing\]" UserStories.md | grep -q "^---$"; then
    echo "FAIL: Element separators not added"
    exit 1
fi

# Test 3.3a: Check absolute path conversions in subfolders were applied
EXPECTED_SUBFOLDER_TO_ROOT_APPLIED="\[Managing MBSE Models\](../UserStories.md#managing-mbse-models)"
if ! grep -q "$EXPECTED_SUBFOLDER_TO_ROOT_APPLIED" SystemRequirements/Requirements.md; then
    echo "FAIL: Absolute path from subfolder to root not converted correctly"
    exit 1
fi

# Test 3.3b: Check subfolder to subfolder path conversions
EXPECTED_SUBFOLDER_TO_SUBFOLDER_APPLIED="\[Format Test\](../Verifications/Tests.md#format-test)"
if ! grep -q "$EXPECTED_SUBFOLDER_TO_SUBFOLDER_APPLIED" SystemRequirements/Requirements.md; then
    echo "FAIL: Absolute path from subfolder to subfolder not converted correctly"
    exit 1
fi

# Test 3.3c: Check absolute internal path conversions
EXPECTED_ABSOLUTE_INTERNAL_PATH_APPLIED="../core/src/parser.rs"
if ! grep -q "$EXPECTED_ABSOLUTE_INTERNAL_PATH_APPLIED" SystemRequirements/Requirements.md; then
    echo "FAIL: Absolute internal path not converted correctly"
    exit 1
fi

# Test 3.3d: Check verification absolute path conversions
EXPECTED_VERIFICATION_ABSOLUTE_PATH_APPLIED="../core/src/element.rs"
if ! grep -q "$EXPECTED_VERIFICATION_ABSOLUTE_PATH_APPLIED" Verifications/Tests.md; then
    echo "FAIL: Verification absolute path not converted correctly"
    exit 1
fi

# Test 3.4: Check that personas section is preserved exactly
EXPECTED_PERSONA="System Engineer"
if ! grep -A 5 "## Personas" UserStories.md | grep -q "$EXPECTED_PERSONA"; then
    echo "FAIL: Personas section not preserved"
    exit 1
fi

# Test 3.5: Check that user-requirement metadata is preserved/added
EXPECTED_USER_REQ_METADATA="type: user-requirement"
if ! grep -q "$EXPECTED_USER_REQ_METADATA" UserStories.md; then
    echo "FAIL: user-requirement metadata not preserved/added"
    exit 1
fi

# Test 3.6: Check that consecutive separators are normalized to single separator
# Look for any case where a separator is immediately followed by another separator
if grep -A 1 "^---$" UserStories.md | grep -A 1 "^$" | grep -q "^---$"; then
    echo "FAIL: Consecutive separators not normalized to single separator"
    exit 1
fi

# Test 3.7: Check that relation indentation is normalized to 2 spaces
if grep -q "^\\* " UserStories.md; then
    echo "FAIL: Relations not properly indented with 2 spaces"
    exit 1
fi

# Test 3.8: Check that blank line is added before metadata when missing
# The test element "Missing Blank Line Test" should have a blank line added before #### Metadata
if ! grep -A 2 "This element has content but no blank line before metadata." UserStories.md | grep -q "^$"; then
    echo "FAIL: Blank line not added before metadata section"
    echo "Content around the test element:"
    grep -A 3 -B 1 "Missing Blank Line Test" UserStories.md
    exit 1
fi

# Test 3.9: Check that blank line is added before repositioned metadata
# The test element "Metadata Repositioning Test" has metadata early, then Details/Acceptance Criteria
# After formatting, metadata should be at the end with a blank line before it
# Extract just the "Metadata Repositioning Test" element and check for blank line before #### Metadata
if ! awk '/^### Metadata Repositioning Test$/,/^---$/' UserStories.md | grep -B 1 "^#### Metadata$" | head -1 | grep -q "^$"; then
    echo "FAIL: Blank line not added before repositioned metadata section"
    echo "Content around the test element:"
    grep -A 20 "Metadata Repositioning Test" UserStories.md
    exit 1
fi

# Test 3.10: Check that content inside <details> blocks is not formatted
# The element "Details Block Formatting Test" has improperly formatted headers inside <details>
# After formatting, content inside <details> should remain unchanged
if ! grep -A 10 "Details Block Formatting Test" UserStories.md | grep -q "####Another Header Without Space"; then
    echo "FAIL: Content inside <details> block was incorrectly formatted"
    echo "Content around the test element:"
    grep -A 15 "Details Block Formatting Test" UserStories.md
    exit 1
fi

# Test 3.11: Verify complete formatted file matches expected output
# Compare entire UserStories.md file with expected output
if [ -f expected_UserStories.md ]; then
    if ! diff -u expected_UserStories.md UserStories.md > /dev/null; then
        echo "FAIL: UserStories.md does not match expected output"
        echo "Differences:"
        diff -u expected_UserStories.md UserStories.md | head -50
        exit 1
    fi
else
    # Fallback to line count if expected file doesn't exist
    EXPECTED_USERSTORIES_LINES=90
    ACTUAL_USERSTORIES_LINES=$(wc -l < UserStories.md)
    if [ "$ACTUAL_USERSTORIES_LINES" -ne "$EXPECTED_USERSTORIES_LINES" ]; then
        echo "FAIL: UserStories.md line count mismatch. Expected $EXPECTED_USERSTORIES_LINES, got $ACTUAL_USERSTORIES_LINES"
        exit 1
    fi
fi

# Compare entire MOEs.md file with expected output
if [ -f expected_MOEs.md ]; then
    if ! diff -u expected_MOEs.md MOEs.md > /dev/null; then
        echo "FAIL: MOEs.md does not match expected output"
        echo "Differences:"
        diff -u expected_MOEs.md MOEs.md | head -30
        exit 1
    fi
else
    # Fallback to line count
    EXPECTED_MOES_LINES=21
    ACTUAL_MOES_LINES=$(wc -l < MOEs.md)
    if [ "$ACTUAL_MOES_LINES" -ne "$EXPECTED_MOES_LINES" ]; then
        echo "FAIL: MOEs.md line count mismatch. Expected $EXPECTED_MOES_LINES, got $ACTUAL_MOES_LINES"
        exit 1
    fi
fi

# Test 4: Verify no additional changes needed (idempotent behavior)
echo "Running: reqvire format (preview mode - second time)" >> test_results.log
set +e
"$REQVIRE" format > no_changes_output.txt 2>&1
NO_CHANGES_EXIT_CODE=$?
set -e

echo "Exit code: $NO_CHANGES_EXIT_CODE" >> test_results.log
cat no_changes_output.txt >> test_results.log

if [ $NO_CHANGES_EXIT_CODE -ne 0 ]; then
    echo "FAIL: Second dry-run failed with exit code $NO_CHANGES_EXIT_CODE"
    exit 1
fi

EXPECTED_NO_CHANGES_MSG="No formatting changes needed"
if ! grep -q "$EXPECTED_NO_CHANGES_MSG" no_changes_output.txt; then
    echo "FAIL: Format command should report no changes needed after formatting"
    exit 1
fi

# Test 5: Verify diff output format contains expected elements
if ! grep -q "UserStories.md" dry_run_clean.txt; then
    echo "FAIL: Dry run output missing UserStories.md filename"
    exit 1
fi

if ! grep -q "+" dry_run_clean.txt; then
    echo "FAIL: Dry run output missing + addition markers"
    exit 1
fi

if ! grep -q "\-" dry_run_clean.txt; then
    echo "FAIL: Dry run output missing - removal markers"
    exit 1
fi

# Test 6: Verify line numbering is sequential and consistent
# Extract line numbers from diff output and check for proper sequencing
DIFF_LINE_NUMBERS=$(grep -E "^  [0-9]+" dry_run_clean.txt | sed 's/^  *//' | sed 's/ .*//' | tr '\n' ' ')

# Check that we have some line numbers
if [ -z "$DIFF_LINE_NUMBERS" ]; then
    echo "FAIL: No line numbers found in diff output"
    exit 1
fi

# Test 6.1: Line number verification is now handled by expected_diff.txt comparison
# The expected_diff.txt file captures the exact line numbers in the diff output
# Any changes to line numbering will be caught by the diff comparison in Test 1.2

# Test 7: Test JSON output functionality
# Run format with --json flag on current files to test JSON output (default preview mode)
echo "Running: reqvire format --json" >> test_results.log
set +e
"$REQVIRE" format --json > json_format_output.txt 2>&1
JSON_FORMAT_EXIT_CODE=$?
set -e

echo "Exit code: $JSON_FORMAT_EXIT_CODE" >> test_results.log
cat json_format_output.txt >> test_results.log

if [ $JSON_FORMAT_EXIT_CODE -ne 0 ]; then
    echo "FAIL: Format command with --json flag failed with exit code $JSON_FORMAT_EXIT_CODE"
    exit 1
fi

# Check if output is valid JSON
if ! python3 -m json.tool < json_format_output.txt > /dev/null 2>&1; then
    echo "FAIL: Format command --json output is not valid JSON"
    echo "Output:"
    cat json_format_output.txt
    exit 1
fi

# Test 8: Document Structure Normalization - Missing Headers
# Test 8.1: File with no page header and no section header
if [ -f expected_TestNoHeaders.md ]; then
    if ! diff -u expected_TestNoHeaders.md TestNoHeaders.md > /dev/null; then
        echo "FAIL: TestNoHeaders.md does not match expected output (headers not added correctly)"
        echo "Differences:"
        diff -u expected_TestNoHeaders.md TestNoHeaders.md | head -30
        exit 1
    fi
fi

# Test 8.2: File with page header but no section header
if [ -f expected_TestNoSectionHeader.md ]; then
    if ! diff -u expected_TestNoSectionHeader.md TestNoSectionHeader.md > /dev/null; then
        echo "FAIL: TestNoSectionHeader.md does not match expected output (section header not added)"
        echo "Differences:"
        diff -u expected_TestNoSectionHeader.md TestNoSectionHeader.md | head -30
        exit 1
    fi
fi

# Test 8.3: File with section header but no page header
if [ -f expected_TestNoPageHeader.md ]; then
    if ! diff -u expected_TestNoPageHeader.md TestNoPageHeader.md > /dev/null; then
        echo "FAIL: TestNoPageHeader.md does not match expected output (page header not added)"
        echo "Differences:"
        diff -u expected_TestNoPageHeader.md TestNoPageHeader.md | head -30
        exit 1
    fi
fi

# No cleanup needed - temporary directory will be deleted
