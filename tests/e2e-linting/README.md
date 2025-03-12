# ReqFlow Linting Tests

This directory contains end-to-end tests for ReqFlow's linting functionality.

## Test Scope

These tests verify that the linting functionality properly identifies and fixes various issues in markdown files:

1. **Whitespace Linting:** Fixes excess whitespace after headers
2. **Newlines Linting:** Adds proper newlines around headers
3. **Separators Linting:** Adds missing separators between elements
4. **Indentation Linting:** Fixes inconsistent indentation in relation lists
5. **Absolute Links Linting:** Converts absolute links to relative links
6. **Dry Run Mode:** Shows potential changes without applying them
7. **Git-Style Output:** Displays changes in a diff-like format

## Running Tests

To run all linting tests:

```bash
./run_all_tests.sh
```

To run a specific test:

```bash
./test_whitespace_linting.sh
```

## Test Fixtures

Test fixtures are located in the `/tests/fixtures/` directory. These fixtures contain intentionally problematic markdown files to verify that linting correctly identifies and fixes them.


