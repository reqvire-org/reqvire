# ReqFlow Test Suite

This is the main test runner for ReqFlow's verification tests.

## Running All Tests

To run all tests in all test directories:

```bash
./run_all_tests.sh
```

This will:
1. Find all test directories starting with `e2e-`
2. For each directory, run the `run_all_tests.sh` script if it exists
3. If no `run_all_tests.sh` exists, run all test scripts that match `test_*.sh`
4. Report the overall test results

## Test Categories

The test suite includes the following categories of tests:

1. **Validation Tests** (`e2e-validation`): Tests for validation functionality, including relation validation
2. **Linting Tests** (`e2e-linting`): Tests for linting functionality, including whitespace and formatting fixes
3. **External Folders Tests** (`e2e-external-folders`): Tests for external folder support
4. **Initialization Tests** (`e2e-init`): Tests for the init command

## Adding New Tests

To add a new test category:
1. Create a new directory with the prefix `e2e-`
2. Add test scripts with the prefix `test_`
3. Optionally, add a `run_all_tests.sh` script to the directory

## Test Reporting

The test runner will report:
- The success/failure of each test suite
- The overall test success rate
- A non-zero exit code if any tests fail, for CI/CD integration