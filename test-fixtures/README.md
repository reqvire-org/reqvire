# Test Fixtures for ReqFlow

This directory contains test fixtures used for automated testing of the ReqFlow application.

## Directories

- **test-lint-headers**: Tests for handling of level 3 (`###`) and level 4 (`####`) heading whitespace
- **test-lint-simple**: Simple requirements files for testing basic linting functionality
- **test-lint-mixed**: Mixed files with various linting issues
- **test-lint-reqs**: Test fixtures for requirements documents
- **test-lint-other**: Test fixtures for non-requirements documents

## Purpose

These fixtures are specifically designed to test:

1. **Requirements Detection**: Files that should or should not be identified as requirements documents
2. **Whitespace Linting**: Excess whitespace in headers and relation identifiers
3. **Newline Linting**: Proper blank lines before subsections
4. **Separator Linting**: Separator lines between elements
5. **Indentation Linting**: Consistent indentation in relation lists

## Usage

These fixtures are used by the automated tests in `src/tests/linting_tests.rs`. To run the linting tests:

```
cargo test -- --nocapture test_lint_directory_with_fixtures
```
