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

## Requirements Coverage

These tests verify the following requirements:

- **System Requirements/Initialization Command**
  - The system shall implement an `init` command that bootstraps a basic ReqFlow project structure with example requirements, folder hierarchy, and a configuration file.

- **System Requirements/Initialization Command Configuration Check**
  - The system shall prevent the initialization command from modifying an existing project by detecting if a configuration file already exists (specifically "reqflow.yaml" or "reqflow.yml") and report an error instead of proceeding. Other configuration file formats that may be valid for general use (such as ".reqflow.yml", ".reqflow.yaml", etc.) shall not prevent initialization.

## Test Dependencies

- Bash shell
- ReqFlow binary (built with `cargo build`)