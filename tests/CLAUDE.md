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
test-feature-name/
├── test.sh                    # Test execution script (REQUIRED)
├── reqvire.yaml              # Configuration file (optional)
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

# Test: Feature Description
# Acceptance Criteria:
# - List specific behaviors to test
# - Define success conditions
#
# Test Criteria:
# - Command exits with success (0)
# - Output matches expected format
# - Files are modified as expected

# Optional: Create reqvire.yaml config
cat > "$TEST_DIR/reqvire.yaml" << EOF
paths:
  output_folder: "output"
  excluded_filename_patterns: []
style:
  theme: "default"
EOF

# Run reqvire command
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --config "$TEST_DIR/reqvire.yaml" command 2>&1)
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
### Test Feature Requirement

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
  * verify: ../Requirements.md#test-feature-requirement
```

## Configuration Files

### reqvire.yaml Template
```yaml
paths:
  output_folder: "output"
  excluded_filename_patterns: []
style:
  theme: "default"
  max_width: 1200
  diagram_direction: "LR"
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

### HTML Export Tests
```bash
# Generate HTML
"$REQVIRE_BIN" html

# Check output directory
test -d output/
test -f output/index.html
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
cd tests/test-feature-name

# Set environment variables
export TEST_DIR=$(pwd)
export REQVIRE_BIN="../../target/debug/reqvire"

# Run test script
./test.sh
```

### Inspect Test Files
```bash
# Check test specifications
ls tests/test-feature-name/specifications/

# Review test logic
cat tests/test-feature-name/test.sh
```

## Best Practices

1. **Clear Test Names**: Use descriptive test directory names
2. **Focused Tests**: Each test should validate specific functionality
3. **Comprehensive Checks**: Validate both success and error conditions
4. **Deterministic Results**: Tests should produce consistent outputs
5. **Clean Specifications**: Use minimal test data that demonstrates features
6. **Error Messages**: Provide clear failure messages with context
7. **Documentation**: Include acceptance criteria in test scripts
8. **Git Setup**: Tests run in temporary git repositories
9. **Cleanup**: Test runner handles cleanup automatically
10. **Fast Execution**: Keep tests efficient for CI/CD pipelines
11. **Silent Success**: Never add debug or echo outputs except for failure messages - tests should be silent on success

## Adding New Tests

1. Create test directory:
```bash
mkdir tests/test-new-feature
```

2. Create test script:
```bash
touch tests/test-new-feature/test.sh
chmod +x tests/test-new-feature/test.sh
```

3. Add test specifications:
```bash
mkdir -p tests/test-new-feature/specifications/Verifications
# Create markdown files with test requirements and verifications
```

4. Run test:
```bash
./tests/run_tests.sh test-new-feature
```

5. Add to test suite (automatic - any `test-*` directory is included)