# ReqFlow Initialization End-to-End Tests

This directory contains end-to-end tests for the ReqFlow initialization feature.

## Tests Overview

These tests verify the functionality of the `--init` command, specifically:

1. **Successful Initialization**: Verifies that a new project can be initialized in an empty directory
2. **Existing YAML Configuration**: Verifies that initialization fails when reqflow.yaml exists
3. **Existing YML Configuration**: Verifies that initialization fails when reqflow.yml exists
4. **Dot ReqFlow Configuration**: Verifies that initialization succeeds even if .reqflow.yml exists

## Running the Tests

To run all tests:

```bash
./run_all_tests.sh
```

To run a specific test:

```bash
./test_init_success.sh
```

## Test Dependencies

- Bash shell
- ReqFlow binary (built with `cargo build`)
