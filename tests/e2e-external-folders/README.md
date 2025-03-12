# ReqFlow External Folders Tests

This directory contains end-to-end tests for ReqFlow's external folders functionality.

## Test Scope

These tests verify that the external folders functionality correctly processes requirements stored outside the main specifications directory:

1. **External Folder Processing:** Process requirements from multiple directories
2. **Validation Rules:** Enforce restrictions (e.g., no user requirements in external folders)
3. **Model Integration:** Include external requirements in diagrams and traceability

## Running Tests

To run all external folders tests:

```bash
./run_all_tests.sh
```

To run a specific test:

```bash
./test_external_folders.sh
```

## Test Fixtures

Test fixtures are located in the `/tests/fixtures/` directory:

- `test-external-folders/` - Contains external folders with invalid setup (user requirements in external folder)
- `test-external-folders-clean/` - Contains external folders with valid setup


