# End-to-End Test Guide for Reqvire

## Overview
This guide covers how to write and execute end-to-end tests for Reqvire. Tests validate complete functionality by running reqvire commands and comparing outputs against expected results.

## Test Execution

### Run All Tests
From the root directory:
```bash
./tests/run_tests.sh
```

### Run Specific Test
```bash
./tests/run_tests.sh test-diagram-generation
```

### Build Before Testing
Tests use the debug binary:
```bash
cargo build
./tests/run_tests.sh
```

## Test Structure

### Test Directory Layout
Each test is a directory containing:
```
test-capability-name/
├── test.sh                    # Test execution script (REQUIRED)
└── specifications/           # Test markdown files
    ├── Requirements.md      # Test requirements
    └── Verifications/       # Test verifications
```

### Available Tests
Use `ls tests/` to see all available test directories. Each `test-*` directory contains end-to-end tests for specific functionality.

## Writing Test Scripts

### Test Script Template (test.sh)
```bash
#!/bin/bash

# Test: Capability Description
# Acceptance Criteria:
# - List specific behaviors to test
# - Define success conditions
#
# Test Criteria:
# - Command exits with success (0)
# - Output matches expected format
# - Files are modified as expected

# Run reqvire command
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" command 2>&1)
EXIT_CODE=$?

# Save output for debugging
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# Check exit code
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Command returned error: $EXIT_CODE"
  cat "${TEST_DIR}/test_results.log"
  exit 1
fi

# Perform specific validations
# ... test-specific checks ...

exit 0
```

### Environment Variables
Available in test scripts:
- `$TEST_DIR` - Temporary test directory with copied files
- `$REQVIRE_BIN` - Path to reqvire binary
- Git repository is initialized in `$TEST_DIR`

### Test Infrastructure
The test runner (`run_tests.sh`):
1. Creates temporary directory for each test
2. Copies test files to temp directory
3. Initializes git repository
4. Runs `test.sh` in test context
5. Reports pass/fail results
6. Cleans up temporary files

## Common Test Patterns

### CRITICAL: Expected Output Files with Diff Comparison

**This is the preferred and REQUIRED pattern for file modification tests.** Instead of using inline grep checks, tests MUST use expected output files and `diff -u` for comparison. This ensures:
- Clear visibility of what changed when tests fail
- Easy updates when intentional changes are made
- Deterministic, reproducible test results

#### Directory Structure
```
test-capability-name/
├── test.sh                    # Test execution script
├── .reqvireignore             # Excludes expected/ from reqvire parsing
├── expected/                  # Expected output files (committed to git)
│   ├── 01-after-step1.md     # Expected state after step 1
│   ├── 02-after-step2.md     # Expected state after step 2
│   └── output.txt            # Expected command output
└── specifications/           # Initial test files
    └── Requirements.md
```

#### CRITICAL: .reqvireignore vs .gitignore for Expected Files

**IMPORTANT - When to use .reqvireignore vs .gitignore:**

- **Use `.reqvireignore`** (PREFERRED for most tests):
  - Excludes files from reqvire parsing ONLY
  - Files remain tracked by git and committed to repository
  - Expected output files can be version controlled
  - Use this for tests with `expected/` directories that should be committed

- **Use `.gitignore`** (ONLY for specific .gitignore functionality tests):
  - Excludes files from BOTH git AND reqvire
  - Files are untracked and not committed to repository
  - Only use in tests that specifically test .gitignore behavior:
    - `test-gitignore-integration` - Tests .gitignore integration
    - `test-crud-target-location-validation` - Tests .gitignore exclusion validation

**For regular tests with expected output files**, create `.reqvireignore`:
```
# .reqvireignore in test directory
expected/
```

This prevents reqvire from parsing expected files (which contain `# Elements` headers that would cause duplicate element errors) while allowing them to be committed to git for version control.

#### Helper Function Pattern
```bash
#!/bin/bash
set -uo pipefail  # NOTE: Do NOT use -e, it causes silent failures with diff

TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Helper function to compare files and show diff on failure
assert_file_matches() {
  local expected="$1"
  local actual="$2"
  local description="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "❌ FAILED: $description"
    echo ""
    echo "If changes are intentional, update $expected"
    exit 1
  fi
}

# Usage in tests:
cd "$TEST_DIR" && "$REQVIRE_BIN" some-command > /dev/null 2>&1

assert_file_matches "${TEST_SCRIPT_DIR}/expected/01-after-command.md" \
  "$TEST_DIR/specifications/Requirements.md" \
  "File content after command does not match expected"

echo "✅ Test passed"
```

#### Generating Expected Files
When creating a new test or updating expected outputs:
```bash
# Set up test environment
cd /tmp && mkdir test-gen && cp -r tests/test-capability/* test-gen/
cd test-gen && git init && git add -A && git commit -m "init"

# Run commands and capture expected outputs
reqvire some-command
cp specifications/Requirements.md /path/to/tests/test-capability/expected/01-after-command.md
```

#### Why NOT to Use Inline Grep Checks
```bash
# ❌ BAD - Silent failure, unclear what's wrong
if ! grep -q "expected content" "$FILE"; then
  echo "FAILED"
  exit 1
fi

# ✅ GOOD - Shows exactly what differs
if ! diff -u "$EXPECTED" "$ACTUAL"; then
  echo "FAILED - see diff above"
  exit 1
fi
```

### JSON Output Validation
```bash
# Run command that produces JSON
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model-summary --json 2>&1)

# Validate JSON structure
echo "$OUTPUT" | jq . >/dev/null 2>&1
if ! echo "$OUTPUT" | jq 'has("files")' | grep -q true; then
  echo "❌ FAILED: JSON missing expected structure"
  exit 1
fi

# Check specific values
TOTAL=$(echo "$OUTPUT" | jq '.global_counters.total_elements')
if [ "$TOTAL" -ne 5 ]; then
  echo "❌ FAILED: Expected 5 elements, got $TOTAL"
  exit 1
fi
```

### File Modification Checks
```bash
# Make backup for comparison
mkdir -p "$TEST_DIR/backup"
cp -r "$TEST_DIR/specifications" "$TEST_DIR/backup/"

# Run command that modifies files
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" generate-diagrams 2>&1)

# Check files were modified
if cmp -s "$TEST_DIR/specifications/file.md" "$TEST_DIR/backup/specifications/file.md"; then
  echo "❌ FAILED: Expected file to be modified"
  exit 1
fi

# Check for specific content
if ! grep -q 'expected-content' "$TEST_DIR/specifications/file.md"; then
  echo "❌ FAILED: Missing expected content"
  exit 1
fi
```

### Filter Testing
```bash
# Test different filter combinations
FILTERS=(
  "--filter-type=requirement"
  "--filter-is-not-verified"
  "--filter-file=specifications/*.md"
  "--filter-name=.*auth.*"
  "--filter-content=SHALL"
)

for filter in "${FILTERS[@]}"; do
  OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model-summary $filter --json 2>&1)
  if [ $? -ne 0 ]; then
    echo "❌ FAILED: Filter failed: $filter"
    exit 1
  fi
done
```

### Error Condition Testing
```bash
# Test invalid regex
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" model-summary --filter-name="[invalid" 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "❌ FAILED: Expected error for invalid regex"
  exit 1
fi

if ! echo "$OUTPUT" | grep -q "Invalid regex"; then
  echo "❌ FAILED: Expected 'Invalid regex' error message"
  exit 1
fi
```

## Test Specifications

### Test Requirements Format
Create testable requirements in `specifications/Requirements.md`:
```markdown
### Test Capability Requirement

The system SHALL generate diagrams when requested.

#### Metadata
  * type: requirement

#### Relations
  * verifiedBy: Verifications/Tests.md#diagram-generation-test
```

### Test Verification Format
Create verification in `specifications/Verifications/Tests.md`:
```markdown
### Diagram Generation Test

Test verifies that diagrams are generated correctly:
1. Run generate-diagrams command
2. Verify mermaid diagrams are added to files
3. Confirm REQVIRE-AUTOGENERATED-DIAGRAM markers present

Expected: Files contain valid mermaid diagrams with proper markers

#### Metadata
  * type: test-verification

#### Relations
  * verify: ../Requirements.md#test-capability-requirement
```

## Testing Specific Commands

### Model Summary Tests
```bash
# Basic summary
"$REQVIRE_BIN" model-summary --json

# With filters
"$REQVIRE_BIN" model-summary --filter-type="requirement"
"$REQVIRE_BIN" model-summary --filter-is-not-verified
"$REQVIRE_BIN" model-summary --filter-file="specifications/*.md"
```

### Diagram Generation Tests
```bash
# Generate diagrams
"$REQVIRE_BIN" generate-diagrams

# Check for mermaid content
grep -q '```mermaid' specifications/file.md

# Verify autogenerated markers
grep -q 'REQVIRE-AUTOGENERATED-DIAGRAM' specifications/file.md
```

### Format Tests
```bash
# Preview changes (default dry-run mode)
"$REQVIRE_BIN" format

# Apply formatting
"$REQVIRE_BIN" format --fix
```

### Change Impact Tests
```bash
# Analyze changes
"$REQVIRE_BIN" change-impact --git-commit=HEAD~1 --json
```

### Traceability Tests
```bash
# Generate traces
"$REQVIRE_BIN" traces --json

# Validate trace structure
echo "$OUTPUT" | jq '.traces | length'
```

## Debugging Failed Tests

### View Test Output
```bash
# Test logs are saved to test_results.log
cat /path/to/test/directory/test_results.log
```

### Run Test Manually
```bash
# Navigate to test directory
cd tests/test-capability-name

# Set environment variables
export TEST_DIR=$(pwd)
export REQVIRE_BIN="../../target/debug/reqvire"

# Run test script
./test.sh
```

### Inspect Test Files
```bash
# Check test specifications
ls tests/test-capability-name/specifications/

# Review test logic
cat tests/test-capability-name/test.sh
```

## Best Practices

1. **Clear Test Names**: Use descriptive test directory names
2. **Focused Tests**: Each test should validate specific functionality
3. **Comprehensive Checks**: Validate both success and error conditions
4. **Deterministic Results**: Tests should produce consistent outputs
5. **Clean Specifications**: Use minimal test data that demonstrates capabilities
6. **Error Messages**: Provide clear failure messages with context
7. **Documentation**: Include acceptance criteria in test scripts
8. **Git Setup**: Tests run in temporary git repositories
9. **Cleanup**: Test runner handles cleanup automatically
10. **Fast Execution**: Keep tests efficient for CI/CD pipelines
11. **Silent Success**: Never add debug or echo outputs except for failure messages - tests should be silent on success

## Adding New Tests

1. Create test directory:
```bash
mkdir tests/test-new-capability
```

2. Create test script:
```bash
touch tests/test-new-capability/test.sh
chmod +x tests/test-new-capability/test.sh
```

3. Add test specifications:
```bash
mkdir -p tests/test-new-capability/specifications/Verifications
# Create markdown files with test requirements and verifications
```

4. Run test:
```bash
./tests/run_tests.sh test-new-capability
```

5. Add to test suite (automatic - any `test-*` directory is included)
